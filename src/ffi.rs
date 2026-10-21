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

//! C FFI bindings for [Pica].
//!
//! # Thread Safety
//!
//! A [`Pica`] instance is not internally synchronized. All `pica_*` calls on a
//! given instance must be made from a single thread or externally serialized.

#![allow(unsafe_code)]

use std::ffi::c_void;
use std::ptr;
use std::slice;

use crate::{Handle, MacAddress, Pica, RangingEstimator, RangingMeasurement};

/// Maximum serialized size in bytes of an individual UCI packet fragment (4-byte header + 1020-byte
/// data payload).
pub const MAX_PACKET_SIZE: usize =
    crate::packets::uci::HEADER_SIZE + crate::packets::uci::MAX_DATA_PACKET_PAYLOAD_SIZE;

/// Error returned by [`pica_pop_packet`] when an argument is null or invalid.
pub const ERR_INVALID_ARG: isize = -1;
/// Error returned by [`pica_pop_packet`] when `buf_len` is smaller than the next packet.
pub const ERR_BUFFER_TOO_SMALL: isize = -2;

/// C callback function for custom ranging estimation.
pub type RangingEstimatorFn = unsafe extern "C" fn(
    context: *mut c_void,
    local: Handle,
    peer: Handle,
    out_measurement: *mut RangingMeasurement,
) -> bool;

struct FfiRangingEstimator {
    callback: RangingEstimatorFn,
    context: *mut c_void,
}

// SAFETY: The callback and context are only invoked synchronously during `pica_*` calls on that
// `Pica` instance, access to which must be single-threaded or externally serialized.
unsafe impl Send for FfiRangingEstimator {}
unsafe impl Sync for FfiRangingEstimator {}

impl RangingEstimator for FfiRangingEstimator {
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement> {
        let mut measurement = RangingMeasurement::default();
        let success = unsafe { (self.callback)(self.context, *left, *right, &mut measurement) };
        success.then_some(measurement)
    }
}

/// Creates a new [`Pica`] emulation context with the given ranging estimator.
///
/// Returns null if `estimator` is `None`.
///
/// # Safety
///
/// If `estimator` is provided, `estimator` and `context` must remain valid for the lifetime of the
/// returned [`Pica`] instance. The callback is invoked synchronously during [`pica_tick`], must not
/// re-enter `pica_*` functions on the same [`Pica`] instance, and must not unwind across the FFI
/// boundary. The returned pointer must be freed with [`pica_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_new(
    estimator: Option<RangingEstimatorFn>,
    context: *mut c_void,
) -> *mut Pica {
    let Some(callback) = estimator else {
        return ptr::null_mut();
    };
    let ranging_estimator = Box::new(FfiRangingEstimator { callback, context });
    Box::into_raw(Box::new(Pica::new(ranging_estimator)))
}

/// Frees a [`Pica`] instance created by [`pica_new`].
///
/// # Safety
///
/// `pica` must be either null or a valid pointer obtained from [`pica_new`] that has not already
/// been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_free(pica: *mut Pica) {
    if !pica.is_null() {
        unsafe {
            drop(Box::from_raw(pica));
        }
    }
}

/// Connects a new UCI device and writes its allocated [`Handle`] to `*out_handle`.
///
/// Returns `false` if `pica` or `out_handle` is null.
///
/// # Safety
///
/// If non-null, `pica` must be a valid pointer to an initialized [`Pica`] instance,
/// and `out_handle` must be a valid, aligned, writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_add_device(pica: *mut Pica, out_handle: *mut Handle) -> bool {
    if pica.is_null() || out_handle.is_null() {
        return false;
    }
    unsafe {
        *out_handle = (*pica).add_device();
    }
    true
}

/// Disconnects and removes the UCI device with `handle`, returning `true` if it existed.
///
/// # Safety
///
/// `pica` must be either null or a valid pointer to an initialized [`Pica`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_remove_device(pica: *mut Pica, handle: Handle) -> bool {
    if pica.is_null() {
        return false;
    }
    unsafe { (*pica).remove_device(handle) }
}

/// Creates a virtual anchor with `mac_address` and writes its allocated [`Handle`] to
/// `*out_handle`.
///
/// Returns `false` if any pointer is null, `mac_address_len` is not 2 or 8, or an anchor with
/// `mac_address` already exists.
///
/// # Safety
///
/// If non-null, `pica` must be a valid pointer to an initialized [`Pica`] instance,
/// `mac_address` must point to `mac_address_len` readable bytes (2 for short, 8 for extended),
/// and `out_handle` must be a valid, aligned, writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_create_anchor(
    pica: *mut Pica,
    mac_address: *const u8,
    mac_address_len: usize,
    out_handle: *mut Handle,
) -> bool {
    if pica.is_null() || mac_address.is_null() || out_handle.is_null() {
        return false;
    }
    let mac = match mac_address_len {
        2 => MacAddress::Short(unsafe { *mac_address.cast::<[u8; 2]>() }),
        8 => MacAddress::Extended(unsafe { *mac_address.cast::<[u8; 8]>() }),
        _ => return false,
    };
    let pica = unsafe { &mut *pica };
    match pica.create_anchor(mac) {
        Ok(handle) => {
            unsafe {
                *out_handle = handle;
            }
            true
        }
        Err(_) => false,
    }
}

/// Removes the virtual anchor with `handle`, returning `true` if it existed.
///
/// # Safety
///
/// `pica` must be either null or a valid pointer to an initialized [`Pica`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_destroy_anchor(pica: *mut Pica, handle: Handle) -> bool {
    if pica.is_null() {
        return false;
    }
    unsafe { (*pica).destroy_anchor(handle) }
}

/// Feeds an incoming segmented UCI packet from `handle` into [`Pica`].
///
/// Ignored if `pica` or `data` is null, `data_len` is 0 or exceeds `isize::MAX as usize`,
/// or `handle` does not refer to an active connected device.
///
/// # Safety
///
/// If non-null, `pica` and `data` must be valid pointers, and `data` must point to at least
/// `data_len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_handle_packet(
    pica: *mut Pica,
    handle: Handle,
    data: *const u8,
    data_len: usize,
) {
    if pica.is_null() || data.is_null() || data_len == 0 || data_len > isize::MAX as usize {
        return;
    }
    let pica = unsafe { &mut *pica };
    let slice = unsafe { slice::from_raw_parts(data, data_len) };
    pica.handle_packet(handle, slice);
}

/// Returns the duration in milliseconds until [`pica_tick`] should next be called.
///
/// Returns `-1` if `pica` is null or no timers are active, `0` if due immediately, or
/// milliseconds rounded up.
///
/// # Safety
///
/// `pica` must be either null or a valid pointer to an initialized [`Pica`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_next_tick_ms(pica: *const Pica) -> i64 {
    if pica.is_null() {
        return -1;
    }
    let pica = unsafe { &*pica };
    match pica.next_tick() {
        None => -1,
        Some(d) => d
            .as_nanos()
            .div_ceil(1_000_000)
            .try_into()
            .unwrap_or(i64::MAX),
    }
}

/// Advances due ranging sessions.
///
/// # Safety
///
/// `pica` must be either null or a valid pointer to an initialized [`Pica`] instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_tick(pica: *mut Pica) {
    if !pica.is_null() {
        unsafe {
            (*pica).tick();
        }
    }
}

/// Pops the front outbound UCI packet from [`Pica`]'s transmission queue into `buf`.
///
/// Returns:
/// - `> 0`: number of bytes written to `buf` (and recipient written to `*out_handle`).
/// - `0`: the transmission queue is empty.
/// - [`ERR_INVALID_ARG`] (`-1`): `pica`, `out_handle`, or `buf` is null.
/// - [`ERR_BUFFER_TOO_SMALL`] (`-2`): `buf_len` is smaller than the next packet (packet is not
///   popped).
///
/// # Safety
///
/// If non-null, `pica`, `out_handle`, and `buf` must be valid pointers, and `buf` must point to a
/// writable buffer of at least `buf_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pica_pop_packet(
    pica: *mut Pica,
    out_handle: *mut Handle,
    buf: *mut u8,
    buf_len: usize,
) -> isize {
    if pica.is_null() || out_handle.is_null() || buf.is_null() {
        return ERR_INVALID_ARG;
    }
    let pica = unsafe { &mut *pica };
    let Some(&(handle, ref packet)) = pica.tx_queue.front() else {
        return 0;
    };
    if buf_len < packet.len() {
        return ERR_BUFFER_TOO_SMALL;
    }
    let len = packet.len() as isize;
    unsafe {
        *out_handle = handle;
        ptr::copy_nonoverlapping(packet.as_ptr(), buf, packet.len());
    }
    pica.tx_queue.pop_front();
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn test_estimator(
        _context: *mut c_void,
        _local: Handle,
        _peer: Handle,
        out_measurement: *mut RangingMeasurement,
    ) -> bool {
        if !out_measurement.is_null() {
            unsafe {
                *out_measurement = RangingMeasurement::default();
            }
        }
        true
    }

    #[test]
    fn full_lifecycle() {
        unsafe {
            let mut dev: Handle = 0;
            assert!(!pica_add_device(ptr::null_mut(), &mut dev));
            assert!(!pica_add_device(ptr::null_mut(), ptr::null_mut()));
            assert!(!pica_remove_device(ptr::null_mut(), 0));

            assert!(pica_new(None, ptr::null_mut()).is_null());

            let pica = pica_new(Some(test_estimator), ptr::null_mut());
            assert!(!pica.is_null());

            assert!(!pica_add_device(pica, ptr::null_mut()));
            assert!(pica_add_device(pica, &mut dev));

            let mac: [u8; 2] = [0x12, 0x34];
            let mut anchor_handle: Handle = 0;
            assert!(pica_create_anchor(
                pica,
                mac.as_ptr(),
                mac.len(),
                &mut anchor_handle
            ));

            let mut dup_handle: Handle = 0;
            assert!(!pica_create_anchor(
                pica,
                mac.as_ptr(),
                mac.len(),
                &mut dup_handle
            ));

            assert!(!pica_create_anchor(
                pica,
                mac.as_ptr(),
                mac.len(),
                ptr::null_mut()
            ));
            assert!(!pica_create_anchor(
                ptr::null_mut(),
                mac.as_ptr(),
                mac.len(),
                &mut anchor_handle
            ));
            assert!(!pica_create_anchor(
                pica,
                ptr::null(),
                mac.len(),
                &mut anchor_handle
            ));
            assert!(!pica_create_anchor(
                pica,
                mac.as_ptr(),
                3,
                &mut anchor_handle
            ));

            assert!(pica_destroy_anchor(pica, anchor_handle));
            assert!(!pica_destroy_anchor(pica, anchor_handle));
            assert!(!pica_destroy_anchor(ptr::null_mut(), anchor_handle));

            assert!(pica_remove_device(pica, dev));
            assert!(!pica_remove_device(pica, dev));

            pica_free(pica);
            pica_free(ptr::null_mut());
        }
    }

    #[test]
    fn packet_exchange() {
        unsafe {
            assert_eq!(MAX_PACKET_SIZE, 1024);

            let pica = pica_new(Some(test_estimator), ptr::null_mut());
            assert!(!pica.is_null());
            let mut dev: Handle = 0;
            assert!(pica_add_device(pica, &mut dev));

            let mut out_handle: Handle = 0;
            let mut out_data = [0u8; MAX_PACKET_SIZE];

            assert_eq!(
                pica_pop_packet(
                    ptr::null_mut(),
                    &mut out_handle,
                    out_data.as_mut_ptr(),
                    MAX_PACKET_SIZE
                ),
                ERR_INVALID_ARG
            );
            assert_eq!(
                pica_pop_packet(
                    pica,
                    ptr::null_mut(),
                    out_data.as_mut_ptr(),
                    MAX_PACKET_SIZE
                ),
                ERR_INVALID_ARG
            );
            assert_eq!(
                pica_pop_packet(pica, &mut out_handle, ptr::null_mut(), MAX_PACKET_SIZE),
                ERR_INVALID_ARG
            );

            assert_eq!(
                pica_pop_packet(pica, &mut out_handle, out_data.as_mut_ptr(), 2),
                ERR_BUFFER_TOO_SMALL
            );

            let len = pica_pop_packet(
                pica,
                &mut out_handle,
                out_data.as_mut_ptr(),
                MAX_PACKET_SIZE,
            );
            assert!(len > 0);
            assert_eq!(out_handle, dev);

            assert_eq!(
                pica_pop_packet(
                    pica,
                    &mut out_handle,
                    out_data.as_mut_ptr(),
                    MAX_PACKET_SIZE
                ),
                0
            );

            let packet_len = usize::try_from(len).unwrap();
            pica_handle_packet(ptr::null_mut(), dev, out_data.as_ptr(), packet_len);
            pica_handle_packet(pica, dev, ptr::null(), packet_len);
            pica_handle_packet(pica, dev, out_data.as_ptr(), 0);
            pica_handle_packet(pica, dev + 9999, out_data.as_ptr(), 4);
            assert_eq!(
                pica_pop_packet(
                    pica,
                    &mut out_handle,
                    out_data.as_mut_ptr(),
                    MAX_PACKET_SIZE,
                ),
                0
            );

            let get_device_info_cmd = [0x20, 0x02, 0x00, 0x00];
            pica_handle_packet(
                pica,
                dev,
                get_device_info_cmd.as_ptr(),
                get_device_info_cmd.len(),
            );

            let len = pica_pop_packet(
                pica,
                &mut out_handle,
                out_data.as_mut_ptr(),
                MAX_PACKET_SIZE,
            );
            assert!(len >= 4);
            assert_eq!(out_handle, dev);

            assert_eq!(
                pica_pop_packet(
                    pica,
                    &mut out_handle,
                    out_data.as_mut_ptr(),
                    MAX_PACKET_SIZE
                ),
                0
            );

            pica_free(pica);
        }
    }

    #[test]
    fn custom_ranging_estimator_and_idle_tick() {
        unsafe extern "C" fn custom_estimator(
            context: *mut c_void,
            _local: Handle,
            _peer: Handle,
            out_measurement: *mut RangingMeasurement,
        ) -> bool {
            let counter = unsafe { &mut *(context as *mut usize) };
            *counter += 1;
            if !out_measurement.is_null() {
                unsafe {
                    *out_measurement = RangingMeasurement {
                        range: 42,
                        azimuth: 10,
                        elevation: 5,
                    };
                }
            }
            true
        }

        unsafe {
            let mut call_count: usize = 0;
            let pica = pica_new(
                Some(custom_estimator),
                &mut call_count as *mut usize as *mut c_void,
            );
            assert!(!pica.is_null());

            let mut dev0: Handle = 0;
            let mut dev1: Handle = 0;
            assert!(pica_add_device(pica, &mut dev0));
            assert!(pica_add_device(pica, &mut dev1));

            let measurement = (*pica).ranging_estimator.estimate(&dev0, &dev1);
            assert_eq!(call_count, 1);
            assert_eq!(
                measurement,
                Some(RangingMeasurement {
                    range: 42,
                    azimuth: 10,
                    elevation: 5,
                })
            );

            assert_eq!(pica_next_tick_ms(pica), -1);
            assert_eq!(pica_next_tick_ms(ptr::null()), -1);
            pica_tick(pica);
            pica_tick(ptr::null_mut());

            pica_free(pica);
        }
    }
}
