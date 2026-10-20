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

#ifndef PICA_H_
#define PICA_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#if defined(_MSC_VER)
typedef ptrdiff_t ssize_t;
#else
#include <sys/types.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

// Maximum serialized size in bytes of an individual UCI packet fragment (4-byte header + 1020-byte
// data payload).
#define PICA_MAX_PACKET_SIZE 1024

// Error returned by pica_pop_packet when an argument is NULL or invalid.
#define PICA_ERR_INVALID_ARG (-1)
// Error returned by pica_pop_packet when buf_len is smaller than the next packet.
#define PICA_ERR_BUFFER_TOO_SMALL (-2)

// Thread safety: A Pica instance is not internally synchronized. All pica_*
// calls on a given instance must be made from a single thread or externally
// serialized.

// Opaque handle to a Pica emulation context.
typedef struct Pica Pica;

// Handle allocated for connected devices or virtual anchors.
typedef size_t PicaHandle;

// Ranging measurement produced by a ranging estimator.
typedef struct PicaRangingMeasurement {
    uint16_t range;     // Range in centimeters.
    int16_t azimuth;    // Azimuth angle in degrees [-180, 180].
    int8_t elevation;   // Elevation angle in degrees [-90, 90].
} PicaRangingMeasurement;

// Callback function for custom ranging estimation.
//
// The callback is invoked synchronously during pica_tick. It must not re-enter pica_* functions
// on the same Pica instance, and must not unwind (e.g. via C++ throw or longjmp) across the FFI
// boundary.
//
// Returns true if a valid measurement was computed and written to out_measurement.
// Returns false if the measurement could not be estimated (e.g. out of range).
typedef bool (*PicaRangingEstimatorFn)(
    void *context,
    PicaHandle local,
    PicaHandle peer,
    PicaRangingMeasurement *out_measurement
);

// Creates a new Pica emulation context with the given ranging estimator.
//
// estimator and context must remain valid for the lifetime of the returned Pica instance. The
// returned pointer must be freed with pica_free.
//
// Returns NULL if estimator is NULL.
Pica *pica_new(PicaRangingEstimatorFn estimator, void *context);

// Frees a Pica emulation context created by pica_new.
void pica_free(Pica *pica);

// Connects a new UCI device and writes its allocated handle to *out_handle.
//
// Returns false if pica or out_handle is NULL.
bool pica_add_device(Pica *pica, PicaHandle *out_handle);

// Disconnects and removes the UCI device with handle, returning true if it existed.
bool pica_remove_device(Pica *pica, PicaHandle handle);

// Creates a virtual anchor with the specified MAC address and writes its allocated handle to
// *out_handle.
//
// mac_address must point to mac_address_len bytes (2 for short, 8 for extended).
// Returns false if pica, mac_address, or out_handle is NULL, mac_address_len is not 2 or 8, or an
// anchor with mac_address already exists.
bool pica_create_anchor(
    Pica *pica,
    const uint8_t *mac_address,
    size_t mac_address_len,
    PicaHandle *out_handle
);

// Removes the virtual anchor with handle, returning true if it existed.
bool pica_destroy_anchor(Pica *pica, PicaHandle handle);

// Feeds an incoming segmented UCI packet from handle into Pica.
//
// Ignored if pica or data is NULL, data_len is 0, or handle does not refer to an active
// connected device.
void pica_handle_packet(Pica *pica, PicaHandle handle, const uint8_t *data, size_t data_len);

// Returns the duration until pica_tick should next be called in milliseconds.
//
// Returns -1 if pica is NULL or no timers are active, 0 if due immediately, or milliseconds
// rounded up.
int64_t pica_next_tick_ms(const Pica *pica);

// Advances due ranging sessions.
void pica_tick(Pica *pica);

// Pops the front outbound UCI packet from Pica's transmission queue into buf.
//
// Returns:
// - > 0: number of bytes written to buf (and recipient written to *out_handle).
// - 0: the transmission queue is empty.
// - PICA_ERR_INVALID_ARG (-1): pica, out_handle, or buf is NULL.
// - PICA_ERR_BUFFER_TOO_SMALL (-2): buf_len is smaller than the next packet (packet is not
//   popped).
ssize_t pica_pop_packet(
    Pica *pica,
    PicaHandle *out_handle,
    uint8_t *buf,
    size_t buf_len
);

#ifdef __cplusplus
}
#endif

#endif  // PICA_H_
