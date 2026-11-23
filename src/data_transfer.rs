// Copyright 2026 Google LLC
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

use bytes::Bytes;
use pdl_runtime::Packet;

use crate::device::Device;
use crate::packets::uci::{self, *};
use crate::session::Session;
use crate::{Handle, MacAddress, Pica, UciPacket};

pub struct DataTransfer {
    pub sequence_number: u16,
    pub dst_mac_address: MacAddress,
    pub payload: Bytes,
    pub tx_count: u8,
}

impl DataTransfer {
    fn new(
        sequence_number: u16,
        dst_mac_address: u64,
        mac_address_mode: MacAddressMode,
        payload: Bytes,
    ) -> Option<Self> {
        let dst_mac_address = match mac_address_mode {
            MacAddressMode::Mode0 => u16::try_from(dst_mac_address)
                .ok()
                .map(|addr| MacAddress::Short(addr.to_le_bytes()))?,
            MacAddressMode::Mode2 => MacAddress::Extended(dst_mac_address.to_le_bytes()),
            MacAddressMode::Mode1 => return None,
        };
        Some(Self {
            sequence_number,
            dst_mac_address,
            payload,
            tx_count: 0,
        })
    }
}

impl Session {
    pub fn is_session_data_transfer_status_ntf_enabled(&self) -> bool {
        self.app_config.session_data_transfer_status_ntf_config()
            != uci::SessionDataTransferStatusNtfConfig::Disable
    }

    pub fn supports_data_transfer(&self) -> bool {
        matches!(
            self.session_type,
            SessionType::FiraRangingAndInBandDataSession | SessionType::FiraDataTransferSession
        )
    }

    pub fn can_start_data_transfer(&self) -> bool {
        self.state == SessionState::SessionStateActive
            && self.supports_data_transfer()
            && self.app_config.can_start_data_transfer()
    }

    pub fn can_receive_data_transfer(&self, dst_mac_address: MacAddress) -> bool {
        self.state == SessionState::SessionStateActive
            && self.supports_data_transfer()
            && self.app_config.can_receive_data_transfer()
            && self.app_config.device_mac_address() == Some(dst_mac_address)
    }

    pub fn clear_data(&mut self) {
        self.pending_data.clear();
        self.data_transfer = None;
    }

    pub fn has_pending_data(&self) -> bool {
        !self.pending_data.is_empty()
    }

    pub fn data_message_snd(
        &mut self,
        data: DataPacket,
    ) -> Result<ControlPacket, pdl_runtime::EncodeError> {
        tracing::debug!("[{}] data_message_snd", self.device_handle);
        let session_token = self.id;
        let uci_sequence_number = self
            .pending_data
            .get(12..)
            .and_then(|bytes| bytes.first_chunk::<2>())
            .or_else(|| {
                data.payload
                    .get(12..)
                    .and_then(|bytes| bytes.first_chunk::<2>())
            })
            .copied()
            .map_or(0, u16::from_le_bytes);

        let status_ntf = |status| {
            SessionDataTransferStatusNtf {
                session_token,
                status,
                tx_count: 1,
                uci_sequence_number,
            }
            .try_into()
        };

        if !self.supports_data_transfer() {
            self.pending_data.clear();
            return status_ntf(
                DataTransferNtfStatusCode::UciDataTransferStatusSessionTypeNotSupported,
            );
        }

        if !self.can_start_data_transfer() {
            self.pending_data.clear();
            return status_ntf(DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected);
        }

        if self.data_transfer.is_some() {
            self.pending_data.clear();
            return status_ntf(
                DataTransferNtfStatusCode::UciDataTransferStatusErrorNoCreditAvailable,
            );
        }

        if data.payload.len() > MAX_DATA_PACKET_PAYLOAD_SIZE
            || (self.pending_data.len() + data.payload.len())
                .saturating_sub(DATA_MESSAGE_SND_HEADER_SIZE)
                > MAX_DATA_MESSAGE_SIZE
        {
            self.pending_data.clear();
            return status_ntf(DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat);
        }

        self.pending_data.extend_from_slice(&data.payload);
        if data.pbf == PacketBoundaryFlag::Complete {
            let raw_bytes = self.pending_data.split().to_vec();
            let data_packet = DataPacket {
                dpf: DataPacketFormat::DataSnd,
                pbf: PacketBoundaryFlag::Complete,
                mt: MessageType::Data,
                payload: raw_bytes,
            };
            let Ok(DataPacketChild::DataMessageSnd(data_msg_snd)) = data_packet.specialize() else {
                return status_ntf(DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat);
            };
            let Some(data_transfer) = DataTransfer::new(
                data_msg_snd.data_sequence_number,
                data_msg_snd.destination_address,
                self.app_config.mac_address_mode(),
                data_msg_snd.application_data.into(),
            ) else {
                return status_ntf(DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat);
            };
            self.data_transfer = Some(data_transfer);
        }

        let credit_availability = if self.data_transfer.is_some() {
            CreditAvailability::CreditNotAvailable
        } else {
            CreditAvailability::CreditAvailable
        };

        SessionDataCreditNtf {
            credit_availability,
            session_token,
        }
        .try_into()
    }

    pub fn step_data_transfer(
        &mut self,
        session_id: u32,
        peer_handle: Option<Handle>,
    ) -> (Option<(Handle, UciPacket)>, Vec<UciPacket>) {
        let mut peer_data_packet: Option<(Handle, UciPacket)> = None;
        let mut local_packets: Vec<UciPacket> = Vec::new();

        if let Some(data_transfer) = self.data_transfer.as_mut() {
            data_transfer.tx_count = data_transfer.tx_count.saturating_add(1);
            let tx_count = data_transfer.tx_count;
            let sequence_number = data_transfer.sequence_number;
            let payload = data_transfer.payload.clone();
            let repetition_count = self.app_config.data_repetition_count();
            let is_status_ntf_enabled = self.is_session_data_transfer_status_ntf_enabled();

            let (status, is_complete) = if let Some((peer_handle, source_address)) =
                peer_handle.zip(self.app_config.device_mac_address())
            {
                let packet = DataMessageRcv {
                    application_data: payload.into(),
                    data_sequence_number: sequence_number,
                    pbf: PacketBoundaryFlag::Complete,
                    session_handle: session_id,
                    source_address: source_address.into(),
                    status: uci::DataRcvStatusCode::UciStatusSuccess,
                }
                .encode_to_vec()
                .expect("failed to encode DataMessageRcv");

                peer_data_packet = Some((peer_handle, packet));

                if tx_count <= repetition_count {
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk,
                        false,
                    )
                } else {
                    (DataTransferNtfStatusCode::UciDataTransferStatusOk, true)
                }
            } else {
                (
                    DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransfer,
                    true,
                )
            };

            if is_complete {
                self.clear_data();
                local_packets.push(
                    SessionDataCreditNtf {
                        credit_availability: CreditAvailability::CreditAvailable,
                        session_token: session_id,
                    }
                    .encode_to_vec()
                    .expect("failed to encode SessionDataCreditNtf"),
                );
            }

            if is_status_ntf_enabled || is_complete {
                local_packets.push(
                    SessionDataTransferStatusNtf {
                        session_token: session_id,
                        uci_sequence_number: sequence_number,
                        status,
                        tx_count,
                    }
                    .encode_to_vec()
                    .expect("failed to encode SessionDataTransferStatusNtf"),
                );
            }
        }

        (peer_data_packet, local_packets)
    }
}

impl Device {
    pub fn can_receive_data_transfer(&self, session_id: u32, dst_mac_address: MacAddress) -> bool {
        self.session(session_id)
            .is_some_and(|session| session.can_receive_data_transfer(dst_mac_address))
    }

    pub fn data_message_snd(
        &mut self,
        data: DataPacket,
    ) -> Result<ControlPacket, pdl_runtime::EncodeError> {
        tracing::debug!("[{}] data_message_send", self.handle);
        let session_token = data
            .payload
            .first_chunk::<4>()
            .copied()
            .map_or(0, u32::from_le_bytes);

        let (status, uci_sequence_number) = match data.dpf {
            DataPacketFormat::DataSnd => {
                if let Some(session) = self.sessions.values_mut().find(|s| s.has_pending_data()) {
                    return session.data_message_snd(data);
                }

                if data.payload.len() < DATA_MESSAGE_SND_HEADER_SIZE {
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                        0,
                    )
                } else {
                    if let Some(session) = self.session_mut(session_token) {
                        return session.data_message_snd(data);
                    }
                    let uci_sequence_number = data
                        .payload
                        .get(12..)
                        .and_then(|s| s.first_chunk::<2>())
                        .copied()
                        .map_or(0, u16::from_le_bytes);
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected,
                        uci_sequence_number,
                    )
                }
            }
            DataPacketFormat::DataRcv => (
                DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                0,
            ),
        };

        SessionDataTransferStatusNtf {
            session_token,
            status,
            tx_count: 1,
            uci_sequence_number,
        }
        .try_into()
    }
}

impl Pica {
    pub fn resolve_data_transfer_peer(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Option<Handle> {
        let data_transfer = session.data_transfer.as_ref()?;
        if !session.can_start_data_transfer() {
            return None;
        }
        let dst_mac_address = data_transfer.dst_mac_address;
        self.devices
            .values()
            .find(|peer_entry| {
                peer_entry.device.handle != device_handle
                    && peer_entry
                        .device
                        .can_receive_data_transfer(session_id, dst_mac_address)
            })
            .map(|peer_entry| peer_entry.device.handle)
    }
}
