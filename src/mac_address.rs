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
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SHORT_MAC_ADDRESS_SIZE: usize = 2;
const SHORT_MAC_ADDRESS_STR_SIZE: usize = 2 * SHORT_MAC_ADDRESS_SIZE;

const EXTENDED_MAC_ADDRESS_SIZE: usize = 8;
const EXTENDED_MAC_ADDRESS_STR_SIZE: usize = 2 * EXTENDED_MAC_ADDRESS_SIZE;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MacAddress has the wrong format: 0")]
    MacAddressWrongFormat(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum MacAddress {
    Short([u8; SHORT_MAC_ADDRESS_SIZE]),
    Extended([u8; EXTENDED_MAC_ADDRESS_SIZE]),
}

impl MacAddress {
    pub fn new(mac_address: String) -> Result<Self, Error> {
        mac_address.try_into()
    }
}

impl From<usize> for MacAddress {
    fn from(device_handle: usize) -> Self {
        let handle = device_handle as u64;
        MacAddress::Extended(handle.to_be_bytes())
    }
}

impl From<MacAddress> for u64 {
    fn from(mac_address: MacAddress) -> Self {
        match mac_address {
            MacAddress::Short(addr) => u16::from_le_bytes(addr) as u64,
            MacAddress::Extended(addr) => u64::from_le_bytes(addr),
        }
    }
}

impl TryFrom<String> for MacAddress {
    type Error = Error;
    fn try_from(mac_address: String) -> std::result::Result<Self, Error> {
        let mac_address = mac_address.replace(':', "").replace("%3A", "");
        Ok(match mac_address.len() {
            SHORT_MAC_ADDRESS_STR_SIZE => MacAddress::Short(
                <[u8; SHORT_MAC_ADDRESS_SIZE]>::from_hex(mac_address)
                    .map_err(|err| Error::MacAddressWrongFormat(err.to_string()))?,
            ),
            EXTENDED_MAC_ADDRESS_STR_SIZE => MacAddress::Extended(
                <[u8; EXTENDED_MAC_ADDRESS_SIZE]>::from_hex(mac_address)
                    .map_err(|err| Error::MacAddressWrongFormat(err.to_string()))?,
            ),
            _ => return Err(Error::MacAddressWrongFormat(mac_address)),
        })
    }
}

impl From<&MacAddress> for String {
    fn from(mac_address: &MacAddress) -> Self {
        let to_string = |addr: &[u8]| -> String {
            let mac_address: Vec<_> = addr.iter().map(|byte| format!("{:02X}:", byte)).collect();
            let s = mac_address
                .iter()
                .flat_map(|byte| byte.chars())
                .collect::<String>();
            s.trim_end_matches(':').into()
        };
        match mac_address {
            MacAddress::Short(address) => to_string(address),
            MacAddress::Extended(address) => to_string(address),
        }
    }
}

impl From<MacAddress> for String {
    fn from(mac_address: MacAddress) -> Self {
        String::from(&mac_address)
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
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
            MacAddress::Extended([0xFF, 0x77, 0xAA, 0xDD, 0xEE, 0xBB, 0xCC, 0x10])
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
        let extended_mac_address = "00:FF:77:AA:DD:EE:CC:45";
        let short_mac_address = "00:FF";
        assert_eq!(
            format!("{}", MacAddress::new(extended_mac_address.into()).unwrap()),
            extended_mac_address
        );
        assert_eq!(
            format!("{}", MacAddress::new(short_mac_address.into()).unwrap()),
            short_mac_address
        );
        assert_eq!(extended_mac_address.to_string(), extended_mac_address);
    }

    #[test]
    fn test_short_mac_to_u64() {
        let short_mac = MacAddress::Short([0x01, 0x02]);
        let result: u64 = short_mac.into();
        let expected: u64 = 0x0201;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extended_mac_to_u64() {
        let extended_mac = MacAddress::Extended([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        let result: u64 = extended_mac.into();
        let expected: u64 = 0x0807060504030201;
        assert_eq!(result, expected);
    }
}
