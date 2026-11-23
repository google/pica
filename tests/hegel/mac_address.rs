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

use hegel::TestCase;
use hegel::generators as gs;
use pica::MacAddress;

fn format_mac_bytes(bytes: &[u8], sep: &str, uppercase: bool) -> String {
    bytes
        .iter()
        .map(|byte| {
            if uppercase {
                format!("{byte:02X}")
            } else {
                format!("{byte:02x}")
            }
        })
        .collect::<Vec<_>>()
        .join(sep)
}

#[hegel::test]
fn short_mac_round_trips_through_string(tc: TestCase) {
    let b0 = tc.draw(gs::integers::<u8>());
    let b1 = tc.draw(gs::integers::<u8>());
    let sep = tc.draw(gs::sampled_from(vec!["", ":", "%3A"]));
    let uppercase = tc.draw(gs::booleans());
    let formatted = format_mac_bytes(&[b0, b1], sep, uppercase);

    let parsed = MacAddress::try_from(formatted).expect("valid short MAC");
    assert_eq!(parsed, MacAddress::Short([b0, b1]));
    assert_eq!(Vec::<u8>::from(parsed), vec![b0, b1]);
    assert_eq!(String::from(parsed), format_mac_bytes(&[b0, b1], ":", true));
}

#[hegel::test]
fn extended_mac_round_trips_through_string(tc: TestCase) {
    let mut bytes = [0u8; 8];
    for slot in &mut bytes {
        *slot = tc.draw(gs::integers::<u8>());
    }
    let sep = tc.draw(gs::sampled_from(vec!["", ":", "%3A"]));
    let uppercase = tc.draw(gs::booleans());
    let formatted = format_mac_bytes(&bytes, sep, uppercase);

    let parsed = MacAddress::try_from(formatted).expect("valid extended MAC");
    assert_eq!(parsed, MacAddress::Extended(bytes));
    assert_eq!(Vec::<u8>::from(parsed), bytes.to_vec());
    assert_eq!(String::from(parsed), format_mac_bytes(&bytes, ":", true));
}
