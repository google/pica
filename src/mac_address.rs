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

use std::fmt::Display;

use hex::FromHex;
use thiserror::Error;

const SHORT_MAC_ADDRESS_SIZE: usize = 2;
const STRING_SHORT_MAC_ADDRESS_SIZE: usize = 2 * SHORT_MAC_ADDRESS_SIZE;

const EXTEND_MAC_ADDRESS_SIZE: usize = 8;
const STRING_EXTEND_MAC_ADDRESS_SIZE: usize = 2 * EXTEND_MAC_ADDRESS_SIZE;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MacAddress has the wrong format: 0")]
    MacAddressWrongFormat(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacAddress {
    Short([u8; SHORT_MAC_ADDRESS_SIZE]),
    Extend([u8; EXTEND_MAC_ADDRESS_SIZE]),
}

impl MacAddress {
    pub fn new(mac_address: String) -> Result<Self, Error> {
        let mac_address = mac_address.replace(':', "");
        let mac_address = mac_address.replace("%3A", "");
        let uwb_mac_address = match mac_address.len() {
            STRING_SHORT_MAC_ADDRESS_SIZE => MacAddress::Short(
                <[u8; SHORT_MAC_ADDRESS_SIZE]>::from_hex(mac_address)
                    .map_err(|err| Error::MacAddressWrongFormat(err.to_string()))?,
            ),
            STRING_EXTEND_MAC_ADDRESS_SIZE => MacAddress::Extend(
                <[u8; EXTEND_MAC_ADDRESS_SIZE]>::from_hex(mac_address)
                    .map_err(|err| Error::MacAddressWrongFormat(err.to_string()))?,
            ),
            _ => return Err(Error::MacAddressWrongFormat(mac_address)),
        };
        Ok(uwb_mac_address)
    }
}

impl From<usize> for MacAddress {
    fn from(device_handle: usize) -> Self {
        MacAddress::Extend(device_handle.to_be_bytes())
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_string = |addr: &[u8]| -> String {
            let mac_address: Vec<_> = addr.iter().map(|byte| format!("{:02X}:", byte)).collect();
            let s = mac_address
                .iter()
                .flat_map(|byte| byte.chars())
                .collect::<String>();
            s.trim_end_matches(':').into()
        };
        match *self {
            MacAddress::Short(address) => write!(f, "{}", to_string(&address)),
            MacAddress::Extend(address) => write!(f, "{}", to_string(&address)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_mac_address() {
        let valid_mac_address = "00:11";
        assert_eq!(
            MacAddress::new(valid_mac_address.into()).unwrap(),
            MacAddress::Short([0x00, 0x11])
        );
        let valid_mac_address = "FF:77:AA:DD:EE:BB:CC:10";
        assert_eq!(
            MacAddress::new(valid_mac_address.into()).unwrap(),
            MacAddress::Extend([0xFF, 0x77, 0xAA, 0xDD, 0xEE, 0xBB, 0xCC, 0x10])
        );
    }

    #[test]
    #[should_panic]
    fn invalid_mac_address_short() {
        let invalid_mac_address = "00:11:22";
        MacAddress::new(invalid_mac_address.into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_mac_address_extend() {
        let invalid_mac_address = "00:11:22:33:44:55:66";
        MacAddress::new(invalid_mac_address.into()).unwrap();
    }

    #[test]
    fn display_mac_address() {
        let extend_mac_address = "00:FF:77:AA:DD:EE:CC:45";
        let short_mac_address = "00:FF";
        assert_eq!(
            format!("{}", MacAddress::new(extend_mac_address.into()).unwrap()),
            extend_mac_address
        );
        assert_eq!(
            format!("{}", MacAddress::new(short_mac_address.into()).unwrap()),
            short_mac_address
        );
        assert_eq!(extend_mac_address.to_string(), extend_mac_address);
    }
}
