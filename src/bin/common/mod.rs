// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use parking_lot::Mutex;
use pica::Pica;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub enum DeviceEvent {
    Packet(pica::Handle, pica::UciPacket),
    Disconnected(pica::Handle),
}

pub fn spawn_device(
    handle: pica::Handle,
    socket: TcpStream,
    pcapng_dir: Option<&Path>,
    event_tx: &mpsc::UnboundedSender<DeviceEvent>,
) -> mpsc::UnboundedSender<pica::UciPacket> {
    let (mut read_half, mut write_half) = socket.into_split();

    let pcapng_file = pcapng_dir.and_then(|dir| {
        let full_path = dir.join(format!("device-{handle}.pcapng"));
        tracing::debug!("Recording pcapng to file {}", full_path.as_path().display());
        std::fs::File::create(full_path)
            .and_then(|mut file| {
                file.write_all(&pica::pcapng::HEADER)?;
                Ok((Arc::new(Mutex::new(file)), Instant::now()))
            })
            .inspect_err(|err| tracing::error!("Failed to create pcapng file: {err}"))
            .ok()
    });

    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<pica::UciPacket>();

    let event_tx_clone = event_tx.clone();
    let reader_pcapng = pcapng_file.clone();
    let reader_abort = tokio::spawn(async move {
        while let Some((packet, next_read_half)) = pica::packets::uci::read(read_half).await {
            read_half = next_read_half;
            if let Some((file, start_time)) = &reader_pcapng {
                let _ = pica::pcapng::Packet::new(&packet, start_time.elapsed())
                    .write_to(&mut *file.lock())
                    .inspect_err(|err| tracing::warn!("Failed to write pcapng packet: {err}"));
            }
            if event_tx_clone
                .send(DeviceEvent::Packet(handle, packet))
                .is_err()
            {
                return;
            }
        }
        let _ = event_tx_clone.send(DeviceEvent::Disconnected(handle));
    })
    .abort_handle();

    let writer_event_tx = event_tx.clone();
    tokio::spawn(async move {
        while let Some(packet) = out_rx.recv().await {
            if let Some((file, start_time)) = &pcapng_file {
                let _ = pica::pcapng::Packet::new(&packet, start_time.elapsed())
                    .write_to(&mut *file.lock())
                    .inspect_err(|err| tracing::warn!("Failed to write pcapng packet: {err}"));
            }
            let Ok(next_write_half) = pica::packets::uci::write(write_half, packet).await else {
                reader_abort.abort();
                let _ = writer_event_tx.send(DeviceEvent::Disconnected(handle));
                break;
            };
            write_half = next_write_half;
        }
    });

    out_tx
}

pub async fn wait_for_deadline(deadline: Option<Instant>) {
    match deadline {
        Some(d) => tokio::time::sleep_until(tokio::time::Instant::from_std(d)).await,
        None => std::future::pending::<()>().await,
    }
}

pub fn drain_packets(
    pica: &mut Pica,
    device_sinks: &HashMap<pica::Handle, mpsc::UnboundedSender<pica::UciPacket>>,
) {
    for (handle, packet) in pica.drain_packets() {
        if let Some(sink) = device_sinks.get(&handle) {
            let _ = sink.send(packet);
        }
    }
}
