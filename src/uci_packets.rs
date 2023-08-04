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
// @generated rust packets from uci_packets.pdl

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(missing_docs)]

use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::cell::Cell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use thiserror::Error;
type Result<T> = std::result::Result<T, Error>;
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("Got {actual:x}, expected {expected:x}")]
    InvalidFixedValue { expected: u64, actual: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError {
        obj: String,
        wanted: usize,
        got: usize,
    },
    #[error("array size ({array} bytes) is not a multiple of the element size ({element} bytes)")]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError {
        obj: String,
        field: String,
        value: u64,
        type_: String,
    },
    #[error("expected child {expected}, got {actual}")]
    InvalidChildError {
        expected: &'static str,
        actual: String,
    },
}
pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum PacketBoundaryFlag {
    Complete = 0x0,
    NotComplete = 0x1,
}
impl TryFrom<u8> for PacketBoundaryFlag {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(PacketBoundaryFlag::Complete),
            0x1 => Ok(PacketBoundaryFlag::NotComplete),
            _ => Err(value),
        }
    }
}
impl From<&PacketBoundaryFlag> for u8 {
    fn from(value: &PacketBoundaryFlag) -> Self {
        match value {
            PacketBoundaryFlag::Complete => 0x0,
            PacketBoundaryFlag::NotComplete => 0x1,
        }
    }
}
impl From<PacketBoundaryFlag> for u8 {
    fn from(value: PacketBoundaryFlag) -> Self {
        (&value).into()
    }
}
impl From<PacketBoundaryFlag> for i8 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for i16 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for i32 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for i64 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for u16 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for u32 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
impl From<PacketBoundaryFlag> for u64 {
    fn from(value: PacketBoundaryFlag) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum GroupId {
    Core = 0x0,
    SessionConfig = 0x1,
    SessionControl = 0x2,
    DataControl = 0x3,
    Test = 0xd,
    VendorReserved9 = 0x9,
    VendorReservedA = 0xa,
    VendorReservedB = 0xb,
    VendorAndroid = 0xc,
    VendorReservedE = 0xe,
    VendorReservedF = 0xf,
}
impl TryFrom<u8> for GroupId {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(GroupId::Core),
            0x1 => Ok(GroupId::SessionConfig),
            0x2 => Ok(GroupId::SessionControl),
            0x3 => Ok(GroupId::DataControl),
            0xd => Ok(GroupId::Test),
            0x9 => Ok(GroupId::VendorReserved9),
            0xa => Ok(GroupId::VendorReservedA),
            0xb => Ok(GroupId::VendorReservedB),
            0xc => Ok(GroupId::VendorAndroid),
            0xe => Ok(GroupId::VendorReservedE),
            0xf => Ok(GroupId::VendorReservedF),
            _ => Err(value),
        }
    }
}
impl From<&GroupId> for u8 {
    fn from(value: &GroupId) -> Self {
        match value {
            GroupId::Core => 0x0,
            GroupId::SessionConfig => 0x1,
            GroupId::SessionControl => 0x2,
            GroupId::DataControl => 0x3,
            GroupId::Test => 0xd,
            GroupId::VendorReserved9 => 0x9,
            GroupId::VendorReservedA => 0xa,
            GroupId::VendorReservedB => 0xb,
            GroupId::VendorAndroid => 0xc,
            GroupId::VendorReservedE => 0xe,
            GroupId::VendorReservedF => 0xf,
        }
    }
}
impl From<GroupId> for u8 {
    fn from(value: GroupId) -> Self {
        (&value).into()
    }
}
impl From<GroupId> for i8 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for i16 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for i32 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for i64 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for u16 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for u32 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupId> for u64 {
    fn from(value: GroupId) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum DataPacketFormat {
    DataSnd = 0x1,
    DataRcv = 0x2,
}
impl TryFrom<u8> for DataPacketFormat {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(DataPacketFormat::DataSnd),
            0x2 => Ok(DataPacketFormat::DataRcv),
            _ => Err(value),
        }
    }
}
impl From<&DataPacketFormat> for u8 {
    fn from(value: &DataPacketFormat) -> Self {
        match value {
            DataPacketFormat::DataSnd => 0x1,
            DataPacketFormat::DataRcv => 0x2,
        }
    }
}
impl From<DataPacketFormat> for u8 {
    fn from(value: DataPacketFormat) -> Self {
        (&value).into()
    }
}
impl From<DataPacketFormat> for i8 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for i16 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for i32 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for i64 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for u16 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for u32 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataPacketFormat> for u64 {
    fn from(value: DataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum GroupIdOrDataPacketFormat {
    Core = 0x0,
    SessionConfigOrDataSnd = 0x1,
    SessionControlOrDataRcv = 0x2,
    DataControl = 0x3,
    Test = 0xd,
    VendorReserved9 = 0x9,
    VendorReservedA = 0xa,
    VendorReservedB = 0xb,
    VendorAndroid = 0xc,
    VendorReservedE = 0xe,
    VendorReservedF = 0xf,
}
impl TryFrom<u8> for GroupIdOrDataPacketFormat {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(GroupIdOrDataPacketFormat::Core),
            0x1 => Ok(GroupIdOrDataPacketFormat::SessionConfigOrDataSnd),
            0x2 => Ok(GroupIdOrDataPacketFormat::SessionControlOrDataRcv),
            0x3 => Ok(GroupIdOrDataPacketFormat::DataControl),
            0xd => Ok(GroupIdOrDataPacketFormat::Test),
            0x9 => Ok(GroupIdOrDataPacketFormat::VendorReserved9),
            0xa => Ok(GroupIdOrDataPacketFormat::VendorReservedA),
            0xb => Ok(GroupIdOrDataPacketFormat::VendorReservedB),
            0xc => Ok(GroupIdOrDataPacketFormat::VendorAndroid),
            0xe => Ok(GroupIdOrDataPacketFormat::VendorReservedE),
            0xf => Ok(GroupIdOrDataPacketFormat::VendorReservedF),
            _ => Err(value),
        }
    }
}
impl From<&GroupIdOrDataPacketFormat> for u8 {
    fn from(value: &GroupIdOrDataPacketFormat) -> Self {
        match value {
            GroupIdOrDataPacketFormat::Core => 0x0,
            GroupIdOrDataPacketFormat::SessionConfigOrDataSnd => 0x1,
            GroupIdOrDataPacketFormat::SessionControlOrDataRcv => 0x2,
            GroupIdOrDataPacketFormat::DataControl => 0x3,
            GroupIdOrDataPacketFormat::Test => 0xd,
            GroupIdOrDataPacketFormat::VendorReserved9 => 0x9,
            GroupIdOrDataPacketFormat::VendorReservedA => 0xa,
            GroupIdOrDataPacketFormat::VendorReservedB => 0xb,
            GroupIdOrDataPacketFormat::VendorAndroid => 0xc,
            GroupIdOrDataPacketFormat::VendorReservedE => 0xe,
            GroupIdOrDataPacketFormat::VendorReservedF => 0xf,
        }
    }
}
impl From<GroupIdOrDataPacketFormat> for u8 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        (&value).into()
    }
}
impl From<GroupIdOrDataPacketFormat> for i8 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for i16 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for i32 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for i64 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for u16 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for u32 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
impl From<GroupIdOrDataPacketFormat> for u64 {
    fn from(value: GroupIdOrDataPacketFormat) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CoreOpCode {
    CoreDeviceReset = 0x0,
    CoreDeviceStatusNtf = 0x1,
    CoreDeviceInfo = 0x2,
    CoreGetCapsInfo = 0x3,
    CoreSetConfig = 0x4,
    CoreGetConfig = 0x5,
    CoreDeviceSuspend = 0x6,
    CoreGenericErrorNtf = 0x7,
    CoreQueryUwbsTimestamp = 0x8,
}
impl TryFrom<u8> for CoreOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CoreOpCode::CoreDeviceReset),
            0x1 => Ok(CoreOpCode::CoreDeviceStatusNtf),
            0x2 => Ok(CoreOpCode::CoreDeviceInfo),
            0x3 => Ok(CoreOpCode::CoreGetCapsInfo),
            0x4 => Ok(CoreOpCode::CoreSetConfig),
            0x5 => Ok(CoreOpCode::CoreGetConfig),
            0x6 => Ok(CoreOpCode::CoreDeviceSuspend),
            0x7 => Ok(CoreOpCode::CoreGenericErrorNtf),
            0x8 => Ok(CoreOpCode::CoreQueryUwbsTimestamp),
            _ => Err(value),
        }
    }
}
impl From<&CoreOpCode> for u8 {
    fn from(value: &CoreOpCode) -> Self {
        match value {
            CoreOpCode::CoreDeviceReset => 0x0,
            CoreOpCode::CoreDeviceStatusNtf => 0x1,
            CoreOpCode::CoreDeviceInfo => 0x2,
            CoreOpCode::CoreGetCapsInfo => 0x3,
            CoreOpCode::CoreSetConfig => 0x4,
            CoreOpCode::CoreGetConfig => 0x5,
            CoreOpCode::CoreDeviceSuspend => 0x6,
            CoreOpCode::CoreGenericErrorNtf => 0x7,
            CoreOpCode::CoreQueryUwbsTimestamp => 0x8,
        }
    }
}
impl From<CoreOpCode> for u8 {
    fn from(value: CoreOpCode) -> Self {
        (&value).into()
    }
}
impl From<CoreOpCode> for i8 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for i16 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for i32 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for i64 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for u16 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for u32 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<CoreOpCode> for u64 {
    fn from(value: CoreOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum SessionConfigOpCode {
    SessionInit = 0x0,
    SessionDeinit = 0x1,
    SessionStatusNtf = 0x2,
    SessionSetAppConfig = 0x3,
    SessionGetAppConfig = 0x4,
    SessionGetCount = 0x5,
    SessionGetState = 0x6,
    SessionUpdateControllerMulticastList = 0x7,
    SessionUpdateActiveRoundsAnchor = 0x8,
    SessionUpdateActiveRoundsDtTag = 0x9,
    SessionSetInitiatorDtAnchorRrRdmList = 0xa,
    SessionQueryDataSizeInRanging = 0xb,
    SessionSetHusConfig = 0xc,
}
impl TryFrom<u8> for SessionConfigOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SessionConfigOpCode::SessionInit),
            0x1 => Ok(SessionConfigOpCode::SessionDeinit),
            0x2 => Ok(SessionConfigOpCode::SessionStatusNtf),
            0x3 => Ok(SessionConfigOpCode::SessionSetAppConfig),
            0x4 => Ok(SessionConfigOpCode::SessionGetAppConfig),
            0x5 => Ok(SessionConfigOpCode::SessionGetCount),
            0x6 => Ok(SessionConfigOpCode::SessionGetState),
            0x7 => Ok(SessionConfigOpCode::SessionUpdateControllerMulticastList),
            0x8 => Ok(SessionConfigOpCode::SessionUpdateActiveRoundsAnchor),
            0x9 => Ok(SessionConfigOpCode::SessionUpdateActiveRoundsDtTag),
            0xa => Ok(SessionConfigOpCode::SessionSetInitiatorDtAnchorRrRdmList),
            0xb => Ok(SessionConfigOpCode::SessionQueryDataSizeInRanging),
            0xc => Ok(SessionConfigOpCode::SessionSetHusConfig),
            _ => Err(value),
        }
    }
}
impl From<&SessionConfigOpCode> for u8 {
    fn from(value: &SessionConfigOpCode) -> Self {
        match value {
            SessionConfigOpCode::SessionInit => 0x0,
            SessionConfigOpCode::SessionDeinit => 0x1,
            SessionConfigOpCode::SessionStatusNtf => 0x2,
            SessionConfigOpCode::SessionSetAppConfig => 0x3,
            SessionConfigOpCode::SessionGetAppConfig => 0x4,
            SessionConfigOpCode::SessionGetCount => 0x5,
            SessionConfigOpCode::SessionGetState => 0x6,
            SessionConfigOpCode::SessionUpdateControllerMulticastList => 0x7,
            SessionConfigOpCode::SessionUpdateActiveRoundsAnchor => 0x8,
            SessionConfigOpCode::SessionUpdateActiveRoundsDtTag => 0x9,
            SessionConfigOpCode::SessionSetInitiatorDtAnchorRrRdmList => 0xa,
            SessionConfigOpCode::SessionQueryDataSizeInRanging => 0xb,
            SessionConfigOpCode::SessionSetHusConfig => 0xc,
        }
    }
}
impl From<SessionConfigOpCode> for u8 {
    fn from(value: SessionConfigOpCode) -> Self {
        (&value).into()
    }
}
impl From<SessionConfigOpCode> for i8 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for i16 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for i32 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for i64 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for u16 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for u32 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionConfigOpCode> for u64 {
    fn from(value: SessionConfigOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum SessionControlOpCode {
    SessionStart = 0x0,
    SessionStop = 0x1,
    SessionReserved = 0x2,
    SessionGetRangingCount = 0x3,
    SessionDataCreditNtf = 0x4,
    SessionDataTransferStatusNtf = 0x5,
}
impl TryFrom<u8> for SessionControlOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SessionControlOpCode::SessionStart),
            0x1 => Ok(SessionControlOpCode::SessionStop),
            0x2 => Ok(SessionControlOpCode::SessionReserved),
            0x3 => Ok(SessionControlOpCode::SessionGetRangingCount),
            0x4 => Ok(SessionControlOpCode::SessionDataCreditNtf),
            0x5 => Ok(SessionControlOpCode::SessionDataTransferStatusNtf),
            _ => Err(value),
        }
    }
}
impl From<&SessionControlOpCode> for u8 {
    fn from(value: &SessionControlOpCode) -> Self {
        match value {
            SessionControlOpCode::SessionStart => 0x0,
            SessionControlOpCode::SessionStop => 0x1,
            SessionControlOpCode::SessionReserved => 0x2,
            SessionControlOpCode::SessionGetRangingCount => 0x3,
            SessionControlOpCode::SessionDataCreditNtf => 0x4,
            SessionControlOpCode::SessionDataTransferStatusNtf => 0x5,
        }
    }
}
impl From<SessionControlOpCode> for u8 {
    fn from(value: SessionControlOpCode) -> Self {
        (&value).into()
    }
}
impl From<SessionControlOpCode> for i8 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for i16 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for i32 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for i64 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for u16 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for u32 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionControlOpCode> for u64 {
    fn from(value: SessionControlOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum AppDataOpCode {
    AppDataTx = 0x0,
    AppDataRx = 0x1,
}
impl TryFrom<u8> for AppDataOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(AppDataOpCode::AppDataTx),
            0x1 => Ok(AppDataOpCode::AppDataRx),
            _ => Err(value),
        }
    }
}
impl From<&AppDataOpCode> for u8 {
    fn from(value: &AppDataOpCode) -> Self {
        match value {
            AppDataOpCode::AppDataTx => 0x0,
            AppDataOpCode::AppDataRx => 0x1,
        }
    }
}
impl From<AppDataOpCode> for u8 {
    fn from(value: AppDataOpCode) -> Self {
        (&value).into()
    }
}
impl From<AppDataOpCode> for i8 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for i16 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for i32 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for i64 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for u16 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for u32 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppDataOpCode> for u64 {
    fn from(value: AppDataOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum PicaOpCode {
    PicaInitDevice = 0x0,
    PicaSetDevicePosition = 0x1,
    PicaCreateBeacon = 0x2,
    PicaSetBeaconPosition = 0x3,
    PicaDestroyBeacon = 0x4,
}
impl TryFrom<u8> for PicaOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(PicaOpCode::PicaInitDevice),
            0x1 => Ok(PicaOpCode::PicaSetDevicePosition),
            0x2 => Ok(PicaOpCode::PicaCreateBeacon),
            0x3 => Ok(PicaOpCode::PicaSetBeaconPosition),
            0x4 => Ok(PicaOpCode::PicaDestroyBeacon),
            _ => Err(value),
        }
    }
}
impl From<&PicaOpCode> for u8 {
    fn from(value: &PicaOpCode) -> Self {
        match value {
            PicaOpCode::PicaInitDevice => 0x0,
            PicaOpCode::PicaSetDevicePosition => 0x1,
            PicaOpCode::PicaCreateBeacon => 0x2,
            PicaOpCode::PicaSetBeaconPosition => 0x3,
            PicaOpCode::PicaDestroyBeacon => 0x4,
        }
    }
}
impl From<PicaOpCode> for u8 {
    fn from(value: PicaOpCode) -> Self {
        (&value).into()
    }
}
impl From<PicaOpCode> for i8 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for i16 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for i32 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for i64 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for u16 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for u32 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<PicaOpCode> for u64 {
    fn from(value: PicaOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum AndroidOpCode {
    AndroidGetPowerStats = 0x0,
    AndroidSetCountryCode = 0x1,
    AndroidFiraRangeDiagnostics = 0x2,
}
impl TryFrom<u8> for AndroidOpCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(AndroidOpCode::AndroidGetPowerStats),
            0x1 => Ok(AndroidOpCode::AndroidSetCountryCode),
            0x2 => Ok(AndroidOpCode::AndroidFiraRangeDiagnostics),
            _ => Err(value),
        }
    }
}
impl From<&AndroidOpCode> for u8 {
    fn from(value: &AndroidOpCode) -> Self {
        match value {
            AndroidOpCode::AndroidGetPowerStats => 0x0,
            AndroidOpCode::AndroidSetCountryCode => 0x1,
            AndroidOpCode::AndroidFiraRangeDiagnostics => 0x2,
        }
    }
}
impl From<AndroidOpCode> for u8 {
    fn from(value: AndroidOpCode) -> Self {
        (&value).into()
    }
}
impl From<AndroidOpCode> for i8 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for i16 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for i32 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for i64 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for u16 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for u32 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<AndroidOpCode> for u64 {
    fn from(value: AndroidOpCode) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum StatusCode {
    UciStatusOk,
    UciStatusRejected,
    UciStatusFailed,
    UciStatusSyntaxError,
    UciStatusInvalidParam,
    UciStatusInvalidRange,
    UciStatusInvalidMsgSize,
    UciStatusUnknownGid,
    UciStatusUnknownOid,
    UciStatusReadOnly,
    UciStatusCommandRetry,
    UciStatusUnknown,
    UciStatusNotApplicable,
    RfuStatusCodeRange1(Private<u8>),
    UciStatusSessionNotExist,
    UciStatusSessionDuplicate,
    UciStatusSessionActive,
    UciStatusMaxSessionsExceeded,
    UciStatusSessionNotConfigured,
    UciStatusActiveSessionsOngoing,
    UciStatusMulticastListFull,
    UciStatusAddressNotFound,
    UciStatusAddressAlreadyPresent,
    UciStatusErrorUwbInitiationTimeTooOld,
    UciStatusOkNegativeDistanceReport,
    RfuStatusCodeRange2(Private<u8>),
    UciStatusRangingTxFailed,
    UciStatusRangingRxTimeout,
    UciStatusRangingRxPhyDecFailed,
    UciStatusRangingRxPhyToaFailed,
    UciStatusRangingRxPhyStsFailed,
    UciStatusRangingRxMacDecFailed,
    UciStatusRangingRxMacIeDecFailed,
    UciStatusRangingRxMacIeMissing,
    UciStatusErrorRoundIndexNotActivated,
    UciStatusErrorNumberOfActiveRangingRoundsExceeded,
    UciStatusErrorDlTdoaDeviceAddressNotMatchingInReplyTimeList,
    RfuStatusCodeRange3(Private<u8>),
    UciStatusDataMaxTxPsduSizeExceeded,
    UciStatusDataRxCrcError,
    RfuStatusCodeRange4(Private<u8>),
    UciStatusErrorCccSeBusy,
    UciStatusErrorCccLifecycle,
    UciStatusErrorStoppedDueToOtherSessionConflict,
    UciStatusRegulationUwbOff,
    VendorSpecificStatusCodeRange1(Private<u8>),
    VendorSpecificStatusCode2,
}
impl TryFrom<u8> for StatusCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(StatusCode::UciStatusOk),
            0x1 => Ok(StatusCode::UciStatusRejected),
            0x2 => Ok(StatusCode::UciStatusFailed),
            0x3 => Ok(StatusCode::UciStatusSyntaxError),
            0x4 => Ok(StatusCode::UciStatusInvalidParam),
            0x5 => Ok(StatusCode::UciStatusInvalidRange),
            0x6 => Ok(StatusCode::UciStatusInvalidMsgSize),
            0x7 => Ok(StatusCode::UciStatusUnknownGid),
            0x8 => Ok(StatusCode::UciStatusUnknownOid),
            0x9 => Ok(StatusCode::UciStatusReadOnly),
            0xa => Ok(StatusCode::UciStatusCommandRetry),
            0xb => Ok(StatusCode::UciStatusUnknown),
            0xc => Ok(StatusCode::UciStatusNotApplicable),
            0xd..=0x10 => Ok(StatusCode::RfuStatusCodeRange1(Private(value))),
            0x11 => Ok(StatusCode::UciStatusSessionNotExist),
            0x12 => Ok(StatusCode::UciStatusSessionDuplicate),
            0x13 => Ok(StatusCode::UciStatusSessionActive),
            0x14 => Ok(StatusCode::UciStatusMaxSessionsExceeded),
            0x15 => Ok(StatusCode::UciStatusSessionNotConfigured),
            0x16 => Ok(StatusCode::UciStatusActiveSessionsOngoing),
            0x17 => Ok(StatusCode::UciStatusMulticastListFull),
            0x18 => Ok(StatusCode::UciStatusAddressNotFound),
            0x19 => Ok(StatusCode::UciStatusAddressAlreadyPresent),
            0x1a => Ok(StatusCode::UciStatusErrorUwbInitiationTimeTooOld),
            0x1b => Ok(StatusCode::UciStatusOkNegativeDistanceReport),
            0x1c..=0x1f => Ok(StatusCode::RfuStatusCodeRange2(Private(value))),
            0x20 => Ok(StatusCode::UciStatusRangingTxFailed),
            0x21 => Ok(StatusCode::UciStatusRangingRxTimeout),
            0x22 => Ok(StatusCode::UciStatusRangingRxPhyDecFailed),
            0x23 => Ok(StatusCode::UciStatusRangingRxPhyToaFailed),
            0x24 => Ok(StatusCode::UciStatusRangingRxPhyStsFailed),
            0x25 => Ok(StatusCode::UciStatusRangingRxMacDecFailed),
            0x26 => Ok(StatusCode::UciStatusRangingRxMacIeDecFailed),
            0x27 => Ok(StatusCode::UciStatusRangingRxMacIeMissing),
            0x28 => Ok(StatusCode::UciStatusErrorRoundIndexNotActivated),
            0x29 => Ok(StatusCode::UciStatusErrorNumberOfActiveRangingRoundsExceeded),
            0x2a => Ok(StatusCode::UciStatusErrorDlTdoaDeviceAddressNotMatchingInReplyTimeList),
            0x2b..=0x2f => Ok(StatusCode::RfuStatusCodeRange3(Private(value))),
            0x30 => Ok(StatusCode::UciStatusDataMaxTxPsduSizeExceeded),
            0x31 => Ok(StatusCode::UciStatusDataRxCrcError),
            0x32..=0x4f => Ok(StatusCode::RfuStatusCodeRange4(Private(value))),
            0x50 => Ok(StatusCode::UciStatusErrorCccSeBusy),
            0x51 => Ok(StatusCode::UciStatusErrorCccLifecycle),
            0x52 => Ok(StatusCode::UciStatusErrorStoppedDueToOtherSessionConflict),
            0x53 => Ok(StatusCode::UciStatusRegulationUwbOff),
            0x50..=0xfe => Ok(StatusCode::VendorSpecificStatusCodeRange1(Private(value))),
            0xff => Ok(StatusCode::VendorSpecificStatusCode2),
        }
    }
}
impl From<&StatusCode> for u8 {
    fn from(value: &StatusCode) -> Self {
        match value {
            StatusCode::UciStatusOk => 0x0,
            StatusCode::UciStatusRejected => 0x1,
            StatusCode::UciStatusFailed => 0x2,
            StatusCode::UciStatusSyntaxError => 0x3,
            StatusCode::UciStatusInvalidParam => 0x4,
            StatusCode::UciStatusInvalidRange => 0x5,
            StatusCode::UciStatusInvalidMsgSize => 0x6,
            StatusCode::UciStatusUnknownGid => 0x7,
            StatusCode::UciStatusUnknownOid => 0x8,
            StatusCode::UciStatusReadOnly => 0x9,
            StatusCode::UciStatusCommandRetry => 0xa,
            StatusCode::UciStatusUnknown => 0xb,
            StatusCode::UciStatusNotApplicable => 0xc,
            StatusCode::RfuStatusCodeRange1(Private(value)) => *value,
            StatusCode::UciStatusSessionNotExist => 0x11,
            StatusCode::UciStatusSessionDuplicate => 0x12,
            StatusCode::UciStatusSessionActive => 0x13,
            StatusCode::UciStatusMaxSessionsExceeded => 0x14,
            StatusCode::UciStatusSessionNotConfigured => 0x15,
            StatusCode::UciStatusActiveSessionsOngoing => 0x16,
            StatusCode::UciStatusMulticastListFull => 0x17,
            StatusCode::UciStatusAddressNotFound => 0x18,
            StatusCode::UciStatusAddressAlreadyPresent => 0x19,
            StatusCode::UciStatusErrorUwbInitiationTimeTooOld => 0x1a,
            StatusCode::UciStatusOkNegativeDistanceReport => 0x1b,
            StatusCode::RfuStatusCodeRange2(Private(value)) => *value,
            StatusCode::UciStatusRangingTxFailed => 0x20,
            StatusCode::UciStatusRangingRxTimeout => 0x21,
            StatusCode::UciStatusRangingRxPhyDecFailed => 0x22,
            StatusCode::UciStatusRangingRxPhyToaFailed => 0x23,
            StatusCode::UciStatusRangingRxPhyStsFailed => 0x24,
            StatusCode::UciStatusRangingRxMacDecFailed => 0x25,
            StatusCode::UciStatusRangingRxMacIeDecFailed => 0x26,
            StatusCode::UciStatusRangingRxMacIeMissing => 0x27,
            StatusCode::UciStatusErrorRoundIndexNotActivated => 0x28,
            StatusCode::UciStatusErrorNumberOfActiveRangingRoundsExceeded => 0x29,
            StatusCode::UciStatusErrorDlTdoaDeviceAddressNotMatchingInReplyTimeList => 0x2a,
            StatusCode::RfuStatusCodeRange3(Private(value)) => *value,
            StatusCode::UciStatusDataMaxTxPsduSizeExceeded => 0x30,
            StatusCode::UciStatusDataRxCrcError => 0x31,
            StatusCode::RfuStatusCodeRange4(Private(value)) => *value,
            StatusCode::UciStatusErrorCccSeBusy => 0x50,
            StatusCode::UciStatusErrorCccLifecycle => 0x51,
            StatusCode::UciStatusErrorStoppedDueToOtherSessionConflict => 0x52,
            StatusCode::UciStatusRegulationUwbOff => 0x53,
            StatusCode::VendorSpecificStatusCodeRange1(Private(value)) => *value,
            StatusCode::VendorSpecificStatusCode2 => 0xff,
        }
    }
}
impl From<StatusCode> for u8 {
    fn from(value: StatusCode) -> Self {
        (&value).into()
    }
}
impl From<StatusCode> for i16 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<StatusCode> for i32 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<StatusCode> for i64 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<StatusCode> for u16 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<StatusCode> for u32 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<StatusCode> for u64 {
    fn from(value: StatusCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum DataRcvStatusCode {
    UciStatusSuccess = 0x0,
    UciStatusError = 0x1,
    UciStatusUnknown = 0x2,
}
impl TryFrom<u8> for DataRcvStatusCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(DataRcvStatusCode::UciStatusSuccess),
            0x1 => Ok(DataRcvStatusCode::UciStatusError),
            0x2 => Ok(DataRcvStatusCode::UciStatusUnknown),
            _ => Err(value),
        }
    }
}
impl From<&DataRcvStatusCode> for u8 {
    fn from(value: &DataRcvStatusCode) -> Self {
        match value {
            DataRcvStatusCode::UciStatusSuccess => 0x0,
            DataRcvStatusCode::UciStatusError => 0x1,
            DataRcvStatusCode::UciStatusUnknown => 0x2,
        }
    }
}
impl From<DataRcvStatusCode> for u8 {
    fn from(value: DataRcvStatusCode) -> Self {
        (&value).into()
    }
}
impl From<DataRcvStatusCode> for i16 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataRcvStatusCode> for i32 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataRcvStatusCode> for i64 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataRcvStatusCode> for u16 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataRcvStatusCode> for u32 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataRcvStatusCode> for u64 {
    fn from(value: DataRcvStatusCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CreditAvailability {
    CreditNotAvailable = 0x0,
    CreditAvailable = 0x1,
}
impl TryFrom<u8> for CreditAvailability {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CreditAvailability::CreditNotAvailable),
            0x1 => Ok(CreditAvailability::CreditAvailable),
            _ => Err(value),
        }
    }
}
impl From<&CreditAvailability> for u8 {
    fn from(value: &CreditAvailability) -> Self {
        match value {
            CreditAvailability::CreditNotAvailable => 0x0,
            CreditAvailability::CreditAvailable => 0x1,
        }
    }
}
impl From<CreditAvailability> for u8 {
    fn from(value: CreditAvailability) -> Self {
        (&value).into()
    }
}
impl From<CreditAvailability> for i16 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
impl From<CreditAvailability> for i32 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
impl From<CreditAvailability> for i64 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
impl From<CreditAvailability> for u16 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
impl From<CreditAvailability> for u32 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
impl From<CreditAvailability> for u64 {
    fn from(value: CreditAvailability) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum DataTransferNtfStatusCode {
    UciDataTransferStatusRepetitionOk = 0x0,
    UciDataTransferStatusOk = 0x1,
    UciDataTransferStatusErrorDataTransfer = 0x2,
    UciDataTransferStatusErrorNoCreditAvailable = 0x3,
    UciDataTransferStatusErrorRejected = 0x4,
    UciDataTransferStatusSessionTypeNotSupported = 0x5,
    UciDataTransferStatusErrorDataTransferIsOngoing = 0x6,
}
impl TryFrom<u8> for DataTransferNtfStatusCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk),
            0x1 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusOk),
            0x2 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransfer),
            0x3 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusErrorNoCreditAvailable),
            0x4 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected),
            0x5 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusSessionTypeNotSupported),
            0x6 => Ok(DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransferIsOngoing),
            _ => Err(value),
        }
    }
}
impl From<&DataTransferNtfStatusCode> for u8 {
    fn from(value: &DataTransferNtfStatusCode) -> Self {
        match value {
            DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk => 0x0,
            DataTransferNtfStatusCode::UciDataTransferStatusOk => 0x1,
            DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransfer => 0x2,
            DataTransferNtfStatusCode::UciDataTransferStatusErrorNoCreditAvailable => 0x3,
            DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected => 0x4,
            DataTransferNtfStatusCode::UciDataTransferStatusSessionTypeNotSupported => 0x5,
            DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransferIsOngoing => 0x6,
        }
    }
}
impl From<DataTransferNtfStatusCode> for u8 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        (&value).into()
    }
}
impl From<DataTransferNtfStatusCode> for i16 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataTransferNtfStatusCode> for i32 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataTransferNtfStatusCode> for i64 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataTransferNtfStatusCode> for u16 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataTransferNtfStatusCode> for u32 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<DataTransferNtfStatusCode> for u64 {
    fn from(value: DataTransferNtfStatusCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum ResetConfig {
    UwbsReset = 0x0,
}
impl TryFrom<u8> for ResetConfig {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(ResetConfig::UwbsReset),
            _ => Err(value),
        }
    }
}
impl From<&ResetConfig> for u8 {
    fn from(value: &ResetConfig) -> Self {
        match value {
            ResetConfig::UwbsReset => 0x0,
        }
    }
}
impl From<ResetConfig> for u8 {
    fn from(value: ResetConfig) -> Self {
        (&value).into()
    }
}
impl From<ResetConfig> for i16 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
impl From<ResetConfig> for i32 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
impl From<ResetConfig> for i64 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
impl From<ResetConfig> for u16 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
impl From<ResetConfig> for u32 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
impl From<ResetConfig> for u64 {
    fn from(value: ResetConfig) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum DeviceConfigId {
    DeviceState = 0x0,
    LowPowerMode = 0x1,
}
impl TryFrom<u8> for DeviceConfigId {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(DeviceConfigId::DeviceState),
            0x1 => Ok(DeviceConfigId::LowPowerMode),
            _ => Err(value),
        }
    }
}
impl From<&DeviceConfigId> for u8 {
    fn from(value: &DeviceConfigId) -> Self {
        match value {
            DeviceConfigId::DeviceState => 0x0,
            DeviceConfigId::LowPowerMode => 0x1,
        }
    }
}
impl From<DeviceConfigId> for u8 {
    fn from(value: DeviceConfigId) -> Self {
        (&value).into()
    }
}
impl From<DeviceConfigId> for i16 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceConfigId> for i32 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceConfigId> for i64 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceConfigId> for u16 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceConfigId> for u32 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceConfigId> for u64 {
    fn from(value: DeviceConfigId) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum AppConfigTlvType {
    DeviceType,
    RangingRoundUsage,
    StsConfig,
    MultiNodeMode,
    ChannelNumber,
    NoOfControlee,
    DeviceMacAddress,
    DstMacAddress,
    SlotDuration,
    RangingDuration,
    StsIndex,
    MacFcsType,
    RangingRoundControl,
    AoaResultReq,
    RngDataNtf,
    RngDataNtfProximityNear,
    RngDataNtfProximityFar,
    DeviceRole,
    RframeConfig,
    RssiReporting,
    PreambleCodeIndex,
    SfdId,
    PsduDataRate,
    PreambleDuration,
    LinkLayerMode,
    DataRepetitionCount,
    RangingTimeStruct,
    SlotsPerRr,
    TxAdaptivePayloadPower,
    RngDataNtfAoaBound,
    ResponderSlotIndex,
    PrfMode,
    CapSizeRange,
    ScheduledMode,
    KeyRotation,
    KeyRotationRate,
    SessionPriority,
    MacAddressMode,
    VendorId,
    StaticStsIv,
    NumberOfStsSegments,
    MaxRrRetry,
    UwbInitiationTime,
    HoppingMode,
    BlockStrideLength,
    ResultReportConfig,
    InBandTerminationAttemptCount,
    SubSessionId,
    BprfPhrDataRate,
    MaxNumberOfMeasurements,
    UlTdoaTxInterval,
    UlTdoaRandomWindow,
    StsLength,
    SuspendRangingRounds,
    UlTdoaNtfReportConfig,
    UlTdoaDeviceId,
    UlTdoaTxTimestamp,
    MinFramesPerRr,
    MtuSize,
    InterFrameInterval,
    RfuAppCfgTlvTypeRange1(Private<u8>),
    SessionKey,
    SubsessionKey,
    SessionDataTransferStatusNtfConfig,
    RfuAppCfgTlvTypeRange2(Private<u8>),
    CccHopModeKey,
    CccUwbTime0,
    CccRangingProtocolVer,
    CccUwbConfigId,
    CccPulseshapeCombo,
    CccUrskTtl,
    CccLastIndexUsed,
    VendorSpecificAppCfgTlvTypeRange1(Private<u8>),
    RfuAppCfgTlvTypeRange3(Private<u8>),
    NbOfRangeMeasurements,
    NbOfAzimuthMeasurements,
    NbOfElevationMeasurements,
    EnableDiagnostics,
    DiagramsFrameReportsFields,
    VendorSpecificAppCfgTlvTypeRange2(Private<u8>),
}
impl TryFrom<u8> for AppConfigTlvType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(AppConfigTlvType::DeviceType),
            0x1 => Ok(AppConfigTlvType::RangingRoundUsage),
            0x2 => Ok(AppConfigTlvType::StsConfig),
            0x3 => Ok(AppConfigTlvType::MultiNodeMode),
            0x4 => Ok(AppConfigTlvType::ChannelNumber),
            0x5 => Ok(AppConfigTlvType::NoOfControlee),
            0x6 => Ok(AppConfigTlvType::DeviceMacAddress),
            0x7 => Ok(AppConfigTlvType::DstMacAddress),
            0x8 => Ok(AppConfigTlvType::SlotDuration),
            0x9 => Ok(AppConfigTlvType::RangingDuration),
            0xa => Ok(AppConfigTlvType::StsIndex),
            0xb => Ok(AppConfigTlvType::MacFcsType),
            0xc => Ok(AppConfigTlvType::RangingRoundControl),
            0xd => Ok(AppConfigTlvType::AoaResultReq),
            0xe => Ok(AppConfigTlvType::RngDataNtf),
            0xf => Ok(AppConfigTlvType::RngDataNtfProximityNear),
            0x10 => Ok(AppConfigTlvType::RngDataNtfProximityFar),
            0x11 => Ok(AppConfigTlvType::DeviceRole),
            0x12 => Ok(AppConfigTlvType::RframeConfig),
            0x13 => Ok(AppConfigTlvType::RssiReporting),
            0x14 => Ok(AppConfigTlvType::PreambleCodeIndex),
            0x15 => Ok(AppConfigTlvType::SfdId),
            0x16 => Ok(AppConfigTlvType::PsduDataRate),
            0x17 => Ok(AppConfigTlvType::PreambleDuration),
            0x18 => Ok(AppConfigTlvType::LinkLayerMode),
            0x19 => Ok(AppConfigTlvType::DataRepetitionCount),
            0x1a => Ok(AppConfigTlvType::RangingTimeStruct),
            0x1b => Ok(AppConfigTlvType::SlotsPerRr),
            0x1c => Ok(AppConfigTlvType::TxAdaptivePayloadPower),
            0x1d => Ok(AppConfigTlvType::RngDataNtfAoaBound),
            0x1e => Ok(AppConfigTlvType::ResponderSlotIndex),
            0x1f => Ok(AppConfigTlvType::PrfMode),
            0x20 => Ok(AppConfigTlvType::CapSizeRange),
            0x22 => Ok(AppConfigTlvType::ScheduledMode),
            0x23 => Ok(AppConfigTlvType::KeyRotation),
            0x24 => Ok(AppConfigTlvType::KeyRotationRate),
            0x25 => Ok(AppConfigTlvType::SessionPriority),
            0x26 => Ok(AppConfigTlvType::MacAddressMode),
            0x27 => Ok(AppConfigTlvType::VendorId),
            0x28 => Ok(AppConfigTlvType::StaticStsIv),
            0x29 => Ok(AppConfigTlvType::NumberOfStsSegments),
            0x2a => Ok(AppConfigTlvType::MaxRrRetry),
            0x2b => Ok(AppConfigTlvType::UwbInitiationTime),
            0x2c => Ok(AppConfigTlvType::HoppingMode),
            0x2d => Ok(AppConfigTlvType::BlockStrideLength),
            0x2e => Ok(AppConfigTlvType::ResultReportConfig),
            0x2f => Ok(AppConfigTlvType::InBandTerminationAttemptCount),
            0x30 => Ok(AppConfigTlvType::SubSessionId),
            0x31 => Ok(AppConfigTlvType::BprfPhrDataRate),
            0x32 => Ok(AppConfigTlvType::MaxNumberOfMeasurements),
            0x33 => Ok(AppConfigTlvType::UlTdoaTxInterval),
            0x34 => Ok(AppConfigTlvType::UlTdoaRandomWindow),
            0x35 => Ok(AppConfigTlvType::StsLength),
            0x36 => Ok(AppConfigTlvType::SuspendRangingRounds),
            0x37 => Ok(AppConfigTlvType::UlTdoaNtfReportConfig),
            0x38 => Ok(AppConfigTlvType::UlTdoaDeviceId),
            0x39 => Ok(AppConfigTlvType::UlTdoaTxTimestamp),
            0x3a => Ok(AppConfigTlvType::MinFramesPerRr),
            0x3b => Ok(AppConfigTlvType::MtuSize),
            0x3c => Ok(AppConfigTlvType::InterFrameInterval),
            0x3d..=0x44 => Ok(AppConfigTlvType::RfuAppCfgTlvTypeRange1(Private(value))),
            0x45 => Ok(AppConfigTlvType::SessionKey),
            0x46 => Ok(AppConfigTlvType::SubsessionKey),
            0x47 => Ok(AppConfigTlvType::SessionDataTransferStatusNtfConfig),
            0x48..=0x9f => Ok(AppConfigTlvType::RfuAppCfgTlvTypeRange2(Private(value))),
            0xa0 => Ok(AppConfigTlvType::CccHopModeKey),
            0xa1 => Ok(AppConfigTlvType::CccUwbTime0),
            0xa3 => Ok(AppConfigTlvType::CccRangingProtocolVer),
            0xa4 => Ok(AppConfigTlvType::CccUwbConfigId),
            0xa5 => Ok(AppConfigTlvType::CccPulseshapeCombo),
            0xa6 => Ok(AppConfigTlvType::CccUrskTtl),
            0xa8 => Ok(AppConfigTlvType::CccLastIndexUsed),
            0xa0..=0xdf => Ok(AppConfigTlvType::VendorSpecificAppCfgTlvTypeRange1(
                Private(value),
            )),
            0xe0..=0xe2 => Ok(AppConfigTlvType::RfuAppCfgTlvTypeRange3(Private(value))),
            0xe3 => Ok(AppConfigTlvType::NbOfRangeMeasurements),
            0xe4 => Ok(AppConfigTlvType::NbOfAzimuthMeasurements),
            0xe5 => Ok(AppConfigTlvType::NbOfElevationMeasurements),
            0xe8 => Ok(AppConfigTlvType::EnableDiagnostics),
            0xe9 => Ok(AppConfigTlvType::DiagramsFrameReportsFields),
            0xe3..=0xff => Ok(AppConfigTlvType::VendorSpecificAppCfgTlvTypeRange2(
                Private(value),
            )),
            _ => Err(value),
        }
    }
}
impl From<&AppConfigTlvType> for u8 {
    fn from(value: &AppConfigTlvType) -> Self {
        match value {
            AppConfigTlvType::DeviceType => 0x0,
            AppConfigTlvType::RangingRoundUsage => 0x1,
            AppConfigTlvType::StsConfig => 0x2,
            AppConfigTlvType::MultiNodeMode => 0x3,
            AppConfigTlvType::ChannelNumber => 0x4,
            AppConfigTlvType::NoOfControlee => 0x5,
            AppConfigTlvType::DeviceMacAddress => 0x6,
            AppConfigTlvType::DstMacAddress => 0x7,
            AppConfigTlvType::SlotDuration => 0x8,
            AppConfigTlvType::RangingDuration => 0x9,
            AppConfigTlvType::StsIndex => 0xa,
            AppConfigTlvType::MacFcsType => 0xb,
            AppConfigTlvType::RangingRoundControl => 0xc,
            AppConfigTlvType::AoaResultReq => 0xd,
            AppConfigTlvType::RngDataNtf => 0xe,
            AppConfigTlvType::RngDataNtfProximityNear => 0xf,
            AppConfigTlvType::RngDataNtfProximityFar => 0x10,
            AppConfigTlvType::DeviceRole => 0x11,
            AppConfigTlvType::RframeConfig => 0x12,
            AppConfigTlvType::RssiReporting => 0x13,
            AppConfigTlvType::PreambleCodeIndex => 0x14,
            AppConfigTlvType::SfdId => 0x15,
            AppConfigTlvType::PsduDataRate => 0x16,
            AppConfigTlvType::PreambleDuration => 0x17,
            AppConfigTlvType::LinkLayerMode => 0x18,
            AppConfigTlvType::DataRepetitionCount => 0x19,
            AppConfigTlvType::RangingTimeStruct => 0x1a,
            AppConfigTlvType::SlotsPerRr => 0x1b,
            AppConfigTlvType::TxAdaptivePayloadPower => 0x1c,
            AppConfigTlvType::RngDataNtfAoaBound => 0x1d,
            AppConfigTlvType::ResponderSlotIndex => 0x1e,
            AppConfigTlvType::PrfMode => 0x1f,
            AppConfigTlvType::CapSizeRange => 0x20,
            AppConfigTlvType::ScheduledMode => 0x22,
            AppConfigTlvType::KeyRotation => 0x23,
            AppConfigTlvType::KeyRotationRate => 0x24,
            AppConfigTlvType::SessionPriority => 0x25,
            AppConfigTlvType::MacAddressMode => 0x26,
            AppConfigTlvType::VendorId => 0x27,
            AppConfigTlvType::StaticStsIv => 0x28,
            AppConfigTlvType::NumberOfStsSegments => 0x29,
            AppConfigTlvType::MaxRrRetry => 0x2a,
            AppConfigTlvType::UwbInitiationTime => 0x2b,
            AppConfigTlvType::HoppingMode => 0x2c,
            AppConfigTlvType::BlockStrideLength => 0x2d,
            AppConfigTlvType::ResultReportConfig => 0x2e,
            AppConfigTlvType::InBandTerminationAttemptCount => 0x2f,
            AppConfigTlvType::SubSessionId => 0x30,
            AppConfigTlvType::BprfPhrDataRate => 0x31,
            AppConfigTlvType::MaxNumberOfMeasurements => 0x32,
            AppConfigTlvType::UlTdoaTxInterval => 0x33,
            AppConfigTlvType::UlTdoaRandomWindow => 0x34,
            AppConfigTlvType::StsLength => 0x35,
            AppConfigTlvType::SuspendRangingRounds => 0x36,
            AppConfigTlvType::UlTdoaNtfReportConfig => 0x37,
            AppConfigTlvType::UlTdoaDeviceId => 0x38,
            AppConfigTlvType::UlTdoaTxTimestamp => 0x39,
            AppConfigTlvType::MinFramesPerRr => 0x3a,
            AppConfigTlvType::MtuSize => 0x3b,
            AppConfigTlvType::InterFrameInterval => 0x3c,
            AppConfigTlvType::RfuAppCfgTlvTypeRange1(Private(value)) => *value,
            AppConfigTlvType::SessionKey => 0x45,
            AppConfigTlvType::SubsessionKey => 0x46,
            AppConfigTlvType::SessionDataTransferStatusNtfConfig => 0x47,
            AppConfigTlvType::RfuAppCfgTlvTypeRange2(Private(value)) => *value,
            AppConfigTlvType::CccHopModeKey => 0xa0,
            AppConfigTlvType::CccUwbTime0 => 0xa1,
            AppConfigTlvType::CccRangingProtocolVer => 0xa3,
            AppConfigTlvType::CccUwbConfigId => 0xa4,
            AppConfigTlvType::CccPulseshapeCombo => 0xa5,
            AppConfigTlvType::CccUrskTtl => 0xa6,
            AppConfigTlvType::CccLastIndexUsed => 0xa8,
            AppConfigTlvType::VendorSpecificAppCfgTlvTypeRange1(Private(value)) => *value,
            AppConfigTlvType::RfuAppCfgTlvTypeRange3(Private(value)) => *value,
            AppConfigTlvType::NbOfRangeMeasurements => 0xe3,
            AppConfigTlvType::NbOfAzimuthMeasurements => 0xe4,
            AppConfigTlvType::NbOfElevationMeasurements => 0xe5,
            AppConfigTlvType::EnableDiagnostics => 0xe8,
            AppConfigTlvType::DiagramsFrameReportsFields => 0xe9,
            AppConfigTlvType::VendorSpecificAppCfgTlvTypeRange2(Private(value)) => *value,
        }
    }
}
impl From<AppConfigTlvType> for u8 {
    fn from(value: AppConfigTlvType) -> Self {
        (&value).into()
    }
}
impl From<AppConfigTlvType> for i16 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppConfigTlvType> for i32 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppConfigTlvType> for i64 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppConfigTlvType> for u16 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppConfigTlvType> for u32 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AppConfigTlvType> for u64 {
    fn from(value: AppConfigTlvType) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum FrameReportTlvType {
    Rssi = 0x0,
    Aoa = 0x1,
    Cir = 0x2,
}
impl TryFrom<u8> for FrameReportTlvType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FrameReportTlvType::Rssi),
            0x1 => Ok(FrameReportTlvType::Aoa),
            0x2 => Ok(FrameReportTlvType::Cir),
            _ => Err(value),
        }
    }
}
impl From<&FrameReportTlvType> for u8 {
    fn from(value: &FrameReportTlvType) -> Self {
        match value {
            FrameReportTlvType::Rssi => 0x0,
            FrameReportTlvType::Aoa => 0x1,
            FrameReportTlvType::Cir => 0x2,
        }
    }
}
impl From<FrameReportTlvType> for u8 {
    fn from(value: FrameReportTlvType) -> Self {
        (&value).into()
    }
}
impl From<FrameReportTlvType> for i16 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<FrameReportTlvType> for i32 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<FrameReportTlvType> for i64 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<FrameReportTlvType> for u16 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<FrameReportTlvType> for u32 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<FrameReportTlvType> for u64 {
    fn from(value: FrameReportTlvType) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CapTlvType {
    SupportedFiraPhyVersionRange,
    SupportedFiraMacVersionRange,
    SupportedDeviceRoles,
    SupportedRangingMethod,
    SupportedStsConfig,
    SupportedMultiNodeModes,
    SupportedRangingTimeStruct,
    SupportedScheduledMode,
    SupportedHoppingMode,
    SupportedBlockStriding,
    SupportedUwbInitiationTime,
    SupportedChannels,
    SupportedRframeConfig,
    SupportedCcConstraintLength,
    SupportedBprfParameterSets,
    SupportedHprfParameterSets,
    SupportedAoa,
    SupportedExtendedMacAddress,
    SupportedMaxMessageSize,
    SupportedMaxDataPacketPayloadSize,
    RfuCapTlvTypeRange1(Private<u8>),
    CccSupportedChapsPerSlot,
    CccSupportedSyncCodes,
    CccSupportedHoppingConfigModesAndSequences,
    CccSupportedChannels,
    CccSupportedVersions,
    CccSupportedUwbConfigs,
    CccSupportedPulseShapeCombos,
    CccSupportedRanMultiplier,
    CccSupportedMaxRangingSessionNumber,
    VendorSpecificCapTlvTypeRange1(Private<u8>),
    SupportedPowerStats,
    VendorSpecificCapTlvTypeRange2(Private<u8>),
    RfuCapTlvTypeRange2(Private<u8>),
    SupportedAoaResultReqAntennaInterleaving,
    SupportedMinRangingIntervalMs,
    SupportedRangeDataNtfConfig,
    SupportedRssiReporting,
    SupportedDiagnostics,
    SupportedMinSlotDurationRstu,
    SupportedMaxRangingSessionNumber,
    VendorSpecificCapTlvTypeRange3(Private<u8>),
}
impl TryFrom<u8> for CapTlvType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CapTlvType::SupportedFiraPhyVersionRange),
            0x1 => Ok(CapTlvType::SupportedFiraMacVersionRange),
            0x2 => Ok(CapTlvType::SupportedDeviceRoles),
            0x3 => Ok(CapTlvType::SupportedRangingMethod),
            0x4 => Ok(CapTlvType::SupportedStsConfig),
            0x5 => Ok(CapTlvType::SupportedMultiNodeModes),
            0x6 => Ok(CapTlvType::SupportedRangingTimeStruct),
            0x7 => Ok(CapTlvType::SupportedScheduledMode),
            0x8 => Ok(CapTlvType::SupportedHoppingMode),
            0x9 => Ok(CapTlvType::SupportedBlockStriding),
            0xa => Ok(CapTlvType::SupportedUwbInitiationTime),
            0xb => Ok(CapTlvType::SupportedChannels),
            0xc => Ok(CapTlvType::SupportedRframeConfig),
            0xd => Ok(CapTlvType::SupportedCcConstraintLength),
            0xe => Ok(CapTlvType::SupportedBprfParameterSets),
            0xf => Ok(CapTlvType::SupportedHprfParameterSets),
            0x10 => Ok(CapTlvType::SupportedAoa),
            0x11 => Ok(CapTlvType::SupportedExtendedMacAddress),
            0x12 => Ok(CapTlvType::SupportedMaxMessageSize),
            0x13 => Ok(CapTlvType::SupportedMaxDataPacketPayloadSize),
            0x14..=0x9f => Ok(CapTlvType::RfuCapTlvTypeRange1(Private(value))),
            0xa0 => Ok(CapTlvType::CccSupportedChapsPerSlot),
            0xa1 => Ok(CapTlvType::CccSupportedSyncCodes),
            0xa2 => Ok(CapTlvType::CccSupportedHoppingConfigModesAndSequences),
            0xa3 => Ok(CapTlvType::CccSupportedChannels),
            0xa4 => Ok(CapTlvType::CccSupportedVersions),
            0xa5 => Ok(CapTlvType::CccSupportedUwbConfigs),
            0xa6 => Ok(CapTlvType::CccSupportedPulseShapeCombos),
            0xa7 => Ok(CapTlvType::CccSupportedRanMultiplier),
            0xa8 => Ok(CapTlvType::CccSupportedMaxRangingSessionNumber),
            0xa0..=0xbf => Ok(CapTlvType::VendorSpecificCapTlvTypeRange1(Private(value))),
            0xc0 => Ok(CapTlvType::SupportedPowerStats),
            0xc1..=0xdf => Ok(CapTlvType::VendorSpecificCapTlvTypeRange2(Private(value))),
            0xe0..=0xe2 => Ok(CapTlvType::RfuCapTlvTypeRange2(Private(value))),
            0xe3 => Ok(CapTlvType::SupportedAoaResultReqAntennaInterleaving),
            0xe4 => Ok(CapTlvType::SupportedMinRangingIntervalMs),
            0xe5 => Ok(CapTlvType::SupportedRangeDataNtfConfig),
            0xe6 => Ok(CapTlvType::SupportedRssiReporting),
            0xe7 => Ok(CapTlvType::SupportedDiagnostics),
            0xe8 => Ok(CapTlvType::SupportedMinSlotDurationRstu),
            0xe9 => Ok(CapTlvType::SupportedMaxRangingSessionNumber),
            0xe3..=0xff => Ok(CapTlvType::VendorSpecificCapTlvTypeRange3(Private(value))),
        }
    }
}
impl From<&CapTlvType> for u8 {
    fn from(value: &CapTlvType) -> Self {
        match value {
            CapTlvType::SupportedFiraPhyVersionRange => 0x0,
            CapTlvType::SupportedFiraMacVersionRange => 0x1,
            CapTlvType::SupportedDeviceRoles => 0x2,
            CapTlvType::SupportedRangingMethod => 0x3,
            CapTlvType::SupportedStsConfig => 0x4,
            CapTlvType::SupportedMultiNodeModes => 0x5,
            CapTlvType::SupportedRangingTimeStruct => 0x6,
            CapTlvType::SupportedScheduledMode => 0x7,
            CapTlvType::SupportedHoppingMode => 0x8,
            CapTlvType::SupportedBlockStriding => 0x9,
            CapTlvType::SupportedUwbInitiationTime => 0xa,
            CapTlvType::SupportedChannels => 0xb,
            CapTlvType::SupportedRframeConfig => 0xc,
            CapTlvType::SupportedCcConstraintLength => 0xd,
            CapTlvType::SupportedBprfParameterSets => 0xe,
            CapTlvType::SupportedHprfParameterSets => 0xf,
            CapTlvType::SupportedAoa => 0x10,
            CapTlvType::SupportedExtendedMacAddress => 0x11,
            CapTlvType::SupportedMaxMessageSize => 0x12,
            CapTlvType::SupportedMaxDataPacketPayloadSize => 0x13,
            CapTlvType::RfuCapTlvTypeRange1(Private(value)) => *value,
            CapTlvType::CccSupportedChapsPerSlot => 0xa0,
            CapTlvType::CccSupportedSyncCodes => 0xa1,
            CapTlvType::CccSupportedHoppingConfigModesAndSequences => 0xa2,
            CapTlvType::CccSupportedChannels => 0xa3,
            CapTlvType::CccSupportedVersions => 0xa4,
            CapTlvType::CccSupportedUwbConfigs => 0xa5,
            CapTlvType::CccSupportedPulseShapeCombos => 0xa6,
            CapTlvType::CccSupportedRanMultiplier => 0xa7,
            CapTlvType::CccSupportedMaxRangingSessionNumber => 0xa8,
            CapTlvType::VendorSpecificCapTlvTypeRange1(Private(value)) => *value,
            CapTlvType::SupportedPowerStats => 0xc0,
            CapTlvType::VendorSpecificCapTlvTypeRange2(Private(value)) => *value,
            CapTlvType::RfuCapTlvTypeRange2(Private(value)) => *value,
            CapTlvType::SupportedAoaResultReqAntennaInterleaving => 0xe3,
            CapTlvType::SupportedMinRangingIntervalMs => 0xe4,
            CapTlvType::SupportedRangeDataNtfConfig => 0xe5,
            CapTlvType::SupportedRssiReporting => 0xe6,
            CapTlvType::SupportedDiagnostics => 0xe7,
            CapTlvType::SupportedMinSlotDurationRstu => 0xe8,
            CapTlvType::SupportedMaxRangingSessionNumber => 0xe9,
            CapTlvType::VendorSpecificCapTlvTypeRange3(Private(value)) => *value,
        }
    }
}
impl From<CapTlvType> for u8 {
    fn from(value: CapTlvType) -> Self {
        (&value).into()
    }
}
impl From<CapTlvType> for i16 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<CapTlvType> for i32 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<CapTlvType> for i64 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<CapTlvType> for u16 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<CapTlvType> for u32 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
impl From<CapTlvType> for u64 {
    fn from(value: CapTlvType) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum AoaResultReqType {
    AoaDisable = 0x0,
    AoaEnable = 0x1,
    AoaEnableAzimuth = 0x2,
    AoaEnableElevation = 0x3,
    AoaEnableInterleaved = 0xf0,
}
impl TryFrom<u8> for AoaResultReqType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(AoaResultReqType::AoaDisable),
            0x1 => Ok(AoaResultReqType::AoaEnable),
            0x2 => Ok(AoaResultReqType::AoaEnableAzimuth),
            0x3 => Ok(AoaResultReqType::AoaEnableElevation),
            0xf0 => Ok(AoaResultReqType::AoaEnableInterleaved),
            _ => Err(value),
        }
    }
}
impl From<&AoaResultReqType> for u8 {
    fn from(value: &AoaResultReqType) -> Self {
        match value {
            AoaResultReqType::AoaDisable => 0x0,
            AoaResultReqType::AoaEnable => 0x1,
            AoaResultReqType::AoaEnableAzimuth => 0x2,
            AoaResultReqType::AoaEnableElevation => 0x3,
            AoaResultReqType::AoaEnableInterleaved => 0xf0,
        }
    }
}
impl From<AoaResultReqType> for u8 {
    fn from(value: AoaResultReqType) -> Self {
        (&value).into()
    }
}
impl From<AoaResultReqType> for i16 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AoaResultReqType> for i32 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AoaResultReqType> for i64 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AoaResultReqType> for u16 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AoaResultReqType> for u32 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
impl From<AoaResultReqType> for u64 {
    fn from(value: AoaResultReqType) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum DeviceState {
    DeviceStateReady = 0x1,
    DeviceStateActive = 0x2,
    DeviceStateError = 0xff,
}
impl TryFrom<u8> for DeviceState {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(DeviceState::DeviceStateReady),
            0x2 => Ok(DeviceState::DeviceStateActive),
            0xff => Ok(DeviceState::DeviceStateError),
            _ => Err(value),
        }
    }
}
impl From<&DeviceState> for u8 {
    fn from(value: &DeviceState) -> Self {
        match value {
            DeviceState::DeviceStateReady => 0x1,
            DeviceState::DeviceStateActive => 0x2,
            DeviceState::DeviceStateError => 0xff,
        }
    }
}
impl From<DeviceState> for u8 {
    fn from(value: DeviceState) -> Self {
        (&value).into()
    }
}
impl From<DeviceState> for i16 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceState> for i32 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceState> for i64 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceState> for u16 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceState> for u32 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
impl From<DeviceState> for u64 {
    fn from(value: DeviceState) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum SessionState {
    SessionStateInit = 0x0,
    SessionStateDeinit = 0x1,
    SessionStateActive = 0x2,
    SessionStateIdle = 0x3,
}
impl TryFrom<u8> for SessionState {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SessionState::SessionStateInit),
            0x1 => Ok(SessionState::SessionStateDeinit),
            0x2 => Ok(SessionState::SessionStateActive),
            0x3 => Ok(SessionState::SessionStateIdle),
            _ => Err(value),
        }
    }
}
impl From<&SessionState> for u8 {
    fn from(value: &SessionState) -> Self {
        match value {
            SessionState::SessionStateInit => 0x0,
            SessionState::SessionStateDeinit => 0x1,
            SessionState::SessionStateActive => 0x2,
            SessionState::SessionStateIdle => 0x3,
        }
    }
}
impl From<SessionState> for u8 {
    fn from(value: SessionState) -> Self {
        (&value).into()
    }
}
impl From<SessionState> for i16 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionState> for i32 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionState> for i64 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionState> for u16 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionState> for u32 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionState> for u64 {
    fn from(value: SessionState) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum ReasonCode {
    StateChangeWithSessionManagementCommands,
    MaxRangingRoundRetryCountReached,
    MaxNumberOfMeasurementsReached,
    SessionSuspendedDueToInbandSignal,
    SessionResumedDueToInbandSignal,
    SessionStoppedDueToInbandSignal,
    RfuReasonCodeRange1(Private<u8>),
    ErrorInvalidUlTdoaRandomWindow,
    ErrorMinRframesPerRrNotSupported,
    ErrorTxDelayNotSupported,
    ErrorSlotLengthNotSupported,
    ErrorInsufficientSlotsPerRr,
    ErrorMacAddressModeNotSupported,
    ErrorInvalidRangingDuration,
    ErrorInvalidStsConfig,
    ErrorInvalidRframeConfig,
    ErrorHusNotEnoughSlots,
    ErrorHusCfpPhaseTooShort,
    ErrorHusCapPhaseTooShort,
    ErrorHusOthers,
    ErrorStatusSessionKeyNotFound,
    ErrorStatusSubSessionKeyNotFound,
    ErrorInvalidPreambleCodeIndex,
    ErrorInvalidSfdId,
    ErrorInvalidPsduDataRate,
    ErrorInvalidPhrDataRate,
    ErrorInvalidPreambleDuration,
    ErrorInvalidStsLength,
    ErrorInvalidNumOfStsSegments,
    ErrorInvalidNumOfControlees,
    ErrorMaxRangingReplyTimeExceeded,
    ErrorInvalidDstAddressList,
    ErrorInvalidOrNotFoundSubSessionId,
    ErrorInvalidResultReportConfig,
    ErrorInvalidRangingRoundControlConfig,
    ErrorInvalidRangingRoundUsage,
    ErrorInvalidMultiNodeMode,
    ErrorRdsFetchFailure,
    ErrorRefUwbSessionDoesNotExist,
    ErrorRefUwbSessionRangingDurationMismatch,
    ErrorRefUwbSessionInvalidOffsetTime,
    ErrorRefUwbSessionLost,
    ErrorDtAnchorRangingRoundsNotConfigured,
    ErrorDtTagRangingRoundsNotConfigured,
    RfuReasonCodeRange2(Private<u8>),
    ErrorInvalidChannelWithAoa,
    ErrorStoppedDueToOtherSessionConflict,
    VendorSpecificReasonCodeRange1(Private<u8>),
    VendorSpecificReasonCode2,
}
impl TryFrom<u8> for ReasonCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(ReasonCode::StateChangeWithSessionManagementCommands),
            0x1 => Ok(ReasonCode::MaxRangingRoundRetryCountReached),
            0x2 => Ok(ReasonCode::MaxNumberOfMeasurementsReached),
            0x3 => Ok(ReasonCode::SessionSuspendedDueToInbandSignal),
            0x4 => Ok(ReasonCode::SessionResumedDueToInbandSignal),
            0x5 => Ok(ReasonCode::SessionStoppedDueToInbandSignal),
            0x6..=0x1c => Ok(ReasonCode::RfuReasonCodeRange1(Private(value))),
            0x1d => Ok(ReasonCode::ErrorInvalidUlTdoaRandomWindow),
            0x1e => Ok(ReasonCode::ErrorMinRframesPerRrNotSupported),
            0x1f => Ok(ReasonCode::ErrorTxDelayNotSupported),
            0x20 => Ok(ReasonCode::ErrorSlotLengthNotSupported),
            0x21 => Ok(ReasonCode::ErrorInsufficientSlotsPerRr),
            0x22 => Ok(ReasonCode::ErrorMacAddressModeNotSupported),
            0x23 => Ok(ReasonCode::ErrorInvalidRangingDuration),
            0x24 => Ok(ReasonCode::ErrorInvalidStsConfig),
            0x25 => Ok(ReasonCode::ErrorInvalidRframeConfig),
            0x26 => Ok(ReasonCode::ErrorHusNotEnoughSlots),
            0x27 => Ok(ReasonCode::ErrorHusCfpPhaseTooShort),
            0x28 => Ok(ReasonCode::ErrorHusCapPhaseTooShort),
            0x29 => Ok(ReasonCode::ErrorHusOthers),
            0x2a => Ok(ReasonCode::ErrorStatusSessionKeyNotFound),
            0x2b => Ok(ReasonCode::ErrorStatusSubSessionKeyNotFound),
            0x2c => Ok(ReasonCode::ErrorInvalidPreambleCodeIndex),
            0x2d => Ok(ReasonCode::ErrorInvalidSfdId),
            0x2e => Ok(ReasonCode::ErrorInvalidPsduDataRate),
            0x2f => Ok(ReasonCode::ErrorInvalidPhrDataRate),
            0x30 => Ok(ReasonCode::ErrorInvalidPreambleDuration),
            0x31 => Ok(ReasonCode::ErrorInvalidStsLength),
            0x32 => Ok(ReasonCode::ErrorInvalidNumOfStsSegments),
            0x33 => Ok(ReasonCode::ErrorInvalidNumOfControlees),
            0x34 => Ok(ReasonCode::ErrorMaxRangingReplyTimeExceeded),
            0x35 => Ok(ReasonCode::ErrorInvalidDstAddressList),
            0x36 => Ok(ReasonCode::ErrorInvalidOrNotFoundSubSessionId),
            0x37 => Ok(ReasonCode::ErrorInvalidResultReportConfig),
            0x38 => Ok(ReasonCode::ErrorInvalidRangingRoundControlConfig),
            0x39 => Ok(ReasonCode::ErrorInvalidRangingRoundUsage),
            0x3a => Ok(ReasonCode::ErrorInvalidMultiNodeMode),
            0x3b => Ok(ReasonCode::ErrorRdsFetchFailure),
            0x3c => Ok(ReasonCode::ErrorRefUwbSessionDoesNotExist),
            0x3d => Ok(ReasonCode::ErrorRefUwbSessionRangingDurationMismatch),
            0x3e => Ok(ReasonCode::ErrorRefUwbSessionInvalidOffsetTime),
            0x3f => Ok(ReasonCode::ErrorRefUwbSessionLost),
            0x40 => Ok(ReasonCode::ErrorDtAnchorRangingRoundsNotConfigured),
            0x41 => Ok(ReasonCode::ErrorDtTagRangingRoundsNotConfigured),
            0x40..=0x7f => Ok(ReasonCode::RfuReasonCodeRange2(Private(value))),
            0x80 => Ok(ReasonCode::ErrorInvalidChannelWithAoa),
            0x81 => Ok(ReasonCode::ErrorStoppedDueToOtherSessionConflict),
            0x80..=0xfe => Ok(ReasonCode::VendorSpecificReasonCodeRange1(Private(value))),
            0xff => Ok(ReasonCode::VendorSpecificReasonCode2),
        }
    }
}
impl From<&ReasonCode> for u8 {
    fn from(value: &ReasonCode) -> Self {
        match value {
            ReasonCode::StateChangeWithSessionManagementCommands => 0x0,
            ReasonCode::MaxRangingRoundRetryCountReached => 0x1,
            ReasonCode::MaxNumberOfMeasurementsReached => 0x2,
            ReasonCode::SessionSuspendedDueToInbandSignal => 0x3,
            ReasonCode::SessionResumedDueToInbandSignal => 0x4,
            ReasonCode::SessionStoppedDueToInbandSignal => 0x5,
            ReasonCode::RfuReasonCodeRange1(Private(value)) => *value,
            ReasonCode::ErrorInvalidUlTdoaRandomWindow => 0x1d,
            ReasonCode::ErrorMinRframesPerRrNotSupported => 0x1e,
            ReasonCode::ErrorTxDelayNotSupported => 0x1f,
            ReasonCode::ErrorSlotLengthNotSupported => 0x20,
            ReasonCode::ErrorInsufficientSlotsPerRr => 0x21,
            ReasonCode::ErrorMacAddressModeNotSupported => 0x22,
            ReasonCode::ErrorInvalidRangingDuration => 0x23,
            ReasonCode::ErrorInvalidStsConfig => 0x24,
            ReasonCode::ErrorInvalidRframeConfig => 0x25,
            ReasonCode::ErrorHusNotEnoughSlots => 0x26,
            ReasonCode::ErrorHusCfpPhaseTooShort => 0x27,
            ReasonCode::ErrorHusCapPhaseTooShort => 0x28,
            ReasonCode::ErrorHusOthers => 0x29,
            ReasonCode::ErrorStatusSessionKeyNotFound => 0x2a,
            ReasonCode::ErrorStatusSubSessionKeyNotFound => 0x2b,
            ReasonCode::ErrorInvalidPreambleCodeIndex => 0x2c,
            ReasonCode::ErrorInvalidSfdId => 0x2d,
            ReasonCode::ErrorInvalidPsduDataRate => 0x2e,
            ReasonCode::ErrorInvalidPhrDataRate => 0x2f,
            ReasonCode::ErrorInvalidPreambleDuration => 0x30,
            ReasonCode::ErrorInvalidStsLength => 0x31,
            ReasonCode::ErrorInvalidNumOfStsSegments => 0x32,
            ReasonCode::ErrorInvalidNumOfControlees => 0x33,
            ReasonCode::ErrorMaxRangingReplyTimeExceeded => 0x34,
            ReasonCode::ErrorInvalidDstAddressList => 0x35,
            ReasonCode::ErrorInvalidOrNotFoundSubSessionId => 0x36,
            ReasonCode::ErrorInvalidResultReportConfig => 0x37,
            ReasonCode::ErrorInvalidRangingRoundControlConfig => 0x38,
            ReasonCode::ErrorInvalidRangingRoundUsage => 0x39,
            ReasonCode::ErrorInvalidMultiNodeMode => 0x3a,
            ReasonCode::ErrorRdsFetchFailure => 0x3b,
            ReasonCode::ErrorRefUwbSessionDoesNotExist => 0x3c,
            ReasonCode::ErrorRefUwbSessionRangingDurationMismatch => 0x3d,
            ReasonCode::ErrorRefUwbSessionInvalidOffsetTime => 0x3e,
            ReasonCode::ErrorRefUwbSessionLost => 0x3f,
            ReasonCode::ErrorDtAnchorRangingRoundsNotConfigured => 0x40,
            ReasonCode::ErrorDtTagRangingRoundsNotConfigured => 0x41,
            ReasonCode::RfuReasonCodeRange2(Private(value)) => *value,
            ReasonCode::ErrorInvalidChannelWithAoa => 0x80,
            ReasonCode::ErrorStoppedDueToOtherSessionConflict => 0x81,
            ReasonCode::VendorSpecificReasonCodeRange1(Private(value)) => *value,
            ReasonCode::VendorSpecificReasonCode2 => 0xff,
        }
    }
}
impl From<ReasonCode> for u8 {
    fn from(value: ReasonCode) -> Self {
        (&value).into()
    }
}
impl From<ReasonCode> for i16 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<ReasonCode> for i32 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<ReasonCode> for i64 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<ReasonCode> for u16 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<ReasonCode> for u32 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<ReasonCode> for u64 {
    fn from(value: ReasonCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum MulticastUpdateStatusCode {
    StatusOkMulticastListUpdate = 0x0,
    StatusErrorMulticastListFull = 0x1,
    StatusErrorKeyFetchFail = 0x2,
    StatusErrorSubSessionIdNotFound = 0x3,
    StatusErrorSubSessionKeyNotFound = 0x5,
    StatusErrorSubSessionKeyNotApplicable = 0x6,
    StatusErrorSessionKeyNotFound = 0x7,
    StatusErrorAddressAlreadyPresent = 0x8,
}
impl TryFrom<u8> for MulticastUpdateStatusCode {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(MulticastUpdateStatusCode::StatusOkMulticastListUpdate),
            0x1 => Ok(MulticastUpdateStatusCode::StatusErrorMulticastListFull),
            0x2 => Ok(MulticastUpdateStatusCode::StatusErrorKeyFetchFail),
            0x3 => Ok(MulticastUpdateStatusCode::StatusErrorSubSessionIdNotFound),
            0x5 => Ok(MulticastUpdateStatusCode::StatusErrorSubSessionKeyNotFound),
            0x6 => Ok(MulticastUpdateStatusCode::StatusErrorSubSessionKeyNotApplicable),
            0x7 => Ok(MulticastUpdateStatusCode::StatusErrorSessionKeyNotFound),
            0x8 => Ok(MulticastUpdateStatusCode::StatusErrorAddressAlreadyPresent),
            _ => Err(value),
        }
    }
}
impl From<&MulticastUpdateStatusCode> for u8 {
    fn from(value: &MulticastUpdateStatusCode) -> Self {
        match value {
            MulticastUpdateStatusCode::StatusOkMulticastListUpdate => 0x0,
            MulticastUpdateStatusCode::StatusErrorMulticastListFull => 0x1,
            MulticastUpdateStatusCode::StatusErrorKeyFetchFail => 0x2,
            MulticastUpdateStatusCode::StatusErrorSubSessionIdNotFound => 0x3,
            MulticastUpdateStatusCode::StatusErrorSubSessionKeyNotFound => 0x5,
            MulticastUpdateStatusCode::StatusErrorSubSessionKeyNotApplicable => 0x6,
            MulticastUpdateStatusCode::StatusErrorSessionKeyNotFound => 0x7,
            MulticastUpdateStatusCode::StatusErrorAddressAlreadyPresent => 0x8,
        }
    }
}
impl From<MulticastUpdateStatusCode> for u8 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        (&value).into()
    }
}
impl From<MulticastUpdateStatusCode> for i16 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<MulticastUpdateStatusCode> for i32 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<MulticastUpdateStatusCode> for i64 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<MulticastUpdateStatusCode> for u16 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<MulticastUpdateStatusCode> for u32 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
impl From<MulticastUpdateStatusCode> for u64 {
    fn from(value: MulticastUpdateStatusCode) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum MacAddressIndicator {
    ShortAddress = 0x0,
    ExtendedAddress = 0x1,
}
impl TryFrom<u8> for MacAddressIndicator {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(MacAddressIndicator::ShortAddress),
            0x1 => Ok(MacAddressIndicator::ExtendedAddress),
            _ => Err(value),
        }
    }
}
impl From<&MacAddressIndicator> for u8 {
    fn from(value: &MacAddressIndicator) -> Self {
        match value {
            MacAddressIndicator::ShortAddress => 0x0,
            MacAddressIndicator::ExtendedAddress => 0x1,
        }
    }
}
impl From<MacAddressIndicator> for u8 {
    fn from(value: MacAddressIndicator) -> Self {
        (&value).into()
    }
}
impl From<MacAddressIndicator> for i16 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
impl From<MacAddressIndicator> for i32 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
impl From<MacAddressIndicator> for i64 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
impl From<MacAddressIndicator> for u16 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
impl From<MacAddressIndicator> for u32 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
impl From<MacAddressIndicator> for u64 {
    fn from(value: MacAddressIndicator) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum SessionType {
    FiraRangingSession = 0x0,
    FiraRangingAndInBandDataSession = 0x1,
    FiraDataTransferSession = 0x2,
    FiraRangingOnlyPhase = 0x3,
    FiraInBandDataPhase = 0x4,
    FiraRangingWithDataPhase = 0x5,
    Ccc = 0xa0,
    DeviceTestMode = 0xd0,
}
impl TryFrom<u8> for SessionType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SessionType::FiraRangingSession),
            0x1 => Ok(SessionType::FiraRangingAndInBandDataSession),
            0x2 => Ok(SessionType::FiraDataTransferSession),
            0x3 => Ok(SessionType::FiraRangingOnlyPhase),
            0x4 => Ok(SessionType::FiraInBandDataPhase),
            0x5 => Ok(SessionType::FiraRangingWithDataPhase),
            0xa0 => Ok(SessionType::Ccc),
            0xd0 => Ok(SessionType::DeviceTestMode),
            _ => Err(value),
        }
    }
}
impl From<&SessionType> for u8 {
    fn from(value: &SessionType) -> Self {
        match value {
            SessionType::FiraRangingSession => 0x0,
            SessionType::FiraRangingAndInBandDataSession => 0x1,
            SessionType::FiraDataTransferSession => 0x2,
            SessionType::FiraRangingOnlyPhase => 0x3,
            SessionType::FiraInBandDataPhase => 0x4,
            SessionType::FiraRangingWithDataPhase => 0x5,
            SessionType::Ccc => 0xa0,
            SessionType::DeviceTestMode => 0xd0,
        }
    }
}
impl From<SessionType> for u8 {
    fn from(value: SessionType) -> Self {
        (&value).into()
    }
}
impl From<SessionType> for i16 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionType> for i32 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionType> for i64 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionType> for u16 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionType> for u32 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
impl From<SessionType> for u64 {
    fn from(value: SessionType) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum MessageType {
    Data = 0x0,
    Command = 0x1,
    Response = 0x2,
    Notification = 0x3,
    ReservedForTesting1 = 0x4,
    ReservedForTesting2 = 0x5,
}
impl TryFrom<u8> for MessageType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(MessageType::Data),
            0x1 => Ok(MessageType::Command),
            0x2 => Ok(MessageType::Response),
            0x3 => Ok(MessageType::Notification),
            0x4 => Ok(MessageType::ReservedForTesting1),
            0x5 => Ok(MessageType::ReservedForTesting2),
            _ => Err(value),
        }
    }
}
impl From<&MessageType> for u8 {
    fn from(value: &MessageType) -> Self {
        match value {
            MessageType::Data => 0x0,
            MessageType::Command => 0x1,
            MessageType::Response => 0x2,
            MessageType::Notification => 0x3,
            MessageType::ReservedForTesting1 => 0x4,
            MessageType::ReservedForTesting2 => 0x5,
        }
    }
}
impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        (&value).into()
    }
}
impl From<MessageType> for i8 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for i16 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for i32 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for i64 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for u16 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for u32 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
impl From<MessageType> for u64 {
    fn from(value: MessageType) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciPacketDataChild {
    UciCommand(UciCommandData),
    UciResponse(UciResponseData),
    UciNotification(UciNotificationData),
    Payload(Bytes),
    None,
}
impl UciPacketDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciPacketDataChild::UciCommand(value) => value.get_total_size(),
            UciPacketDataChild::UciResponse(value) => value.get_total_size(),
            UciPacketDataChild::UciNotification(value) => value.get_total_size(),
            UciPacketDataChild::Payload(bytes) => bytes.len(),
            UciPacketDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciPacketChild {
    UciCommand(UciCommand),
    UciResponse(UciResponse),
    UciNotification(UciNotification),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciPacketData {
    group_id: GroupId,
    packet_boundary_flag: PacketBoundaryFlag,
    message_type: MessageType,
    opcode: u8,
    child: UciPacketDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciPacket {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciPacketBuilder {
    pub group_id: GroupId,
    pub message_type: MessageType,
    pub opcode: u8,
    pub packet_boundary_flag: PacketBoundaryFlag,
    pub payload: Option<Bytes>,
}
impl UciPacketData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u8();
        let group_id = GroupId::try_from((chunk & 0xf)).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "UciPacket".to_string(),
                field: "group_id".to_string(),
                value: unknown_val as u64,
                type_: "GroupId".to_string(),
            }
        })?;
        let packet_boundary_flag =
            PacketBoundaryFlag::try_from(((chunk >> 4) & 0x1)).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "UciPacket".to_string(),
                    field: "packet_boundary_flag".to_string(),
                    value: unknown_val as u64,
                    type_: "PacketBoundaryFlag".to_string(),
                }
            })?;
        let message_type = MessageType::try_from(((chunk >> 5) & 0x7)).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "UciPacket".to_string(),
                field: "message_type".to_string(),
                value: unknown_val as u64,
                type_: "MessageType".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u8();
        let opcode = (chunk & 0x3f);
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(1);
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let payload_size = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < payload_size {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                wanted: payload_size,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..payload_size];
        bytes.get_mut().advance(payload_size);
        let child = match (message_type, packet_boundary_flag) {
            (MessageType::Command, PacketBoundaryFlag::Complete)
                if UciCommandData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = UciCommandData::parse_inner(&mut cell, group_id, opcode)?;
                UciPacketDataChild::UciCommand(child_data)
            }
            (MessageType::Response, PacketBoundaryFlag::Complete)
                if UciResponseData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = UciResponseData::parse_inner(&mut cell, group_id, opcode)?;
                UciPacketDataChild::UciResponse(child_data)
            }
            (MessageType::Notification, PacketBoundaryFlag::Complete)
                if UciNotificationData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = UciNotificationData::parse_inner(&mut cell, group_id, opcode)?;
                UciPacketDataChild::UciNotification(child_data)
            }
            _ if !payload.is_empty() => {
                UciPacketDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciPacketDataChild::None,
        };
        Ok(Self {
            group_id,
            packet_boundary_flag,
            message_type,
            opcode,
            child,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let value = u8::from(self.group_id)
            | (u8::from(self.packet_boundary_flag) << 4)
            | (u8::from(self.message_type) << 5);
        buffer.put_u8(value);
        if self.opcode > 0x3f {
            panic!(
                "Invalid value for {}::{}: {} > {}",
                "UciPacket", "opcode", self.opcode, 0x3f
            );
        }
        buffer.put_u8(self.opcode);
        buffer.put_bytes(0, 1);
        if self.child.get_total_size() > 0xff {
            panic!(
                "Invalid length for {}::{}: {} > {}",
                "UciPacket",
                "_payload_",
                self.child.get_total_size(),
                0xff
            );
        }
        buffer.put_u8(self.child.get_total_size() as u8);
        match &self.child {
            UciPacketDataChild::UciCommand(child) => child.write_to(buffer),
            UciPacketDataChild::UciResponse(child) => child.write_to(buffer),
            UciPacketDataChild::UciNotification(child) => child.write_to(buffer),
            UciPacketDataChild::Payload(payload) => buffer.put_slice(payload),
            UciPacketDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4 + self.child.get_total_size()
    }
}
impl Packet for UciPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciPacket> for Bytes {
    fn from(packet: UciPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciPacket> for Vec<u8> {
    fn from(packet: UciPacket) -> Self {
        packet.to_vec()
    }
}
impl UciPacket {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciPacketChild {
        match &self.ucipacket.child {
            UciPacketDataChild::UciCommand(_) => {
                UciPacketChild::UciCommand(UciCommand::new(self.ucipacket.clone()).unwrap())
            }
            UciPacketDataChild::UciResponse(_) => {
                UciPacketChild::UciResponse(UciResponse::new(self.ucipacket.clone()).unwrap())
            }
            UciPacketDataChild::UciNotification(_) => UciPacketChild::UciNotification(
                UciNotification::new(self.ucipacket.clone()).unwrap(),
            ),
            UciPacketDataChild::Payload(payload) => UciPacketChild::Payload(payload.clone()),
            UciPacketDataChild::None => UciPacketChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        Ok(Self { ucipacket })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucipacket.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciPacketBuilder {
    pub fn build(self) -> UciPacket {
        let ucipacket = UciPacketData {
            group_id: self.group_id,
            message_type: self.message_type,
            opcode: self.opcode,
            packet_boundary_flag: self.packet_boundary_flag,
            child: match self.payload {
                None => UciPacketDataChild::None,
                Some(bytes) => UciPacketDataChild::Payload(bytes),
            },
        };
        UciPacket::new(ucipacket).unwrap()
    }
}
impl From<UciPacketBuilder> for UciPacket {
    fn from(builder: UciPacketBuilder) -> UciPacket {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciCommandDataChild {
    CoreCommand(CoreCommandData),
    SessionConfigCommand(SessionConfigCommandData),
    SessionControlCommand(SessionControlCommandData),
    AndroidCommand(AndroidCommandData),
    Payload(Bytes),
    None,
}
impl UciCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciCommandDataChild::CoreCommand(value) => value.get_total_size(),
            UciCommandDataChild::SessionConfigCommand(value) => value.get_total_size(),
            UciCommandDataChild::SessionControlCommand(value) => value.get_total_size(),
            UciCommandDataChild::AndroidCommand(value) => value.get_total_size(),
            UciCommandDataChild::Payload(bytes) => bytes.len(),
            UciCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciCommandChild {
    CoreCommand(CoreCommand),
    SessionConfigCommand(SessionConfigCommand),
    SessionControlCommand(SessionControlCommand),
    AndroidCommand(AndroidCommand),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciCommandData {
    child: UciCommandDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciCommand {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciCommandBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, group_id, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (group_id) {
            (GroupId::Core) if CoreCommandData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CoreCommandData::parse_inner(&mut cell, opcode)?;
                UciCommandDataChild::CoreCommand(child_data)
            }
            (GroupId::SessionConfig) if SessionConfigCommandData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionConfigCommandData::parse_inner(&mut cell, opcode)?;
                UciCommandDataChild::SessionConfigCommand(child_data)
            }
            (GroupId::SessionControl) if SessionControlCommandData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionControlCommandData::parse_inner(&mut cell, opcode)?;
                UciCommandDataChild::SessionControlCommand(child_data)
            }
            (GroupId::VendorAndroid) if AndroidCommandData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidCommandData::parse_inner(&mut cell, opcode)?;
                UciCommandDataChild::AndroidCommand(child_data)
            }
            _ if !payload.is_empty() => {
                UciCommandDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciCommandDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciCommandDataChild::CoreCommand(child) => child.write_to(buffer),
            UciCommandDataChild::SessionConfigCommand(child) => child.write_to(buffer),
            UciCommandDataChild::SessionControlCommand(child) => child.write_to(buffer),
            UciCommandDataChild::AndroidCommand(child) => child.write_to(buffer),
            UciCommandDataChild::Payload(payload) => buffer.put_slice(payload),
            UciCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciCommand {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciCommand> for Bytes {
    fn from(packet: UciCommand) -> Self {
        packet.to_bytes()
    }
}
impl From<UciCommand> for Vec<u8> {
    fn from(packet: UciCommand) -> Self {
        packet.to_vec()
    }
}
impl From<UciCommand> for UciPacket {
    fn from(packet: UciCommand) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciCommand {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciCommand> {
        UciCommand::new(packet.ucipacket)
    }
}
impl UciCommand {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciCommandChild {
        match &self.ucicommand.child {
            UciCommandDataChild::CoreCommand(_) => {
                UciCommandChild::CoreCommand(CoreCommand::new(self.ucipacket.clone()).unwrap())
            }
            UciCommandDataChild::SessionConfigCommand(_) => UciCommandChild::SessionConfigCommand(
                SessionConfigCommand::new(self.ucipacket.clone()).unwrap(),
            ),
            UciCommandDataChild::SessionControlCommand(_) => {
                UciCommandChild::SessionControlCommand(
                    SessionControlCommand::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciCommandDataChild::AndroidCommand(_) => UciCommandChild::AndroidCommand(
                AndroidCommand::new(self.ucipacket.clone()).unwrap(),
            ),
            UciCommandDataChild::Payload(payload) => UciCommandChild::Payload(payload.clone()),
            UciCommandDataChild::None => UciCommandChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucicommand.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciCommandBuilder {
    pub fn build(self) -> UciCommand {
        let ucicommand = UciCommandData {
            child: match self.payload {
                None => UciCommandDataChild::None,
                Some(bytes) => UciCommandDataChild::Payload(bytes),
            },
        };
        let ucipacket = UciPacketData {
            group_id: self.group_id,
            message_type: MessageType::Command,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        UciCommand::new(ucipacket).unwrap()
    }
}
impl From<UciCommandBuilder> for UciPacket {
    fn from(builder: UciCommandBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciCommandBuilder> for UciCommand {
    fn from(builder: UciCommandBuilder) -> UciCommand {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciResponseDataChild {
    CoreResponse(CoreResponseData),
    SessionConfigResponse(SessionConfigResponseData),
    SessionControlResponse(SessionControlResponseData),
    AndroidResponse(AndroidResponseData),
    UciVendor_9_Response(UciVendor_9_ResponseData),
    UciVendor_A_Response(UciVendor_A_ResponseData),
    UciVendor_B_Response(UciVendor_B_ResponseData),
    UciVendor_E_Response(UciVendor_E_ResponseData),
    UciVendor_F_Response(UciVendor_F_ResponseData),
    Payload(Bytes),
    None,
}
impl UciResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciResponseDataChild::CoreResponse(value) => value.get_total_size(),
            UciResponseDataChild::SessionConfigResponse(value) => value.get_total_size(),
            UciResponseDataChild::SessionControlResponse(value) => value.get_total_size(),
            UciResponseDataChild::AndroidResponse(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_9_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_A_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_B_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_E_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_F_Response(value) => value.get_total_size(),
            UciResponseDataChild::Payload(bytes) => bytes.len(),
            UciResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciResponseChild {
    CoreResponse(CoreResponse),
    SessionConfigResponse(SessionConfigResponse),
    SessionControlResponse(SessionControlResponse),
    AndroidResponse(AndroidResponse),
    UciVendor_9_Response(UciVendor_9_Response),
    UciVendor_A_Response(UciVendor_A_Response),
    UciVendor_B_Response(UciVendor_B_Response),
    UciVendor_E_Response(UciVendor_E_Response),
    UciVendor_F_Response(UciVendor_F_Response),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciResponseData {
    child: UciResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciResponseBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, group_id, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (group_id) {
            (GroupId::Core) if CoreResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CoreResponseData::parse_inner(&mut cell, opcode)?;
                UciResponseDataChild::CoreResponse(child_data)
            }
            (GroupId::SessionConfig) if SessionConfigResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionConfigResponseData::parse_inner(&mut cell, opcode)?;
                UciResponseDataChild::SessionConfigResponse(child_data)
            }
            (GroupId::SessionControl) if SessionControlResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionControlResponseData::parse_inner(&mut cell, opcode)?;
                UciResponseDataChild::SessionControlResponse(child_data)
            }
            (GroupId::VendorAndroid) if AndroidResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidResponseData::parse_inner(&mut cell, opcode)?;
                UciResponseDataChild::AndroidResponse(child_data)
            }
            (GroupId::VendorReserved9) if UciVendor_9_ResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_9_ResponseData::parse_inner(&mut cell)?;
                UciResponseDataChild::UciVendor_9_Response(child_data)
            }
            (GroupId::VendorReservedA) if UciVendor_A_ResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_A_ResponseData::parse_inner(&mut cell)?;
                UciResponseDataChild::UciVendor_A_Response(child_data)
            }
            (GroupId::VendorReservedB) if UciVendor_B_ResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_B_ResponseData::parse_inner(&mut cell)?;
                UciResponseDataChild::UciVendor_B_Response(child_data)
            }
            (GroupId::VendorReservedE) if UciVendor_E_ResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_E_ResponseData::parse_inner(&mut cell)?;
                UciResponseDataChild::UciVendor_E_Response(child_data)
            }
            (GroupId::VendorReservedF) if UciVendor_F_ResponseData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_F_ResponseData::parse_inner(&mut cell)?;
                UciResponseDataChild::UciVendor_F_Response(child_data)
            }
            _ if !payload.is_empty() => {
                UciResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciResponseDataChild::CoreResponse(child) => child.write_to(buffer),
            UciResponseDataChild::SessionConfigResponse(child) => child.write_to(buffer),
            UciResponseDataChild::SessionControlResponse(child) => child.write_to(buffer),
            UciResponseDataChild::AndroidResponse(child) => child.write_to(buffer),
            UciResponseDataChild::UciVendor_9_Response(child) => child.write_to(buffer),
            UciResponseDataChild::UciVendor_A_Response(child) => child.write_to(buffer),
            UciResponseDataChild::UciVendor_B_Response(child) => child.write_to(buffer),
            UciResponseDataChild::UciVendor_E_Response(child) => child.write_to(buffer),
            UciResponseDataChild::UciVendor_F_Response(child) => child.write_to(buffer),
            UciResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciResponse {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciResponse> for Bytes {
    fn from(packet: UciResponse) -> Self {
        packet.to_bytes()
    }
}
impl From<UciResponse> for Vec<u8> {
    fn from(packet: UciResponse) -> Self {
        packet.to_vec()
    }
}
impl From<UciResponse> for UciPacket {
    fn from(packet: UciResponse) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciResponse {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciResponse> {
        UciResponse::new(packet.ucipacket)
    }
}
impl UciResponse {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciResponseChild {
        match &self.uciresponse.child {
            UciResponseDataChild::CoreResponse(_) => {
                UciResponseChild::CoreResponse(CoreResponse::new(self.ucipacket.clone()).unwrap())
            }
            UciResponseDataChild::SessionConfigResponse(_) => {
                UciResponseChild::SessionConfigResponse(
                    SessionConfigResponse::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::SessionControlResponse(_) => {
                UciResponseChild::SessionControlResponse(
                    SessionControlResponse::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::AndroidResponse(_) => UciResponseChild::AndroidResponse(
                AndroidResponse::new(self.ucipacket.clone()).unwrap(),
            ),
            UciResponseDataChild::UciVendor_9_Response(_) => {
                UciResponseChild::UciVendor_9_Response(
                    UciVendor_9_Response::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_A_Response(_) => {
                UciResponseChild::UciVendor_A_Response(
                    UciVendor_A_Response::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_B_Response(_) => {
                UciResponseChild::UciVendor_B_Response(
                    UciVendor_B_Response::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_E_Response(_) => {
                UciResponseChild::UciVendor_E_Response(
                    UciVendor_E_Response::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_F_Response(_) => {
                UciResponseChild::UciVendor_F_Response(
                    UciVendor_F_Response::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciResponseDataChild::Payload(payload) => UciResponseChild::Payload(payload.clone()),
            UciResponseDataChild::None => UciResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.uciresponse.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciResponseBuilder {
    pub fn build(self) -> UciResponse {
        let uciresponse = UciResponseData {
            child: match self.payload {
                None => UciResponseDataChild::None,
                Some(bytes) => UciResponseDataChild::Payload(bytes),
            },
        };
        let ucipacket = UciPacketData {
            group_id: self.group_id,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciResponse::new(ucipacket).unwrap()
    }
}
impl From<UciResponseBuilder> for UciPacket {
    fn from(builder: UciResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciResponseBuilder> for UciResponse {
    fn from(builder: UciResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciNotificationDataChild {
    CoreNotification(CoreNotificationData),
    SessionConfigNotification(SessionConfigNotificationData),
    SessionControlNotification(SessionControlNotificationData),
    AndroidNotification(AndroidNotificationData),
    UciVendor_9_Notification(UciVendor_9_NotificationData),
    UciVendor_A_Notification(UciVendor_A_NotificationData),
    UciVendor_B_Notification(UciVendor_B_NotificationData),
    UciVendor_E_Notification(UciVendor_E_NotificationData),
    UciVendor_F_Notification(UciVendor_F_NotificationData),
    TestNotification(TestNotificationData),
    Payload(Bytes),
    None,
}
impl UciNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciNotificationDataChild::CoreNotification(value) => value.get_total_size(),
            UciNotificationDataChild::SessionConfigNotification(value) => value.get_total_size(),
            UciNotificationDataChild::SessionControlNotification(value) => value.get_total_size(),
            UciNotificationDataChild::AndroidNotification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_9_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_A_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_B_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_E_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_F_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::TestNotification(value) => value.get_total_size(),
            UciNotificationDataChild::Payload(bytes) => bytes.len(),
            UciNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciNotificationChild {
    CoreNotification(CoreNotification),
    SessionConfigNotification(SessionConfigNotification),
    SessionControlNotification(SessionControlNotification),
    AndroidNotification(AndroidNotification),
    UciVendor_9_Notification(UciVendor_9_Notification),
    UciVendor_A_Notification(UciVendor_A_Notification),
    UciVendor_B_Notification(UciVendor_B_Notification),
    UciVendor_E_Notification(UciVendor_E_Notification),
    UciVendor_F_Notification(UciVendor_F_Notification),
    TestNotification(TestNotification),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciNotificationData {
    child: UciNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciNotificationBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, group_id, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (group_id) {
            (GroupId::Core) if CoreNotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CoreNotificationData::parse_inner(&mut cell, opcode)?;
                UciNotificationDataChild::CoreNotification(child_data)
            }
            (GroupId::SessionConfig) if SessionConfigNotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionConfigNotificationData::parse_inner(&mut cell, opcode)?;
                UciNotificationDataChild::SessionConfigNotification(child_data)
            }
            (GroupId::SessionControl) if SessionControlNotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionControlNotificationData::parse_inner(&mut cell, opcode)?;
                UciNotificationDataChild::SessionControlNotification(child_data)
            }
            (GroupId::VendorAndroid) if AndroidNotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidNotificationData::parse_inner(&mut cell, opcode)?;
                UciNotificationDataChild::AndroidNotification(child_data)
            }
            (GroupId::VendorReserved9) if UciVendor_9_NotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_9_NotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::UciVendor_9_Notification(child_data)
            }
            (GroupId::VendorReservedA) if UciVendor_A_NotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_A_NotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::UciVendor_A_Notification(child_data)
            }
            (GroupId::VendorReservedB) if UciVendor_B_NotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_B_NotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::UciVendor_B_Notification(child_data)
            }
            (GroupId::VendorReservedE) if UciVendor_E_NotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_E_NotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::UciVendor_E_Notification(child_data)
            }
            (GroupId::VendorReservedF) if UciVendor_F_NotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = UciVendor_F_NotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::UciVendor_F_Notification(child_data)
            }
            (GroupId::Test) if TestNotificationData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = TestNotificationData::parse_inner(&mut cell)?;
                UciNotificationDataChild::TestNotification(child_data)
            }
            _ if !payload.is_empty() => {
                UciNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciNotificationDataChild::CoreNotification(child) => child.write_to(buffer),
            UciNotificationDataChild::SessionConfigNotification(child) => child.write_to(buffer),
            UciNotificationDataChild::SessionControlNotification(child) => child.write_to(buffer),
            UciNotificationDataChild::AndroidNotification(child) => child.write_to(buffer),
            UciNotificationDataChild::UciVendor_9_Notification(child) => child.write_to(buffer),
            UciNotificationDataChild::UciVendor_A_Notification(child) => child.write_to(buffer),
            UciNotificationDataChild::UciVendor_B_Notification(child) => child.write_to(buffer),
            UciNotificationDataChild::UciVendor_E_Notification(child) => child.write_to(buffer),
            UciNotificationDataChild::UciVendor_F_Notification(child) => child.write_to(buffer),
            UciNotificationDataChild::TestNotification(child) => child.write_to(buffer),
            UciNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciNotification> for Bytes {
    fn from(packet: UciNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciNotification> for Vec<u8> {
    fn from(packet: UciNotification) -> Self {
        packet.to_vec()
    }
}
impl From<UciNotification> for UciPacket {
    fn from(packet: UciNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciNotification> {
        UciNotification::new(packet.ucipacket)
    }
}
impl UciNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciNotificationChild {
        match &self.ucinotification.child {
            UciNotificationDataChild::CoreNotification(_) => {
                UciNotificationChild::CoreNotification(
                    CoreNotification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::SessionConfigNotification(_) => {
                UciNotificationChild::SessionConfigNotification(
                    SessionConfigNotification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::SessionControlNotification(_) => {
                UciNotificationChild::SessionControlNotification(
                    SessionControlNotification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::AndroidNotification(_) => {
                UciNotificationChild::AndroidNotification(
                    AndroidNotification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_9_Notification(_) => {
                UciNotificationChild::UciVendor_9_Notification(
                    UciVendor_9_Notification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_A_Notification(_) => {
                UciNotificationChild::UciVendor_A_Notification(
                    UciVendor_A_Notification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_B_Notification(_) => {
                UciNotificationChild::UciVendor_B_Notification(
                    UciVendor_B_Notification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_E_Notification(_) => {
                UciNotificationChild::UciVendor_E_Notification(
                    UciVendor_E_Notification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_F_Notification(_) => {
                UciNotificationChild::UciVendor_F_Notification(
                    UciVendor_F_Notification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::TestNotification(_) => {
                UciNotificationChild::TestNotification(
                    TestNotification::new(self.ucipacket.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::Payload(payload) => {
                UciNotificationChild::Payload(payload.clone())
            }
            UciNotificationDataChild::None => UciNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucinotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciNotificationBuilder {
    pub fn build(self) -> UciNotification {
        let ucinotification = UciNotificationData {
            child: match self.payload {
                None => UciNotificationDataChild::None,
                Some(bytes) => UciNotificationDataChild::Payload(bytes),
            },
        };
        let ucipacket = UciPacketData {
            group_id: self.group_id,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciNotification::new(ucipacket).unwrap()
    }
}
impl From<UciNotificationBuilder> for UciPacket {
    fn from(builder: UciNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciNotificationBuilder> for UciNotification {
    fn from(builder: UciNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreCommandDataChild {
    DeviceResetCmd(DeviceResetCmdData),
    GetDeviceInfoCmd(GetDeviceInfoCmdData),
    GetCapsInfoCmd(GetCapsInfoCmdData),
    SetConfigCmd(SetConfigCmdData),
    GetConfigCmd(GetConfigCmdData),
    CoreQueryTimeStampCmd(CoreQueryTimeStampCmdData),
    Payload(Bytes),
    None,
}
impl CoreCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreCommandDataChild::DeviceResetCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetDeviceInfoCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetCapsInfoCmd(value) => value.get_total_size(),
            CoreCommandDataChild::SetConfigCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetConfigCmd(value) => value.get_total_size(),
            CoreCommandDataChild::CoreQueryTimeStampCmd(value) => value.get_total_size(),
            CoreCommandDataChild::Payload(bytes) => bytes.len(),
            CoreCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreCommandChild {
    DeviceResetCmd(DeviceResetCmd),
    GetDeviceInfoCmd(GetDeviceInfoCmd),
    GetCapsInfoCmd(GetCapsInfoCmd),
    SetConfigCmd(SetConfigCmd),
    GetConfigCmd(GetConfigCmd),
    CoreQueryTimeStampCmd(CoreQueryTimeStampCmd),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreCommandData {
    child: CoreCommandDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreCommand {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreCommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl CoreCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if DeviceResetCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = DeviceResetCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::DeviceResetCmd(child_data)
            }
            (2) if GetDeviceInfoCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetDeviceInfoCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::GetDeviceInfoCmd(child_data)
            }
            (3) if GetCapsInfoCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetCapsInfoCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::GetCapsInfoCmd(child_data)
            }
            (4) if SetConfigCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SetConfigCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::SetConfigCmd(child_data)
            }
            (5) if GetConfigCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetConfigCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::GetConfigCmd(child_data)
            }
            (8) if CoreQueryTimeStampCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CoreQueryTimeStampCmdData::parse_inner(&mut cell)?;
                CoreCommandDataChild::CoreQueryTimeStampCmd(child_data)
            }
            _ if !payload.is_empty() => {
                CoreCommandDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => CoreCommandDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreCommandDataChild::DeviceResetCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::GetDeviceInfoCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::GetCapsInfoCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::SetConfigCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::GetConfigCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::CoreQueryTimeStampCmd(child) => child.write_to(buffer),
            CoreCommandDataChild::Payload(payload) => buffer.put_slice(payload),
            CoreCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for CoreCommand {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreCommand> for Bytes {
    fn from(packet: CoreCommand) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreCommand> for Vec<u8> {
    fn from(packet: CoreCommand) -> Self {
        packet.to_vec()
    }
}
impl From<CoreCommand> for UciPacket {
    fn from(packet: CoreCommand) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreCommand> for UciCommand {
    fn from(packet: CoreCommand) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for CoreCommand {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<CoreCommand> {
        CoreCommand::new(packet.ucipacket)
    }
}
impl CoreCommand {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> CoreCommandChild {
        match &self.corecommand.child {
            CoreCommandDataChild::DeviceResetCmd(_) => CoreCommandChild::DeviceResetCmd(
                DeviceResetCmd::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreCommandDataChild::GetDeviceInfoCmd(_) => CoreCommandChild::GetDeviceInfoCmd(
                GetDeviceInfoCmd::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreCommandDataChild::GetCapsInfoCmd(_) => CoreCommandChild::GetCapsInfoCmd(
                GetCapsInfoCmd::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreCommandDataChild::SetConfigCmd(_) => {
                CoreCommandChild::SetConfigCmd(SetConfigCmd::new(self.ucipacket.clone()).unwrap())
            }
            CoreCommandDataChild::GetConfigCmd(_) => {
                CoreCommandChild::GetConfigCmd(GetConfigCmd::new(self.ucipacket.clone()).unwrap())
            }
            CoreCommandDataChild::CoreQueryTimeStampCmd(_) => {
                CoreCommandChild::CoreQueryTimeStampCmd(
                    CoreQueryTimeStampCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            CoreCommandDataChild::Payload(payload) => CoreCommandChild::Payload(payload.clone()),
            CoreCommandDataChild::None => CoreCommandChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.corecommand.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl CoreCommandBuilder {
    pub fn build(self) -> CoreCommand {
        let corecommand = CoreCommandData {
            child: match self.payload {
                None => CoreCommandDataChild::None,
                Some(bytes) => CoreCommandDataChild::Payload(bytes),
            },
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        CoreCommand::new(ucipacket).unwrap()
    }
}
impl From<CoreCommandBuilder> for UciPacket {
    fn from(builder: CoreCommandBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<CoreCommandBuilder> for UciCommand {
    fn from(builder: CoreCommandBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<CoreCommandBuilder> for CoreCommand {
    fn from(builder: CoreCommandBuilder) -> CoreCommand {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreResponseDataChild {
    DeviceResetRsp(DeviceResetRspData),
    GetDeviceInfoRsp(GetDeviceInfoRspData),
    GetCapsInfoRsp(GetCapsInfoRspData),
    SetConfigRsp(SetConfigRspData),
    GetConfigRsp(GetConfigRspData),
    CoreQueryTimeStampRsp(CoreQueryTimeStampRspData),
    Payload(Bytes),
    None,
}
impl CoreResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreResponseDataChild::DeviceResetRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetDeviceInfoRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetCapsInfoRsp(value) => value.get_total_size(),
            CoreResponseDataChild::SetConfigRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetConfigRsp(value) => value.get_total_size(),
            CoreResponseDataChild::CoreQueryTimeStampRsp(value) => value.get_total_size(),
            CoreResponseDataChild::Payload(bytes) => bytes.len(),
            CoreResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreResponseChild {
    DeviceResetRsp(DeviceResetRsp),
    GetDeviceInfoRsp(GetDeviceInfoRsp),
    GetCapsInfoRsp(GetCapsInfoRsp),
    SetConfigRsp(SetConfigRsp),
    GetConfigRsp(GetConfigRsp),
    CoreQueryTimeStampRsp(CoreQueryTimeStampRsp),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreResponseData {
    child: CoreResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl CoreResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if DeviceResetRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = DeviceResetRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::DeviceResetRsp(child_data)
            }
            (2) if GetDeviceInfoRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetDeviceInfoRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::GetDeviceInfoRsp(child_data)
            }
            (3) if GetCapsInfoRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetCapsInfoRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::GetCapsInfoRsp(child_data)
            }
            (4) if SetConfigRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SetConfigRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::SetConfigRsp(child_data)
            }
            (5) if GetConfigRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GetConfigRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::GetConfigRsp(child_data)
            }
            (8) if CoreQueryTimeStampRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CoreQueryTimeStampRspData::parse_inner(&mut cell)?;
                CoreResponseDataChild::CoreQueryTimeStampRsp(child_data)
            }
            _ if !payload.is_empty() => {
                CoreResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => CoreResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreResponseDataChild::DeviceResetRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::GetDeviceInfoRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::GetCapsInfoRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::SetConfigRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::GetConfigRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::CoreQueryTimeStampRsp(child) => child.write_to(buffer),
            CoreResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            CoreResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for CoreResponse {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreResponse> for Bytes {
    fn from(packet: CoreResponse) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreResponse> for Vec<u8> {
    fn from(packet: CoreResponse) -> Self {
        packet.to_vec()
    }
}
impl From<CoreResponse> for UciPacket {
    fn from(packet: CoreResponse) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreResponse> for UciResponse {
    fn from(packet: CoreResponse) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for CoreResponse {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<CoreResponse> {
        CoreResponse::new(packet.ucipacket)
    }
}
impl CoreResponse {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> CoreResponseChild {
        match &self.coreresponse.child {
            CoreResponseDataChild::DeviceResetRsp(_) => CoreResponseChild::DeviceResetRsp(
                DeviceResetRsp::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreResponseDataChild::GetDeviceInfoRsp(_) => CoreResponseChild::GetDeviceInfoRsp(
                GetDeviceInfoRsp::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreResponseDataChild::GetCapsInfoRsp(_) => CoreResponseChild::GetCapsInfoRsp(
                GetCapsInfoRsp::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreResponseDataChild::SetConfigRsp(_) => {
                CoreResponseChild::SetConfigRsp(SetConfigRsp::new(self.ucipacket.clone()).unwrap())
            }
            CoreResponseDataChild::GetConfigRsp(_) => {
                CoreResponseChild::GetConfigRsp(GetConfigRsp::new(self.ucipacket.clone()).unwrap())
            }
            CoreResponseDataChild::CoreQueryTimeStampRsp(_) => {
                CoreResponseChild::CoreQueryTimeStampRsp(
                    CoreQueryTimeStampRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            CoreResponseDataChild::Payload(payload) => CoreResponseChild::Payload(payload.clone()),
            CoreResponseDataChild::None => CoreResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.coreresponse.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl CoreResponseBuilder {
    pub fn build(self) -> CoreResponse {
        let coreresponse = CoreResponseData {
            child: match self.payload {
                None => CoreResponseDataChild::None,
                Some(bytes) => CoreResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        CoreResponse::new(ucipacket).unwrap()
    }
}
impl From<CoreResponseBuilder> for UciPacket {
    fn from(builder: CoreResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<CoreResponseBuilder> for UciResponse {
    fn from(builder: CoreResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<CoreResponseBuilder> for CoreResponse {
    fn from(builder: CoreResponseBuilder) -> CoreResponse {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreNotificationDataChild {
    DeviceStatusNtf(DeviceStatusNtfData),
    GenericError(GenericErrorData),
    Payload(Bytes),
    None,
}
impl CoreNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreNotificationDataChild::DeviceStatusNtf(value) => value.get_total_size(),
            CoreNotificationDataChild::GenericError(value) => value.get_total_size(),
            CoreNotificationDataChild::Payload(bytes) => bytes.len(),
            CoreNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CoreNotificationChild {
    DeviceStatusNtf(DeviceStatusNtf),
    GenericError(GenericError),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreNotificationData {
    child: CoreNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corenotification: CoreNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreNotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl CoreNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (1) if DeviceStatusNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = DeviceStatusNtfData::parse_inner(&mut cell)?;
                CoreNotificationDataChild::DeviceStatusNtf(child_data)
            }
            (7) if GenericErrorData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GenericErrorData::parse_inner(&mut cell)?;
                CoreNotificationDataChild::GenericError(child_data)
            }
            _ if !payload.is_empty() => {
                CoreNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => CoreNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreNotificationDataChild::DeviceStatusNtf(child) => child.write_to(buffer),
            CoreNotificationDataChild::GenericError(child) => child.write_to(buffer),
            CoreNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            CoreNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for CoreNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreNotification> for Bytes {
    fn from(packet: CoreNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreNotification> for Vec<u8> {
    fn from(packet: CoreNotification) -> Self {
        packet.to_vec()
    }
}
impl From<CoreNotification> for UciPacket {
    fn from(packet: CoreNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreNotification> for UciNotification {
    fn from(packet: CoreNotification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for CoreNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<CoreNotification> {
        CoreNotification::new(packet.ucipacket)
    }
}
impl CoreNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> CoreNotificationChild {
        match &self.corenotification.child {
            CoreNotificationDataChild::DeviceStatusNtf(_) => {
                CoreNotificationChild::DeviceStatusNtf(
                    DeviceStatusNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            CoreNotificationDataChild::GenericError(_) => CoreNotificationChild::GenericError(
                GenericError::new(self.ucipacket.clone()).unwrap(),
            ),
            CoreNotificationDataChild::Payload(payload) => {
                CoreNotificationChild::Payload(payload.clone())
            }
            CoreNotificationDataChild::None => CoreNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corenotification = match &ucinotification.child {
            UciNotificationDataChild::CoreNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::CoreNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            corenotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.corenotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl CoreNotificationBuilder {
    pub fn build(self) -> CoreNotification {
        let corenotification = CoreNotificationData {
            child: match self.payload {
                None => CoreNotificationDataChild::None,
                Some(bytes) => CoreNotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(corenotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        CoreNotification::new(ucipacket).unwrap()
    }
}
impl From<CoreNotificationBuilder> for UciPacket {
    fn from(builder: CoreNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<CoreNotificationBuilder> for UciNotification {
    fn from(builder: CoreNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<CoreNotificationBuilder> for CoreNotification {
    fn from(builder: CoreNotificationBuilder) -> CoreNotification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigCommandDataChild {
    SessionInitCmd(SessionInitCmdData),
    SessionDeinitCmd(SessionDeinitCmdData),
    SessionSetAppConfigCmd(SessionSetAppConfigCmdData),
    SessionGetAppConfigCmd(SessionGetAppConfigCmdData),
    SessionGetCountCmd(SessionGetCountCmdData),
    SessionGetStateCmd(SessionGetStateCmdData),
    SessionUpdateDtTagRangingRoundsCmd(SessionUpdateDtTagRangingRoundsCmdData),
    SessionUpdateControllerMulticastListCmd(SessionUpdateControllerMulticastListCmdData),
    SessionSetHybridConfigCmd(SessionSetHybridConfigCmdData),
    SessionQueryMaxDataSizeCmd(SessionQueryMaxDataSizeCmdData),
    Payload(Bytes),
    None,
}
impl SessionConfigCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionConfigCommandDataChild::SessionInitCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionDeinitCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionSetAppConfigCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionGetAppConfigCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionGetCountCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionGetStateCmd(value) => value.get_total_size(),
            SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(value) => {
                value.get_total_size()
            }
            SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(value) => {
                value.get_total_size()
            }
            SessionConfigCommandDataChild::SessionSetHybridConfigCmd(value) => {
                value.get_total_size()
            }
            SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(value) => {
                value.get_total_size()
            }
            SessionConfigCommandDataChild::Payload(bytes) => bytes.len(),
            SessionConfigCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigCommandChild {
    SessionInitCmd(SessionInitCmd),
    SessionDeinitCmd(SessionDeinitCmd),
    SessionSetAppConfigCmd(SessionSetAppConfigCmd),
    SessionGetAppConfigCmd(SessionGetAppConfigCmd),
    SessionGetCountCmd(SessionGetCountCmd),
    SessionGetStateCmd(SessionGetStateCmd),
    SessionUpdateDtTagRangingRoundsCmd(SessionUpdateDtTagRangingRoundsCmd),
    SessionUpdateControllerMulticastListCmd(SessionUpdateControllerMulticastListCmd),
    SessionSetHybridConfigCmd(SessionSetHybridConfigCmd),
    SessionQueryMaxDataSizeCmd(SessionQueryMaxDataSizeCmd),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigCommandData {
    child: SessionConfigCommandDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigCommand {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigCommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl SessionConfigCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if SessionInitCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionInitCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionInitCmd(child_data)
            }
            (1) if SessionDeinitCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionDeinitCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionDeinitCmd(child_data)
            }
            (3) if SessionSetAppConfigCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionSetAppConfigCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionSetAppConfigCmd(child_data)
            }
            (4) if SessionGetAppConfigCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetAppConfigCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionGetAppConfigCmd(child_data)
            }
            (5) if SessionGetCountCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetCountCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionGetCountCmd(child_data)
            }
            (6) if SessionGetStateCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetStateCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionGetStateCmd(child_data)
            }
            (9) if SessionUpdateDtTagRangingRoundsCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionUpdateDtTagRangingRoundsCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(child_data)
            }
            (7) if SessionUpdateControllerMulticastListCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data =
                    SessionUpdateControllerMulticastListCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(child_data)
            }
            (12) if SessionSetHybridConfigCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionSetHybridConfigCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionSetHybridConfigCmd(child_data)
            }
            (11) if SessionQueryMaxDataSizeCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionQueryMaxDataSizeCmdData::parse_inner(&mut cell)?;
                SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(child_data)
            }
            _ if !payload.is_empty() => {
                SessionConfigCommandDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionConfigCommandDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionConfigCommandDataChild::SessionInitCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionDeinitCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionSetAppConfigCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionGetAppConfigCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionGetCountCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionGetStateCmd(child) => child.write_to(buffer),
            SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(child) => {
                child.write_to(buffer)
            }
            SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(child) => {
                child.write_to(buffer)
            }
            SessionConfigCommandDataChild::SessionSetHybridConfigCmd(child) => {
                child.write_to(buffer)
            }
            SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(child) => {
                child.write_to(buffer)
            }
            SessionConfigCommandDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionConfigCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for SessionConfigCommand {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionConfigCommand> for Bytes {
    fn from(packet: SessionConfigCommand) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionConfigCommand> for Vec<u8> {
    fn from(packet: SessionConfigCommand) -> Self {
        packet.to_vec()
    }
}
impl From<SessionConfigCommand> for UciPacket {
    fn from(packet: SessionConfigCommand) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionConfigCommand> for UciCommand {
    fn from(packet: SessionConfigCommand) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionConfigCommand {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionConfigCommand> {
        SessionConfigCommand::new(packet.ucipacket)
    }
}
impl SessionConfigCommand {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionConfigCommandChild {
        match &self.sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionInitCmd(_) => {
                SessionConfigCommandChild::SessionInitCmd(
                    SessionInitCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionDeinitCmd(_) => {
                SessionConfigCommandChild::SessionDeinitCmd(
                    SessionDeinitCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionSetAppConfigCmd(_) => {
                SessionConfigCommandChild::SessionSetAppConfigCmd(
                    SessionSetAppConfigCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionGetAppConfigCmd(_) => {
                SessionConfigCommandChild::SessionGetAppConfigCmd(
                    SessionGetAppConfigCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionGetCountCmd(_) => {
                SessionConfigCommandChild::SessionGetCountCmd(
                    SessionGetCountCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionGetStateCmd(_) => {
                SessionConfigCommandChild::SessionGetStateCmd(
                    SessionGetStateCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(_) => {
                SessionConfigCommandChild::SessionUpdateDtTagRangingRoundsCmd(
                    SessionUpdateDtTagRangingRoundsCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(_) => {
                SessionConfigCommandChild::SessionUpdateControllerMulticastListCmd(
                    SessionUpdateControllerMulticastListCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionSetHybridConfigCmd(_) => {
                SessionConfigCommandChild::SessionSetHybridConfigCmd(
                    SessionSetHybridConfigCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(_) => {
                SessionConfigCommandChild::SessionQueryMaxDataSizeCmd(
                    SessionQueryMaxDataSizeCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigCommandDataChild::Payload(payload) => {
                SessionConfigCommandChild::Payload(payload.clone())
            }
            SessionConfigCommandDataChild::None => SessionConfigCommandChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionconfigcommand.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionConfigCommandBuilder {
    pub fn build(self) -> SessionConfigCommand {
        let sessionconfigcommand = SessionConfigCommandData {
            child: match self.payload {
                None => SessionConfigCommandDataChild::None,
                Some(bytes) => SessionConfigCommandDataChild::Payload(bytes),
            },
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionConfigCommand::new(ucipacket).unwrap()
    }
}
impl From<SessionConfigCommandBuilder> for UciPacket {
    fn from(builder: SessionConfigCommandBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionConfigCommandBuilder> for UciCommand {
    fn from(builder: SessionConfigCommandBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionConfigCommandBuilder> for SessionConfigCommand {
    fn from(builder: SessionConfigCommandBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigResponseDataChild {
    SessionInitRsp_V2(SessionInitRsp_V2Data),
    SessionInitRsp(SessionInitRspData),
    SessionDeinitRsp(SessionDeinitRspData),
    SessionSetAppConfigRsp(SessionSetAppConfigRspData),
    SessionGetAppConfigRsp(SessionGetAppConfigRspData),
    SessionGetCountRsp(SessionGetCountRspData),
    SessionGetStateRsp(SessionGetStateRspData),
    SessionUpdateDtTagRangingRoundsRsp(SessionUpdateDtTagRangingRoundsRspData),
    SessionSetHybridConfigRsp(SessionSetHybridConfigRspData),
    SessionUpdateControllerMulticastListRsp(SessionUpdateControllerMulticastListRspData),
    SessionQueryMaxDataSizeRsp(SessionQueryMaxDataSizeRspData),
    Payload(Bytes),
    None,
}
impl SessionConfigResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionConfigResponseDataChild::SessionInitRsp_V2(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionInitRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionDeinitRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionSetAppConfigRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionGetAppConfigRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionGetCountRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionGetStateRsp(value) => value.get_total_size(),
            SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(value) => {
                value.get_total_size()
            }
            SessionConfigResponseDataChild::SessionSetHybridConfigRsp(value) => {
                value.get_total_size()
            }
            SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(value) => {
                value.get_total_size()
            }
            SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(value) => {
                value.get_total_size()
            }
            SessionConfigResponseDataChild::Payload(bytes) => bytes.len(),
            SessionConfigResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigResponseChild {
    SessionInitRsp_V2(SessionInitRsp_V2),
    SessionInitRsp(SessionInitRsp),
    SessionDeinitRsp(SessionDeinitRsp),
    SessionSetAppConfigRsp(SessionSetAppConfigRsp),
    SessionGetAppConfigRsp(SessionGetAppConfigRsp),
    SessionGetCountRsp(SessionGetCountRsp),
    SessionGetStateRsp(SessionGetStateRsp),
    SessionUpdateDtTagRangingRoundsRsp(SessionUpdateDtTagRangingRoundsRsp),
    SessionSetHybridConfigRsp(SessionSetHybridConfigRsp),
    SessionUpdateControllerMulticastListRsp(SessionUpdateControllerMulticastListRsp),
    SessionQueryMaxDataSizeRsp(SessionQueryMaxDataSizeRsp),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigResponseData {
    child: SessionConfigResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl SessionConfigResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if SessionInitRsp_V2Data::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionInitRsp_V2Data::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionInitRsp_V2(child_data)
            }
            (0) if SessionInitRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionInitRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionInitRsp(child_data)
            }
            (1) if SessionDeinitRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionDeinitRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionDeinitRsp(child_data)
            }
            (3) if SessionSetAppConfigRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionSetAppConfigRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionSetAppConfigRsp(child_data)
            }
            (4) if SessionGetAppConfigRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetAppConfigRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionGetAppConfigRsp(child_data)
            }
            (5) if SessionGetCountRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetCountRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionGetCountRsp(child_data)
            }
            (6) if SessionGetStateRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetStateRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionGetStateRsp(child_data)
            }
            (9) if SessionUpdateDtTagRangingRoundsRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionUpdateDtTagRangingRoundsRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(child_data)
            }
            (12) if SessionSetHybridConfigRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionSetHybridConfigRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionSetHybridConfigRsp(child_data)
            }
            (7) if SessionUpdateControllerMulticastListRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data =
                    SessionUpdateControllerMulticastListRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(child_data)
            }
            (11) if SessionQueryMaxDataSizeRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionQueryMaxDataSizeRspData::parse_inner(&mut cell)?;
                SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(child_data)
            }
            _ if !payload.is_empty() => {
                SessionConfigResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionConfigResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionConfigResponseDataChild::SessionInitRsp_V2(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionInitRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionDeinitRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionSetAppConfigRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionGetAppConfigRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionGetCountRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionGetStateRsp(child) => child.write_to(buffer),
            SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(child) => {
                child.write_to(buffer)
            }
            SessionConfigResponseDataChild::SessionSetHybridConfigRsp(child) => {
                child.write_to(buffer)
            }
            SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(child) => {
                child.write_to(buffer)
            }
            SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(child) => {
                child.write_to(buffer)
            }
            SessionConfigResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionConfigResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for SessionConfigResponse {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionConfigResponse> for Bytes {
    fn from(packet: SessionConfigResponse) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionConfigResponse> for Vec<u8> {
    fn from(packet: SessionConfigResponse) -> Self {
        packet.to_vec()
    }
}
impl From<SessionConfigResponse> for UciPacket {
    fn from(packet: SessionConfigResponse) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionConfigResponse> for UciResponse {
    fn from(packet: SessionConfigResponse) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionConfigResponse {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionConfigResponse> {
        SessionConfigResponse::new(packet.ucipacket)
    }
}
impl SessionConfigResponse {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionConfigResponseChild {
        match &self.sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionInitRsp_V2(_) => {
                SessionConfigResponseChild::SessionInitRsp_V2(
                    SessionInitRsp_V2::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionInitRsp(_) => {
                SessionConfigResponseChild::SessionInitRsp(
                    SessionInitRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionDeinitRsp(_) => {
                SessionConfigResponseChild::SessionDeinitRsp(
                    SessionDeinitRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionSetAppConfigRsp(_) => {
                SessionConfigResponseChild::SessionSetAppConfigRsp(
                    SessionSetAppConfigRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionGetAppConfigRsp(_) => {
                SessionConfigResponseChild::SessionGetAppConfigRsp(
                    SessionGetAppConfigRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionGetCountRsp(_) => {
                SessionConfigResponseChild::SessionGetCountRsp(
                    SessionGetCountRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionGetStateRsp(_) => {
                SessionConfigResponseChild::SessionGetStateRsp(
                    SessionGetStateRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(_) => {
                SessionConfigResponseChild::SessionUpdateDtTagRangingRoundsRsp(
                    SessionUpdateDtTagRangingRoundsRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionSetHybridConfigRsp(_) => {
                SessionConfigResponseChild::SessionSetHybridConfigRsp(
                    SessionSetHybridConfigRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(_) => {
                SessionConfigResponseChild::SessionUpdateControllerMulticastListRsp(
                    SessionUpdateControllerMulticastListRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(_) => {
                SessionConfigResponseChild::SessionQueryMaxDataSizeRsp(
                    SessionQueryMaxDataSizeRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigResponseDataChild::Payload(payload) => {
                SessionConfigResponseChild::Payload(payload.clone())
            }
            SessionConfigResponseDataChild::None => SessionConfigResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionconfigresponse.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionConfigResponseBuilder {
    pub fn build(self) -> SessionConfigResponse {
        let sessionconfigresponse = SessionConfigResponseData {
            child: match self.payload {
                None => SessionConfigResponseDataChild::None,
                Some(bytes) => SessionConfigResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionConfigResponse::new(ucipacket).unwrap()
    }
}
impl From<SessionConfigResponseBuilder> for UciPacket {
    fn from(builder: SessionConfigResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionConfigResponseBuilder> for UciResponse {
    fn from(builder: SessionConfigResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionConfigResponseBuilder> for SessionConfigResponse {
    fn from(builder: SessionConfigResponseBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigNotificationDataChild {
    SessionStatusNtf(SessionStatusNtfData),
    SessionUpdateControllerMulticastListNtf(SessionUpdateControllerMulticastListNtfData),
    Payload(Bytes),
    None,
}
impl SessionConfigNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionConfigNotificationDataChild::SessionStatusNtf(value) => value.get_total_size(),
            SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(value) => {
                value.get_total_size()
            }
            SessionConfigNotificationDataChild::Payload(bytes) => bytes.len(),
            SessionConfigNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionConfigNotificationChild {
    SessionStatusNtf(SessionStatusNtf),
    SessionUpdateControllerMulticastListNtf(SessionUpdateControllerMulticastListNtf),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigNotificationData {
    child: SessionConfigNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfignotification: SessionConfigNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfigNotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl SessionConfigNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (2) if SessionStatusNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionStatusNtfData::parse_inner(&mut cell)?;
                SessionConfigNotificationDataChild::SessionStatusNtf(child_data)
            }
            (7) if SessionUpdateControllerMulticastListNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data =
                    SessionUpdateControllerMulticastListNtfData::parse_inner(&mut cell)?;
                SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(
                    child_data,
                )
            }
            _ if !payload.is_empty() => {
                SessionConfigNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionConfigNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionConfigNotificationDataChild::SessionStatusNtf(child) => child.write_to(buffer),
            SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(child) => {
                child.write_to(buffer)
            }
            SessionConfigNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionConfigNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for SessionConfigNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionConfigNotification> for Bytes {
    fn from(packet: SessionConfigNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionConfigNotification> for Vec<u8> {
    fn from(packet: SessionConfigNotification) -> Self {
        packet.to_vec()
    }
}
impl From<SessionConfigNotification> for UciPacket {
    fn from(packet: SessionConfigNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionConfigNotification> for UciNotification {
    fn from(packet: SessionConfigNotification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionConfigNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionConfigNotification> {
        SessionConfigNotification::new(packet.ucipacket)
    }
}
impl SessionConfigNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionConfigNotificationChild {
        match &self.sessionconfignotification.child {
            SessionConfigNotificationDataChild::SessionStatusNtf(_) => {
                SessionConfigNotificationChild::SessionStatusNtf(
                    SessionStatusNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(_) => {
                SessionConfigNotificationChild::SessionUpdateControllerMulticastListNtf(
                    SessionUpdateControllerMulticastListNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionConfigNotificationDataChild::Payload(payload) => {
                SessionConfigNotificationChild::Payload(payload.clone())
            }
            SessionConfigNotificationDataChild::None => SessionConfigNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfignotification = match &ucinotification.child {
            UciNotificationDataChild::SessionConfigNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionConfigNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessionconfignotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionconfignotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionConfigNotificationBuilder {
    pub fn build(self) -> SessionConfigNotification {
        let sessionconfignotification = SessionConfigNotificationData {
            child: match self.payload {
                None => SessionConfigNotificationDataChild::None,
                Some(bytes) => SessionConfigNotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionConfigNotification(sessionconfignotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        SessionConfigNotification::new(ucipacket).unwrap()
    }
}
impl From<SessionConfigNotificationBuilder> for UciPacket {
    fn from(builder: SessionConfigNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionConfigNotificationBuilder> for UciNotification {
    fn from(builder: SessionConfigNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<SessionConfigNotificationBuilder> for SessionConfigNotification {
    fn from(builder: SessionConfigNotificationBuilder) -> SessionConfigNotification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlCommandDataChild {
    SessionStartCmd(SessionStartCmdData),
    SessionStopCmd(SessionStopCmdData),
    SessionGetRangingCountCmd(SessionGetRangingCountCmdData),
    Payload(Bytes),
    None,
}
impl SessionControlCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionControlCommandDataChild::SessionStartCmd(value) => value.get_total_size(),
            SessionControlCommandDataChild::SessionStopCmd(value) => value.get_total_size(),
            SessionControlCommandDataChild::SessionGetRangingCountCmd(value) => {
                value.get_total_size()
            }
            SessionControlCommandDataChild::Payload(bytes) => bytes.len(),
            SessionControlCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlCommandChild {
    SessionStartCmd(SessionStartCmd),
    SessionStopCmd(SessionStopCmd),
    SessionGetRangingCountCmd(SessionGetRangingCountCmd),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlCommandData {
    session_id: u32,
    child: SessionControlCommandDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlCommand {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolcommand: SessionControlCommandData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlCommandBuilder {
    pub opcode: u8,
    pub session_id: u32,
    pub payload: Option<Bytes>,
}
impl SessionControlCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionControlCommand".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_id = bytes.get_mut().get_u32_le();
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if SessionStartCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionStartCmdData::parse_inner(&mut cell)?;
                SessionControlCommandDataChild::SessionStartCmd(child_data)
            }
            (1) if SessionStopCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionStopCmdData::parse_inner(&mut cell)?;
                SessionControlCommandDataChild::SessionStopCmd(child_data)
            }
            (3) if SessionGetRangingCountCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetRangingCountCmdData::parse_inner(&mut cell)?;
                SessionControlCommandDataChild::SessionGetRangingCountCmd(child_data)
            }
            _ if !payload.is_empty() => {
                SessionControlCommandDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionControlCommandDataChild::None,
        };
        Ok(Self { session_id, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_id);
        match &self.child {
            SessionControlCommandDataChild::SessionStartCmd(child) => child.write_to(buffer),
            SessionControlCommandDataChild::SessionStopCmd(child) => child.write_to(buffer),
            SessionControlCommandDataChild::SessionGetRangingCountCmd(child) => {
                child.write_to(buffer)
            }
            SessionControlCommandDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionControlCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4 + self.child.get_total_size()
    }
}
impl Packet for SessionControlCommand {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionControlCommand> for Bytes {
    fn from(packet: SessionControlCommand) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionControlCommand> for Vec<u8> {
    fn from(packet: SessionControlCommand) -> Self {
        packet.to_vec()
    }
}
impl From<SessionControlCommand> for UciPacket {
    fn from(packet: SessionControlCommand) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionControlCommand> for UciCommand {
    fn from(packet: SessionControlCommand) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionControlCommand {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionControlCommand> {
        SessionControlCommand::new(packet.ucipacket)
    }
}
impl SessionControlCommand {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionControlCommandChild {
        match &self.sessioncontrolcommand.child {
            SessionControlCommandDataChild::SessionStartCmd(_) => {
                SessionControlCommandChild::SessionStartCmd(
                    SessionStartCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlCommandDataChild::SessionStopCmd(_) => {
                SessionControlCommandChild::SessionStopCmd(
                    SessionStopCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlCommandDataChild::SessionGetRangingCountCmd(_) => {
                SessionControlCommandChild::SessionGetRangingCountCmd(
                    SessionGetRangingCountCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlCommandDataChild::Payload(payload) => {
                SessionControlCommandChild::Payload(payload.clone())
            }
            SessionControlCommandDataChild::None => SessionControlCommandChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolcommand = match &ucicommand.child {
            UciCommandDataChild::SessionControlCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionControlCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessioncontrolcommand,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_id(&self) -> u32 {
        self.sessioncontrolcommand.session_id
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioncontrolcommand.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionControlCommandBuilder {
    pub fn build(self) -> SessionControlCommand {
        let sessioncontrolcommand = SessionControlCommandData {
            session_id: self.session_id,
            child: match self.payload {
                None => SessionControlCommandDataChild::None,
                Some(bytes) => SessionControlCommandDataChild::Payload(bytes),
            },
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionControlCommand(sessioncontrolcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Command,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionControlCommand::new(ucipacket).unwrap()
    }
}
impl From<SessionControlCommandBuilder> for UciPacket {
    fn from(builder: SessionControlCommandBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionControlCommandBuilder> for UciCommand {
    fn from(builder: SessionControlCommandBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionControlCommandBuilder> for SessionControlCommand {
    fn from(builder: SessionControlCommandBuilder) -> SessionControlCommand {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlResponseDataChild {
    SessionStartRsp(SessionStartRspData),
    SessionStopRsp(SessionStopRspData),
    SessionGetRangingCountRsp(SessionGetRangingCountRspData),
    Payload(Bytes),
    None,
}
impl SessionControlResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionControlResponseDataChild::SessionStartRsp(value) => value.get_total_size(),
            SessionControlResponseDataChild::SessionStopRsp(value) => value.get_total_size(),
            SessionControlResponseDataChild::SessionGetRangingCountRsp(value) => {
                value.get_total_size()
            }
            SessionControlResponseDataChild::Payload(bytes) => bytes.len(),
            SessionControlResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlResponseChild {
    SessionStartRsp(SessionStartRsp),
    SessionStopRsp(SessionStopRsp),
    SessionGetRangingCountRsp(SessionGetRangingCountRsp),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlResponseData {
    child: SessionControlResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolresponse: SessionControlResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl SessionControlResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if SessionStartRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionStartRspData::parse_inner(&mut cell)?;
                SessionControlResponseDataChild::SessionStartRsp(child_data)
            }
            (1) if SessionStopRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionStopRspData::parse_inner(&mut cell)?;
                SessionControlResponseDataChild::SessionStopRsp(child_data)
            }
            (3) if SessionGetRangingCountRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionGetRangingCountRspData::parse_inner(&mut cell)?;
                SessionControlResponseDataChild::SessionGetRangingCountRsp(child_data)
            }
            _ if !payload.is_empty() => {
                SessionControlResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionControlResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionControlResponseDataChild::SessionStartRsp(child) => child.write_to(buffer),
            SessionControlResponseDataChild::SessionStopRsp(child) => child.write_to(buffer),
            SessionControlResponseDataChild::SessionGetRangingCountRsp(child) => {
                child.write_to(buffer)
            }
            SessionControlResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionControlResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for SessionControlResponse {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionControlResponse> for Bytes {
    fn from(packet: SessionControlResponse) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionControlResponse> for Vec<u8> {
    fn from(packet: SessionControlResponse) -> Self {
        packet.to_vec()
    }
}
impl From<SessionControlResponse> for UciPacket {
    fn from(packet: SessionControlResponse) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionControlResponse> for UciResponse {
    fn from(packet: SessionControlResponse) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionControlResponse {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionControlResponse> {
        SessionControlResponse::new(packet.ucipacket)
    }
}
impl SessionControlResponse {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionControlResponseChild {
        match &self.sessioncontrolresponse.child {
            SessionControlResponseDataChild::SessionStartRsp(_) => {
                SessionControlResponseChild::SessionStartRsp(
                    SessionStartRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlResponseDataChild::SessionStopRsp(_) => {
                SessionControlResponseChild::SessionStopRsp(
                    SessionStopRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlResponseDataChild::SessionGetRangingCountRsp(_) => {
                SessionControlResponseChild::SessionGetRangingCountRsp(
                    SessionGetRangingCountRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlResponseDataChild::Payload(payload) => {
                SessionControlResponseChild::Payload(payload.clone())
            }
            SessionControlResponseDataChild::None => SessionControlResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolresponse = match &uciresponse.child {
            UciResponseDataChild::SessionControlResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionControlResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessioncontrolresponse,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioncontrolresponse.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionControlResponseBuilder {
    pub fn build(self) -> SessionControlResponse {
        let sessioncontrolresponse = SessionControlResponseData {
            child: match self.payload {
                None => SessionControlResponseDataChild::None,
                Some(bytes) => SessionControlResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionControlResponse(sessioncontrolresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionControlResponse::new(ucipacket).unwrap()
    }
}
impl From<SessionControlResponseBuilder> for UciPacket {
    fn from(builder: SessionControlResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionControlResponseBuilder> for UciResponse {
    fn from(builder: SessionControlResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionControlResponseBuilder> for SessionControlResponse {
    fn from(builder: SessionControlResponseBuilder) -> SessionControlResponse {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlNotificationDataChild {
    DataCreditNtf(DataCreditNtfData),
    DataTransferStatusNtf(DataTransferStatusNtfData),
    SessionInfoNtf(SessionInfoNtfData),
    Payload(Bytes),
    None,
}
impl SessionControlNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionControlNotificationDataChild::DataCreditNtf(value) => value.get_total_size(),
            SessionControlNotificationDataChild::DataTransferStatusNtf(value) => {
                value.get_total_size()
            }
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.get_total_size(),
            SessionControlNotificationDataChild::Payload(bytes) => bytes.len(),
            SessionControlNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionControlNotificationChild {
    DataCreditNtf(DataCreditNtf),
    DataTransferStatusNtf(DataTransferStatusNtf),
    SessionInfoNtf(SessionInfoNtf),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlNotificationData {
    child: SessionControlNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionControlNotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl SessionControlNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (4) if DataCreditNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = DataCreditNtfData::parse_inner(&mut cell)?;
                SessionControlNotificationDataChild::DataCreditNtf(child_data)
            }
            (5) if DataTransferStatusNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = DataTransferStatusNtfData::parse_inner(&mut cell)?;
                SessionControlNotificationDataChild::DataTransferStatusNtf(child_data)
            }
            (0) if SessionInfoNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = SessionInfoNtfData::parse_inner(&mut cell)?;
                SessionControlNotificationDataChild::SessionInfoNtf(child_data)
            }
            _ if !payload.is_empty() => {
                SessionControlNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionControlNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionControlNotificationDataChild::DataCreditNtf(child) => child.write_to(buffer),
            SessionControlNotificationDataChild::DataTransferStatusNtf(child) => {
                child.write_to(buffer)
            }
            SessionControlNotificationDataChild::SessionInfoNtf(child) => child.write_to(buffer),
            SessionControlNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionControlNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for SessionControlNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionControlNotification> for Bytes {
    fn from(packet: SessionControlNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionControlNotification> for Vec<u8> {
    fn from(packet: SessionControlNotification) -> Self {
        packet.to_vec()
    }
}
impl From<SessionControlNotification> for UciPacket {
    fn from(packet: SessionControlNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionControlNotification> for UciNotification {
    fn from(packet: SessionControlNotification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionControlNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionControlNotification> {
        SessionControlNotification::new(packet.ucipacket)
    }
}
impl SessionControlNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionControlNotificationChild {
        match &self.sessioncontrolnotification.child {
            SessionControlNotificationDataChild::DataCreditNtf(_) => {
                SessionControlNotificationChild::DataCreditNtf(
                    DataCreditNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlNotificationDataChild::DataTransferStatusNtf(_) => {
                SessionControlNotificationChild::DataTransferStatusNtf(
                    DataTransferStatusNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlNotificationDataChild::SessionInfoNtf(_) => {
                SessionControlNotificationChild::SessionInfoNtf(
                    SessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionControlNotificationDataChild::Payload(payload) => {
                SessionControlNotificationChild::Payload(payload.clone())
            }
            SessionControlNotificationDataChild::None => SessionControlNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioncontrolnotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionControlNotificationBuilder {
    pub fn build(self) -> SessionControlNotification {
        let sessioncontrolnotification = SessionControlNotificationData {
            child: match self.payload {
                None => SessionControlNotificationDataChild::None,
                Some(bytes) => SessionControlNotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        SessionControlNotification::new(ucipacket).unwrap()
    }
}
impl From<SessionControlNotificationBuilder> for UciPacket {
    fn from(builder: SessionControlNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionControlNotificationBuilder> for UciNotification {
    fn from(builder: SessionControlNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<SessionControlNotificationBuilder> for SessionControlNotification {
    fn from(builder: SessionControlNotificationBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidCommandDataChild {
    AndroidGetPowerStatsCmd(AndroidGetPowerStatsCmdData),
    AndroidSetCountryCodeCmd(AndroidSetCountryCodeCmdData),
    Payload(Bytes),
    None,
}
impl AndroidCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(value) => value.get_total_size(),
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(value) => value.get_total_size(),
            AndroidCommandDataChild::Payload(bytes) => bytes.len(),
            AndroidCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidCommandChild {
    AndroidGetPowerStatsCmd(AndroidGetPowerStatsCmd),
    AndroidSetCountryCodeCmd(AndroidSetCountryCodeCmd),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidCommandData {
    child: AndroidCommandDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidCommand {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidcommand: AndroidCommandData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidCommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl AndroidCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if AndroidGetPowerStatsCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidGetPowerStatsCmdData::parse_inner(&mut cell)?;
                AndroidCommandDataChild::AndroidGetPowerStatsCmd(child_data)
            }
            (1) if AndroidSetCountryCodeCmdData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidSetCountryCodeCmdData::parse_inner(&mut cell)?;
                AndroidCommandDataChild::AndroidSetCountryCodeCmd(child_data)
            }
            _ if !payload.is_empty() => {
                AndroidCommandDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => AndroidCommandDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(child) => child.write_to(buffer),
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(child) => child.write_to(buffer),
            AndroidCommandDataChild::Payload(payload) => buffer.put_slice(payload),
            AndroidCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for AndroidCommand {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidCommand> for Bytes {
    fn from(packet: AndroidCommand) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidCommand> for Vec<u8> {
    fn from(packet: AndroidCommand) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidCommand> for UciPacket {
    fn from(packet: AndroidCommand) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidCommand> for UciCommand {
    fn from(packet: AndroidCommand) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidCommand {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidCommand> {
        AndroidCommand::new(packet.ucipacket)
    }
}
impl AndroidCommand {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> AndroidCommandChild {
        match &self.androidcommand.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(_) => {
                AndroidCommandChild::AndroidGetPowerStatsCmd(
                    AndroidGetPowerStatsCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(_) => {
                AndroidCommandChild::AndroidSetCountryCodeCmd(
                    AndroidSetCountryCodeCmd::new(self.ucipacket.clone()).unwrap(),
                )
            }
            AndroidCommandDataChild::Payload(payload) => {
                AndroidCommandChild::Payload(payload.clone())
            }
            AndroidCommandDataChild::None => AndroidCommandChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidcommand = match &ucicommand.child {
            UciCommandDataChild::AndroidCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::AndroidCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            androidcommand,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidcommand.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidCommandBuilder {
    pub fn build(self) -> AndroidCommand {
        let androidcommand = AndroidCommandData {
            child: match self.payload {
                None => AndroidCommandDataChild::None,
                Some(bytes) => AndroidCommandDataChild::Payload(bytes),
            },
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::AndroidCommand(androidcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Command,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        AndroidCommand::new(ucipacket).unwrap()
    }
}
impl From<AndroidCommandBuilder> for UciPacket {
    fn from(builder: AndroidCommandBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidCommandBuilder> for UciCommand {
    fn from(builder: AndroidCommandBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<AndroidCommandBuilder> for AndroidCommand {
    fn from(builder: AndroidCommandBuilder) -> AndroidCommand {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidResponseDataChild {
    AndroidGetPowerStatsRsp(AndroidGetPowerStatsRspData),
    AndroidSetCountryCodeRsp(AndroidSetCountryCodeRspData),
    Payload(Bytes),
    None,
}
impl AndroidResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(value) => value.get_total_size(),
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(value) => value.get_total_size(),
            AndroidResponseDataChild::Payload(bytes) => bytes.len(),
            AndroidResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidResponseChild {
    AndroidGetPowerStatsRsp(AndroidGetPowerStatsRsp),
    AndroidSetCountryCodeRsp(AndroidSetCountryCodeRsp),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidResponseData {
    child: AndroidResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidresponse: AndroidResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl AndroidResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (0) if AndroidGetPowerStatsRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidGetPowerStatsRspData::parse_inner(&mut cell)?;
                AndroidResponseDataChild::AndroidGetPowerStatsRsp(child_data)
            }
            (1) if AndroidSetCountryCodeRspData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidSetCountryCodeRspData::parse_inner(&mut cell)?;
                AndroidResponseDataChild::AndroidSetCountryCodeRsp(child_data)
            }
            _ if !payload.is_empty() => {
                AndroidResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => AndroidResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(child) => child.write_to(buffer),
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(child) => child.write_to(buffer),
            AndroidResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            AndroidResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for AndroidResponse {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidResponse> for Bytes {
    fn from(packet: AndroidResponse) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidResponse> for Vec<u8> {
    fn from(packet: AndroidResponse) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidResponse> for UciPacket {
    fn from(packet: AndroidResponse) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidResponse> for UciResponse {
    fn from(packet: AndroidResponse) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidResponse {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidResponse> {
        AndroidResponse::new(packet.ucipacket)
    }
}
impl AndroidResponse {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> AndroidResponseChild {
        match &self.androidresponse.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(_) => {
                AndroidResponseChild::AndroidGetPowerStatsRsp(
                    AndroidGetPowerStatsRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(_) => {
                AndroidResponseChild::AndroidSetCountryCodeRsp(
                    AndroidSetCountryCodeRsp::new(self.ucipacket.clone()).unwrap(),
                )
            }
            AndroidResponseDataChild::Payload(payload) => {
                AndroidResponseChild::Payload(payload.clone())
            }
            AndroidResponseDataChild::None => AndroidResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidresponse = match &uciresponse.child {
            UciResponseDataChild::AndroidResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::AndroidResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            androidresponse,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidresponse.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidResponseBuilder {
    pub fn build(self) -> AndroidResponse {
        let androidresponse = AndroidResponseData {
            child: match self.payload {
                None => AndroidResponseDataChild::None,
                Some(bytes) => AndroidResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::AndroidResponse(androidresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        AndroidResponse::new(ucipacket).unwrap()
    }
}
impl From<AndroidResponseBuilder> for UciPacket {
    fn from(builder: AndroidResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidResponseBuilder> for UciResponse {
    fn from(builder: AndroidResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<AndroidResponseBuilder> for AndroidResponse {
    fn from(builder: AndroidResponseBuilder) -> AndroidResponse {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidNotificationDataChild {
    AndroidRangeDiagnosticsNtf(AndroidRangeDiagnosticsNtfData),
    Payload(Bytes),
    None,
}
impl AndroidNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(value) => {
                value.get_total_size()
            }
            AndroidNotificationDataChild::Payload(bytes) => bytes.len(),
            AndroidNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AndroidNotificationChild {
    AndroidRangeDiagnosticsNtf(AndroidRangeDiagnosticsNtf),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidNotificationData {
    child: AndroidNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidnotification: AndroidNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidNotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl AndroidNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, opcode)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, opcode: u8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (opcode) {
            (2) if AndroidRangeDiagnosticsNtfData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AndroidRangeDiagnosticsNtfData::parse_inner(&mut cell)?;
                AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(child_data)
            }
            _ if !payload.is_empty() => {
                AndroidNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => AndroidNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(child) => {
                child.write_to(buffer)
            }
            AndroidNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            AndroidNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for AndroidNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidNotification> for Bytes {
    fn from(packet: AndroidNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidNotification> for Vec<u8> {
    fn from(packet: AndroidNotification) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidNotification> for UciPacket {
    fn from(packet: AndroidNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidNotification> for UciNotification {
    fn from(packet: AndroidNotification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidNotification> {
        AndroidNotification::new(packet.ucipacket)
    }
}
impl AndroidNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> AndroidNotificationChild {
        match &self.androidnotification.child {
            AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(_) => {
                AndroidNotificationChild::AndroidRangeDiagnosticsNtf(
                    AndroidRangeDiagnosticsNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            AndroidNotificationDataChild::Payload(payload) => {
                AndroidNotificationChild::Payload(payload.clone())
            }
            AndroidNotificationDataChild::None => AndroidNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidnotification = match &ucinotification.child {
            UciNotificationDataChild::AndroidNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::AndroidNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            androidnotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidnotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidNotificationBuilder {
    pub fn build(self) -> AndroidNotification {
        let androidnotification = AndroidNotificationData {
            child: match self.payload {
                None => AndroidNotificationDataChild::None,
                Some(bytes) => AndroidNotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::AndroidNotification(androidnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        AndroidNotification::new(ucipacket).unwrap()
    }
}
impl From<AndroidNotificationBuilder> for UciPacket {
    fn from(builder: AndroidNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidNotificationBuilder> for UciNotification {
    fn from(builder: AndroidNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<AndroidNotificationBuilder> for AndroidNotification {
    fn from(builder: AndroidNotificationBuilder) -> AndroidNotification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetCmdData {
    reset_config: ResetConfig,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    deviceresetcmd: DeviceResetCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetCmdBuilder {
    pub reset_config: ResetConfig,
}
impl DeviceResetCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceResetCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let reset_config =
            ResetConfig::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "DeviceResetCmd".to_string(),
                    field: "reset_config".to_string(),
                    value: unknown_val as u64,
                    type_: "ResetConfig".to_string(),
                }
            })?;
        Ok(Self { reset_config })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.reset_config));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for DeviceResetCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceResetCmd> for Bytes {
    fn from(packet: DeviceResetCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceResetCmd> for Vec<u8> {
    fn from(packet: DeviceResetCmd) -> Self {
        packet.to_vec()
    }
}
impl From<DeviceResetCmd> for UciPacket {
    fn from(packet: DeviceResetCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceResetCmd> for UciCommand {
    fn from(packet: DeviceResetCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceResetCmd> for CoreCommand {
    fn from(packet: DeviceResetCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for DeviceResetCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<DeviceResetCmd> {
        DeviceResetCmd::new(packet.ucipacket)
    }
}
impl DeviceResetCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let deviceresetcmd = match &corecommand.child {
            CoreCommandDataChild::DeviceResetCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::DeviceResetCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            deviceresetcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_reset_config(&self) -> ResetConfig {
        self.deviceresetcmd.reset_config
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.deviceresetcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl DeviceResetCmdBuilder {
    pub fn build(self) -> DeviceResetCmd {
        let deviceresetcmd = DeviceResetCmdData {
            reset_config: self.reset_config,
        };
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::DeviceResetCmd(deviceresetcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        DeviceResetCmd::new(ucipacket).unwrap()
    }
}
impl From<DeviceResetCmdBuilder> for UciPacket {
    fn from(builder: DeviceResetCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<DeviceResetCmdBuilder> for UciCommand {
    fn from(builder: DeviceResetCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<DeviceResetCmdBuilder> for CoreCommand {
    fn from(builder: DeviceResetCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<DeviceResetCmdBuilder> for DeviceResetCmd {
    fn from(builder: DeviceResetCmdBuilder) -> DeviceResetCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    deviceresetrsp: DeviceResetRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceResetRspBuilder {
    pub status: StatusCode,
}
impl DeviceResetRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceResetRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "DeviceResetRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for DeviceResetRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceResetRsp> for Bytes {
    fn from(packet: DeviceResetRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceResetRsp> for Vec<u8> {
    fn from(packet: DeviceResetRsp) -> Self {
        packet.to_vec()
    }
}
impl From<DeviceResetRsp> for UciPacket {
    fn from(packet: DeviceResetRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceResetRsp> for UciResponse {
    fn from(packet: DeviceResetRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceResetRsp> for CoreResponse {
    fn from(packet: DeviceResetRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for DeviceResetRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<DeviceResetRsp> {
        DeviceResetRsp::new(packet.ucipacket)
    }
}
impl DeviceResetRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let deviceresetrsp = match &coreresponse.child {
            CoreResponseDataChild::DeviceResetRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::DeviceResetRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            deviceresetrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.deviceresetrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.deviceresetrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl DeviceResetRspBuilder {
    pub fn build(self) -> DeviceResetRsp {
        let deviceresetrsp = DeviceResetRspData {
            status: self.status,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::DeviceResetRsp(deviceresetrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        DeviceResetRsp::new(ucipacket).unwrap()
    }
}
impl From<DeviceResetRspBuilder> for UciPacket {
    fn from(builder: DeviceResetRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<DeviceResetRspBuilder> for UciResponse {
    fn from(builder: DeviceResetRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<DeviceResetRspBuilder> for CoreResponse {
    fn from(builder: DeviceResetRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<DeviceResetRspBuilder> for DeviceResetRsp {
    fn from(builder: DeviceResetRspBuilder) -> DeviceResetRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceStatusNtfData {
    device_state: DeviceState,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceStatusNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corenotification: CoreNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    devicestatusntf: DeviceStatusNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceStatusNtfBuilder {
    pub device_state: DeviceState,
}
impl DeviceStatusNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let device_state =
            DeviceState::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "DeviceStatusNtf".to_string(),
                    field: "device_state".to_string(),
                    value: unknown_val as u64,
                    type_: "DeviceState".to_string(),
                }
            })?;
        Ok(Self { device_state })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.device_state));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for DeviceStatusNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceStatusNtf> for Bytes {
    fn from(packet: DeviceStatusNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceStatusNtf> for Vec<u8> {
    fn from(packet: DeviceStatusNtf) -> Self {
        packet.to_vec()
    }
}
impl From<DeviceStatusNtf> for UciPacket {
    fn from(packet: DeviceStatusNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceStatusNtf> for UciNotification {
    fn from(packet: DeviceStatusNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<DeviceStatusNtf> for CoreNotification {
    fn from(packet: DeviceStatusNtf) -> CoreNotification {
        CoreNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for DeviceStatusNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<DeviceStatusNtf> {
        DeviceStatusNtf::new(packet.ucipacket)
    }
}
impl DeviceStatusNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corenotification = match &ucinotification.child {
            UciNotificationDataChild::CoreNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::CoreNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let devicestatusntf = match &corenotification.child {
            CoreNotificationDataChild::DeviceStatusNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreNotificationDataChild::DeviceStatusNtf),
                    actual: format!("{:?}", &corenotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            corenotification,
            devicestatusntf,
        })
    }
    pub fn get_device_state(&self) -> DeviceState {
        self.devicestatusntf.device_state
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.devicestatusntf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl DeviceStatusNtfBuilder {
    pub fn build(self) -> DeviceStatusNtf {
        let devicestatusntf = DeviceStatusNtfData {
            device_state: self.device_state,
        };
        let corenotification = CoreNotificationData {
            child: CoreNotificationDataChild::DeviceStatusNtf(devicestatusntf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(corenotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Notification,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        DeviceStatusNtf::new(ucipacket).unwrap()
    }
}
impl From<DeviceStatusNtfBuilder> for UciPacket {
    fn from(builder: DeviceStatusNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<DeviceStatusNtfBuilder> for UciNotification {
    fn from(builder: DeviceStatusNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<DeviceStatusNtfBuilder> for CoreNotification {
    fn from(builder: DeviceStatusNtfBuilder) -> CoreNotification {
        builder.build().into()
    }
}
impl From<DeviceStatusNtfBuilder> for DeviceStatusNtf {
    fn from(builder: DeviceStatusNtfBuilder) -> DeviceStatusNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getdeviceinfocmd: GetDeviceInfoCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoCmdBuilder {}
impl GetDeviceInfoCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for GetDeviceInfoCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetDeviceInfoCmd> for Bytes {
    fn from(packet: GetDeviceInfoCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<GetDeviceInfoCmd> for Vec<u8> {
    fn from(packet: GetDeviceInfoCmd) -> Self {
        packet.to_vec()
    }
}
impl From<GetDeviceInfoCmd> for UciPacket {
    fn from(packet: GetDeviceInfoCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoCmd> for UciCommand {
    fn from(packet: GetDeviceInfoCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoCmd> for CoreCommand {
    fn from(packet: GetDeviceInfoCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetDeviceInfoCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetDeviceInfoCmd> {
        GetDeviceInfoCmd::new(packet.ucipacket)
    }
}
impl GetDeviceInfoCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let getdeviceinfocmd = match &corecommand.child {
            CoreCommandDataChild::GetDeviceInfoCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::GetDeviceInfoCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            getdeviceinfocmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getdeviceinfocmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetDeviceInfoCmdBuilder {
    pub fn build(self) -> GetDeviceInfoCmd {
        let getdeviceinfocmd = GetDeviceInfoCmdData {};
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::GetDeviceInfoCmd(getdeviceinfocmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 2,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        GetDeviceInfoCmd::new(ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoCmdBuilder> for UciPacket {
    fn from(builder: GetDeviceInfoCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetDeviceInfoCmdBuilder> for UciCommand {
    fn from(builder: GetDeviceInfoCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<GetDeviceInfoCmdBuilder> for CoreCommand {
    fn from(builder: GetDeviceInfoCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<GetDeviceInfoCmdBuilder> for GetDeviceInfoCmd {
    fn from(builder: GetDeviceInfoCmdBuilder) -> GetDeviceInfoCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoRspData {
    status: StatusCode,
    uci_version: u16,
    mac_version: u16,
    phy_version: u16,
    uci_test_version: u16,
    vendor_spec_info: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getdeviceinforsp: GetDeviceInfoRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetDeviceInfoRspBuilder {
    pub mac_version: u16,
    pub phy_version: u16,
    pub status: StatusCode,
    pub uci_test_version: u16,
    pub uci_version: u16,
    pub vendor_spec_info: Vec<u8>,
}
impl GetDeviceInfoRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 10
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let uci_version = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let mac_version = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let phy_version = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let uci_test_version = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let vendor_spec_info_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < vendor_spec_info_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                wanted: vendor_spec_info_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let vendor_spec_info = (0..vendor_spec_info_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            status,
            uci_version,
            mac_version,
            phy_version,
            uci_test_version,
            vendor_spec_info,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u16_le(self.uci_version);
        buffer.put_u16_le(self.mac_version);
        buffer.put_u16_le(self.phy_version);
        buffer.put_u16_le(self.uci_test_version);
        buffer.put_u8(self.vendor_spec_info.len() as u8);
        for elem in &self.vendor_spec_info {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        10 + self.vendor_spec_info.len()
    }
}
impl Packet for GetDeviceInfoRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetDeviceInfoRsp> for Bytes {
    fn from(packet: GetDeviceInfoRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<GetDeviceInfoRsp> for Vec<u8> {
    fn from(packet: GetDeviceInfoRsp) -> Self {
        packet.to_vec()
    }
}
impl From<GetDeviceInfoRsp> for UciPacket {
    fn from(packet: GetDeviceInfoRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoRsp> for UciResponse {
    fn from(packet: GetDeviceInfoRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoRsp> for CoreResponse {
    fn from(packet: GetDeviceInfoRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetDeviceInfoRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetDeviceInfoRsp> {
        GetDeviceInfoRsp::new(packet.ucipacket)
    }
}
impl GetDeviceInfoRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let getdeviceinforsp = match &coreresponse.child {
            CoreResponseDataChild::GetDeviceInfoRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::GetDeviceInfoRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            getdeviceinforsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_version(&self) -> u16 {
        self.getdeviceinforsp.mac_version
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_phy_version(&self) -> u16 {
        self.getdeviceinforsp.phy_version
    }
    pub fn get_status(&self) -> StatusCode {
        self.getdeviceinforsp.status
    }
    pub fn get_uci_test_version(&self) -> u16 {
        self.getdeviceinforsp.uci_test_version
    }
    pub fn get_uci_version(&self) -> u16 {
        self.getdeviceinforsp.uci_version
    }
    pub fn get_vendor_spec_info(&self) -> &Vec<u8> {
        &self.getdeviceinforsp.vendor_spec_info
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getdeviceinforsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetDeviceInfoRspBuilder {
    pub fn build(self) -> GetDeviceInfoRsp {
        let getdeviceinforsp = GetDeviceInfoRspData {
            mac_version: self.mac_version,
            phy_version: self.phy_version,
            status: self.status,
            uci_test_version: self.uci_test_version,
            uci_version: self.uci_version,
            vendor_spec_info: self.vendor_spec_info,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::GetDeviceInfoRsp(getdeviceinforsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 2,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        GetDeviceInfoRsp::new(ucipacket).unwrap()
    }
}
impl From<GetDeviceInfoRspBuilder> for UciPacket {
    fn from(builder: GetDeviceInfoRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetDeviceInfoRspBuilder> for UciResponse {
    fn from(builder: GetDeviceInfoRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<GetDeviceInfoRspBuilder> for CoreResponse {
    fn from(builder: GetDeviceInfoRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<GetDeviceInfoRspBuilder> for GetDeviceInfoRsp {
    fn from(builder: GetDeviceInfoRspBuilder) -> GetDeviceInfoRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getcapsinfocmd: GetCapsInfoCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoCmdBuilder {}
impl GetCapsInfoCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for GetCapsInfoCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetCapsInfoCmd> for Bytes {
    fn from(packet: GetCapsInfoCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<GetCapsInfoCmd> for Vec<u8> {
    fn from(packet: GetCapsInfoCmd) -> Self {
        packet.to_vec()
    }
}
impl From<GetCapsInfoCmd> for UciPacket {
    fn from(packet: GetCapsInfoCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetCapsInfoCmd> for UciCommand {
    fn from(packet: GetCapsInfoCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<GetCapsInfoCmd> for CoreCommand {
    fn from(packet: GetCapsInfoCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetCapsInfoCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetCapsInfoCmd> {
        GetCapsInfoCmd::new(packet.ucipacket)
    }
}
impl GetCapsInfoCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let getcapsinfocmd = match &corecommand.child {
            CoreCommandDataChild::GetCapsInfoCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::GetCapsInfoCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            getcapsinfocmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getcapsinfocmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetCapsInfoCmdBuilder {
    pub fn build(self) -> GetCapsInfoCmd {
        let getcapsinfocmd = GetCapsInfoCmdData {};
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::GetCapsInfoCmd(getcapsinfocmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        GetCapsInfoCmd::new(ucipacket).unwrap()
    }
}
impl From<GetCapsInfoCmdBuilder> for UciPacket {
    fn from(builder: GetCapsInfoCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetCapsInfoCmdBuilder> for UciCommand {
    fn from(builder: GetCapsInfoCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<GetCapsInfoCmdBuilder> for CoreCommand {
    fn from(builder: GetCapsInfoCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<GetCapsInfoCmdBuilder> for GetCapsInfoCmd {
    fn from(builder: GetCapsInfoCmdBuilder) -> GetCapsInfoCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CapTlv {
    pub t: CapTlvType,
    pub v: Vec<u8>,
}
impl CapTlv {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let t = CapTlvType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "CapTlv".to_string(),
                field: "t".to_string(),
                value: unknown_val as u64,
                type_: "CapTlvType".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let v_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < v_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                wanted: v_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let v = (0..v_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { t, v })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.t));
        buffer.put_u8(self.v.len() as u8);
        for elem in &self.v {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.v.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoRspData {
    status: StatusCode,
    tlvs: Vec<CapTlv>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getcapsinforsp: GetCapsInfoRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetCapsInfoRspBuilder {
    pub status: StatusCode,
    pub tlvs: Vec<CapTlv>,
}
impl GetCapsInfoRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetCapsInfoRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "GetCapsInfoRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetCapsInfoRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tlvs_count = bytes.get_mut().get_u8() as usize;
        let tlvs = (0..tlvs_count)
            .map(|_| CapTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { status, tlvs })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.tlvs.len() as u8);
        for elem in &self.tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.tlvs.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for GetCapsInfoRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetCapsInfoRsp> for Bytes {
    fn from(packet: GetCapsInfoRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<GetCapsInfoRsp> for Vec<u8> {
    fn from(packet: GetCapsInfoRsp) -> Self {
        packet.to_vec()
    }
}
impl From<GetCapsInfoRsp> for UciPacket {
    fn from(packet: GetCapsInfoRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetCapsInfoRsp> for UciResponse {
    fn from(packet: GetCapsInfoRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<GetCapsInfoRsp> for CoreResponse {
    fn from(packet: GetCapsInfoRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetCapsInfoRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetCapsInfoRsp> {
        GetCapsInfoRsp::new(packet.ucipacket)
    }
}
impl GetCapsInfoRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let getcapsinforsp = match &coreresponse.child {
            CoreResponseDataChild::GetCapsInfoRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::GetCapsInfoRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            getcapsinforsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.getcapsinforsp.status
    }
    pub fn get_tlvs(&self) -> &Vec<CapTlv> {
        &self.getcapsinforsp.tlvs
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getcapsinforsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetCapsInfoRspBuilder {
    pub fn build(self) -> GetCapsInfoRsp {
        let getcapsinforsp = GetCapsInfoRspData {
            status: self.status,
            tlvs: self.tlvs,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::GetCapsInfoRsp(getcapsinforsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        GetCapsInfoRsp::new(ucipacket).unwrap()
    }
}
impl From<GetCapsInfoRspBuilder> for UciPacket {
    fn from(builder: GetCapsInfoRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetCapsInfoRspBuilder> for UciResponse {
    fn from(builder: GetCapsInfoRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<GetCapsInfoRspBuilder> for CoreResponse {
    fn from(builder: GetCapsInfoRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<GetCapsInfoRspBuilder> for GetCapsInfoRsp {
    fn from(builder: GetCapsInfoRspBuilder) -> GetCapsInfoRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceConfigTlv {
    pub cfg_id: DeviceConfigId,
    pub v: Vec<u8>,
}
impl DeviceConfigTlv {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id = DeviceConfigId::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "DeviceConfigTlv".to_string(),
                field: "cfg_id".to_string(),
                value: unknown_val as u64,
                type_: "DeviceConfigId".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let v_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < v_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigTlv".to_string(),
                wanted: v_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let v = (0..v_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cfg_id, v })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.cfg_id));
        buffer.put_u8(self.v.len() as u8);
        for elem in &self.v {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.v.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigCmdData {
    tlvs: Vec<DeviceConfigTlv>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    setconfigcmd: SetConfigCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigCmdBuilder {
    pub tlvs: Vec<DeviceConfigTlv>,
}
impl SetConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tlvs_count = bytes.get_mut().get_u8() as usize;
        let tlvs = (0..tlvs_count)
            .map(|_| DeviceConfigTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { tlvs })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.tlvs.len() as u8);
        for elem in &self.tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.tlvs.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for SetConfigCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SetConfigCmd> for Bytes {
    fn from(packet: SetConfigCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SetConfigCmd> for Vec<u8> {
    fn from(packet: SetConfigCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SetConfigCmd> for UciPacket {
    fn from(packet: SetConfigCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SetConfigCmd> for UciCommand {
    fn from(packet: SetConfigCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SetConfigCmd> for CoreCommand {
    fn from(packet: SetConfigCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SetConfigCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SetConfigCmd> {
        SetConfigCmd::new(packet.ucipacket)
    }
}
impl SetConfigCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let setconfigcmd = match &corecommand.child {
            CoreCommandDataChild::SetConfigCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::SetConfigCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            setconfigcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_tlvs(&self) -> &Vec<DeviceConfigTlv> {
        &self.setconfigcmd.tlvs
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.setconfigcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SetConfigCmdBuilder {
    pub fn build(self) -> SetConfigCmd {
        let setconfigcmd = SetConfigCmdData { tlvs: self.tlvs };
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::SetConfigCmd(setconfigcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 4,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SetConfigCmd::new(ucipacket).unwrap()
    }
}
impl From<SetConfigCmdBuilder> for UciPacket {
    fn from(builder: SetConfigCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SetConfigCmdBuilder> for UciCommand {
    fn from(builder: SetConfigCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SetConfigCmdBuilder> for CoreCommand {
    fn from(builder: SetConfigCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<SetConfigCmdBuilder> for SetConfigCmd {
    fn from(builder: SetConfigCmdBuilder) -> SetConfigCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceConfigStatus {
    pub cfg_id: DeviceConfigId,
    pub status: StatusCode,
}
impl DeviceConfigStatus {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigStatus".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id = DeviceConfigId::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "DeviceConfigStatus".to_string(),
                field: "cfg_id".to_string(),
                value: unknown_val as u64,
                type_: "DeviceConfigId".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigStatus".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "DeviceConfigStatus".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { cfg_id, status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.cfg_id));
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigRspData {
    status: StatusCode,
    cfg_status: Vec<DeviceConfigStatus>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    setconfigrsp: SetConfigRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetConfigRspBuilder {
    pub cfg_status: Vec<DeviceConfigStatus>,
    pub status: StatusCode,
}
impl SetConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SetConfigRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_status_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < cfg_status_count * 2usize {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                wanted: cfg_status_count * 2usize,
                got: bytes.get().remaining(),
            });
        }
        let cfg_status = (0..cfg_status_count)
            .map(|_| DeviceConfigStatus::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { status, cfg_status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.cfg_status.len() as u8);
        for elem in &self.cfg_status {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self
            .cfg_status
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for SetConfigRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SetConfigRsp> for Bytes {
    fn from(packet: SetConfigRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SetConfigRsp> for Vec<u8> {
    fn from(packet: SetConfigRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SetConfigRsp> for UciPacket {
    fn from(packet: SetConfigRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SetConfigRsp> for UciResponse {
    fn from(packet: SetConfigRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SetConfigRsp> for CoreResponse {
    fn from(packet: SetConfigRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SetConfigRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SetConfigRsp> {
        SetConfigRsp::new(packet.ucipacket)
    }
}
impl SetConfigRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let setconfigrsp = match &coreresponse.child {
            CoreResponseDataChild::SetConfigRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::SetConfigRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            setconfigrsp,
        })
    }
    pub fn get_cfg_status(&self) -> &Vec<DeviceConfigStatus> {
        &self.setconfigrsp.cfg_status
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.setconfigrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.setconfigrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SetConfigRspBuilder {
    pub fn build(self) -> SetConfigRsp {
        let setconfigrsp = SetConfigRspData {
            cfg_status: self.cfg_status,
            status: self.status,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::SetConfigRsp(setconfigrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 4,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SetConfigRsp::new(ucipacket).unwrap()
    }
}
impl From<SetConfigRspBuilder> for UciPacket {
    fn from(builder: SetConfigRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SetConfigRspBuilder> for UciResponse {
    fn from(builder: SetConfigRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SetConfigRspBuilder> for CoreResponse {
    fn from(builder: SetConfigRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<SetConfigRspBuilder> for SetConfigRsp {
    fn from(builder: SetConfigRspBuilder) -> SetConfigRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigCmdData {
    cfg_id: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getconfigcmd: GetConfigCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigCmdBuilder {
    pub cfg_id: Vec<u8>,
}
impl GetConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < cfg_id_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigCmd".to_string(),
                wanted: cfg_id_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id = (0..cfg_id_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cfg_id })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.cfg_id.len() as u8);
        for elem in &self.cfg_id {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.cfg_id.len()
    }
}
impl Packet for GetConfigCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetConfigCmd> for Bytes {
    fn from(packet: GetConfigCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<GetConfigCmd> for Vec<u8> {
    fn from(packet: GetConfigCmd) -> Self {
        packet.to_vec()
    }
}
impl From<GetConfigCmd> for UciPacket {
    fn from(packet: GetConfigCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetConfigCmd> for UciCommand {
    fn from(packet: GetConfigCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<GetConfigCmd> for CoreCommand {
    fn from(packet: GetConfigCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetConfigCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetConfigCmd> {
        GetConfigCmd::new(packet.ucipacket)
    }
}
impl GetConfigCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let getconfigcmd = match &corecommand.child {
            CoreCommandDataChild::GetConfigCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::GetConfigCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            getconfigcmd,
        })
    }
    pub fn get_cfg_id(&self) -> &Vec<u8> {
        &self.getconfigcmd.cfg_id
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getconfigcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetConfigCmdBuilder {
    pub fn build(self) -> GetConfigCmd {
        let getconfigcmd = GetConfigCmdData {
            cfg_id: self.cfg_id,
        };
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::GetConfigCmd(getconfigcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 5,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        GetConfigCmd::new(ucipacket).unwrap()
    }
}
impl From<GetConfigCmdBuilder> for UciPacket {
    fn from(builder: GetConfigCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetConfigCmdBuilder> for UciCommand {
    fn from(builder: GetConfigCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<GetConfigCmdBuilder> for CoreCommand {
    fn from(builder: GetConfigCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<GetConfigCmdBuilder> for GetConfigCmd {
    fn from(builder: GetConfigCmdBuilder) -> GetConfigCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigRspData {
    status: StatusCode,
    tlvs: Vec<DeviceConfigTlv>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    getconfigrsp: GetConfigRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetConfigRspBuilder {
    pub status: StatusCode,
    pub tlvs: Vec<DeviceConfigTlv>,
}
impl GetConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "GetConfigRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tlvs_count = bytes.get_mut().get_u8() as usize;
        let tlvs = (0..tlvs_count)
            .map(|_| DeviceConfigTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { status, tlvs })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.tlvs.len() as u8);
        for elem in &self.tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.tlvs.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for GetConfigRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetConfigRsp> for Bytes {
    fn from(packet: GetConfigRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<GetConfigRsp> for Vec<u8> {
    fn from(packet: GetConfigRsp) -> Self {
        packet.to_vec()
    }
}
impl From<GetConfigRsp> for UciPacket {
    fn from(packet: GetConfigRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GetConfigRsp> for UciResponse {
    fn from(packet: GetConfigRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<GetConfigRsp> for CoreResponse {
    fn from(packet: GetConfigRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GetConfigRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GetConfigRsp> {
        GetConfigRsp::new(packet.ucipacket)
    }
}
impl GetConfigRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let getconfigrsp = match &coreresponse.child {
            CoreResponseDataChild::GetConfigRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::GetConfigRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            getconfigrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.getconfigrsp.status
    }
    pub fn get_tlvs(&self) -> &Vec<DeviceConfigTlv> {
        &self.getconfigrsp.tlvs
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.getconfigrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GetConfigRspBuilder {
    pub fn build(self) -> GetConfigRsp {
        let getconfigrsp = GetConfigRspData {
            status: self.status,
            tlvs: self.tlvs,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::GetConfigRsp(getconfigrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 5,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        GetConfigRsp::new(ucipacket).unwrap()
    }
}
impl From<GetConfigRspBuilder> for UciPacket {
    fn from(builder: GetConfigRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GetConfigRspBuilder> for UciResponse {
    fn from(builder: GetConfigRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<GetConfigRspBuilder> for CoreResponse {
    fn from(builder: GetConfigRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<GetConfigRspBuilder> for GetConfigRsp {
    fn from(builder: GetConfigRspBuilder) -> GetConfigRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericErrorData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericError {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corenotification: CoreNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    genericerror: GenericErrorData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericErrorBuilder {
    pub status: StatusCode,
}
impl GenericErrorData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "GenericError".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "GenericError".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for GenericError {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GenericError> for Bytes {
    fn from(packet: GenericError) -> Self {
        packet.to_bytes()
    }
}
impl From<GenericError> for Vec<u8> {
    fn from(packet: GenericError) -> Self {
        packet.to_vec()
    }
}
impl From<GenericError> for UciPacket {
    fn from(packet: GenericError) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<GenericError> for UciNotification {
    fn from(packet: GenericError) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<GenericError> for CoreNotification {
    fn from(packet: GenericError) -> CoreNotification {
        CoreNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for GenericError {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<GenericError> {
        GenericError::new(packet.ucipacket)
    }
}
impl GenericError {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corenotification = match &ucinotification.child {
            UciNotificationDataChild::CoreNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::CoreNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let genericerror = match &corenotification.child {
            CoreNotificationDataChild::GenericError(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreNotificationDataChild::GenericError),
                    actual: format!("{:?}", &corenotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            corenotification,
            genericerror,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.genericerror.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.genericerror.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl GenericErrorBuilder {
    pub fn build(self) -> GenericError {
        let genericerror = GenericErrorData {
            status: self.status,
        };
        let corenotification = CoreNotificationData {
            child: CoreNotificationDataChild::GenericError(genericerror),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(corenotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Notification,
            opcode: 7,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        GenericError::new(ucipacket).unwrap()
    }
}
impl From<GenericErrorBuilder> for UciPacket {
    fn from(builder: GenericErrorBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<GenericErrorBuilder> for UciNotification {
    fn from(builder: GenericErrorBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<GenericErrorBuilder> for CoreNotification {
    fn from(builder: GenericErrorBuilder) -> CoreNotification {
        builder.build().into()
    }
}
impl From<GenericErrorBuilder> for GenericError {
    fn from(builder: GenericErrorBuilder) -> GenericError {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corecommand: CoreCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corequerytimestampcmd: CoreQueryTimeStampCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampCmdBuilder {}
impl CoreQueryTimeStampCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for CoreQueryTimeStampCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreQueryTimeStampCmd> for Bytes {
    fn from(packet: CoreQueryTimeStampCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreQueryTimeStampCmd> for Vec<u8> {
    fn from(packet: CoreQueryTimeStampCmd) -> Self {
        packet.to_vec()
    }
}
impl From<CoreQueryTimeStampCmd> for UciPacket {
    fn from(packet: CoreQueryTimeStampCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampCmd> for UciCommand {
    fn from(packet: CoreQueryTimeStampCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampCmd> for CoreCommand {
    fn from(packet: CoreQueryTimeStampCmd) -> CoreCommand {
        CoreCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for CoreQueryTimeStampCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<CoreQueryTimeStampCmd> {
        CoreQueryTimeStampCmd::new(packet.ucipacket)
    }
}
impl CoreQueryTimeStampCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let corecommand = match &ucicommand.child {
            UciCommandDataChild::CoreCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::CoreCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let corequerytimestampcmd = match &corecommand.child {
            CoreCommandDataChild::CoreQueryTimeStampCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreCommandDataChild::CoreQueryTimeStampCmd),
                    actual: format!("{:?}", &corecommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            corecommand,
            corequerytimestampcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.corequerytimestampcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl CoreQueryTimeStampCmdBuilder {
    pub fn build(self) -> CoreQueryTimeStampCmd {
        let corequerytimestampcmd = CoreQueryTimeStampCmdData {};
        let corecommand = CoreCommandData {
            child: CoreCommandDataChild::CoreQueryTimeStampCmd(corequerytimestampcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::CoreCommand(corecommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Command,
            opcode: 8,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        CoreQueryTimeStampCmd::new(ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampCmdBuilder> for UciPacket {
    fn from(builder: CoreQueryTimeStampCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampCmdBuilder> for UciCommand {
    fn from(builder: CoreQueryTimeStampCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampCmdBuilder> for CoreCommand {
    fn from(builder: CoreQueryTimeStampCmdBuilder) -> CoreCommand {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampCmdBuilder> for CoreQueryTimeStampCmd {
    fn from(builder: CoreQueryTimeStampCmdBuilder) -> CoreQueryTimeStampCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampRspData {
    status: StatusCode,
    timeStamp: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    coreresponse: CoreResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    corequerytimestamprsp: CoreQueryTimeStampRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoreQueryTimeStampRspBuilder {
    pub status: StatusCode,
    pub timeStamp: u64,
}
impl CoreQueryTimeStampRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 9
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CoreQueryTimeStampRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "CoreQueryTimeStampRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "CoreQueryTimeStampRsp".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        let timeStamp = bytes.get_mut().get_u64_le();
        Ok(Self { status, timeStamp })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u64_le(self.timeStamp);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        9
    }
}
impl Packet for CoreQueryTimeStampRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreQueryTimeStampRsp> for Bytes {
    fn from(packet: CoreQueryTimeStampRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreQueryTimeStampRsp> for Vec<u8> {
    fn from(packet: CoreQueryTimeStampRsp) -> Self {
        packet.to_vec()
    }
}
impl From<CoreQueryTimeStampRsp> for UciPacket {
    fn from(packet: CoreQueryTimeStampRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampRsp> for UciResponse {
    fn from(packet: CoreQueryTimeStampRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampRsp> for CoreResponse {
    fn from(packet: CoreQueryTimeStampRsp) -> CoreResponse {
        CoreResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for CoreQueryTimeStampRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<CoreQueryTimeStampRsp> {
        CoreQueryTimeStampRsp::new(packet.ucipacket)
    }
}
impl CoreQueryTimeStampRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let coreresponse = match &uciresponse.child {
            UciResponseDataChild::CoreResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::CoreResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let corequerytimestamprsp = match &coreresponse.child {
            CoreResponseDataChild::CoreQueryTimeStampRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(CoreResponseDataChild::CoreQueryTimeStampRsp),
                    actual: format!("{:?}", &coreresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            coreresponse,
            corequerytimestamprsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.corequerytimestamprsp.status
    }
    pub fn get_timeStamp(&self) -> u64 {
        self.corequerytimestamprsp.timeStamp
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.corequerytimestamprsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl CoreQueryTimeStampRspBuilder {
    pub fn build(self) -> CoreQueryTimeStampRsp {
        let corequerytimestamprsp = CoreQueryTimeStampRspData {
            status: self.status,
            timeStamp: self.timeStamp,
        };
        let coreresponse = CoreResponseData {
            child: CoreResponseDataChild::CoreQueryTimeStampRsp(corequerytimestamprsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::CoreResponse(coreresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Core,
            message_type: MessageType::Response,
            opcode: 8,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        CoreQueryTimeStampRsp::new(ucipacket).unwrap()
    }
}
impl From<CoreQueryTimeStampRspBuilder> for UciPacket {
    fn from(builder: CoreQueryTimeStampRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampRspBuilder> for UciResponse {
    fn from(builder: CoreQueryTimeStampRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampRspBuilder> for CoreResponse {
    fn from(builder: CoreQueryTimeStampRspBuilder) -> CoreResponse {
        builder.build().into()
    }
}
impl From<CoreQueryTimeStampRspBuilder> for CoreQueryTimeStampRsp {
    fn from(builder: CoreQueryTimeStampRspBuilder) -> CoreQueryTimeStampRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitCmdData {
    session_id: u32,
    session_type: SessionType,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninitcmd: SessionInitCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitCmdBuilder {
    pub session_id: u32,
    pub session_type: SessionType,
}
impl SessionInitCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_id = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let session_type =
            SessionType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "SessionInitCmd".to_string(),
                    field: "session_type".to_string(),
                    value: unknown_val as u64,
                    type_: "SessionType".to_string(),
                }
            })?;
        Ok(Self {
            session_id,
            session_type,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_id);
        buffer.put_u8(u8::from(self.session_type));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5
    }
}
impl Packet for SessionInitCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInitCmd> for Bytes {
    fn from(packet: SessionInitCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInitCmd> for Vec<u8> {
    fn from(packet: SessionInitCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionInitCmd> for UciPacket {
    fn from(packet: SessionInitCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitCmd> for UciCommand {
    fn from(packet: SessionInitCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitCmd> for SessionConfigCommand {
    fn from(packet: SessionInitCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionInitCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionInitCmd> {
        SessionInitCmd::new(packet.ucipacket)
    }
}
impl SessionInitCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessioninitcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionInitCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionInitCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessioninitcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_id(&self) -> u32 {
        self.sessioninitcmd.session_id
    }
    pub fn get_session_type(&self) -> SessionType {
        self.sessioninitcmd.session_type
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioninitcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionInitCmdBuilder {
    pub fn build(self) -> SessionInitCmd {
        let sessioninitcmd = SessionInitCmdData {
            session_id: self.session_id,
            session_type: self.session_type,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionInitCmd(sessioninitcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionInitCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionInitCmdBuilder> for UciPacket {
    fn from(builder: SessionInitCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionInitCmdBuilder> for UciCommand {
    fn from(builder: SessionInitCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionInitCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionInitCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionInitCmdBuilder> for SessionInitCmd {
    fn from(builder: SessionInitCmdBuilder) -> SessionInitCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRsp_V2Data {
    status: StatusCode,
    session_handle: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRsp_V2 {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninitrsp_v2: SessionInitRsp_V2Data,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRsp_V2Builder {
    pub session_handle: u32,
    pub status: StatusCode,
}
impl SessionInitRsp_V2Data {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitRsp_V2".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionInitRsp_V2".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitRsp_V2".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_handle = bytes.get_mut().get_u32_le();
        Ok(Self {
            status,
            session_handle,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u32_le(self.session_handle);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5
    }
}
impl Packet for SessionInitRsp_V2 {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInitRsp_V2> for Bytes {
    fn from(packet: SessionInitRsp_V2) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInitRsp_V2> for Vec<u8> {
    fn from(packet: SessionInitRsp_V2) -> Self {
        packet.to_vec()
    }
}
impl From<SessionInitRsp_V2> for UciPacket {
    fn from(packet: SessionInitRsp_V2) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitRsp_V2> for UciResponse {
    fn from(packet: SessionInitRsp_V2) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitRsp_V2> for SessionConfigResponse {
    fn from(packet: SessionInitRsp_V2) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionInitRsp_V2 {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionInitRsp_V2> {
        SessionInitRsp_V2::new(packet.ucipacket)
    }
}
impl SessionInitRsp_V2 {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessioninitrsp_v2 = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionInitRsp_V2(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionInitRsp_V2),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessioninitrsp_v2,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_handle(&self) -> u32 {
        self.sessioninitrsp_v2.session_handle
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessioninitrsp_v2.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioninitrsp_v2.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionInitRsp_V2Builder {
    pub fn build(self) -> SessionInitRsp_V2 {
        let sessioninitrsp_v2 = SessionInitRsp_V2Data {
            session_handle: self.session_handle,
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionInitRsp_V2(sessioninitrsp_v2),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionInitRsp_V2::new(ucipacket).unwrap()
    }
}
impl From<SessionInitRsp_V2Builder> for UciPacket {
    fn from(builder: SessionInitRsp_V2Builder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionInitRsp_V2Builder> for UciResponse {
    fn from(builder: SessionInitRsp_V2Builder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionInitRsp_V2Builder> for SessionConfigResponse {
    fn from(builder: SessionInitRsp_V2Builder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionInitRsp_V2Builder> for SessionInitRsp_V2 {
    fn from(builder: SessionInitRsp_V2Builder) -> SessionInitRsp_V2 {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninitrsp: SessionInitRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInitRspBuilder {
    pub status: StatusCode,
}
impl SessionInitRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionInitRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionInitRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInitRsp> for Bytes {
    fn from(packet: SessionInitRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInitRsp> for Vec<u8> {
    fn from(packet: SessionInitRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionInitRsp> for UciPacket {
    fn from(packet: SessionInitRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitRsp> for UciResponse {
    fn from(packet: SessionInitRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInitRsp> for SessionConfigResponse {
    fn from(packet: SessionInitRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionInitRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionInitRsp> {
        SessionInitRsp::new(packet.ucipacket)
    }
}
impl SessionInitRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessioninitrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionInitRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionInitRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessioninitrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessioninitrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioninitrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionInitRspBuilder {
    pub fn build(self) -> SessionInitRsp {
        let sessioninitrsp = SessionInitRspData {
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionInitRsp(sessioninitrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionInitRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionInitRspBuilder> for UciPacket {
    fn from(builder: SessionInitRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionInitRspBuilder> for UciResponse {
    fn from(builder: SessionInitRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionInitRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionInitRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionInitRspBuilder> for SessionInitRsp {
    fn from(builder: SessionInitRspBuilder) -> SessionInitRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitCmdData {
    session_token: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiondeinitcmd: SessionDeinitCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitCmdBuilder {
    pub session_token: u32,
}
impl SessionDeinitCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionDeinitCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        Ok(Self { session_token })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4
    }
}
impl Packet for SessionDeinitCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionDeinitCmd> for Bytes {
    fn from(packet: SessionDeinitCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionDeinitCmd> for Vec<u8> {
    fn from(packet: SessionDeinitCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionDeinitCmd> for UciPacket {
    fn from(packet: SessionDeinitCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionDeinitCmd> for UciCommand {
    fn from(packet: SessionDeinitCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionDeinitCmd> for SessionConfigCommand {
    fn from(packet: SessionDeinitCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionDeinitCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionDeinitCmd> {
        SessionDeinitCmd::new(packet.ucipacket)
    }
}
impl SessionDeinitCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessiondeinitcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionDeinitCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionDeinitCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessiondeinitcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessiondeinitcmd.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiondeinitcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionDeinitCmdBuilder {
    pub fn build(self) -> SessionDeinitCmd {
        let sessiondeinitcmd = SessionDeinitCmdData {
            session_token: self.session_token,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionDeinitCmd(sessiondeinitcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionDeinitCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionDeinitCmdBuilder> for UciPacket {
    fn from(builder: SessionDeinitCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionDeinitCmdBuilder> for UciCommand {
    fn from(builder: SessionDeinitCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionDeinitCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionDeinitCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionDeinitCmdBuilder> for SessionDeinitCmd {
    fn from(builder: SessionDeinitCmdBuilder) -> SessionDeinitCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiondeinitrsp: SessionDeinitRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionDeinitRspBuilder {
    pub status: StatusCode,
}
impl SessionDeinitRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionDeinitRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionDeinitRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionDeinitRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionDeinitRsp> for Bytes {
    fn from(packet: SessionDeinitRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionDeinitRsp> for Vec<u8> {
    fn from(packet: SessionDeinitRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionDeinitRsp> for UciPacket {
    fn from(packet: SessionDeinitRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionDeinitRsp> for UciResponse {
    fn from(packet: SessionDeinitRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionDeinitRsp> for SessionConfigResponse {
    fn from(packet: SessionDeinitRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionDeinitRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionDeinitRsp> {
        SessionDeinitRsp::new(packet.ucipacket)
    }
}
impl SessionDeinitRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessiondeinitrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionDeinitRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionDeinitRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessiondeinitrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessiondeinitrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiondeinitrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionDeinitRspBuilder {
    pub fn build(self) -> SessionDeinitRsp {
        let sessiondeinitrsp = SessionDeinitRspData {
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionDeinitRsp(sessiondeinitrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionDeinitRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionDeinitRspBuilder> for UciPacket {
    fn from(builder: SessionDeinitRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionDeinitRspBuilder> for UciResponse {
    fn from(builder: SessionDeinitRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionDeinitRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionDeinitRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionDeinitRspBuilder> for SessionDeinitRsp {
    fn from(builder: SessionDeinitRspBuilder) -> SessionDeinitRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStatusNtfData {
    session_token: u32,
    session_state: SessionState,
    reason_code: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStatusNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfignotification: SessionConfigNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionstatusntf: SessionStatusNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStatusNtfBuilder {
    pub reason_code: u8,
    pub session_state: SessionState,
    pub session_token: u32,
}
impl SessionStatusNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 6
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let session_state =
            SessionState::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "SessionStatusNtf".to_string(),
                    field: "session_state".to_string(),
                    value: unknown_val as u64,
                    type_: "SessionState".to_string(),
                }
            })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let reason_code = bytes.get_mut().get_u8();
        Ok(Self {
            session_token,
            session_state,
            reason_code,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(u8::from(self.session_state));
        buffer.put_u8(self.reason_code);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        6
    }
}
impl Packet for SessionStatusNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStatusNtf> for Bytes {
    fn from(packet: SessionStatusNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStatusNtf> for Vec<u8> {
    fn from(packet: SessionStatusNtf) -> Self {
        packet.to_vec()
    }
}
impl From<SessionStatusNtf> for UciPacket {
    fn from(packet: SessionStatusNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStatusNtf> for UciNotification {
    fn from(packet: SessionStatusNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStatusNtf> for SessionConfigNotification {
    fn from(packet: SessionStatusNtf) -> SessionConfigNotification {
        SessionConfigNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionStatusNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionStatusNtf> {
        SessionStatusNtf::new(packet.ucipacket)
    }
}
impl SessionStatusNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfignotification = match &ucinotification.child {
            UciNotificationDataChild::SessionConfigNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionConfigNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessionstatusntf = match &sessionconfignotification.child {
            SessionConfigNotificationDataChild::SessionStatusNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigNotificationDataChild::SessionStatusNtf),
                    actual: format!("{:?}", &sessionconfignotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessionconfignotification,
            sessionstatusntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_reason_code(&self) -> u8 {
        self.sessionstatusntf.reason_code
    }
    pub fn get_session_state(&self) -> SessionState {
        self.sessionstatusntf.session_state
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionstatusntf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionstatusntf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionStatusNtfBuilder {
    pub fn build(self) -> SessionStatusNtf {
        let sessionstatusntf = SessionStatusNtfData {
            reason_code: self.reason_code,
            session_state: self.session_state,
            session_token: self.session_token,
        };
        let sessionconfignotification = SessionConfigNotificationData {
            child: SessionConfigNotificationDataChild::SessionStatusNtf(sessionstatusntf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionConfigNotification(sessionconfignotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Notification,
            opcode: 2,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        SessionStatusNtf::new(ucipacket).unwrap()
    }
}
impl From<SessionStatusNtfBuilder> for UciPacket {
    fn from(builder: SessionStatusNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionStatusNtfBuilder> for UciNotification {
    fn from(builder: SessionStatusNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<SessionStatusNtfBuilder> for SessionConfigNotification {
    fn from(builder: SessionStatusNtfBuilder) -> SessionConfigNotification {
        builder.build().into()
    }
}
impl From<SessionStatusNtfBuilder> for SessionStatusNtf {
    fn from(builder: SessionStatusNtfBuilder) -> SessionStatusNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppConfigTlv {
    pub cfg_id: AppConfigTlvType,
    pub v: Vec<u8>,
}
impl AppConfigTlv {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id =
            AppConfigTlvType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "AppConfigTlv".to_string(),
                    field: "cfg_id".to_string(),
                    value: unknown_val as u64,
                    type_: "AppConfigTlvType".to_string(),
                }
            })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let v_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < v_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigTlv".to_string(),
                wanted: v_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let v = (0..v_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cfg_id, v })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.cfg_id));
        buffer.put_u8(self.v.len() as u8);
        for elem in &self.v {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.v.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigCmdData {
    session_token: u32,
    tlvs: Vec<AppConfigTlv>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionsetappconfigcmd: SessionSetAppConfigCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigCmdBuilder {
    pub session_token: u32,
    pub tlvs: Vec<AppConfigTlv>,
}
impl SessionSetAppConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tlvs_count = bytes.get_mut().get_u8() as usize;
        let tlvs = (0..tlvs_count)
            .map(|_| AppConfigTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            session_token,
            tlvs,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.tlvs.len() as u8);
        for elem in &self.tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.tlvs.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for SessionSetAppConfigCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetAppConfigCmd> for Bytes {
    fn from(packet: SessionSetAppConfigCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetAppConfigCmd> for Vec<u8> {
    fn from(packet: SessionSetAppConfigCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionSetAppConfigCmd> for UciPacket {
    fn from(packet: SessionSetAppConfigCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigCmd> for UciCommand {
    fn from(packet: SessionSetAppConfigCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigCmd> for SessionConfigCommand {
    fn from(packet: SessionSetAppConfigCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionSetAppConfigCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionSetAppConfigCmd> {
        SessionSetAppConfigCmd::new(packet.ucipacket)
    }
}
impl SessionSetAppConfigCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionsetappconfigcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionSetAppConfigCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionSetAppConfigCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessionsetappconfigcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionsetappconfigcmd.session_token
    }
    pub fn get_tlvs(&self) -> &Vec<AppConfigTlv> {
        &self.sessionsetappconfigcmd.tlvs
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionsetappconfigcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionSetAppConfigCmdBuilder {
    pub fn build(self) -> SessionSetAppConfigCmd {
        let sessionsetappconfigcmd = SessionSetAppConfigCmdData {
            session_token: self.session_token,
            tlvs: self.tlvs,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionSetAppConfigCmd(sessionsetappconfigcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionSetAppConfigCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigCmdBuilder> for UciPacket {
    fn from(builder: SessionSetAppConfigCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigCmdBuilder> for UciCommand {
    fn from(builder: SessionSetAppConfigCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionSetAppConfigCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigCmdBuilder> for SessionSetAppConfigCmd {
    fn from(builder: SessionSetAppConfigCmdBuilder) -> SessionSetAppConfigCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppConfigStatus {
    pub cfg_id: AppConfigTlvType,
    pub status: StatusCode,
}
impl AppConfigStatus {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigStatus".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_id =
            AppConfigTlvType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "AppConfigStatus".to_string(),
                    field: "cfg_id".to_string(),
                    value: unknown_val as u64,
                    type_: "AppConfigTlvType".to_string(),
                }
            })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigStatus".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "AppConfigStatus".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { cfg_id, status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.cfg_id));
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigRspData {
    status: StatusCode,
    cfg_status: Vec<AppConfigStatus>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionsetappconfigrsp: SessionSetAppConfigRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetAppConfigRspBuilder {
    pub cfg_status: Vec<AppConfigStatus>,
    pub status: StatusCode,
}
impl SessionSetAppConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionSetAppConfigRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cfg_status_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < cfg_status_count * 2usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                wanted: cfg_status_count * 2usize,
                got: bytes.get().remaining(),
            });
        }
        let cfg_status = (0..cfg_status_count)
            .map(|_| AppConfigStatus::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { status, cfg_status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.cfg_status.len() as u8);
        for elem in &self.cfg_status {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self
            .cfg_status
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for SessionSetAppConfigRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetAppConfigRsp> for Bytes {
    fn from(packet: SessionSetAppConfigRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetAppConfigRsp> for Vec<u8> {
    fn from(packet: SessionSetAppConfigRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionSetAppConfigRsp> for UciPacket {
    fn from(packet: SessionSetAppConfigRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigRsp> for UciResponse {
    fn from(packet: SessionSetAppConfigRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigRsp> for SessionConfigResponse {
    fn from(packet: SessionSetAppConfigRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionSetAppConfigRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionSetAppConfigRsp> {
        SessionSetAppConfigRsp::new(packet.ucipacket)
    }
}
impl SessionSetAppConfigRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionsetappconfigrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionSetAppConfigRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionSetAppConfigRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessionsetappconfigrsp,
        })
    }
    pub fn get_cfg_status(&self) -> &Vec<AppConfigStatus> {
        &self.sessionsetappconfigrsp.cfg_status
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionsetappconfigrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionsetappconfigrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionSetAppConfigRspBuilder {
    pub fn build(self) -> SessionSetAppConfigRsp {
        let sessionsetappconfigrsp = SessionSetAppConfigRspData {
            cfg_status: self.cfg_status,
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionSetAppConfigRsp(sessionsetappconfigrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionSetAppConfigRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionSetAppConfigRspBuilder> for UciPacket {
    fn from(builder: SessionSetAppConfigRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigRspBuilder> for UciResponse {
    fn from(builder: SessionSetAppConfigRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionSetAppConfigRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionSetAppConfigRspBuilder> for SessionSetAppConfigRsp {
    fn from(builder: SessionSetAppConfigRspBuilder) -> SessionSetAppConfigRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigCmdData {
    session_token: u32,
    app_cfg: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetappconfigcmd: SessionGetAppConfigCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigCmdBuilder {
    pub app_cfg: Vec<u8>,
    pub session_token: u32,
}
impl SessionGetAppConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let app_cfg_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < app_cfg_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                wanted: app_cfg_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let app_cfg = (0..app_cfg_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            session_token,
            app_cfg,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.app_cfg.len() as u8);
        for elem in &self.app_cfg {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.app_cfg.len()
    }
}
impl Packet for SessionGetAppConfigCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetAppConfigCmd> for Bytes {
    fn from(packet: SessionGetAppConfigCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetAppConfigCmd> for Vec<u8> {
    fn from(packet: SessionGetAppConfigCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetAppConfigCmd> for UciPacket {
    fn from(packet: SessionGetAppConfigCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigCmd> for UciCommand {
    fn from(packet: SessionGetAppConfigCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigCmd> for SessionConfigCommand {
    fn from(packet: SessionGetAppConfigCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetAppConfigCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetAppConfigCmd> {
        SessionGetAppConfigCmd::new(packet.ucipacket)
    }
}
impl SessionGetAppConfigCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessiongetappconfigcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionGetAppConfigCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionGetAppConfigCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessiongetappconfigcmd,
        })
    }
    pub fn get_app_cfg(&self) -> &Vec<u8> {
        &self.sessiongetappconfigcmd.app_cfg
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessiongetappconfigcmd.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetappconfigcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetAppConfigCmdBuilder {
    pub fn build(self) -> SessionGetAppConfigCmd {
        let sessiongetappconfigcmd = SessionGetAppConfigCmdData {
            app_cfg: self.app_cfg,
            session_token: self.session_token,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionGetAppConfigCmd(sessiongetappconfigcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 4,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionGetAppConfigCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigCmdBuilder> for UciPacket {
    fn from(builder: SessionGetAppConfigCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigCmdBuilder> for UciCommand {
    fn from(builder: SessionGetAppConfigCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionGetAppConfigCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigCmdBuilder> for SessionGetAppConfigCmd {
    fn from(builder: SessionGetAppConfigCmdBuilder) -> SessionGetAppConfigCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigRspData {
    status: StatusCode,
    tlvs: Vec<AppConfigTlv>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetappconfigrsp: SessionGetAppConfigRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetAppConfigRspBuilder {
    pub status: StatusCode,
    pub tlvs: Vec<AppConfigTlv>,
}
impl SessionGetAppConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionGetAppConfigRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tlvs_count = bytes.get_mut().get_u8() as usize;
        let tlvs = (0..tlvs_count)
            .map(|_| AppConfigTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { status, tlvs })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.tlvs.len() as u8);
        for elem in &self.tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.tlvs.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for SessionGetAppConfigRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetAppConfigRsp> for Bytes {
    fn from(packet: SessionGetAppConfigRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetAppConfigRsp> for Vec<u8> {
    fn from(packet: SessionGetAppConfigRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetAppConfigRsp> for UciPacket {
    fn from(packet: SessionGetAppConfigRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigRsp> for UciResponse {
    fn from(packet: SessionGetAppConfigRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigRsp> for SessionConfigResponse {
    fn from(packet: SessionGetAppConfigRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetAppConfigRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetAppConfigRsp> {
        SessionGetAppConfigRsp::new(packet.ucipacket)
    }
}
impl SessionGetAppConfigRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessiongetappconfigrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionGetAppConfigRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionGetAppConfigRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessiongetappconfigrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessiongetappconfigrsp.status
    }
    pub fn get_tlvs(&self) -> &Vec<AppConfigTlv> {
        &self.sessiongetappconfigrsp.tlvs
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetappconfigrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetAppConfigRspBuilder {
    pub fn build(self) -> SessionGetAppConfigRsp {
        let sessiongetappconfigrsp = SessionGetAppConfigRspData {
            status: self.status,
            tlvs: self.tlvs,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionGetAppConfigRsp(sessiongetappconfigrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 4,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionGetAppConfigRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionGetAppConfigRspBuilder> for UciPacket {
    fn from(builder: SessionGetAppConfigRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigRspBuilder> for UciResponse {
    fn from(builder: SessionGetAppConfigRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionGetAppConfigRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionGetAppConfigRspBuilder> for SessionGetAppConfigRsp {
    fn from(builder: SessionGetAppConfigRspBuilder) -> SessionGetAppConfigRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetcountcmd: SessionGetCountCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountCmdBuilder {}
impl SessionGetCountCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for SessionGetCountCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetCountCmd> for Bytes {
    fn from(packet: SessionGetCountCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetCountCmd> for Vec<u8> {
    fn from(packet: SessionGetCountCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetCountCmd> for UciPacket {
    fn from(packet: SessionGetCountCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetCountCmd> for UciCommand {
    fn from(packet: SessionGetCountCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetCountCmd> for SessionConfigCommand {
    fn from(packet: SessionGetCountCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetCountCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetCountCmd> {
        SessionGetCountCmd::new(packet.ucipacket)
    }
}
impl SessionGetCountCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessiongetcountcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionGetCountCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionGetCountCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessiongetcountcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetcountcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetCountCmdBuilder {
    pub fn build(self) -> SessionGetCountCmd {
        let sessiongetcountcmd = SessionGetCountCmdData {};
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionGetCountCmd(sessiongetcountcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 5,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionGetCountCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionGetCountCmdBuilder> for UciPacket {
    fn from(builder: SessionGetCountCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetCountCmdBuilder> for UciCommand {
    fn from(builder: SessionGetCountCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionGetCountCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionGetCountCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionGetCountCmdBuilder> for SessionGetCountCmd {
    fn from(builder: SessionGetCountCmdBuilder) -> SessionGetCountCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountRspData {
    status: StatusCode,
    session_count: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetcountrsp: SessionGetCountRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetCountRspBuilder {
    pub session_count: u8,
    pub status: StatusCode,
}
impl SessionGetCountRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetCountRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionGetCountRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetCountRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let session_count = bytes.get_mut().get_u8();
        Ok(Self {
            status,
            session_count,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.session_count);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
impl Packet for SessionGetCountRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetCountRsp> for Bytes {
    fn from(packet: SessionGetCountRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetCountRsp> for Vec<u8> {
    fn from(packet: SessionGetCountRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetCountRsp> for UciPacket {
    fn from(packet: SessionGetCountRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetCountRsp> for UciResponse {
    fn from(packet: SessionGetCountRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetCountRsp> for SessionConfigResponse {
    fn from(packet: SessionGetCountRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetCountRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetCountRsp> {
        SessionGetCountRsp::new(packet.ucipacket)
    }
}
impl SessionGetCountRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessiongetcountrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionGetCountRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionGetCountRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessiongetcountrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_count(&self) -> u8 {
        self.sessiongetcountrsp.session_count
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessiongetcountrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetcountrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetCountRspBuilder {
    pub fn build(self) -> SessionGetCountRsp {
        let sessiongetcountrsp = SessionGetCountRspData {
            session_count: self.session_count,
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionGetCountRsp(sessiongetcountrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 5,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionGetCountRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionGetCountRspBuilder> for UciPacket {
    fn from(builder: SessionGetCountRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetCountRspBuilder> for UciResponse {
    fn from(builder: SessionGetCountRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionGetCountRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionGetCountRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionGetCountRspBuilder> for SessionGetCountRsp {
    fn from(builder: SessionGetCountRspBuilder) -> SessionGetCountRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateCmdData {
    session_token: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetstatecmd: SessionGetStateCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateCmdBuilder {
    pub session_token: u32,
}
impl SessionGetStateCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        Ok(Self { session_token })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4
    }
}
impl Packet for SessionGetStateCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetStateCmd> for Bytes {
    fn from(packet: SessionGetStateCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetStateCmd> for Vec<u8> {
    fn from(packet: SessionGetStateCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetStateCmd> for UciPacket {
    fn from(packet: SessionGetStateCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetStateCmd> for UciCommand {
    fn from(packet: SessionGetStateCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetStateCmd> for SessionConfigCommand {
    fn from(packet: SessionGetStateCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetStateCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetStateCmd> {
        SessionGetStateCmd::new(packet.ucipacket)
    }
}
impl SessionGetStateCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessiongetstatecmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionGetStateCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionGetStateCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessiongetstatecmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessiongetstatecmd.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetstatecmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetStateCmdBuilder {
    pub fn build(self) -> SessionGetStateCmd {
        let sessiongetstatecmd = SessionGetStateCmdData {
            session_token: self.session_token,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionGetStateCmd(sessiongetstatecmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 6,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionGetStateCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionGetStateCmdBuilder> for UciPacket {
    fn from(builder: SessionGetStateCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetStateCmdBuilder> for UciCommand {
    fn from(builder: SessionGetStateCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionGetStateCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionGetStateCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionGetStateCmdBuilder> for SessionGetStateCmd {
    fn from(builder: SessionGetStateCmdBuilder) -> SessionGetStateCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateRspData {
    status: StatusCode,
    session_state: SessionState,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetstatersp: SessionGetStateRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetStateRspBuilder {
    pub session_state: SessionState,
    pub status: StatusCode,
}
impl SessionGetStateRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionGetStateRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let session_state =
            SessionState::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "SessionGetStateRsp".to_string(),
                    field: "session_state".to_string(),
                    value: unknown_val as u64,
                    type_: "SessionState".to_string(),
                }
            })?;
        Ok(Self {
            status,
            session_state,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(u8::from(self.session_state));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
impl Packet for SessionGetStateRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetStateRsp> for Bytes {
    fn from(packet: SessionGetStateRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetStateRsp> for Vec<u8> {
    fn from(packet: SessionGetStateRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetStateRsp> for UciPacket {
    fn from(packet: SessionGetStateRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetStateRsp> for UciResponse {
    fn from(packet: SessionGetStateRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetStateRsp> for SessionConfigResponse {
    fn from(packet: SessionGetStateRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetStateRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetStateRsp> {
        SessionGetStateRsp::new(packet.ucipacket)
    }
}
impl SessionGetStateRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessiongetstatersp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionGetStateRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionGetStateRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessiongetstatersp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_state(&self) -> SessionState {
        self.sessiongetstatersp.session_state
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessiongetstatersp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetstatersp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetStateRspBuilder {
    pub fn build(self) -> SessionGetStateRsp {
        let sessiongetstatersp = SessionGetStateRspData {
            session_state: self.session_state,
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionGetStateRsp(sessiongetstatersp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 6,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionGetStateRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionGetStateRspBuilder> for UciPacket {
    fn from(builder: SessionGetStateRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetStateRspBuilder> for UciResponse {
    fn from(builder: SessionGetStateRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionGetStateRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionGetStateRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionGetStateRspBuilder> for SessionGetStateRsp {
    fn from(builder: SessionGetStateRspBuilder) -> SessionGetStateRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsCmdData {
    session_token: u32,
    ranging_round_indexes: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionupdatedttagrangingroundscmd: SessionUpdateDtTagRangingRoundsCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsCmdBuilder {
    pub ranging_round_indexes: Vec<u8>,
    pub session_token: u32,
}
impl SessionUpdateDtTagRangingRoundsCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let ranging_round_indexes_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < ranging_round_indexes_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsCmd".to_string(),
                wanted: ranging_round_indexes_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let ranging_round_indexes = (0..ranging_round_indexes_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            session_token,
            ranging_round_indexes,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.ranging_round_indexes.len() as u8);
        for elem in &self.ranging_round_indexes {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.ranging_round_indexes.len()
    }
}
impl Packet for SessionUpdateDtTagRangingRoundsCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmd> for Bytes {
    fn from(packet: SessionUpdateDtTagRangingRoundsCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmd> for Vec<u8> {
    fn from(packet: SessionUpdateDtTagRangingRoundsCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmd> for UciPacket {
    fn from(packet: SessionUpdateDtTagRangingRoundsCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmd> for UciCommand {
    fn from(packet: SessionUpdateDtTagRangingRoundsCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmd> for SessionConfigCommand {
    fn from(packet: SessionUpdateDtTagRangingRoundsCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionUpdateDtTagRangingRoundsCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionUpdateDtTagRangingRoundsCmd> {
        SessionUpdateDtTagRangingRoundsCmd::new(packet.ucipacket)
    }
}
impl SessionUpdateDtTagRangingRoundsCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionupdatedttagrangingroundscmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(value) => {
                value.clone()
            }
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd
                    ),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessionupdatedttagrangingroundscmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_round_indexes(&self) -> &Vec<u8> {
        &self
            .sessionupdatedttagrangingroundscmd
            .ranging_round_indexes
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionupdatedttagrangingroundscmd.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionupdatedttagrangingroundscmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionUpdateDtTagRangingRoundsCmdBuilder {
    pub fn build(self) -> SessionUpdateDtTagRangingRoundsCmd {
        let sessionupdatedttagrangingroundscmd = SessionUpdateDtTagRangingRoundsCmdData {
            ranging_round_indexes: self.ranging_round_indexes,
            session_token: self.session_token,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionUpdateDtTagRangingRoundsCmd(
                sessionupdatedttagrangingroundscmd,
            ),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 9,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionUpdateDtTagRangingRoundsCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmdBuilder> for UciPacket {
    fn from(builder: SessionUpdateDtTagRangingRoundsCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmdBuilder> for UciCommand {
    fn from(builder: SessionUpdateDtTagRangingRoundsCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionUpdateDtTagRangingRoundsCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsCmdBuilder> for SessionUpdateDtTagRangingRoundsCmd {
    fn from(
        builder: SessionUpdateDtTagRangingRoundsCmdBuilder,
    ) -> SessionUpdateDtTagRangingRoundsCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsRspData {
    status: StatusCode,
    ranging_round_indexes: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionupdatedttagrangingroundsrsp: SessionUpdateDtTagRangingRoundsRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateDtTagRangingRoundsRspBuilder {
    pub ranging_round_indexes: Vec<u8>,
    pub status: StatusCode,
}
impl SessionUpdateDtTagRangingRoundsRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionUpdateDtTagRangingRoundsRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let ranging_round_indexes_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < ranging_round_indexes_count * 1usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateDtTagRangingRoundsRsp".to_string(),
                wanted: ranging_round_indexes_count * 1usize,
                got: bytes.get().remaining(),
            });
        }
        let ranging_round_indexes = (0..ranging_round_indexes_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            status,
            ranging_round_indexes,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.ranging_round_indexes.len() as u8);
        for elem in &self.ranging_round_indexes {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.ranging_round_indexes.len()
    }
}
impl Packet for SessionUpdateDtTagRangingRoundsRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRsp> for Bytes {
    fn from(packet: SessionUpdateDtTagRangingRoundsRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRsp> for Vec<u8> {
    fn from(packet: SessionUpdateDtTagRangingRoundsRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRsp> for UciPacket {
    fn from(packet: SessionUpdateDtTagRangingRoundsRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRsp> for UciResponse {
    fn from(packet: SessionUpdateDtTagRangingRoundsRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRsp> for SessionConfigResponse {
    fn from(packet: SessionUpdateDtTagRangingRoundsRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionUpdateDtTagRangingRoundsRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionUpdateDtTagRangingRoundsRsp> {
        SessionUpdateDtTagRangingRoundsRsp::new(packet.ucipacket)
    }
}
impl SessionUpdateDtTagRangingRoundsRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionupdatedttagrangingroundsrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(value) => {
                value.clone()
            }
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp
                    ),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessionupdatedttagrangingroundsrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_round_indexes(&self) -> &Vec<u8> {
        &self
            .sessionupdatedttagrangingroundsrsp
            .ranging_round_indexes
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionupdatedttagrangingroundsrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionupdatedttagrangingroundsrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionUpdateDtTagRangingRoundsRspBuilder {
    pub fn build(self) -> SessionUpdateDtTagRangingRoundsRsp {
        let sessionupdatedttagrangingroundsrsp = SessionUpdateDtTagRangingRoundsRspData {
            ranging_round_indexes: self.ranging_round_indexes,
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionUpdateDtTagRangingRoundsRsp(
                sessionupdatedttagrangingroundsrsp,
            ),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 9,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionUpdateDtTagRangingRoundsRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRspBuilder> for UciPacket {
    fn from(builder: SessionUpdateDtTagRangingRoundsRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRspBuilder> for UciResponse {
    fn from(builder: SessionUpdateDtTagRangingRoundsRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionUpdateDtTagRangingRoundsRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionUpdateDtTagRangingRoundsRspBuilder> for SessionUpdateDtTagRangingRoundsRsp {
    fn from(
        builder: SessionUpdateDtTagRangingRoundsRspBuilder,
    ) -> SessionUpdateDtTagRangingRoundsRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Controlee {
    pub short_address: [u8; 2],
    pub subsession_id: u32,
}
impl Controlee {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 6
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let short_address = (0..2)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let subsession_id = bytes.get_mut().get_u32_le();
        Ok(Self {
            short_address,
            subsession_id,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.short_address {
            buffer.put_u8(*elem);
        }
        buffer.put_u32_le(self.subsession_id);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        6
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Controlee_V2_0_16_Byte_Version {
    pub short_address: [u8; 2],
    pub subsession_id: u32,
    pub subsession_key: [u8; 16],
}
impl Controlee_V2_0_16_Byte_Version {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 22
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_16_Byte_Version".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let short_address = (0..2)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_16_Byte_Version".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let subsession_id = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 16 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_16_Byte_Version".to_string(),
                wanted: 16,
                got: bytes.get().remaining(),
            });
        }
        let subsession_key = (0..16)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        Ok(Self {
            short_address,
            subsession_id,
            subsession_key,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.short_address {
            buffer.put_u8(*elem);
        }
        buffer.put_u32_le(self.subsession_id);
        for elem in &self.subsession_key {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        22
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Controlee_V2_0_32_Byte_Version {
    pub short_address: [u8; 2],
    pub subsession_id: u32,
    pub subsession_key: [u8; 32],
}
impl Controlee_V2_0_32_Byte_Version {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 38
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_32_Byte_Version".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let short_address = (0..2)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_32_Byte_Version".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let subsession_id = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 32 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee_V2_0_32_Byte_Version".to_string(),
                wanted: 32,
                got: bytes.get().remaining(),
            });
        }
        let subsession_key = (0..32)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        Ok(Self {
            short_address,
            subsession_id,
            subsession_key,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.short_address {
            buffer.put_u8(*elem);
        }
        buffer.put_u32_le(self.subsession_id);
        for elem in &self.subsession_key {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        38
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum UpdateMulticastListAction {
    AddControlee = 0x0,
    RemoveControlee = 0x1,
    AddControleeWithShortSubSessionKey = 0x2,
    AddControleeWithLongSubSessionKey = 0x3,
}
impl TryFrom<u8> for UpdateMulticastListAction {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(UpdateMulticastListAction::AddControlee),
            0x1 => Ok(UpdateMulticastListAction::RemoveControlee),
            0x2 => Ok(UpdateMulticastListAction::AddControleeWithShortSubSessionKey),
            0x3 => Ok(UpdateMulticastListAction::AddControleeWithLongSubSessionKey),
            _ => Err(value),
        }
    }
}
impl From<&UpdateMulticastListAction> for u8 {
    fn from(value: &UpdateMulticastListAction) -> Self {
        match value {
            UpdateMulticastListAction::AddControlee => 0x0,
            UpdateMulticastListAction::RemoveControlee => 0x1,
            UpdateMulticastListAction::AddControleeWithShortSubSessionKey => 0x2,
            UpdateMulticastListAction::AddControleeWithLongSubSessionKey => 0x3,
        }
    }
}
impl From<UpdateMulticastListAction> for u8 {
    fn from(value: UpdateMulticastListAction) -> Self {
        (&value).into()
    }
}
impl From<UpdateMulticastListAction> for i16 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
impl From<UpdateMulticastListAction> for i32 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
impl From<UpdateMulticastListAction> for i64 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
impl From<UpdateMulticastListAction> for u16 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
impl From<UpdateMulticastListAction> for u32 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
impl From<UpdateMulticastListAction> for u64 {
    fn from(value: UpdateMulticastListAction) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionUpdateControllerMulticastListCmdDataChild {
    Payload(Bytes),
    None,
}
impl SessionUpdateControllerMulticastListCmdDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionUpdateControllerMulticastListCmdDataChild::Payload(bytes) => bytes.len(),
            SessionUpdateControllerMulticastListCmdDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionUpdateControllerMulticastListCmdChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmdData {
    session_token: u32,
    action: UpdateMulticastListAction,
    child: SessionUpdateControllerMulticastListCmdDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionupdatecontrollermulticastlistcmd: SessionUpdateControllerMulticastListCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmdBuilder {
    pub action: UpdateMulticastListAction,
    pub session_token: u32,
    pub payload: Option<Bytes>,
}
impl SessionUpdateControllerMulticastListCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let action = UpdateMulticastListAction::try_from(bytes.get_mut().get_u8()).map_err(
            |unknown_val| Error::InvalidEnumValueError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                field: "action".to_string(),
                value: unknown_val as u64,
                type_: "UpdateMulticastListAction".to_string(),
            },
        )?;
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => SessionUpdateControllerMulticastListCmdDataChild::Payload(
                Bytes::copy_from_slice(payload),
            ),
            _ => SessionUpdateControllerMulticastListCmdDataChild::None,
        };
        Ok(Self {
            session_token,
            action,
            child,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(u8::from(self.action));
        match &self.child {
            SessionUpdateControllerMulticastListCmdDataChild::Payload(payload) => {
                buffer.put_slice(payload)
            }
            SessionUpdateControllerMulticastListCmdDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.child.get_total_size()
    }
}
impl Packet for SessionUpdateControllerMulticastListCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListCmd> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListCmd> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListCmd> for UciPacket {
    fn from(packet: SessionUpdateControllerMulticastListCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListCmd> for UciCommand {
    fn from(packet: SessionUpdateControllerMulticastListCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListCmd> for SessionConfigCommand {
    fn from(packet: SessionUpdateControllerMulticastListCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionUpdateControllerMulticastListCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionUpdateControllerMulticastListCmd> {
        SessionUpdateControllerMulticastListCmd::new(packet.ucipacket)
    }
}
impl SessionUpdateControllerMulticastListCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionUpdateControllerMulticastListCmdChild {
        match &self.sessionupdatecontrollermulticastlistcmd.child {
            SessionUpdateControllerMulticastListCmdDataChild::Payload(payload) => {
                SessionUpdateControllerMulticastListCmdChild::Payload(payload.clone())
            }
            SessionUpdateControllerMulticastListCmdDataChild::None => {
                SessionUpdateControllerMulticastListCmdChild::None
            }
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionupdatecontrollermulticastlistcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(value) => {
                value.clone()
            }
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd
                    ),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessionupdatecontrollermulticastlistcmd,
        })
    }
    pub fn get_action(&self) -> UpdateMulticastListAction {
        self.sessionupdatecontrollermulticastlistcmd.action
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionupdatecontrollermulticastlistcmd.session_token
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.sessionupdatecontrollermulticastlistcmd.child {
            SessionUpdateControllerMulticastListCmdDataChild::Payload(bytes) => &bytes,
            SessionUpdateControllerMulticastListCmdDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionupdatecontrollermulticastlistcmd
            .write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionUpdateControllerMulticastListCmdBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListCmd {
        let sessionupdatecontrollermulticastlistcmd = SessionUpdateControllerMulticastListCmdData {
            action: self.action,
            session_token: self.session_token,
            child: match self.payload {
                None => SessionUpdateControllerMulticastListCmdDataChild::None,
                Some(bytes) => SessionUpdateControllerMulticastListCmdDataChild::Payload(bytes),
            },
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionUpdateControllerMulticastListCmd(
                sessionupdatecontrollermulticastlistcmd,
            ),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 7,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionUpdateControllerMulticastListCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListCmdBuilder> for UciPacket {
    fn from(builder: SessionUpdateControllerMulticastListCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListCmdBuilder> for UciCommand {
    fn from(builder: SessionUpdateControllerMulticastListCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionUpdateControllerMulticastListCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListCmdBuilder>
    for SessionUpdateControllerMulticastListCmd
{
    fn from(
        builder: SessionUpdateControllerMulticastListCmdBuilder,
    ) -> SessionUpdateControllerMulticastListCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhaseList {
    pub session_token: u32,
    pub start_slot_index: u16,
    pub end_slot_index: u16,
}
impl PhaseList {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 8
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PhaseList".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PhaseList".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let start_slot_index = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PhaseList".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let end_slot_index = bytes.get_mut().get_u16_le();
        Ok(Self {
            session_token,
            start_slot_index,
            end_slot_index,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u16_le(self.start_slot_index);
        buffer.put_u16_le(self.end_slot_index);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        8
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigCmdData {
    session_token: u32,
    number_of_phases: u8,
    update_time: [u8; 8],
    phase_list: Vec<PhaseList>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionsethybridconfigcmd: SessionSetHybridConfigCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigCmdBuilder {
    pub number_of_phases: u8,
    pub phase_list: Vec<PhaseList>,
    pub session_token: u32,
    pub update_time: [u8; 8],
}
impl SessionSetHybridConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 13
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetHybridConfigCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetHybridConfigCmd".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let number_of_phases = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetHybridConfigCmd".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        let update_time = (0..8)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        if bytes.get().remaining() % 8 != 0 {
            return Err(Error::InvalidArraySize {
                array: bytes.get().remaining(),
                element: 8,
            });
        }
        let phase_list_count = bytes.get().remaining() / 8;
        let mut phase_list = Vec::with_capacity(phase_list_count);
        for _ in 0..phase_list_count {
            phase_list.push(PhaseList::parse_inner(bytes)?);
        }
        Ok(Self {
            session_token,
            number_of_phases,
            update_time,
            phase_list,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.number_of_phases);
        for elem in &self.update_time {
            buffer.put_u8(*elem);
        }
        for elem in &self.phase_list {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        13 + self
            .phase_list
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for SessionSetHybridConfigCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetHybridConfigCmd> for Bytes {
    fn from(packet: SessionSetHybridConfigCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetHybridConfigCmd> for Vec<u8> {
    fn from(packet: SessionSetHybridConfigCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionSetHybridConfigCmd> for UciPacket {
    fn from(packet: SessionSetHybridConfigCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigCmd> for UciCommand {
    fn from(packet: SessionSetHybridConfigCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigCmd> for SessionConfigCommand {
    fn from(packet: SessionSetHybridConfigCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionSetHybridConfigCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionSetHybridConfigCmd> {
        SessionSetHybridConfigCmd::new(packet.ucipacket)
    }
}
impl SessionSetHybridConfigCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionsethybridconfigcmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionSetHybridConfigCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionSetHybridConfigCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessionsethybridconfigcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_number_of_phases(&self) -> u8 {
        self.sessionsethybridconfigcmd.number_of_phases
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_phase_list(&self) -> &Vec<PhaseList> {
        &self.sessionsethybridconfigcmd.phase_list
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionsethybridconfigcmd.session_token
    }
    pub fn get_update_time(&self) -> &[u8; 8] {
        &self.sessionsethybridconfigcmd.update_time
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionsethybridconfigcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionSetHybridConfigCmdBuilder {
    pub fn build(self) -> SessionSetHybridConfigCmd {
        let sessionsethybridconfigcmd = SessionSetHybridConfigCmdData {
            number_of_phases: self.number_of_phases,
            phase_list: self.phase_list,
            session_token: self.session_token,
            update_time: self.update_time,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionSetHybridConfigCmd(
                sessionsethybridconfigcmd,
            ),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 12,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionSetHybridConfigCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigCmdBuilder> for UciPacket {
    fn from(builder: SessionSetHybridConfigCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigCmdBuilder> for UciCommand {
    fn from(builder: SessionSetHybridConfigCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionSetHybridConfigCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigCmdBuilder> for SessionSetHybridConfigCmd {
    fn from(builder: SessionSetHybridConfigCmdBuilder) -> SessionSetHybridConfigCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionsethybridconfigrsp: SessionSetHybridConfigRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionSetHybridConfigRspBuilder {
    pub status: StatusCode,
}
impl SessionSetHybridConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetHybridConfigRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionSetHybridConfigRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionSetHybridConfigRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetHybridConfigRsp> for Bytes {
    fn from(packet: SessionSetHybridConfigRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetHybridConfigRsp> for Vec<u8> {
    fn from(packet: SessionSetHybridConfigRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionSetHybridConfigRsp> for UciPacket {
    fn from(packet: SessionSetHybridConfigRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigRsp> for UciResponse {
    fn from(packet: SessionSetHybridConfigRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigRsp> for SessionConfigResponse {
    fn from(packet: SessionSetHybridConfigRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionSetHybridConfigRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionSetHybridConfigRsp> {
        SessionSetHybridConfigRsp::new(packet.ucipacket)
    }
}
impl SessionSetHybridConfigRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionsethybridconfigrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionSetHybridConfigRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigResponseDataChild::SessionSetHybridConfigRsp),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessionsethybridconfigrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionsethybridconfigrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionsethybridconfigrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionSetHybridConfigRspBuilder {
    pub fn build(self) -> SessionSetHybridConfigRsp {
        let sessionsethybridconfigrsp = SessionSetHybridConfigRspData {
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionSetHybridConfigRsp(
                sessionsethybridconfigrsp,
            ),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 12,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionSetHybridConfigRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionSetHybridConfigRspBuilder> for UciPacket {
    fn from(builder: SessionSetHybridConfigRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigRspBuilder> for UciResponse {
    fn from(builder: SessionSetHybridConfigRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionSetHybridConfigRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionSetHybridConfigRspBuilder> for SessionSetHybridConfigRsp {
    fn from(builder: SessionSetHybridConfigRspBuilder) -> SessionSetHybridConfigRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmdPayload {
    pub controlees: Vec<Controlee>,
}
impl SessionUpdateControllerMulticastListCmdPayload {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmdPayload".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let controlees_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < controlees_count * 6usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmdPayload".to_string(),
                wanted: controlees_count * 6usize,
                got: bytes.get().remaining(),
            });
        }
        let controlees = (0..controlees_count)
            .map(|_| Controlee::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { controlees })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.controlees.len() as u8);
        for elem in &self.controlees {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .controlees
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload {
    pub controlees: Vec<Controlee_V2_0_16_Byte_Version>,
}
impl SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let controlees_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < controlees_count * 22usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload".to_string(),
                wanted: controlees_count * 22usize,
                got: bytes.get().remaining(),
            });
        }
        let controlees = (0..controlees_count)
            .map(|_| Controlee_V2_0_16_Byte_Version::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { controlees })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.controlees.len() as u8);
        for elem in &self.controlees {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .controlees
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload {
    pub controlees: Vec<Controlee_V2_0_32_Byte_Version>,
}
impl SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let controlees_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < controlees_count * 38usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload".to_string(),
                wanted: controlees_count * 38usize,
                got: bytes.get().remaining(),
            });
        }
        let controlees = (0..controlees_count)
            .map(|_| Controlee_V2_0_32_Byte_Version::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { controlees })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.controlees.len() as u8);
        for elem in &self.controlees {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .controlees
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionupdatecontrollermulticastlistrsp: SessionUpdateControllerMulticastListRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListRspBuilder {
    pub status: StatusCode,
}
impl SessionUpdateControllerMulticastListRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionUpdateControllerMulticastListRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionUpdateControllerMulticastListRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListRsp> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListRsp> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListRsp> for UciPacket {
    fn from(packet: SessionUpdateControllerMulticastListRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListRsp> for UciResponse {
    fn from(packet: SessionUpdateControllerMulticastListRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListRsp> for SessionConfigResponse {
    fn from(packet: SessionUpdateControllerMulticastListRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionUpdateControllerMulticastListRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionUpdateControllerMulticastListRsp> {
        SessionUpdateControllerMulticastListRsp::new(packet.ucipacket)
    }
}
impl SessionUpdateControllerMulticastListRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionupdatecontrollermulticastlistrsp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(value) => {
                value.clone()
            }
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp
                    ),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessionupdatecontrollermulticastlistrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionupdatecontrollermulticastlistrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionupdatecontrollermulticastlistrsp
            .write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionUpdateControllerMulticastListRspBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListRsp {
        let sessionupdatecontrollermulticastlistrsp = SessionUpdateControllerMulticastListRspData {
            status: self.status,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionUpdateControllerMulticastListRsp(
                sessionupdatecontrollermulticastlistrsp,
            ),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 7,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionUpdateControllerMulticastListRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListRspBuilder> for UciPacket {
    fn from(builder: SessionUpdateControllerMulticastListRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListRspBuilder> for UciResponse {
    fn from(builder: SessionUpdateControllerMulticastListRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionUpdateControllerMulticastListRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListRspBuilder>
    for SessionUpdateControllerMulticastListRsp
{
    fn from(
        builder: SessionUpdateControllerMulticastListRspBuilder,
    ) -> SessionUpdateControllerMulticastListRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControleeStatus {
    pub mac_address: [u8; 2],
    pub subsession_id: u32,
    pub status: MulticastUpdateStatusCode,
}
impl ControleeStatus {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 7
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let mac_address = (0..2)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let subsession_id = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = MulticastUpdateStatusCode::try_from(bytes.get_mut().get_u8()).map_err(
            |unknown_val| Error::InvalidEnumValueError {
                obj: "ControleeStatus".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "MulticastUpdateStatusCode".to_string(),
            },
        )?;
        Ok(Self {
            mac_address,
            subsession_id,
            status,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.mac_address {
            buffer.put_u8(*elem);
        }
        buffer.put_u32_le(self.subsession_id);
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        7
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListNtfData {
    session_token: u32,
    remaining_multicast_list_size: u8,
    controlee_status: Vec<ControleeStatus>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfignotification: SessionConfigNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionupdatecontrollermulticastlistntf: SessionUpdateControllerMulticastListNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionUpdateControllerMulticastListNtfBuilder {
    pub controlee_status: Vec<ControleeStatus>,
    pub remaining_multicast_list_size: u8,
    pub session_token: u32,
}
impl SessionUpdateControllerMulticastListNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 6
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let remaining_multicast_list_size = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let controlee_status_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < controlee_status_count * 7usize {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                wanted: controlee_status_count * 7usize,
                got: bytes.get().remaining(),
            });
        }
        let controlee_status = (0..controlee_status_count)
            .map(|_| ControleeStatus::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            session_token,
            remaining_multicast_list_size,
            controlee_status,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.remaining_multicast_list_size);
        buffer.put_u8(self.controlee_status.len() as u8);
        for elem in &self.controlee_status {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        6 + self
            .controlee_status
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for SessionUpdateControllerMulticastListNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListNtf> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListNtf> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListNtf) -> Self {
        packet.to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListNtf> for UciPacket {
    fn from(packet: SessionUpdateControllerMulticastListNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListNtf> for UciNotification {
    fn from(packet: SessionUpdateControllerMulticastListNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListNtf> for SessionConfigNotification {
    fn from(packet: SessionUpdateControllerMulticastListNtf) -> SessionConfigNotification {
        SessionConfigNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionUpdateControllerMulticastListNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionUpdateControllerMulticastListNtf> {
        SessionUpdateControllerMulticastListNtf::new(packet.ucipacket)
    }
}
impl SessionUpdateControllerMulticastListNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfignotification = match &ucinotification.child {
            UciNotificationDataChild::SessionConfigNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionConfigNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessionupdatecontrollermulticastlistntf = match &sessionconfignotification.child {
            SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(value) => {
                value.clone()
            }
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf
                    ),
                    actual: format!("{:?}", &sessionconfignotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessionconfignotification,
            sessionupdatecontrollermulticastlistntf,
        })
    }
    pub fn get_controlee_status(&self) -> &Vec<ControleeStatus> {
        &self
            .sessionupdatecontrollermulticastlistntf
            .controlee_status
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_remaining_multicast_list_size(&self) -> u8 {
        self.sessionupdatecontrollermulticastlistntf
            .remaining_multicast_list_size
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionupdatecontrollermulticastlistntf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionupdatecontrollermulticastlistntf
            .write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionUpdateControllerMulticastListNtfBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListNtf {
        let sessionupdatecontrollermulticastlistntf = SessionUpdateControllerMulticastListNtfData {
            controlee_status: self.controlee_status,
            remaining_multicast_list_size: self.remaining_multicast_list_size,
            session_token: self.session_token,
        };
        let sessionconfignotification = SessionConfigNotificationData {
            child: SessionConfigNotificationDataChild::SessionUpdateControllerMulticastListNtf(
                sessionupdatecontrollermulticastlistntf,
            ),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionConfigNotification(sessionconfignotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Notification,
            opcode: 7,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        SessionUpdateControllerMulticastListNtf::new(ucipacket).unwrap()
    }
}
impl From<SessionUpdateControllerMulticastListNtfBuilder> for UciPacket {
    fn from(builder: SessionUpdateControllerMulticastListNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListNtfBuilder> for UciNotification {
    fn from(builder: SessionUpdateControllerMulticastListNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListNtfBuilder> for SessionConfigNotification {
    fn from(builder: SessionUpdateControllerMulticastListNtfBuilder) -> SessionConfigNotification {
        builder.build().into()
    }
}
impl From<SessionUpdateControllerMulticastListNtfBuilder>
    for SessionUpdateControllerMulticastListNtf
{
    fn from(
        builder: SessionUpdateControllerMulticastListNtfBuilder,
    ) -> SessionUpdateControllerMulticastListNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataCreditNtfData {
    session_token: u32,
    credit_availability: CreditAvailability,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataCreditNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    datacreditntf: DataCreditNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataCreditNtfBuilder {
    pub credit_availability: CreditAvailability,
    pub session_token: u32,
}
impl DataCreditNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "DataCreditNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DataCreditNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let credit_availability =
            CreditAvailability::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
                Error::InvalidEnumValueError {
                    obj: "DataCreditNtf".to_string(),
                    field: "credit_availability".to_string(),
                    value: unknown_val as u64,
                    type_: "CreditAvailability".to_string(),
                }
            })?;
        Ok(Self {
            session_token,
            credit_availability,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(u8::from(self.credit_availability));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5
    }
}
impl Packet for DataCreditNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DataCreditNtf> for Bytes {
    fn from(packet: DataCreditNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<DataCreditNtf> for Vec<u8> {
    fn from(packet: DataCreditNtf) -> Self {
        packet.to_vec()
    }
}
impl From<DataCreditNtf> for UciPacket {
    fn from(packet: DataCreditNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<DataCreditNtf> for UciNotification {
    fn from(packet: DataCreditNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<DataCreditNtf> for SessionControlNotification {
    fn from(packet: DataCreditNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for DataCreditNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<DataCreditNtf> {
        DataCreditNtf::new(packet.ucipacket)
    }
}
impl DataCreditNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let datacreditntf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::DataCreditNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::DataCreditNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            datacreditntf,
        })
    }
    pub fn get_credit_availability(&self) -> CreditAvailability {
        self.datacreditntf.credit_availability
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.datacreditntf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.datacreditntf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl DataCreditNtfBuilder {
    pub fn build(self) -> DataCreditNtf {
        let datacreditntf = DataCreditNtfData {
            credit_availability: self.credit_availability,
            session_token: self.session_token,
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::DataCreditNtf(datacreditntf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 4,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        DataCreditNtf::new(ucipacket).unwrap()
    }
}
impl From<DataCreditNtfBuilder> for UciPacket {
    fn from(builder: DataCreditNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<DataCreditNtfBuilder> for UciNotification {
    fn from(builder: DataCreditNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<DataCreditNtfBuilder> for SessionControlNotification {
    fn from(builder: DataCreditNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<DataCreditNtfBuilder> for DataCreditNtf {
    fn from(builder: DataCreditNtfBuilder) -> DataCreditNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataTransferStatusNtfData {
    session_token: u32,
    uci_sequence_number: u8,
    status: DataTransferNtfStatusCode,
    tx_count: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataTransferStatusNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    datatransferstatusntf: DataTransferStatusNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataTransferStatusNtfBuilder {
    pub session_token: u32,
    pub status: DataTransferNtfStatusCode,
    pub tx_count: u8,
    pub uci_sequence_number: u8,
}
impl DataTransferStatusNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 7
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "DataTransferStatusNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DataTransferStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let uci_sequence_number = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DataTransferStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = DataTransferNtfStatusCode::try_from(bytes.get_mut().get_u8()).map_err(
            |unknown_val| Error::InvalidEnumValueError {
                obj: "DataTransferStatusNtf".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "DataTransferNtfStatusCode".to_string(),
            },
        )?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DataTransferStatusNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let tx_count = bytes.get_mut().get_u8();
        Ok(Self {
            session_token,
            uci_sequence_number,
            status,
            tx_count,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.uci_sequence_number);
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.tx_count);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        7
    }
}
impl Packet for DataTransferStatusNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DataTransferStatusNtf> for Bytes {
    fn from(packet: DataTransferStatusNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<DataTransferStatusNtf> for Vec<u8> {
    fn from(packet: DataTransferStatusNtf) -> Self {
        packet.to_vec()
    }
}
impl From<DataTransferStatusNtf> for UciPacket {
    fn from(packet: DataTransferStatusNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<DataTransferStatusNtf> for UciNotification {
    fn from(packet: DataTransferStatusNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<DataTransferStatusNtf> for SessionControlNotification {
    fn from(packet: DataTransferStatusNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for DataTransferStatusNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<DataTransferStatusNtf> {
        DataTransferStatusNtf::new(packet.ucipacket)
    }
}
impl DataTransferStatusNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let datatransferstatusntf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::DataTransferStatusNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionControlNotificationDataChild::DataTransferStatusNtf
                    ),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            datatransferstatusntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.datatransferstatusntf.session_token
    }
    pub fn get_status(&self) -> DataTransferNtfStatusCode {
        self.datatransferstatusntf.status
    }
    pub fn get_tx_count(&self) -> u8 {
        self.datatransferstatusntf.tx_count
    }
    pub fn get_uci_sequence_number(&self) -> u8 {
        self.datatransferstatusntf.uci_sequence_number
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.datatransferstatusntf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl DataTransferStatusNtfBuilder {
    pub fn build(self) -> DataTransferStatusNtf {
        let datatransferstatusntf = DataTransferStatusNtfData {
            session_token: self.session_token,
            status: self.status,
            tx_count: self.tx_count,
            uci_sequence_number: self.uci_sequence_number,
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::DataTransferStatusNtf(
                datatransferstatusntf,
            ),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 5,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        DataTransferStatusNtf::new(ucipacket).unwrap()
    }
}
impl From<DataTransferStatusNtfBuilder> for UciPacket {
    fn from(builder: DataTransferStatusNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<DataTransferStatusNtfBuilder> for UciNotification {
    fn from(builder: DataTransferStatusNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<DataTransferStatusNtfBuilder> for SessionControlNotification {
    fn from(builder: DataTransferStatusNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<DataTransferStatusNtfBuilder> for DataTransferStatusNtf {
    fn from(builder: DataTransferStatusNtfBuilder) -> DataTransferStatusNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeCmdData {
    session_token: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigcommand: SessionConfigCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionquerymaxdatasizecmd: SessionQueryMaxDataSizeCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeCmdBuilder {
    pub session_token: u32,
}
impl SessionQueryMaxDataSizeCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionQueryMaxDataSizeCmd".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        Ok(Self { session_token })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4
    }
}
impl Packet for SessionQueryMaxDataSizeCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionQueryMaxDataSizeCmd> for Bytes {
    fn from(packet: SessionQueryMaxDataSizeCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionQueryMaxDataSizeCmd> for Vec<u8> {
    fn from(packet: SessionQueryMaxDataSizeCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionQueryMaxDataSizeCmd> for UciPacket {
    fn from(packet: SessionQueryMaxDataSizeCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeCmd> for UciCommand {
    fn from(packet: SessionQueryMaxDataSizeCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeCmd> for SessionConfigCommand {
    fn from(packet: SessionQueryMaxDataSizeCmd) -> SessionConfigCommand {
        SessionConfigCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionQueryMaxDataSizeCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionQueryMaxDataSizeCmd> {
        SessionQueryMaxDataSizeCmd::new(packet.ucipacket)
    }
}
impl SessionQueryMaxDataSizeCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigcommand = match &ucicommand.child {
            UciCommandDataChild::SessionConfigCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionConfigCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionquerymaxdatasizecmd = match &sessionconfigcommand.child {
            SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd),
                    actual: format!("{:?}", &sessionconfigcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessionconfigcommand,
            sessionquerymaxdatasizecmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionquerymaxdatasizecmd.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionquerymaxdatasizecmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionQueryMaxDataSizeCmdBuilder {
    pub fn build(self) -> SessionQueryMaxDataSizeCmd {
        let sessionquerymaxdatasizecmd = SessionQueryMaxDataSizeCmdData {
            session_token: self.session_token,
        };
        let sessionconfigcommand = SessionConfigCommandData {
            child: SessionConfigCommandDataChild::SessionQueryMaxDataSizeCmd(
                sessionquerymaxdatasizecmd,
            ),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionConfigCommand(sessionconfigcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Command,
            opcode: 11,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionQueryMaxDataSizeCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeCmdBuilder> for UciPacket {
    fn from(builder: SessionQueryMaxDataSizeCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeCmdBuilder> for UciCommand {
    fn from(builder: SessionQueryMaxDataSizeCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeCmdBuilder> for SessionConfigCommand {
    fn from(builder: SessionQueryMaxDataSizeCmdBuilder) -> SessionConfigCommand {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeCmdBuilder> for SessionQueryMaxDataSizeCmd {
    fn from(builder: SessionQueryMaxDataSizeCmdBuilder) -> SessionQueryMaxDataSizeCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeRspData {
    session_token: u32,
    max_data_size: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionconfigresponse: SessionConfigResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionquerymaxdatasizersp: SessionQueryMaxDataSizeRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionQueryMaxDataSizeRspBuilder {
    pub max_data_size: u16,
    pub session_token: u32,
}
impl SessionQueryMaxDataSizeRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 6
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionQueryMaxDataSizeRsp".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "SessionQueryMaxDataSizeRsp".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let max_data_size = bytes.get_mut().get_u16_le();
        Ok(Self {
            session_token,
            max_data_size,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u16_le(self.max_data_size);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        6
    }
}
impl Packet for SessionQueryMaxDataSizeRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionQueryMaxDataSizeRsp> for Bytes {
    fn from(packet: SessionQueryMaxDataSizeRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionQueryMaxDataSizeRsp> for Vec<u8> {
    fn from(packet: SessionQueryMaxDataSizeRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionQueryMaxDataSizeRsp> for UciPacket {
    fn from(packet: SessionQueryMaxDataSizeRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeRsp> for UciResponse {
    fn from(packet: SessionQueryMaxDataSizeRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeRsp> for SessionConfigResponse {
    fn from(packet: SessionQueryMaxDataSizeRsp) -> SessionConfigResponse {
        SessionConfigResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionQueryMaxDataSizeRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionQueryMaxDataSizeRsp> {
        SessionQueryMaxDataSizeRsp::new(packet.ucipacket)
    }
}
impl SessionQueryMaxDataSizeRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessionconfigresponse = match &uciresponse.child {
            UciResponseDataChild::SessionConfigResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionConfigResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionquerymaxdatasizersp = match &sessionconfigresponse.child {
            SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp
                    ),
                    actual: format!("{:?}", &sessionconfigresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessionconfigresponse,
            sessionquerymaxdatasizersp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_max_data_size(&self) -> u16 {
        self.sessionquerymaxdatasizersp.max_data_size
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessionquerymaxdatasizersp.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionquerymaxdatasizersp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionQueryMaxDataSizeRspBuilder {
    pub fn build(self) -> SessionQueryMaxDataSizeRsp {
        let sessionquerymaxdatasizersp = SessionQueryMaxDataSizeRspData {
            max_data_size: self.max_data_size,
            session_token: self.session_token,
        };
        let sessionconfigresponse = SessionConfigResponseData {
            child: SessionConfigResponseDataChild::SessionQueryMaxDataSizeRsp(
                sessionquerymaxdatasizersp,
            ),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionConfigResponse(sessionconfigresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionConfig,
            message_type: MessageType::Response,
            opcode: 11,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionQueryMaxDataSizeRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionQueryMaxDataSizeRspBuilder> for UciPacket {
    fn from(builder: SessionQueryMaxDataSizeRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeRspBuilder> for UciResponse {
    fn from(builder: SessionQueryMaxDataSizeRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeRspBuilder> for SessionConfigResponse {
    fn from(builder: SessionQueryMaxDataSizeRspBuilder) -> SessionConfigResponse {
        builder.build().into()
    }
}
impl From<SessionQueryMaxDataSizeRspBuilder> for SessionQueryMaxDataSizeRsp {
    fn from(builder: SessionQueryMaxDataSizeRspBuilder) -> SessionQueryMaxDataSizeRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolcommand: SessionControlCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionstartcmd: SessionStartCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartCmdBuilder {
    pub session_id: u32,
}
impl SessionStartCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for SessionStartCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStartCmd> for Bytes {
    fn from(packet: SessionStartCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStartCmd> for Vec<u8> {
    fn from(packet: SessionStartCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionStartCmd> for UciPacket {
    fn from(packet: SessionStartCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStartCmd> for UciCommand {
    fn from(packet: SessionStartCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStartCmd> for SessionControlCommand {
    fn from(packet: SessionStartCmd) -> SessionControlCommand {
        SessionControlCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionStartCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionStartCmd> {
        SessionStartCmd::new(packet.ucipacket)
    }
}
impl SessionStartCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolcommand = match &ucicommand.child {
            UciCommandDataChild::SessionControlCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionControlCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionstartcmd = match &sessioncontrolcommand.child {
            SessionControlCommandDataChild::SessionStartCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlCommandDataChild::SessionStartCmd),
                    actual: format!("{:?}", &sessioncontrolcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessioncontrolcommand,
            sessionstartcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_id(&self) -> u32 {
        self.sessioncontrolcommand.session_id
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionstartcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionStartCmdBuilder {
    pub fn build(self) -> SessionStartCmd {
        let sessionstartcmd = SessionStartCmdData {};
        let sessioncontrolcommand = SessionControlCommandData {
            session_id: self.session_id,
            child: SessionControlCommandDataChild::SessionStartCmd(sessionstartcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionControlCommand(sessioncontrolcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Command,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionStartCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionStartCmdBuilder> for UciPacket {
    fn from(builder: SessionStartCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionStartCmdBuilder> for UciCommand {
    fn from(builder: SessionStartCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionStartCmdBuilder> for SessionControlCommand {
    fn from(builder: SessionStartCmdBuilder) -> SessionControlCommand {
        builder.build().into()
    }
}
impl From<SessionStartCmdBuilder> for SessionStartCmd {
    fn from(builder: SessionStartCmdBuilder) -> SessionStartCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolresponse: SessionControlResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionstartrsp: SessionStartRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStartRspBuilder {
    pub status: StatusCode,
}
impl SessionStartRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStartRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionStartRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionStartRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStartRsp> for Bytes {
    fn from(packet: SessionStartRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStartRsp> for Vec<u8> {
    fn from(packet: SessionStartRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionStartRsp> for UciPacket {
    fn from(packet: SessionStartRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStartRsp> for UciResponse {
    fn from(packet: SessionStartRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStartRsp> for SessionControlResponse {
    fn from(packet: SessionStartRsp) -> SessionControlResponse {
        SessionControlResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionStartRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionStartRsp> {
        SessionStartRsp::new(packet.ucipacket)
    }
}
impl SessionStartRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolresponse = match &uciresponse.child {
            UciResponseDataChild::SessionControlResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionControlResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionstartrsp = match &sessioncontrolresponse.child {
            SessionControlResponseDataChild::SessionStartRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlResponseDataChild::SessionStartRsp),
                    actual: format!("{:?}", &sessioncontrolresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessioncontrolresponse,
            sessionstartrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionstartrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionstartrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionStartRspBuilder {
    pub fn build(self) -> SessionStartRsp {
        let sessionstartrsp = SessionStartRspData {
            status: self.status,
        };
        let sessioncontrolresponse = SessionControlResponseData {
            child: SessionControlResponseDataChild::SessionStartRsp(sessionstartrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionControlResponse(sessioncontrolresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Response,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionStartRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionStartRspBuilder> for UciPacket {
    fn from(builder: SessionStartRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionStartRspBuilder> for UciResponse {
    fn from(builder: SessionStartRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionStartRspBuilder> for SessionControlResponse {
    fn from(builder: SessionStartRspBuilder) -> SessionControlResponse {
        builder.build().into()
    }
}
impl From<SessionStartRspBuilder> for SessionStartRsp {
    fn from(builder: SessionStartRspBuilder) -> SessionStartRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortAddressTwoWayRangingMeasurement {
    pub mac_address: u16,
    pub status: StatusCode,
    pub nlos: u8,
    pub distance: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
    pub aoa_destination_azimuth: u16,
    pub aoa_destination_azimuth_fom: u8,
    pub aoa_destination_elevation: u16,
    pub aoa_destination_elevation_fom: u8,
    pub slot_index: u8,
    pub rssi: u8,
}
impl ShortAddressTwoWayRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 31
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let mac_address = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let nlos = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let distance = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_elevation_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let slot_index = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let rssi = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(8);
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(3);
        Ok(Self {
            mac_address,
            status,
            nlos,
            distance,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
            aoa_destination_azimuth,
            aoa_destination_azimuth_fom,
            aoa_destination_elevation,
            aoa_destination_elevation_fom,
            slot_index,
            rssi,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(self.mac_address);
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.nlos);
        buffer.put_u16_le(self.distance);
        buffer.put_u16_le(self.aoa_azimuth);
        buffer.put_u8(self.aoa_azimuth_fom);
        buffer.put_u16_le(self.aoa_elevation);
        buffer.put_u8(self.aoa_elevation_fom);
        buffer.put_u16_le(self.aoa_destination_azimuth);
        buffer.put_u8(self.aoa_destination_azimuth_fom);
        buffer.put_u16_le(self.aoa_destination_elevation);
        buffer.put_u8(self.aoa_destination_elevation_fom);
        buffer.put_u8(self.slot_index);
        buffer.put_u8(self.rssi);
        buffer.put_bytes(0, 8);
        buffer.put_bytes(0, 3);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        31
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedAddressTwoWayRangingMeasurement {
    pub mac_address: u64,
    pub status: StatusCode,
    pub nlos: u8,
    pub distance: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
    pub aoa_destination_azimuth: u16,
    pub aoa_destination_azimuth_fom: u8,
    pub aoa_destination_elevation: u16,
    pub aoa_destination_elevation_fom: u8,
    pub slot_index: u8,
    pub rssi: u8,
}
impl ExtendedAddressTwoWayRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 31
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        let mac_address = bytes.get_mut().get_u64_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let nlos = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let distance = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_destination_elevation_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let slot_index = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let rssi = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                wanted: 5,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(5);
        Ok(Self {
            mac_address,
            status,
            nlos,
            distance,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
            aoa_destination_azimuth,
            aoa_destination_azimuth_fom,
            aoa_destination_elevation,
            aoa_destination_elevation_fom,
            slot_index,
            rssi,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u64_le(self.mac_address);
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.nlos);
        buffer.put_u16_le(self.distance);
        buffer.put_u16_le(self.aoa_azimuth);
        buffer.put_u8(self.aoa_azimuth_fom);
        buffer.put_u16_le(self.aoa_elevation);
        buffer.put_u8(self.aoa_elevation_fom);
        buffer.put_u16_le(self.aoa_destination_azimuth);
        buffer.put_u8(self.aoa_destination_azimuth_fom);
        buffer.put_u16_le(self.aoa_destination_elevation);
        buffer.put_u8(self.aoa_destination_elevation_fom);
        buffer.put_u8(self.slot_index);
        buffer.put_u8(self.rssi);
        buffer.put_bytes(0, 5);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        31
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortAddressOwrAoaRangingMeasurement {
    pub mac_address: u16,
    pub status: StatusCode,
    pub nlos: u8,
    pub frame_sequence_number: u8,
    pub block_index: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
}
impl ShortAddressOwrAoaRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 13
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let mac_address = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let nlos = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let frame_sequence_number = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let block_index = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation_fom = bytes.get_mut().get_u8();
        Ok(Self {
            mac_address,
            status,
            nlos,
            frame_sequence_number,
            block_index,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(self.mac_address);
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.nlos);
        buffer.put_u8(self.frame_sequence_number);
        buffer.put_u16_le(self.block_index);
        buffer.put_u16_le(self.aoa_azimuth);
        buffer.put_u8(self.aoa_azimuth_fom);
        buffer.put_u16_le(self.aoa_elevation);
        buffer.put_u8(self.aoa_elevation_fom);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        13
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedAddressOwrAoaRangingMeasurement {
    pub mac_address: u64,
    pub status: StatusCode,
    pub nlos: u8,
    pub frame_sequence_number: u8,
    pub block_index: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
}
impl ExtendedAddressOwrAoaRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 19
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        let mac_address = bytes.get_mut().get_u64_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let nlos = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let frame_sequence_number = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let block_index = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_azimuth_fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressOwrAoaRangingMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let aoa_elevation_fom = bytes.get_mut().get_u8();
        Ok(Self {
            mac_address,
            status,
            nlos,
            frame_sequence_number,
            block_index,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u64_le(self.mac_address);
        buffer.put_u8(u8::from(self.status));
        buffer.put_u8(self.nlos);
        buffer.put_u8(self.frame_sequence_number);
        buffer.put_u16_le(self.block_index);
        buffer.put_u16_le(self.aoa_azimuth);
        buffer.put_u8(self.aoa_azimuth_fom);
        buffer.put_u16_le(self.aoa_elevation);
        buffer.put_u8(self.aoa_elevation_fom);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        19
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum RangingMeasurementType {
    OneWay = 0x0,
    TwoWay = 0x1,
    DlTdoa = 0x2,
    OwrAoa = 0x3,
}
impl TryFrom<u8> for RangingMeasurementType {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(RangingMeasurementType::OneWay),
            0x1 => Ok(RangingMeasurementType::TwoWay),
            0x2 => Ok(RangingMeasurementType::DlTdoa),
            0x3 => Ok(RangingMeasurementType::OwrAoa),
            _ => Err(value),
        }
    }
}
impl From<&RangingMeasurementType> for u8 {
    fn from(value: &RangingMeasurementType) -> Self {
        match value {
            RangingMeasurementType::OneWay => 0x0,
            RangingMeasurementType::TwoWay => 0x1,
            RangingMeasurementType::DlTdoa => 0x2,
            RangingMeasurementType::OwrAoa => 0x3,
        }
    }
}
impl From<RangingMeasurementType> for u8 {
    fn from(value: RangingMeasurementType) -> Self {
        (&value).into()
    }
}
impl From<RangingMeasurementType> for i16 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
impl From<RangingMeasurementType> for i32 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
impl From<RangingMeasurementType> for i64 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
impl From<RangingMeasurementType> for u16 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
impl From<RangingMeasurementType> for u32 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
impl From<RangingMeasurementType> for u64 {
    fn from(value: RangingMeasurementType) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionInfoNtfDataChild {
    ShortMacTwoWaySessionInfoNtf(ShortMacTwoWaySessionInfoNtfData),
    ExtendedMacTwoWaySessionInfoNtf(ExtendedMacTwoWaySessionInfoNtfData),
    ShortMacDlTDoASessionInfoNtf(ShortMacDlTDoASessionInfoNtfData),
    ExtendedMacDlTDoASessionInfoNtf(ExtendedMacDlTDoASessionInfoNtfData),
    ShortMacOwrAoaSessionInfoNtf(ShortMacOwrAoaSessionInfoNtfData),
    ExtendedMacOwrAoaSessionInfoNtf(ExtendedMacOwrAoaSessionInfoNtfData),
    Payload(Bytes),
    None,
}
impl SessionInfoNtfDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(value) => value.get_total_size(),
            SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(value) => {
                value.get_total_size()
            }
            SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(value) => value.get_total_size(),
            SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(value) => {
                value.get_total_size()
            }
            SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(value) => value.get_total_size(),
            SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(value) => {
                value.get_total_size()
            }
            SessionInfoNtfDataChild::Payload(bytes) => bytes.len(),
            SessionInfoNtfDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionInfoNtfChild {
    ShortMacTwoWaySessionInfoNtf(ShortMacTwoWaySessionInfoNtf),
    ExtendedMacTwoWaySessionInfoNtf(ExtendedMacTwoWaySessionInfoNtf),
    ShortMacDlTDoASessionInfoNtf(ShortMacDlTDoASessionInfoNtf),
    ExtendedMacDlTDoASessionInfoNtf(ExtendedMacDlTDoASessionInfoNtf),
    ShortMacOwrAoaSessionInfoNtf(ShortMacOwrAoaSessionInfoNtf),
    ExtendedMacOwrAoaSessionInfoNtf(ExtendedMacOwrAoaSessionInfoNtf),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInfoNtfData {
    sequence_number: u32,
    session_token: u32,
    rcr_indicator: u8,
    current_ranging_interval: u32,
    ranging_measurement_type: RangingMeasurementType,
    mac_address_indicator: MacAddressIndicator,
    child: SessionInfoNtfDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub mac_address_indicator: MacAddressIndicator,
    pub ranging_measurement_type: RangingMeasurementType,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
    pub payload: Option<Bytes>,
}
impl SessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 24
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let sequence_number = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let rcr_indicator = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let current_ranging_interval = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let ranging_measurement_type = RangingMeasurementType::try_from(bytes.get_mut().get_u8())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
            obj: "SessionInfoNtf".to_string(),
            field: "ranging_measurement_type".to_string(),
            value: unknown_val as u64,
            type_: "RangingMeasurementType".to_string(),
        })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(1);
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let mac_address_indicator = MacAddressIndicator::try_from(bytes.get_mut().get_u8())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "SessionInfoNtf".to_string(),
                field: "mac_address_indicator".to_string(),
                value: unknown_val as u64,
                type_: "MacAddressIndicator".to_string(),
            })?;
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInfoNtf".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        bytes.get_mut().advance(8);
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (mac_address_indicator, ranging_measurement_type) {
            (MacAddressIndicator::ShortAddress, RangingMeasurementType::TwoWay)
                if ShortMacTwoWaySessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ShortMacTwoWaySessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(child_data)
            }
            (MacAddressIndicator::ExtendedAddress, RangingMeasurementType::TwoWay)
                if ExtendedMacTwoWaySessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ExtendedMacTwoWaySessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(child_data)
            }
            (MacAddressIndicator::ShortAddress, RangingMeasurementType::DlTdoa)
                if ShortMacDlTDoASessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ShortMacDlTDoASessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(child_data)
            }
            (MacAddressIndicator::ExtendedAddress, RangingMeasurementType::DlTdoa)
                if ExtendedMacDlTDoASessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ExtendedMacDlTDoASessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(child_data)
            }
            (MacAddressIndicator::ShortAddress, RangingMeasurementType::OwrAoa)
                if ShortMacOwrAoaSessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ShortMacOwrAoaSessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(child_data)
            }
            (MacAddressIndicator::ExtendedAddress, RangingMeasurementType::OwrAoa)
                if ExtendedMacOwrAoaSessionInfoNtfData::conforms(&payload) =>
            {
                let mut cell = Cell::new(payload);
                let child_data = ExtendedMacOwrAoaSessionInfoNtfData::parse_inner(&mut cell)?;
                SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(child_data)
            }
            _ if !payload.is_empty() => {
                SessionInfoNtfDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => SessionInfoNtfDataChild::None,
        };
        Ok(Self {
            sequence_number,
            session_token,
            rcr_indicator,
            current_ranging_interval,
            ranging_measurement_type,
            mac_address_indicator,
            child,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.sequence_number);
        buffer.put_u32_le(self.session_token);
        buffer.put_u8(self.rcr_indicator);
        buffer.put_u32_le(self.current_ranging_interval);
        buffer.put_u8(u8::from(self.ranging_measurement_type));
        buffer.put_bytes(0, 1);
        buffer.put_u8(u8::from(self.mac_address_indicator));
        buffer.put_bytes(0, 8);
        match &self.child {
            SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(child) => child.write_to(buffer),
            SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(child) => {
                child.write_to(buffer)
            }
            SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(child) => child.write_to(buffer),
            SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(child) => {
                child.write_to(buffer)
            }
            SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(child) => child.write_to(buffer),
            SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(child) => {
                child.write_to(buffer)
            }
            SessionInfoNtfDataChild::Payload(payload) => buffer.put_slice(payload),
            SessionInfoNtfDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        24 + self.child.get_total_size()
    }
}
impl Packet for SessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInfoNtf> for Bytes {
    fn from(packet: SessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInfoNtf> for Vec<u8> {
    fn from(packet: SessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<SessionInfoNtf> for UciPacket {
    fn from(packet: SessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInfoNtf> for UciNotification {
    fn from(packet: SessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionInfoNtf> for SessionControlNotification {
    fn from(packet: SessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionInfoNtf> {
        SessionInfoNtf::new(packet.ucipacket)
    }
}
impl SessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> SessionInfoNtfChild {
        match &self.sessioninfontf.child {
            SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(_) => {
                SessionInfoNtfChild::ShortMacTwoWaySessionInfoNtf(
                    ShortMacTwoWaySessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(_) => {
                SessionInfoNtfChild::ExtendedMacTwoWaySessionInfoNtf(
                    ExtendedMacTwoWaySessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(_) => {
                SessionInfoNtfChild::ShortMacDlTDoASessionInfoNtf(
                    ShortMacDlTDoASessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(_) => {
                SessionInfoNtfChild::ExtendedMacDlTDoASessionInfoNtf(
                    ExtendedMacDlTDoASessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(_) => {
                SessionInfoNtfChild::ShortMacOwrAoaSessionInfoNtf(
                    ShortMacOwrAoaSessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(_) => {
                SessionInfoNtfChild::ExtendedMacOwrAoaSessionInfoNtf(
                    ExtendedMacOwrAoaSessionInfoNtf::new(self.ucipacket.clone()).unwrap(),
                )
            }
            SessionInfoNtfDataChild::Payload(payload) => {
                SessionInfoNtfChild::Payload(payload.clone())
            }
            SessionInfoNtfDataChild::None => SessionInfoNtfChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionInfoNtfBuilder {
    pub fn build(self) -> SessionInfoNtf {
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: self.mac_address_indicator,
            ranging_measurement_type: self.ranging_measurement_type,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: match self.payload {
                None => SessionInfoNtfDataChild::None,
                Some(bytes) => SessionInfoNtfDataChild::Payload(bytes),
            },
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        SessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<SessionInfoNtfBuilder> for UciPacket {
    fn from(builder: SessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionInfoNtfBuilder> for UciNotification {
    fn from(builder: SessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<SessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: SessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<SessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: SessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacTwoWaySessionInfoNtfData {
    two_way_ranging_measurements: Vec<ShortAddressTwoWayRangingMeasurement>,
    vendor_data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacTwoWaySessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shortmactwowaysessioninfontf: ShortMacTwoWaySessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacTwoWaySessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
    pub two_way_ranging_measurements: Vec<ShortAddressTwoWayRangingMeasurement>,
    pub vendor_data: Vec<u8>,
}
impl ShortMacTwoWaySessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacTwoWaySessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let two_way_ranging_measurements_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < two_way_ranging_measurements_count * 31usize {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacTwoWaySessionInfoNtf".to_string(),
                wanted: two_way_ranging_measurements_count * 31usize,
                got: bytes.get().remaining(),
            });
        }
        let two_way_ranging_measurements = (0..two_way_ranging_measurements_count)
            .map(|_| ShortAddressTwoWayRangingMeasurement::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        let mut vendor_data = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            vendor_data.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            two_way_ranging_measurements,
            vendor_data,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.two_way_ranging_measurements.len() as u8);
        for elem in &self.two_way_ranging_measurements {
            elem.write_to(buffer);
        }
        for elem in &self.vendor_data {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .two_way_ranging_measurements
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
            + self.vendor_data.len()
    }
}
impl Packet for ShortMacTwoWaySessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for Bytes {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for Vec<u8> {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for UciPacket {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for UciNotification {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for SessionControlNotification {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacTwoWaySessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ShortMacTwoWaySessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ShortMacTwoWaySessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ShortMacTwoWaySessionInfoNtf> {
        ShortMacTwoWaySessionInfoNtf::new(packet.ucipacket)
    }
}
impl ShortMacTwoWaySessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let shortmactwowaysessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            shortmactwowaysessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    pub fn get_two_way_ranging_measurements(&self) -> &Vec<ShortAddressTwoWayRangingMeasurement> {
        &self
            .shortmactwowaysessioninfontf
            .two_way_ranging_measurements
    }
    pub fn get_vendor_data(&self) -> &Vec<u8> {
        &self.shortmactwowaysessioninfontf.vendor_data
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.shortmactwowaysessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ShortMacTwoWaySessionInfoNtfBuilder {
    pub fn build(self) -> ShortMacTwoWaySessionInfoNtf {
        let shortmactwowaysessioninfontf = ShortMacTwoWaySessionInfoNtfData {
            two_way_ranging_measurements: self.two_way_ranging_measurements,
            vendor_data: self.vendor_data,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ShortAddress,
            ranging_measurement_type: RangingMeasurementType::TwoWay,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ShortMacTwoWaySessionInfoNtf(
                shortmactwowaysessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ShortMacTwoWaySessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ShortMacTwoWaySessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ShortMacTwoWaySessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ShortMacTwoWaySessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ShortMacTwoWaySessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ShortMacTwoWaySessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ShortMacTwoWaySessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ShortMacTwoWaySessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ShortMacTwoWaySessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ShortMacTwoWaySessionInfoNtfBuilder> for ShortMacTwoWaySessionInfoNtf {
    fn from(builder: ShortMacTwoWaySessionInfoNtfBuilder) -> ShortMacTwoWaySessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacTwoWaySessionInfoNtfData {
    two_way_ranging_measurements: Vec<ExtendedAddressTwoWayRangingMeasurement>,
    vendor_data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacTwoWaySessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    extendedmactwowaysessioninfontf: ExtendedMacTwoWaySessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacTwoWaySessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
    pub two_way_ranging_measurements: Vec<ExtendedAddressTwoWayRangingMeasurement>,
    pub vendor_data: Vec<u8>,
}
impl ExtendedMacTwoWaySessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacTwoWaySessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let two_way_ranging_measurements_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < two_way_ranging_measurements_count * 31usize {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacTwoWaySessionInfoNtf".to_string(),
                wanted: two_way_ranging_measurements_count * 31usize,
                got: bytes.get().remaining(),
            });
        }
        let two_way_ranging_measurements = (0..two_way_ranging_measurements_count)
            .map(|_| ExtendedAddressTwoWayRangingMeasurement::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        let mut vendor_data = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            vendor_data.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            two_way_ranging_measurements,
            vendor_data,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.two_way_ranging_measurements.len() as u8);
        for elem in &self.two_way_ranging_measurements {
            elem.write_to(buffer);
        }
        for elem in &self.vendor_data {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .two_way_ranging_measurements
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
            + self.vendor_data.len()
    }
}
impl Packet for ExtendedMacTwoWaySessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for Bytes {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for Vec<u8> {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for UciPacket {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for UciNotification {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for SessionControlNotification {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ExtendedMacTwoWaySessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ExtendedMacTwoWaySessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ExtendedMacTwoWaySessionInfoNtf> {
        ExtendedMacTwoWaySessionInfoNtf::new(packet.ucipacket)
    }
}
impl ExtendedMacTwoWaySessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let extendedmactwowaysessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            extendedmactwowaysessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    pub fn get_two_way_ranging_measurements(
        &self,
    ) -> &Vec<ExtendedAddressTwoWayRangingMeasurement> {
        &self
            .extendedmactwowaysessioninfontf
            .two_way_ranging_measurements
    }
    pub fn get_vendor_data(&self) -> &Vec<u8> {
        &self.extendedmactwowaysessioninfontf.vendor_data
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.extendedmactwowaysessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ExtendedMacTwoWaySessionInfoNtfBuilder {
    pub fn build(self) -> ExtendedMacTwoWaySessionInfoNtf {
        let extendedmactwowaysessioninfontf = ExtendedMacTwoWaySessionInfoNtfData {
            two_way_ranging_measurements: self.two_way_ranging_measurements,
            vendor_data: self.vendor_data,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ExtendedAddress,
            ranging_measurement_type: RangingMeasurementType::TwoWay,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ExtendedMacTwoWaySessionInfoNtf(
                extendedmactwowaysessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ExtendedMacTwoWaySessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ExtendedMacTwoWaySessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ExtendedMacTwoWaySessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ExtendedMacTwoWaySessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ExtendedMacTwoWaySessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ExtendedMacTwoWaySessionInfoNtfBuilder> for ExtendedMacTwoWaySessionInfoNtf {
    fn from(builder: ExtendedMacTwoWaySessionInfoNtfBuilder) -> ExtendedMacTwoWaySessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacDlTDoASessionInfoNtfData {
    no_of_ranging_measurements: u8,
    dl_tdoa_measurements: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacDlTDoASessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shortmacdltdoasessioninfontf: ShortMacDlTDoASessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacDlTDoASessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub dl_tdoa_measurements: Vec<u8>,
    pub no_of_ranging_measurements: u8,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
}
impl ShortMacDlTDoASessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacDlTDoASessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let no_of_ranging_measurements = bytes.get_mut().get_u8();
        let mut dl_tdoa_measurements = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            dl_tdoa_measurements.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            no_of_ranging_measurements,
            dl_tdoa_measurements,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.no_of_ranging_measurements);
        for elem in &self.dl_tdoa_measurements {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.dl_tdoa_measurements.len()
    }
}
impl Packet for ShortMacDlTDoASessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for Bytes {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for Vec<u8> {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for UciPacket {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for UciNotification {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for SessionControlNotification {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacDlTDoASessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ShortMacDlTDoASessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ShortMacDlTDoASessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ShortMacDlTDoASessionInfoNtf> {
        ShortMacDlTDoASessionInfoNtf::new(packet.ucipacket)
    }
}
impl ShortMacDlTDoASessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let shortmacdltdoasessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            shortmacdltdoasessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_dl_tdoa_measurements(&self) -> &Vec<u8> {
        &self.shortmacdltdoasessioninfontf.dl_tdoa_measurements
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_no_of_ranging_measurements(&self) -> u8 {
        self.shortmacdltdoasessioninfontf.no_of_ranging_measurements
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.shortmacdltdoasessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ShortMacDlTDoASessionInfoNtfBuilder {
    pub fn build(self) -> ShortMacDlTDoASessionInfoNtf {
        let shortmacdltdoasessioninfontf = ShortMacDlTDoASessionInfoNtfData {
            dl_tdoa_measurements: self.dl_tdoa_measurements,
            no_of_ranging_measurements: self.no_of_ranging_measurements,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ShortAddress,
            ranging_measurement_type: RangingMeasurementType::DlTdoa,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ShortMacDlTDoASessionInfoNtf(
                shortmacdltdoasessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ShortMacDlTDoASessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ShortMacDlTDoASessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ShortMacDlTDoASessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ShortMacDlTDoASessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ShortMacDlTDoASessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ShortMacDlTDoASessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ShortMacDlTDoASessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ShortMacDlTDoASessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ShortMacDlTDoASessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ShortMacDlTDoASessionInfoNtfBuilder> for ShortMacDlTDoASessionInfoNtf {
    fn from(builder: ShortMacDlTDoASessionInfoNtfBuilder) -> ShortMacDlTDoASessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacDlTDoASessionInfoNtfData {
    no_of_ranging_measurements: u8,
    dl_tdoa_measurements: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacDlTDoASessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    extendedmacdltdoasessioninfontf: ExtendedMacDlTDoASessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacDlTDoASessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub dl_tdoa_measurements: Vec<u8>,
    pub no_of_ranging_measurements: u8,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
}
impl ExtendedMacDlTDoASessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacDlTDoASessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let no_of_ranging_measurements = bytes.get_mut().get_u8();
        let mut dl_tdoa_measurements = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            dl_tdoa_measurements.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            no_of_ranging_measurements,
            dl_tdoa_measurements,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.no_of_ranging_measurements);
        for elem in &self.dl_tdoa_measurements {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.dl_tdoa_measurements.len()
    }
}
impl Packet for ExtendedMacDlTDoASessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for Bytes {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for Vec<u8> {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for UciPacket {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for UciNotification {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for SessionControlNotification {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ExtendedMacDlTDoASessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ExtendedMacDlTDoASessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ExtendedMacDlTDoASessionInfoNtf> {
        ExtendedMacDlTDoASessionInfoNtf::new(packet.ucipacket)
    }
}
impl ExtendedMacDlTDoASessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let extendedmacdltdoasessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            extendedmacdltdoasessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_dl_tdoa_measurements(&self) -> &Vec<u8> {
        &self.extendedmacdltdoasessioninfontf.dl_tdoa_measurements
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_no_of_ranging_measurements(&self) -> u8 {
        self.extendedmacdltdoasessioninfontf
            .no_of_ranging_measurements
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.extendedmacdltdoasessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ExtendedMacDlTDoASessionInfoNtfBuilder {
    pub fn build(self) -> ExtendedMacDlTDoASessionInfoNtf {
        let extendedmacdltdoasessioninfontf = ExtendedMacDlTDoASessionInfoNtfData {
            dl_tdoa_measurements: self.dl_tdoa_measurements,
            no_of_ranging_measurements: self.no_of_ranging_measurements,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ExtendedAddress,
            ranging_measurement_type: RangingMeasurementType::DlTdoa,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ExtendedMacDlTDoASessionInfoNtf(
                extendedmacdltdoasessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ExtendedMacDlTDoASessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ExtendedMacDlTDoASessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ExtendedMacDlTDoASessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ExtendedMacDlTDoASessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ExtendedMacDlTDoASessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ExtendedMacDlTDoASessionInfoNtfBuilder> for ExtendedMacDlTDoASessionInfoNtf {
    fn from(builder: ExtendedMacDlTDoASessionInfoNtfBuilder) -> ExtendedMacDlTDoASessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacOwrAoaSessionInfoNtfData {
    owr_aoa_ranging_measurements: Vec<ShortAddressOwrAoaRangingMeasurement>,
    vendor_data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacOwrAoaSessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shortmacowraoasessioninfontf: ShortMacOwrAoaSessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortMacOwrAoaSessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub owr_aoa_ranging_measurements: Vec<ShortAddressOwrAoaRangingMeasurement>,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
    pub vendor_data: Vec<u8>,
}
impl ShortMacOwrAoaSessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacOwrAoaSessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let owr_aoa_ranging_measurements_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < owr_aoa_ranging_measurements_count * 13usize {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacOwrAoaSessionInfoNtf".to_string(),
                wanted: owr_aoa_ranging_measurements_count * 13usize,
                got: bytes.get().remaining(),
            });
        }
        let owr_aoa_ranging_measurements = (0..owr_aoa_ranging_measurements_count)
            .map(|_| ShortAddressOwrAoaRangingMeasurement::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        let mut vendor_data = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            vendor_data.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            owr_aoa_ranging_measurements,
            vendor_data,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.owr_aoa_ranging_measurements.len() as u8);
        for elem in &self.owr_aoa_ranging_measurements {
            elem.write_to(buffer);
        }
        for elem in &self.vendor_data {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .owr_aoa_ranging_measurements
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
            + self.vendor_data.len()
    }
}
impl Packet for ShortMacOwrAoaSessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for Bytes {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for Vec<u8> {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for UciPacket {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for UciNotification {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for SessionControlNotification {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ShortMacOwrAoaSessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ShortMacOwrAoaSessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ShortMacOwrAoaSessionInfoNtf> {
        ShortMacOwrAoaSessionInfoNtf::new(packet.ucipacket)
    }
}
impl ShortMacOwrAoaSessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let shortmacowraoasessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            shortmacowraoasessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_owr_aoa_ranging_measurements(&self) -> &Vec<ShortAddressOwrAoaRangingMeasurement> {
        &self
            .shortmacowraoasessioninfontf
            .owr_aoa_ranging_measurements
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    pub fn get_vendor_data(&self) -> &Vec<u8> {
        &self.shortmacowraoasessioninfontf.vendor_data
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.shortmacowraoasessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ShortMacOwrAoaSessionInfoNtfBuilder {
    pub fn build(self) -> ShortMacOwrAoaSessionInfoNtf {
        let shortmacowraoasessioninfontf = ShortMacOwrAoaSessionInfoNtfData {
            owr_aoa_ranging_measurements: self.owr_aoa_ranging_measurements,
            vendor_data: self.vendor_data,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ShortAddress,
            ranging_measurement_type: RangingMeasurementType::OwrAoa,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ShortMacOwrAoaSessionInfoNtf(
                shortmacowraoasessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ShortMacOwrAoaSessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ShortMacOwrAoaSessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ShortMacOwrAoaSessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ShortMacOwrAoaSessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ShortMacOwrAoaSessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ShortMacOwrAoaSessionInfoNtfBuilder> for ShortMacOwrAoaSessionInfoNtf {
    fn from(builder: ShortMacOwrAoaSessionInfoNtfBuilder) -> ShortMacOwrAoaSessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacOwrAoaSessionInfoNtfData {
    owr_aoa_ranging_measurements: Vec<ExtendedAddressOwrAoaRangingMeasurement>,
    vendor_data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacOwrAoaSessionInfoNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolnotification: SessionControlNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioninfontf: SessionInfoNtfData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    extendedmacowraoasessioninfontf: ExtendedMacOwrAoaSessionInfoNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMacOwrAoaSessionInfoNtfBuilder {
    pub current_ranging_interval: u32,
    pub owr_aoa_ranging_measurements: Vec<ExtendedAddressOwrAoaRangingMeasurement>,
    pub rcr_indicator: u8,
    pub sequence_number: u32,
    pub session_token: u32,
    pub vendor_data: Vec<u8>,
}
impl ExtendedMacOwrAoaSessionInfoNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacOwrAoaSessionInfoNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let owr_aoa_ranging_measurements_count = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < owr_aoa_ranging_measurements_count * 19usize {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacOwrAoaSessionInfoNtf".to_string(),
                wanted: owr_aoa_ranging_measurements_count * 19usize,
                got: bytes.get().remaining(),
            });
        }
        let owr_aoa_ranging_measurements = (0..owr_aoa_ranging_measurements_count)
            .map(|_| ExtendedAddressOwrAoaRangingMeasurement::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        let mut vendor_data = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            vendor_data.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            owr_aoa_ranging_measurements,
            vendor_data,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.owr_aoa_ranging_measurements.len() as u8);
        for elem in &self.owr_aoa_ranging_measurements {
            elem.write_to(buffer);
        }
        for elem in &self.vendor_data {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .owr_aoa_ranging_measurements
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
            + self.vendor_data.len()
    }
}
impl Packet for ExtendedMacOwrAoaSessionInfoNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for Bytes {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for Vec<u8> {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> Self {
        packet.to_vec()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for UciPacket {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for UciNotification {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for SessionControlNotification {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> SessionControlNotification {
        SessionControlNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtf> for SessionInfoNtf {
    fn from(packet: ExtendedMacOwrAoaSessionInfoNtf) -> SessionInfoNtf {
        SessionInfoNtf::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for ExtendedMacOwrAoaSessionInfoNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<ExtendedMacOwrAoaSessionInfoNtf> {
        ExtendedMacOwrAoaSessionInfoNtf::new(packet.ucipacket)
    }
}
impl ExtendedMacOwrAoaSessionInfoNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolnotification = match &ucinotification.child {
            UciNotificationDataChild::SessionControlNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::SessionControlNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let sessioninfontf = match &sessioncontrolnotification.child {
            SessionControlNotificationDataChild::SessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlNotificationDataChild::SessionInfoNtf),
                    actual: format!("{:?}", &sessioncontrolnotification.child),
                });
            }
        };
        let extendedmacowraoasessioninfontf = match &sessioninfontf.child {
            SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf),
                    actual: format!("{:?}", &sessioninfontf.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            sessioncontrolnotification,
            sessioninfontf,
            extendedmacowraoasessioninfontf,
        })
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.sessioninfontf.current_ranging_interval
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.sessioninfontf.mac_address_indicator
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_owr_aoa_ranging_measurements(
        &self,
    ) -> &Vec<ExtendedAddressOwrAoaRangingMeasurement> {
        &self
            .extendedmacowraoasessioninfontf
            .owr_aoa_ranging_measurements
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.sessioninfontf.ranging_measurement_type
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.sessioninfontf.rcr_indicator
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.sessioninfontf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.sessioninfontf.session_token
    }
    pub fn get_vendor_data(&self) -> &Vec<u8> {
        &self.extendedmacowraoasessioninfontf.vendor_data
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.extendedmacowraoasessioninfontf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl ExtendedMacOwrAoaSessionInfoNtfBuilder {
    pub fn build(self) -> ExtendedMacOwrAoaSessionInfoNtf {
        let extendedmacowraoasessioninfontf = ExtendedMacOwrAoaSessionInfoNtfData {
            owr_aoa_ranging_measurements: self.owr_aoa_ranging_measurements,
            vendor_data: self.vendor_data,
        };
        let sessioninfontf = SessionInfoNtfData {
            current_ranging_interval: self.current_ranging_interval,
            mac_address_indicator: MacAddressIndicator::ExtendedAddress,
            ranging_measurement_type: RangingMeasurementType::OwrAoa,
            rcr_indicator: self.rcr_indicator,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
            child: SessionInfoNtfDataChild::ExtendedMacOwrAoaSessionInfoNtf(
                extendedmacowraoasessioninfontf,
            ),
        };
        let sessioncontrolnotification = SessionControlNotificationData {
            child: SessionControlNotificationDataChild::SessionInfoNtf(sessioninfontf),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::SessionControlNotification(sessioncontrolnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Notification,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        ExtendedMacOwrAoaSessionInfoNtf::new(ucipacket).unwrap()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtfBuilder> for UciPacket {
    fn from(builder: ExtendedMacOwrAoaSessionInfoNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtfBuilder> for UciNotification {
    fn from(builder: ExtendedMacOwrAoaSessionInfoNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtfBuilder> for SessionControlNotification {
    fn from(builder: ExtendedMacOwrAoaSessionInfoNtfBuilder) -> SessionControlNotification {
        builder.build().into()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtfBuilder> for SessionInfoNtf {
    fn from(builder: ExtendedMacOwrAoaSessionInfoNtfBuilder) -> SessionInfoNtf {
        builder.build().into()
    }
}
impl From<ExtendedMacOwrAoaSessionInfoNtfBuilder> for ExtendedMacOwrAoaSessionInfoNtf {
    fn from(builder: ExtendedMacOwrAoaSessionInfoNtfBuilder) -> ExtendedMacOwrAoaSessionInfoNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolcommand: SessionControlCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionstopcmd: SessionStopCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopCmdBuilder {
    pub session_id: u32,
}
impl SessionStopCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for SessionStopCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStopCmd> for Bytes {
    fn from(packet: SessionStopCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStopCmd> for Vec<u8> {
    fn from(packet: SessionStopCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionStopCmd> for UciPacket {
    fn from(packet: SessionStopCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStopCmd> for UciCommand {
    fn from(packet: SessionStopCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStopCmd> for SessionControlCommand {
    fn from(packet: SessionStopCmd) -> SessionControlCommand {
        SessionControlCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionStopCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionStopCmd> {
        SessionStopCmd::new(packet.ucipacket)
    }
}
impl SessionStopCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolcommand = match &ucicommand.child {
            UciCommandDataChild::SessionControlCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionControlCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessionstopcmd = match &sessioncontrolcommand.child {
            SessionControlCommandDataChild::SessionStopCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlCommandDataChild::SessionStopCmd),
                    actual: format!("{:?}", &sessioncontrolcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessioncontrolcommand,
            sessionstopcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_id(&self) -> u32 {
        self.sessioncontrolcommand.session_id
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionstopcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionStopCmdBuilder {
    pub fn build(self) -> SessionStopCmd {
        let sessionstopcmd = SessionStopCmdData {};
        let sessioncontrolcommand = SessionControlCommandData {
            session_id: self.session_id,
            child: SessionControlCommandDataChild::SessionStopCmd(sessionstopcmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionControlCommand(sessioncontrolcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Command,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionStopCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionStopCmdBuilder> for UciPacket {
    fn from(builder: SessionStopCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionStopCmdBuilder> for UciCommand {
    fn from(builder: SessionStopCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionStopCmdBuilder> for SessionControlCommand {
    fn from(builder: SessionStopCmdBuilder) -> SessionControlCommand {
        builder.build().into()
    }
}
impl From<SessionStopCmdBuilder> for SessionStopCmd {
    fn from(builder: SessionStopCmdBuilder) -> SessionStopCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolresponse: SessionControlResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessionstoprsp: SessionStopRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionStopRspBuilder {
    pub status: StatusCode,
}
impl SessionStopRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStopRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionStopRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for SessionStopRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStopRsp> for Bytes {
    fn from(packet: SessionStopRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStopRsp> for Vec<u8> {
    fn from(packet: SessionStopRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionStopRsp> for UciPacket {
    fn from(packet: SessionStopRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStopRsp> for UciResponse {
    fn from(packet: SessionStopRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionStopRsp> for SessionControlResponse {
    fn from(packet: SessionStopRsp) -> SessionControlResponse {
        SessionControlResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionStopRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionStopRsp> {
        SessionStopRsp::new(packet.ucipacket)
    }
}
impl SessionStopRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolresponse = match &uciresponse.child {
            UciResponseDataChild::SessionControlResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionControlResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessionstoprsp = match &sessioncontrolresponse.child {
            SessionControlResponseDataChild::SessionStopRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlResponseDataChild::SessionStopRsp),
                    actual: format!("{:?}", &sessioncontrolresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessioncontrolresponse,
            sessionstoprsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessionstoprsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessionstoprsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionStopRspBuilder {
    pub fn build(self) -> SessionStopRsp {
        let sessionstoprsp = SessionStopRspData {
            status: self.status,
        };
        let sessioncontrolresponse = SessionControlResponseData {
            child: SessionControlResponseDataChild::SessionStopRsp(sessionstoprsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionControlResponse(sessioncontrolresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Response,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionStopRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionStopRspBuilder> for UciPacket {
    fn from(builder: SessionStopRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionStopRspBuilder> for UciResponse {
    fn from(builder: SessionStopRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionStopRspBuilder> for SessionControlResponse {
    fn from(builder: SessionStopRspBuilder) -> SessionControlResponse {
        builder.build().into()
    }
}
impl From<SessionStopRspBuilder> for SessionStopRsp {
    fn from(builder: SessionStopRspBuilder) -> SessionStopRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolcommand: SessionControlCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetrangingcountcmd: SessionGetRangingCountCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountCmdBuilder {
    pub session_id: u32,
}
impl SessionGetRangingCountCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for SessionGetRangingCountCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetRangingCountCmd> for Bytes {
    fn from(packet: SessionGetRangingCountCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetRangingCountCmd> for Vec<u8> {
    fn from(packet: SessionGetRangingCountCmd) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetRangingCountCmd> for UciPacket {
    fn from(packet: SessionGetRangingCountCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountCmd> for UciCommand {
    fn from(packet: SessionGetRangingCountCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountCmd> for SessionControlCommand {
    fn from(packet: SessionGetRangingCountCmd) -> SessionControlCommand {
        SessionControlCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetRangingCountCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetRangingCountCmd> {
        SessionGetRangingCountCmd::new(packet.ucipacket)
    }
}
impl SessionGetRangingCountCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolcommand = match &ucicommand.child {
            UciCommandDataChild::SessionControlCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::SessionControlCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let sessiongetrangingcountcmd = match &sessioncontrolcommand.child {
            SessionControlCommandDataChild::SessionGetRangingCountCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(SessionControlCommandDataChild::SessionGetRangingCountCmd),
                    actual: format!("{:?}", &sessioncontrolcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            sessioncontrolcommand,
            sessiongetrangingcountcmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_session_id(&self) -> u32 {
        self.sessioncontrolcommand.session_id
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetrangingcountcmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetRangingCountCmdBuilder {
    pub fn build(self) -> SessionGetRangingCountCmd {
        let sessiongetrangingcountcmd = SessionGetRangingCountCmdData {};
        let sessioncontrolcommand = SessionControlCommandData {
            session_id: self.session_id,
            child: SessionControlCommandDataChild::SessionGetRangingCountCmd(
                sessiongetrangingcountcmd,
            ),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::SessionControlCommand(sessioncontrolcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Command,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        SessionGetRangingCountCmd::new(ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountCmdBuilder> for UciPacket {
    fn from(builder: SessionGetRangingCountCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountCmdBuilder> for UciCommand {
    fn from(builder: SessionGetRangingCountCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountCmdBuilder> for SessionControlCommand {
    fn from(builder: SessionGetRangingCountCmdBuilder) -> SessionControlCommand {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountCmdBuilder> for SessionGetRangingCountCmd {
    fn from(builder: SessionGetRangingCountCmdBuilder) -> SessionGetRangingCountCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountRspData {
    status: StatusCode,
    count: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessioncontrolresponse: SessionControlResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    sessiongetrangingcountrsp: SessionGetRangingCountRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionGetRangingCountRspBuilder {
    pub count: u32,
    pub status: StatusCode,
}
impl SessionGetRangingCountRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetRangingCountRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "SessionGetRangingCountRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetRangingCountRsp".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let count = bytes.get_mut().get_u32_le();
        Ok(Self { status, count })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u32_le(self.count);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5
    }
}
impl Packet for SessionGetRangingCountRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetRangingCountRsp> for Bytes {
    fn from(packet: SessionGetRangingCountRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetRangingCountRsp> for Vec<u8> {
    fn from(packet: SessionGetRangingCountRsp) -> Self {
        packet.to_vec()
    }
}
impl From<SessionGetRangingCountRsp> for UciPacket {
    fn from(packet: SessionGetRangingCountRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountRsp> for UciResponse {
    fn from(packet: SessionGetRangingCountRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountRsp> for SessionControlResponse {
    fn from(packet: SessionGetRangingCountRsp) -> SessionControlResponse {
        SessionControlResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for SessionGetRangingCountRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<SessionGetRangingCountRsp> {
        SessionGetRangingCountRsp::new(packet.ucipacket)
    }
}
impl SessionGetRangingCountRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let sessioncontrolresponse = match &uciresponse.child {
            UciResponseDataChild::SessionControlResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::SessionControlResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let sessiongetrangingcountrsp = match &sessioncontrolresponse.child {
            SessionControlResponseDataChild::SessionGetRangingCountRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(
                        SessionControlResponseDataChild::SessionGetRangingCountRsp
                    ),
                    actual: format!("{:?}", &sessioncontrolresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            sessioncontrolresponse,
            sessiongetrangingcountrsp,
        })
    }
    pub fn get_count(&self) -> u32 {
        self.sessiongetrangingcountrsp.count
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.sessiongetrangingcountrsp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.sessiongetrangingcountrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl SessionGetRangingCountRspBuilder {
    pub fn build(self) -> SessionGetRangingCountRsp {
        let sessiongetrangingcountrsp = SessionGetRangingCountRspData {
            count: self.count,
            status: self.status,
        };
        let sessioncontrolresponse = SessionControlResponseData {
            child: SessionControlResponseDataChild::SessionGetRangingCountRsp(
                sessiongetrangingcountrsp,
            ),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::SessionControlResponse(sessioncontrolresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::SessionControl,
            message_type: MessageType::Response,
            opcode: 3,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        SessionGetRangingCountRsp::new(ucipacket).unwrap()
    }
}
impl From<SessionGetRangingCountRspBuilder> for UciPacket {
    fn from(builder: SessionGetRangingCountRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountRspBuilder> for UciResponse {
    fn from(builder: SessionGetRangingCountRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountRspBuilder> for SessionControlResponse {
    fn from(builder: SessionGetRangingCountRspBuilder) -> SessionControlResponse {
        builder.build().into()
    }
}
impl From<SessionGetRangingCountRspBuilder> for SessionGetRangingCountRsp {
    fn from(builder: SessionGetRangingCountRspBuilder) -> SessionGetRangingCountRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PicaPosition {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub yaw: u16,
    pub pitch: u8,
    pub roll: u16,
}
impl PicaPosition {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 11
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let x = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let y = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let z = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let yaw = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let pitch = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let roll = bytes.get_mut().get_u16_le();
        Ok(Self {
            x,
            y,
            z,
            yaw,
            pitch,
            roll,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(self.x);
        buffer.put_u16_le(self.y);
        buffer.put_u16_le(self.z);
        buffer.put_u16_le(self.yaw);
        buffer.put_u8(self.pitch);
        buffer.put_u16_le(self.roll);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        11
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsCmdData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidcommand: AndroidCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidgetpowerstatscmd: AndroidGetPowerStatsCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsCmdBuilder {}
impl AndroidGetPowerStatsCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for AndroidGetPowerStatsCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidGetPowerStatsCmd> for Bytes {
    fn from(packet: AndroidGetPowerStatsCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidGetPowerStatsCmd> for Vec<u8> {
    fn from(packet: AndroidGetPowerStatsCmd) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidGetPowerStatsCmd> for UciPacket {
    fn from(packet: AndroidGetPowerStatsCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsCmd> for UciCommand {
    fn from(packet: AndroidGetPowerStatsCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsCmd> for AndroidCommand {
    fn from(packet: AndroidGetPowerStatsCmd) -> AndroidCommand {
        AndroidCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidGetPowerStatsCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidGetPowerStatsCmd> {
        AndroidGetPowerStatsCmd::new(packet.ucipacket)
    }
}
impl AndroidGetPowerStatsCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidcommand = match &ucicommand.child {
            UciCommandDataChild::AndroidCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::AndroidCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let androidgetpowerstatscmd = match &androidcommand.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AndroidCommandDataChild::AndroidGetPowerStatsCmd),
                    actual: format!("{:?}", &androidcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            androidcommand,
            androidgetpowerstatscmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidgetpowerstatscmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidGetPowerStatsCmdBuilder {
    pub fn build(self) -> AndroidGetPowerStatsCmd {
        let androidgetpowerstatscmd = AndroidGetPowerStatsCmdData {};
        let androidcommand = AndroidCommandData {
            child: AndroidCommandDataChild::AndroidGetPowerStatsCmd(androidgetpowerstatscmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::AndroidCommand(androidcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Command,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        AndroidGetPowerStatsCmd::new(ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsCmdBuilder> for UciPacket {
    fn from(builder: AndroidGetPowerStatsCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsCmdBuilder> for UciCommand {
    fn from(builder: AndroidGetPowerStatsCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsCmdBuilder> for AndroidCommand {
    fn from(builder: AndroidGetPowerStatsCmdBuilder) -> AndroidCommand {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsCmdBuilder> for AndroidGetPowerStatsCmd {
    fn from(builder: AndroidGetPowerStatsCmdBuilder) -> AndroidGetPowerStatsCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerStats {
    pub status: StatusCode,
    pub idle_time_ms: u32,
    pub tx_time_ms: u32,
    pub rx_time_ms: u32,
    pub total_wake_count: u32,
}
impl PowerStats {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 17
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "PowerStats".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let idle_time_ms = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let tx_time_ms = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let rx_time_ms = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let total_wake_count = bytes.get_mut().get_u32_le();
        Ok(Self {
            status,
            idle_time_ms,
            tx_time_ms,
            rx_time_ms,
            total_wake_count,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
        buffer.put_u32_le(self.idle_time_ms);
        buffer.put_u32_le(self.tx_time_ms);
        buffer.put_u32_le(self.rx_time_ms);
        buffer.put_u32_le(self.total_wake_count);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        17
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsRspData {
    stats: PowerStats,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidresponse: AndroidResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidgetpowerstatsrsp: AndroidGetPowerStatsRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidGetPowerStatsRspBuilder {
    pub stats: PowerStats,
}
impl AndroidGetPowerStatsRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 17
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let (head, tail) = bytes.get().split_at(17);
        bytes.replace(tail);
        let stats = PowerStats::parse(head)?;
        Ok(Self { stats })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.stats.write_to(buffer);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        17
    }
}
impl Packet for AndroidGetPowerStatsRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidGetPowerStatsRsp> for Bytes {
    fn from(packet: AndroidGetPowerStatsRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidGetPowerStatsRsp> for Vec<u8> {
    fn from(packet: AndroidGetPowerStatsRsp) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidGetPowerStatsRsp> for UciPacket {
    fn from(packet: AndroidGetPowerStatsRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsRsp> for UciResponse {
    fn from(packet: AndroidGetPowerStatsRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsRsp> for AndroidResponse {
    fn from(packet: AndroidGetPowerStatsRsp) -> AndroidResponse {
        AndroidResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidGetPowerStatsRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidGetPowerStatsRsp> {
        AndroidGetPowerStatsRsp::new(packet.ucipacket)
    }
}
impl AndroidGetPowerStatsRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidresponse = match &uciresponse.child {
            UciResponseDataChild::AndroidResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::AndroidResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let androidgetpowerstatsrsp = match &androidresponse.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AndroidResponseDataChild::AndroidGetPowerStatsRsp),
                    actual: format!("{:?}", &androidresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            androidresponse,
            androidgetpowerstatsrsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_stats(&self) -> &PowerStats {
        &self.androidgetpowerstatsrsp.stats
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidgetpowerstatsrsp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidGetPowerStatsRspBuilder {
    pub fn build(self) -> AndroidGetPowerStatsRsp {
        let androidgetpowerstatsrsp = AndroidGetPowerStatsRspData { stats: self.stats };
        let androidresponse = AndroidResponseData {
            child: AndroidResponseDataChild::AndroidGetPowerStatsRsp(androidgetpowerstatsrsp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::AndroidResponse(androidresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Response,
            opcode: 0,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        AndroidGetPowerStatsRsp::new(ucipacket).unwrap()
    }
}
impl From<AndroidGetPowerStatsRspBuilder> for UciPacket {
    fn from(builder: AndroidGetPowerStatsRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsRspBuilder> for UciResponse {
    fn from(builder: AndroidGetPowerStatsRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsRspBuilder> for AndroidResponse {
    fn from(builder: AndroidGetPowerStatsRspBuilder) -> AndroidResponse {
        builder.build().into()
    }
}
impl From<AndroidGetPowerStatsRspBuilder> for AndroidGetPowerStatsRsp {
    fn from(builder: AndroidGetPowerStatsRspBuilder) -> AndroidGetPowerStatsRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeCmdData {
    country_code: [u8; 2],
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucicommand: UciCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidcommand: AndroidCommandData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidsetcountrycodecmd: AndroidSetCountryCodeCmdData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeCmdBuilder {
    pub country_code: [u8; 2],
}
impl AndroidSetCountryCodeCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidSetCountryCodeCmd".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let country_code = (0..2)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u8()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        Ok(Self { country_code })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.country_code {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
impl Packet for AndroidSetCountryCodeCmd {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidSetCountryCodeCmd> for Bytes {
    fn from(packet: AndroidSetCountryCodeCmd) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidSetCountryCodeCmd> for Vec<u8> {
    fn from(packet: AndroidSetCountryCodeCmd) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidSetCountryCodeCmd> for UciPacket {
    fn from(packet: AndroidSetCountryCodeCmd) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeCmd> for UciCommand {
    fn from(packet: AndroidSetCountryCodeCmd) -> UciCommand {
        UciCommand::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeCmd> for AndroidCommand {
    fn from(packet: AndroidSetCountryCodeCmd) -> AndroidCommand {
        AndroidCommand::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidSetCountryCodeCmd {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidSetCountryCodeCmd> {
        AndroidSetCountryCodeCmd::new(packet.ucipacket)
    }
}
impl AndroidSetCountryCodeCmd {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucicommand = match &ucipacket.child {
            UciPacketDataChild::UciCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciCommand),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidcommand = match &ucicommand.child {
            UciCommandDataChild::AndroidCommand(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciCommandDataChild::AndroidCommand),
                    actual: format!("{:?}", &ucicommand.child),
                });
            }
        };
        let androidsetcountrycodecmd = match &androidcommand.child {
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AndroidCommandDataChild::AndroidSetCountryCodeCmd),
                    actual: format!("{:?}", &androidcommand.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucicommand,
            androidcommand,
            androidsetcountrycodecmd,
        })
    }
    pub fn get_country_code(&self) -> &[u8; 2] {
        &self.androidsetcountrycodecmd.country_code
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidsetcountrycodecmd.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidSetCountryCodeCmdBuilder {
    pub fn build(self) -> AndroidSetCountryCodeCmd {
        let androidsetcountrycodecmd = AndroidSetCountryCodeCmdData {
            country_code: self.country_code,
        };
        let androidcommand = AndroidCommandData {
            child: AndroidCommandDataChild::AndroidSetCountryCodeCmd(androidsetcountrycodecmd),
        };
        let ucicommand = UciCommandData {
            child: UciCommandDataChild::AndroidCommand(androidcommand),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Command,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciCommand(ucicommand),
        };
        AndroidSetCountryCodeCmd::new(ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeCmdBuilder> for UciPacket {
    fn from(builder: AndroidSetCountryCodeCmdBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeCmdBuilder> for UciCommand {
    fn from(builder: AndroidSetCountryCodeCmdBuilder) -> UciCommand {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeCmdBuilder> for AndroidCommand {
    fn from(builder: AndroidSetCountryCodeCmdBuilder) -> AndroidCommand {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeCmdBuilder> for AndroidSetCountryCodeCmd {
    fn from(builder: AndroidSetCountryCodeCmdBuilder) -> AndroidSetCountryCodeCmd {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeRspData {
    status: StatusCode,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeRsp {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidresponse: AndroidResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidsetcountrycodersp: AndroidSetCountryCodeRspData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidSetCountryCodeRspBuilder {
    pub status: StatusCode,
}
impl AndroidSetCountryCodeRspData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidSetCountryCodeRsp".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let status = StatusCode::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "AndroidSetCountryCodeRsp".to_string(),
                field: "status".to_string(),
                value: unknown_val as u64,
                type_: "StatusCode".to_string(),
            }
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.status));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for AndroidSetCountryCodeRsp {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidSetCountryCodeRsp> for Bytes {
    fn from(packet: AndroidSetCountryCodeRsp) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidSetCountryCodeRsp> for Vec<u8> {
    fn from(packet: AndroidSetCountryCodeRsp) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidSetCountryCodeRsp> for UciPacket {
    fn from(packet: AndroidSetCountryCodeRsp) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeRsp> for UciResponse {
    fn from(packet: AndroidSetCountryCodeRsp) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeRsp> for AndroidResponse {
    fn from(packet: AndroidSetCountryCodeRsp) -> AndroidResponse {
        AndroidResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidSetCountryCodeRsp {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidSetCountryCodeRsp> {
        AndroidSetCountryCodeRsp::new(packet.ucipacket)
    }
}
impl AndroidSetCountryCodeRsp {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidresponse = match &uciresponse.child {
            UciResponseDataChild::AndroidResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::AndroidResponse),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        let androidsetcountrycodersp = match &androidresponse.child {
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AndroidResponseDataChild::AndroidSetCountryCodeRsp),
                    actual: format!("{:?}", &androidresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            androidresponse,
            androidsetcountrycodersp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_status(&self) -> StatusCode {
        self.androidsetcountrycodersp.status
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidsetcountrycodersp.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidSetCountryCodeRspBuilder {
    pub fn build(self) -> AndroidSetCountryCodeRsp {
        let androidsetcountrycodersp = AndroidSetCountryCodeRspData {
            status: self.status,
        };
        let androidresponse = AndroidResponseData {
            child: AndroidResponseDataChild::AndroidSetCountryCodeRsp(androidsetcountrycodersp),
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::AndroidResponse(androidresponse),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Response,
            opcode: 1,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        AndroidSetCountryCodeRsp::new(ucipacket).unwrap()
    }
}
impl From<AndroidSetCountryCodeRspBuilder> for UciPacket {
    fn from(builder: AndroidSetCountryCodeRspBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeRspBuilder> for UciResponse {
    fn from(builder: AndroidSetCountryCodeRspBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeRspBuilder> for AndroidResponse {
    fn from(builder: AndroidSetCountryCodeRspBuilder) -> AndroidResponse {
        builder.build().into()
    }
}
impl From<AndroidSetCountryCodeRspBuilder> for AndroidSetCountryCodeRsp {
    fn from(builder: AndroidSetCountryCodeRspBuilder) -> AndroidSetCountryCodeRsp {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameReportTlv {
    pub t: FrameReportTlvType,
    pub v: Vec<u8>,
}
impl FrameReportTlv {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlv".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let t = FrameReportTlvType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "FrameReportTlv".to_string(),
                field: "t".to_string(),
                value: unknown_val as u64,
                type_: "FrameReportTlvType".to_string(),
            }
        })?;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlv".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let v_size = bytes.get_mut().get_u16_le() as usize;
        if bytes.get().remaining() < v_size {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlv".to_string(),
                wanted: v_size,
                got: bytes.get().remaining(),
            });
        }
        let mut v = Vec::with_capacity(v_size);
        for _ in 0..v_size {
            v.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self { t, v })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.t));
        if self.v.len() > 0xffff {
            panic!(
                "Invalid length for {}::{}: {} > {}",
                "FrameReportTlv",
                "v",
                self.v.len(),
                0xffff
            );
        }
        buffer.put_u16_le(self.v.len() as u16);
        for elem in &self.v {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3 + self.v.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FrameReportTlvPacketDataChild {
    Rssi(RssiData),
    Aoa(AoaData),
    Cir(CirData),
    Payload(Bytes),
    None,
}
impl FrameReportTlvPacketDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            FrameReportTlvPacketDataChild::Rssi(value) => value.get_total_size(),
            FrameReportTlvPacketDataChild::Aoa(value) => value.get_total_size(),
            FrameReportTlvPacketDataChild::Cir(value) => value.get_total_size(),
            FrameReportTlvPacketDataChild::Payload(bytes) => bytes.len(),
            FrameReportTlvPacketDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FrameReportTlvPacketChild {
    Rssi(Rssi),
    Aoa(Aoa),
    Cir(Cir),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameReportTlvPacketData {
    t: FrameReportTlvType,
    child: FrameReportTlvPacketDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameReportTlvPacket {
    #[cfg_attr(feature = "serde", serde(flatten))]
    framereporttlvpacket: FrameReportTlvPacketData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameReportTlvPacketBuilder {
    pub t: FrameReportTlvType,
    pub payload: Option<Bytes>,
}
impl FrameReportTlvPacketData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlvPacket".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let t = FrameReportTlvType::try_from(bytes.get_mut().get_u8()).map_err(|unknown_val| {
            Error::InvalidEnumValueError {
                obj: "FrameReportTlvPacket".to_string(),
                field: "t".to_string(),
                value: unknown_val as u64,
                type_: "FrameReportTlvType".to_string(),
            }
        })?;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlvPacket".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let body_size = bytes.get_mut().get_u16_le() as usize;
        if bytes.get().remaining() < body_size {
            return Err(Error::InvalidLengthError {
                obj: "FrameReportTlvPacket".to_string(),
                wanted: body_size,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..body_size];
        bytes.get_mut().advance(body_size);
        let child = match (t) {
            (FrameReportTlvType::Rssi) if RssiData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = RssiData::parse_inner(&mut cell)?;
                FrameReportTlvPacketDataChild::Rssi(child_data)
            }
            (FrameReportTlvType::Aoa) if AoaData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AoaData::parse_inner(&mut cell)?;
                FrameReportTlvPacketDataChild::Aoa(child_data)
            }
            (FrameReportTlvType::Cir) if CirData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = CirData::parse_inner(&mut cell)?;
                FrameReportTlvPacketDataChild::Cir(child_data)
            }
            _ if !payload.is_empty() => {
                FrameReportTlvPacketDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => FrameReportTlvPacketDataChild::None,
        };
        Ok(Self { t, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.t));
        if self.child.get_total_size() > 0xffff {
            panic!(
                "Invalid length for {}::{}: {} > {}",
                "FrameReportTlvPacket",
                "_body_",
                self.child.get_total_size(),
                0xffff
            );
        }
        buffer.put_u16_le(self.child.get_total_size() as u16);
        match &self.child {
            FrameReportTlvPacketDataChild::Rssi(child) => child.write_to(buffer),
            FrameReportTlvPacketDataChild::Aoa(child) => child.write_to(buffer),
            FrameReportTlvPacketDataChild::Cir(child) => child.write_to(buffer),
            FrameReportTlvPacketDataChild::Payload(payload) => buffer.put_slice(payload),
            FrameReportTlvPacketDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3 + self.child.get_total_size()
    }
}
impl Packet for FrameReportTlvPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.framereporttlvpacket.get_size());
        self.framereporttlvpacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<FrameReportTlvPacket> for Bytes {
    fn from(packet: FrameReportTlvPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<FrameReportTlvPacket> for Vec<u8> {
    fn from(packet: FrameReportTlvPacket) -> Self {
        packet.to_vec()
    }
}
impl FrameReportTlvPacket {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FrameReportTlvPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> FrameReportTlvPacketChild {
        match &self.framereporttlvpacket.child {
            FrameReportTlvPacketDataChild::Rssi(_) => FrameReportTlvPacketChild::Rssi(
                Rssi::new(self.framereporttlvpacket.clone()).unwrap(),
            ),
            FrameReportTlvPacketDataChild::Aoa(_) => {
                FrameReportTlvPacketChild::Aoa(Aoa::new(self.framereporttlvpacket.clone()).unwrap())
            }
            FrameReportTlvPacketDataChild::Cir(_) => {
                FrameReportTlvPacketChild::Cir(Cir::new(self.framereporttlvpacket.clone()).unwrap())
            }
            FrameReportTlvPacketDataChild::Payload(payload) => {
                FrameReportTlvPacketChild::Payload(payload.clone())
            }
            FrameReportTlvPacketDataChild::None => FrameReportTlvPacketChild::None,
        }
    }
    fn new(framereporttlvpacket: FrameReportTlvPacketData) -> Result<Self> {
        Ok(Self {
            framereporttlvpacket,
        })
    }
    pub fn get_t(&self) -> FrameReportTlvType {
        self.framereporttlvpacket.t
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.framereporttlvpacket.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.framereporttlvpacket.get_size()
    }
}
impl FrameReportTlvPacketBuilder {
    pub fn build(self) -> FrameReportTlvPacket {
        let framereporttlvpacket = FrameReportTlvPacketData {
            t: self.t,
            child: match self.payload {
                None => FrameReportTlvPacketDataChild::None,
                Some(bytes) => FrameReportTlvPacketDataChild::Payload(bytes),
            },
        };
        FrameReportTlvPacket::new(framereporttlvpacket).unwrap()
    }
}
impl From<FrameReportTlvPacketBuilder> for FrameReportTlvPacket {
    fn from(builder: FrameReportTlvPacketBuilder) -> FrameReportTlvPacket {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RssiData {
    rssi: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rssi {
    #[cfg_attr(feature = "serde", serde(flatten))]
    framereporttlvpacket: FrameReportTlvPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    rssi: RssiData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RssiBuilder {
    pub rssi: Vec<u8>,
}
impl RssiData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let mut rssi = Vec::with_capacity(bytes.get().remaining());
        for _ in 0..bytes.get().remaining() {
            rssi.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self { rssi })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.rssi {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.rssi.len()
    }
}
impl Packet for Rssi {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.framereporttlvpacket.get_size());
        self.framereporttlvpacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Rssi> for Bytes {
    fn from(packet: Rssi) -> Self {
        packet.to_bytes()
    }
}
impl From<Rssi> for Vec<u8> {
    fn from(packet: Rssi) -> Self {
        packet.to_vec()
    }
}
impl From<Rssi> for FrameReportTlvPacket {
    fn from(packet: Rssi) -> FrameReportTlvPacket {
        FrameReportTlvPacket::new(packet.framereporttlvpacket).unwrap()
    }
}
impl TryFrom<FrameReportTlvPacket> for Rssi {
    type Error = Error;
    fn try_from(packet: FrameReportTlvPacket) -> Result<Rssi> {
        Rssi::new(packet.framereporttlvpacket)
    }
}
impl Rssi {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FrameReportTlvPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(framereporttlvpacket: FrameReportTlvPacketData) -> Result<Self> {
        let rssi = match &framereporttlvpacket.child {
            FrameReportTlvPacketDataChild::Rssi(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(FrameReportTlvPacketDataChild::Rssi),
                    actual: format!("{:?}", &framereporttlvpacket.child),
                });
            }
        };
        Ok(Self {
            framereporttlvpacket,
            rssi,
        })
    }
    pub fn get_rssi(&self) -> &Vec<u8> {
        &self.rssi.rssi
    }
    pub fn get_t(&self) -> FrameReportTlvType {
        self.framereporttlvpacket.t
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.rssi.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.framereporttlvpacket.get_size()
    }
}
impl RssiBuilder {
    pub fn build(self) -> Rssi {
        let rssi = RssiData { rssi: self.rssi };
        let framereporttlvpacket = FrameReportTlvPacketData {
            t: FrameReportTlvType::Rssi,
            child: FrameReportTlvPacketDataChild::Rssi(rssi),
        };
        Rssi::new(framereporttlvpacket).unwrap()
    }
}
impl From<RssiBuilder> for FrameReportTlvPacket {
    fn from(builder: RssiBuilder) -> FrameReportTlvPacket {
        builder.build().into()
    }
}
impl From<RssiBuilder> for Rssi {
    fn from(builder: RssiBuilder) -> Rssi {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AoaMeasurement {
    pub tdoa: u16,
    pub pdoa: u16,
    pub aoa: u16,
    pub fom: u8,
    pub t: u8,
}
impl AoaMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 8
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AoaMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let tdoa = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AoaMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let pdoa = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AoaMeasurement".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let aoa = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AoaMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let fom = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AoaMeasurement".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let t = bytes.get_mut().get_u8();
        Ok(Self {
            tdoa,
            pdoa,
            aoa,
            fom,
            t,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(self.tdoa);
        buffer.put_u16_le(self.pdoa);
        buffer.put_u16_le(self.aoa);
        buffer.put_u8(self.fom);
        buffer.put_u8(self.t);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        8
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AoaData {
    aoa: Vec<AoaMeasurement>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aoa {
    #[cfg_attr(feature = "serde", serde(flatten))]
    framereporttlvpacket: FrameReportTlvPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    aoa: AoaData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AoaBuilder {
    pub aoa: Vec<AoaMeasurement>,
}
impl AoaData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() % 8 != 0 {
            return Err(Error::InvalidArraySize {
                array: bytes.get().remaining(),
                element: 8,
            });
        }
        let aoa_count = bytes.get().remaining() / 8;
        let mut aoa = Vec::with_capacity(aoa_count);
        for _ in 0..aoa_count {
            aoa.push(AoaMeasurement::parse_inner(bytes)?);
        }
        Ok(Self { aoa })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.aoa {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.aoa.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for Aoa {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.framereporttlvpacket.get_size());
        self.framereporttlvpacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Aoa> for Bytes {
    fn from(packet: Aoa) -> Self {
        packet.to_bytes()
    }
}
impl From<Aoa> for Vec<u8> {
    fn from(packet: Aoa) -> Self {
        packet.to_vec()
    }
}
impl From<Aoa> for FrameReportTlvPacket {
    fn from(packet: Aoa) -> FrameReportTlvPacket {
        FrameReportTlvPacket::new(packet.framereporttlvpacket).unwrap()
    }
}
impl TryFrom<FrameReportTlvPacket> for Aoa {
    type Error = Error;
    fn try_from(packet: FrameReportTlvPacket) -> Result<Aoa> {
        Aoa::new(packet.framereporttlvpacket)
    }
}
impl Aoa {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FrameReportTlvPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(framereporttlvpacket: FrameReportTlvPacketData) -> Result<Self> {
        let aoa = match &framereporttlvpacket.child {
            FrameReportTlvPacketDataChild::Aoa(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(FrameReportTlvPacketDataChild::Aoa),
                    actual: format!("{:?}", &framereporttlvpacket.child),
                });
            }
        };
        Ok(Self {
            framereporttlvpacket,
            aoa,
        })
    }
    pub fn get_aoa(&self) -> &Vec<AoaMeasurement> {
        &self.aoa.aoa
    }
    pub fn get_t(&self) -> FrameReportTlvType {
        self.framereporttlvpacket.t
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.aoa.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.framereporttlvpacket.get_size()
    }
}
impl AoaBuilder {
    pub fn build(self) -> Aoa {
        let aoa = AoaData { aoa: self.aoa };
        let framereporttlvpacket = FrameReportTlvPacketData {
            t: FrameReportTlvType::Aoa,
            child: FrameReportTlvPacketDataChild::Aoa(aoa),
        };
        Aoa::new(framereporttlvpacket).unwrap()
    }
}
impl From<AoaBuilder> for FrameReportTlvPacket {
    fn from(builder: AoaBuilder) -> FrameReportTlvPacket {
        builder.build().into()
    }
}
impl From<AoaBuilder> for Aoa {
    fn from(builder: AoaBuilder) -> Aoa {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CirValue {
    pub first_path_index: u16,
    pub first_path_snr: u16,
    pub first_path_ns: u16,
    pub peak_path_index: u16,
    pub peak_path_snr: u16,
    pub peak_path_ns: u16,
    pub first_path_sample_offset: u8,
    pub samples_number: u8,
    pub sample_window: Vec<u8>,
}
impl CirValue {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 16
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let first_path_index = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let first_path_snr = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let first_path_ns = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let peak_path_index = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let peak_path_snr = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let peak_path_ns = bytes.get_mut().get_u16_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let first_path_sample_offset = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let samples_number = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let sample_window_size = bytes.get_mut().get_u16_le() as usize;
        if bytes.get().remaining() < sample_window_size {
            return Err(Error::InvalidLengthError {
                obj: "CirValue".to_string(),
                wanted: sample_window_size,
                got: bytes.get().remaining(),
            });
        }
        let mut sample_window = Vec::with_capacity(sample_window_size);
        for _ in 0..sample_window_size {
            sample_window.push(Ok::<_, Error>(bytes.get_mut().get_u8())?);
        }
        Ok(Self {
            first_path_index,
            first_path_snr,
            first_path_ns,
            peak_path_index,
            peak_path_snr,
            peak_path_ns,
            first_path_sample_offset,
            samples_number,
            sample_window,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(self.first_path_index);
        buffer.put_u16_le(self.first_path_snr);
        buffer.put_u16_le(self.first_path_ns);
        buffer.put_u16_le(self.peak_path_index);
        buffer.put_u16_le(self.peak_path_snr);
        buffer.put_u16_le(self.peak_path_ns);
        buffer.put_u8(self.first_path_sample_offset);
        buffer.put_u8(self.samples_number);
        if self.sample_window.len() > 0xffff {
            panic!(
                "Invalid length for {}::{}: {} > {}",
                "CirValue",
                "sample_window",
                self.sample_window.len(),
                0xffff
            );
        }
        buffer.put_u16_le(self.sample_window.len() as u16);
        for elem in &self.sample_window {
            buffer.put_u8(*elem);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        16 + self.sample_window.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CirData {
    cir_value: Vec<CirValue>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cir {
    #[cfg_attr(feature = "serde", serde(flatten))]
    framereporttlvpacket: FrameReportTlvPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    cir: CirData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CirBuilder {
    pub cir_value: Vec<CirValue>,
}
impl CirData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Cir".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let cir_value_count = bytes.get_mut().get_u8() as usize;
        let cir_value = (0..cir_value_count)
            .map(|_| CirValue::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cir_value })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.cir_value.len() as u8);
        for elem in &self.cir_value {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self
            .cir_value
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for Cir {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.framereporttlvpacket.get_size());
        self.framereporttlvpacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Cir> for Bytes {
    fn from(packet: Cir) -> Self {
        packet.to_bytes()
    }
}
impl From<Cir> for Vec<u8> {
    fn from(packet: Cir) -> Self {
        packet.to_vec()
    }
}
impl From<Cir> for FrameReportTlvPacket {
    fn from(packet: Cir) -> FrameReportTlvPacket {
        FrameReportTlvPacket::new(packet.framereporttlvpacket).unwrap()
    }
}
impl TryFrom<FrameReportTlvPacket> for Cir {
    type Error = Error;
    fn try_from(packet: FrameReportTlvPacket) -> Result<Cir> {
        Cir::new(packet.framereporttlvpacket)
    }
}
impl Cir {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FrameReportTlvPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(framereporttlvpacket: FrameReportTlvPacketData) -> Result<Self> {
        let cir = match &framereporttlvpacket.child {
            FrameReportTlvPacketDataChild::Cir(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(FrameReportTlvPacketDataChild::Cir),
                    actual: format!("{:?}", &framereporttlvpacket.child),
                });
            }
        };
        Ok(Self {
            framereporttlvpacket,
            cir,
        })
    }
    pub fn get_cir_value(&self) -> &Vec<CirValue> {
        &self.cir.cir_value
    }
    pub fn get_t(&self) -> FrameReportTlvType {
        self.framereporttlvpacket.t
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.cir.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.framereporttlvpacket.get_size()
    }
}
impl CirBuilder {
    pub fn build(self) -> Cir {
        let cir = CirData {
            cir_value: self.cir_value,
        };
        let framereporttlvpacket = FrameReportTlvPacketData {
            t: FrameReportTlvType::Cir,
            child: FrameReportTlvPacketDataChild::Cir(cir),
        };
        Cir::new(framereporttlvpacket).unwrap()
    }
}
impl From<CirBuilder> for FrameReportTlvPacket {
    fn from(builder: CirBuilder) -> FrameReportTlvPacket {
        builder.build().into()
    }
}
impl From<CirBuilder> for Cir {
    fn from(builder: CirBuilder) -> Cir {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameReport {
    pub uwb_msg_id: u8,
    pub action: u8,
    pub antenna_set: u8,
    pub frame_report_tlvs: Vec<FrameReportTlv>,
}
impl FrameReport {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReport".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let uwb_msg_id = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReport".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let action = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReport".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let antenna_set = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "FrameReport".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let frame_report_tlvs_count = bytes.get_mut().get_u8() as usize;
        let frame_report_tlvs = (0..frame_report_tlvs_count)
            .map(|_| FrameReportTlv::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            uwb_msg_id,
            action,
            antenna_set,
            frame_report_tlvs,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.uwb_msg_id);
        buffer.put_u8(self.action);
        buffer.put_u8(self.antenna_set);
        buffer.put_u8(self.frame_report_tlvs.len() as u8);
        for elem in &self.frame_report_tlvs {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4 + self
            .frame_report_tlvs
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidRangeDiagnosticsNtfData {
    session_token: u32,
    sequence_number: u32,
    frame_reports: Vec<FrameReport>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidRangeDiagnosticsNtf {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidnotification: AndroidNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    androidrangediagnosticsntf: AndroidRangeDiagnosticsNtfData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AndroidRangeDiagnosticsNtfBuilder {
    pub frame_reports: Vec<FrameReport>,
    pub sequence_number: u32,
    pub session_token: u32,
}
impl AndroidRangeDiagnosticsNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 9
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidRangeDiagnosticsNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let session_token = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidRangeDiagnosticsNtf".to_string(),
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let sequence_number = bytes.get_mut().get_u32_le();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidRangeDiagnosticsNtf".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let frame_reports_count = bytes.get_mut().get_u8() as usize;
        let frame_reports = (0..frame_reports_count)
            .map(|_| FrameReport::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            session_token,
            sequence_number,
            frame_reports,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u32_le(self.session_token);
        buffer.put_u32_le(self.sequence_number);
        buffer.put_u8(self.frame_reports.len() as u8);
        for elem in &self.frame_reports {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        9 + self
            .frame_reports
            .iter()
            .map(|elem| elem.get_size())
            .sum::<usize>()
    }
}
impl Packet for AndroidRangeDiagnosticsNtf {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidRangeDiagnosticsNtf> for Bytes {
    fn from(packet: AndroidRangeDiagnosticsNtf) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidRangeDiagnosticsNtf> for Vec<u8> {
    fn from(packet: AndroidRangeDiagnosticsNtf) -> Self {
        packet.to_vec()
    }
}
impl From<AndroidRangeDiagnosticsNtf> for UciPacket {
    fn from(packet: AndroidRangeDiagnosticsNtf) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidRangeDiagnosticsNtf> for UciNotification {
    fn from(packet: AndroidRangeDiagnosticsNtf) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl From<AndroidRangeDiagnosticsNtf> for AndroidNotification {
    fn from(packet: AndroidRangeDiagnosticsNtf) -> AndroidNotification {
        AndroidNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for AndroidRangeDiagnosticsNtf {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<AndroidRangeDiagnosticsNtf> {
        AndroidRangeDiagnosticsNtf::new(packet.ucipacket)
    }
}
impl AndroidRangeDiagnosticsNtf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let androidnotification = match &ucinotification.child {
            UciNotificationDataChild::AndroidNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::AndroidNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        let androidrangediagnosticsntf = match &androidnotification.child {
            AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf),
                    actual: format!("{:?}", &androidnotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            androidnotification,
            androidrangediagnosticsntf,
        })
    }
    pub fn get_frame_reports(&self) -> &Vec<FrameReport> {
        &self.androidrangediagnosticsntf.frame_reports
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.androidrangediagnosticsntf.sequence_number
    }
    pub fn get_session_token(&self) -> u32 {
        self.androidrangediagnosticsntf.session_token
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.androidrangediagnosticsntf.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl AndroidRangeDiagnosticsNtfBuilder {
    pub fn build(self) -> AndroidRangeDiagnosticsNtf {
        let androidrangediagnosticsntf = AndroidRangeDiagnosticsNtfData {
            frame_reports: self.frame_reports,
            sequence_number: self.sequence_number,
            session_token: self.session_token,
        };
        let androidnotification = AndroidNotificationData {
            child: AndroidNotificationDataChild::AndroidRangeDiagnosticsNtf(
                androidrangediagnosticsntf,
            ),
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::AndroidNotification(androidnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorAndroid,
            message_type: MessageType::Notification,
            opcode: 2,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        AndroidRangeDiagnosticsNtf::new(ucipacket).unwrap()
    }
}
impl From<AndroidRangeDiagnosticsNtfBuilder> for UciPacket {
    fn from(builder: AndroidRangeDiagnosticsNtfBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<AndroidRangeDiagnosticsNtfBuilder> for UciNotification {
    fn from(builder: AndroidRangeDiagnosticsNtfBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<AndroidRangeDiagnosticsNtfBuilder> for AndroidNotification {
    fn from(builder: AndroidRangeDiagnosticsNtfBuilder) -> AndroidNotification {
        builder.build().into()
    }
}
impl From<AndroidRangeDiagnosticsNtfBuilder> for AndroidRangeDiagnosticsNtf {
    fn from(builder: AndroidRangeDiagnosticsNtfBuilder) -> AndroidRangeDiagnosticsNtf {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_9_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_9_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_9_ResponseDataChild::Payload(bytes) => bytes.len(),
            UciVendor_9_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_9_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_ResponseData {
    child: UciVendor_9_ResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_Response {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_9_response: UciVendor_9_ResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_9_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_9_ResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_9_ResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_9_ResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_9_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_9_Response {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_9_Response> for Bytes {
    fn from(packet: UciVendor_9_Response) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_9_Response> for Vec<u8> {
    fn from(packet: UciVendor_9_Response) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_9_Response> for UciPacket {
    fn from(packet: UciVendor_9_Response) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_9_Response> for UciResponse {
    fn from(packet: UciVendor_9_Response) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_9_Response {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_9_Response> {
        UciVendor_9_Response::new(packet.ucipacket)
    }
}
impl UciVendor_9_Response {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_9_ResponseChild {
        match &self.ucivendor_9_response.child {
            UciVendor_9_ResponseDataChild::Payload(payload) => {
                UciVendor_9_ResponseChild::Payload(payload.clone())
            }
            UciVendor_9_ResponseDataChild::None => UciVendor_9_ResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_9_response = match &uciresponse.child {
            UciResponseDataChild::UciVendor_9_Response(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::UciVendor_9_Response),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            ucivendor_9_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_9_response.child {
            UciVendor_9_ResponseDataChild::Payload(bytes) => &bytes,
            UciVendor_9_ResponseDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_9_response.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_9_ResponseBuilder {
    pub fn build(self) -> UciVendor_9_Response {
        let ucivendor_9_response = UciVendor_9_ResponseData {
            child: match self.payload {
                None => UciVendor_9_ResponseDataChild::None,
                Some(bytes) => UciVendor_9_ResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::UciVendor_9_Response(ucivendor_9_response),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReserved9,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciVendor_9_Response::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_9_ResponseBuilder> for UciPacket {
    fn from(builder: UciVendor_9_ResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_9_ResponseBuilder> for UciResponse {
    fn from(builder: UciVendor_9_ResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<UciVendor_9_ResponseBuilder> for UciVendor_9_Response {
    fn from(builder: UciVendor_9_ResponseBuilder) -> UciVendor_9_Response {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_A_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_A_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_A_ResponseDataChild::Payload(bytes) => bytes.len(),
            UciVendor_A_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_A_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_ResponseData {
    child: UciVendor_A_ResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_Response {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_a_response: UciVendor_A_ResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_A_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_A_ResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_A_ResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_A_ResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_A_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_A_Response {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_A_Response> for Bytes {
    fn from(packet: UciVendor_A_Response) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_A_Response> for Vec<u8> {
    fn from(packet: UciVendor_A_Response) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_A_Response> for UciPacket {
    fn from(packet: UciVendor_A_Response) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_A_Response> for UciResponse {
    fn from(packet: UciVendor_A_Response) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_A_Response {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_A_Response> {
        UciVendor_A_Response::new(packet.ucipacket)
    }
}
impl UciVendor_A_Response {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_A_ResponseChild {
        match &self.ucivendor_a_response.child {
            UciVendor_A_ResponseDataChild::Payload(payload) => {
                UciVendor_A_ResponseChild::Payload(payload.clone())
            }
            UciVendor_A_ResponseDataChild::None => UciVendor_A_ResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_a_response = match &uciresponse.child {
            UciResponseDataChild::UciVendor_A_Response(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::UciVendor_A_Response),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            ucivendor_a_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_a_response.child {
            UciVendor_A_ResponseDataChild::Payload(bytes) => &bytes,
            UciVendor_A_ResponseDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_a_response.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_A_ResponseBuilder {
    pub fn build(self) -> UciVendor_A_Response {
        let ucivendor_a_response = UciVendor_A_ResponseData {
            child: match self.payload {
                None => UciVendor_A_ResponseDataChild::None,
                Some(bytes) => UciVendor_A_ResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::UciVendor_A_Response(ucivendor_a_response),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedA,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciVendor_A_Response::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_A_ResponseBuilder> for UciPacket {
    fn from(builder: UciVendor_A_ResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_A_ResponseBuilder> for UciResponse {
    fn from(builder: UciVendor_A_ResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<UciVendor_A_ResponseBuilder> for UciVendor_A_Response {
    fn from(builder: UciVendor_A_ResponseBuilder) -> UciVendor_A_Response {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_B_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_B_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_B_ResponseDataChild::Payload(bytes) => bytes.len(),
            UciVendor_B_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_B_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_ResponseData {
    child: UciVendor_B_ResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_Response {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_b_response: UciVendor_B_ResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_B_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_B_ResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_B_ResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_B_ResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_B_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_B_Response {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_B_Response> for Bytes {
    fn from(packet: UciVendor_B_Response) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_B_Response> for Vec<u8> {
    fn from(packet: UciVendor_B_Response) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_B_Response> for UciPacket {
    fn from(packet: UciVendor_B_Response) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_B_Response> for UciResponse {
    fn from(packet: UciVendor_B_Response) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_B_Response {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_B_Response> {
        UciVendor_B_Response::new(packet.ucipacket)
    }
}
impl UciVendor_B_Response {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_B_ResponseChild {
        match &self.ucivendor_b_response.child {
            UciVendor_B_ResponseDataChild::Payload(payload) => {
                UciVendor_B_ResponseChild::Payload(payload.clone())
            }
            UciVendor_B_ResponseDataChild::None => UciVendor_B_ResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_b_response = match &uciresponse.child {
            UciResponseDataChild::UciVendor_B_Response(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::UciVendor_B_Response),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            ucivendor_b_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_b_response.child {
            UciVendor_B_ResponseDataChild::Payload(bytes) => &bytes,
            UciVendor_B_ResponseDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_b_response.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_B_ResponseBuilder {
    pub fn build(self) -> UciVendor_B_Response {
        let ucivendor_b_response = UciVendor_B_ResponseData {
            child: match self.payload {
                None => UciVendor_B_ResponseDataChild::None,
                Some(bytes) => UciVendor_B_ResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::UciVendor_B_Response(ucivendor_b_response),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedB,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciVendor_B_Response::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_B_ResponseBuilder> for UciPacket {
    fn from(builder: UciVendor_B_ResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_B_ResponseBuilder> for UciResponse {
    fn from(builder: UciVendor_B_ResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<UciVendor_B_ResponseBuilder> for UciVendor_B_Response {
    fn from(builder: UciVendor_B_ResponseBuilder) -> UciVendor_B_Response {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_E_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_E_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_E_ResponseDataChild::Payload(bytes) => bytes.len(),
            UciVendor_E_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_E_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_ResponseData {
    child: UciVendor_E_ResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_Response {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_e_response: UciVendor_E_ResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_E_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_E_ResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_E_ResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_E_ResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_E_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_E_Response {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_E_Response> for Bytes {
    fn from(packet: UciVendor_E_Response) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_E_Response> for Vec<u8> {
    fn from(packet: UciVendor_E_Response) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_E_Response> for UciPacket {
    fn from(packet: UciVendor_E_Response) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_E_Response> for UciResponse {
    fn from(packet: UciVendor_E_Response) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_E_Response {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_E_Response> {
        UciVendor_E_Response::new(packet.ucipacket)
    }
}
impl UciVendor_E_Response {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_E_ResponseChild {
        match &self.ucivendor_e_response.child {
            UciVendor_E_ResponseDataChild::Payload(payload) => {
                UciVendor_E_ResponseChild::Payload(payload.clone())
            }
            UciVendor_E_ResponseDataChild::None => UciVendor_E_ResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_e_response = match &uciresponse.child {
            UciResponseDataChild::UciVendor_E_Response(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::UciVendor_E_Response),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            ucivendor_e_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_e_response.child {
            UciVendor_E_ResponseDataChild::Payload(bytes) => &bytes,
            UciVendor_E_ResponseDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_e_response.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_E_ResponseBuilder {
    pub fn build(self) -> UciVendor_E_Response {
        let ucivendor_e_response = UciVendor_E_ResponseData {
            child: match self.payload {
                None => UciVendor_E_ResponseDataChild::None,
                Some(bytes) => UciVendor_E_ResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::UciVendor_E_Response(ucivendor_e_response),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedE,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciVendor_E_Response::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_E_ResponseBuilder> for UciPacket {
    fn from(builder: UciVendor_E_ResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_E_ResponseBuilder> for UciResponse {
    fn from(builder: UciVendor_E_ResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<UciVendor_E_ResponseBuilder> for UciVendor_E_Response {
    fn from(builder: UciVendor_E_ResponseBuilder) -> UciVendor_E_Response {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_F_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_F_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_F_ResponseDataChild::Payload(bytes) => bytes.len(),
            UciVendor_F_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_F_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_ResponseData {
    child: UciVendor_F_ResponseDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_Response {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    uciresponse: UciResponseData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_f_response: UciVendor_F_ResponseData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_F_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_F_ResponseDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_F_ResponseDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_F_ResponseDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_F_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_F_Response {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_F_Response> for Bytes {
    fn from(packet: UciVendor_F_Response) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_F_Response> for Vec<u8> {
    fn from(packet: UciVendor_F_Response) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_F_Response> for UciPacket {
    fn from(packet: UciVendor_F_Response) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_F_Response> for UciResponse {
    fn from(packet: UciVendor_F_Response) -> UciResponse {
        UciResponse::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_F_Response {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_F_Response> {
        UciVendor_F_Response::new(packet.ucipacket)
    }
}
impl UciVendor_F_Response {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_F_ResponseChild {
        match &self.ucivendor_f_response.child {
            UciVendor_F_ResponseDataChild::Payload(payload) => {
                UciVendor_F_ResponseChild::Payload(payload.clone())
            }
            UciVendor_F_ResponseDataChild::None => UciVendor_F_ResponseChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let uciresponse = match &ucipacket.child {
            UciPacketDataChild::UciResponse(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciResponse),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_f_response = match &uciresponse.child {
            UciResponseDataChild::UciVendor_F_Response(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciResponseDataChild::UciVendor_F_Response),
                    actual: format!("{:?}", &uciresponse.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            uciresponse,
            ucivendor_f_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_f_response.child {
            UciVendor_F_ResponseDataChild::Payload(bytes) => &bytes,
            UciVendor_F_ResponseDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_f_response.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_F_ResponseBuilder {
    pub fn build(self) -> UciVendor_F_Response {
        let ucivendor_f_response = UciVendor_F_ResponseData {
            child: match self.payload {
                None => UciVendor_F_ResponseDataChild::None,
                Some(bytes) => UciVendor_F_ResponseDataChild::Payload(bytes),
            },
        };
        let uciresponse = UciResponseData {
            child: UciResponseDataChild::UciVendor_F_Response(ucivendor_f_response),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedF,
            message_type: MessageType::Response,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciResponse(uciresponse),
        };
        UciVendor_F_Response::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_F_ResponseBuilder> for UciPacket {
    fn from(builder: UciVendor_F_ResponseBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_F_ResponseBuilder> for UciResponse {
    fn from(builder: UciVendor_F_ResponseBuilder) -> UciResponse {
        builder.build().into()
    }
}
impl From<UciVendor_F_ResponseBuilder> for UciVendor_F_Response {
    fn from(builder: UciVendor_F_ResponseBuilder) -> UciVendor_F_Response {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_9_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_9_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_9_NotificationDataChild::Payload(bytes) => bytes.len(),
            UciVendor_9_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_9_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_NotificationData {
    child: UciVendor_9_NotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_9_notification: UciVendor_9_NotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_9_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_9_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_9_NotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_9_NotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_9_NotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_9_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_9_Notification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_9_Notification> for Bytes {
    fn from(packet: UciVendor_9_Notification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_9_Notification> for Vec<u8> {
    fn from(packet: UciVendor_9_Notification) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_9_Notification> for UciPacket {
    fn from(packet: UciVendor_9_Notification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_9_Notification> for UciNotification {
    fn from(packet: UciVendor_9_Notification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_9_Notification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_9_Notification> {
        UciVendor_9_Notification::new(packet.ucipacket)
    }
}
impl UciVendor_9_Notification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_9_NotificationChild {
        match &self.ucivendor_9_notification.child {
            UciVendor_9_NotificationDataChild::Payload(payload) => {
                UciVendor_9_NotificationChild::Payload(payload.clone())
            }
            UciVendor_9_NotificationDataChild::None => UciVendor_9_NotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_9_notification = match &ucinotification.child {
            UciNotificationDataChild::UciVendor_9_Notification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::UciVendor_9_Notification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            ucivendor_9_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_9_notification.child {
            UciVendor_9_NotificationDataChild::Payload(bytes) => &bytes,
            UciVendor_9_NotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_9_notification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_9_NotificationBuilder {
    pub fn build(self) -> UciVendor_9_Notification {
        let ucivendor_9_notification = UciVendor_9_NotificationData {
            child: match self.payload {
                None => UciVendor_9_NotificationDataChild::None,
                Some(bytes) => UciVendor_9_NotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::UciVendor_9_Notification(ucivendor_9_notification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReserved9,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciVendor_9_Notification::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_9_NotificationBuilder> for UciPacket {
    fn from(builder: UciVendor_9_NotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_9_NotificationBuilder> for UciNotification {
    fn from(builder: UciVendor_9_NotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<UciVendor_9_NotificationBuilder> for UciVendor_9_Notification {
    fn from(builder: UciVendor_9_NotificationBuilder) -> UciVendor_9_Notification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_A_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_A_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_A_NotificationDataChild::Payload(bytes) => bytes.len(),
            UciVendor_A_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_A_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_NotificationData {
    child: UciVendor_A_NotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_a_notification: UciVendor_A_NotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_A_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_A_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_A_NotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_A_NotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_A_NotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_A_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_A_Notification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_A_Notification> for Bytes {
    fn from(packet: UciVendor_A_Notification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_A_Notification> for Vec<u8> {
    fn from(packet: UciVendor_A_Notification) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_A_Notification> for UciPacket {
    fn from(packet: UciVendor_A_Notification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_A_Notification> for UciNotification {
    fn from(packet: UciVendor_A_Notification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_A_Notification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_A_Notification> {
        UciVendor_A_Notification::new(packet.ucipacket)
    }
}
impl UciVendor_A_Notification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_A_NotificationChild {
        match &self.ucivendor_a_notification.child {
            UciVendor_A_NotificationDataChild::Payload(payload) => {
                UciVendor_A_NotificationChild::Payload(payload.clone())
            }
            UciVendor_A_NotificationDataChild::None => UciVendor_A_NotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_a_notification = match &ucinotification.child {
            UciNotificationDataChild::UciVendor_A_Notification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::UciVendor_A_Notification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            ucivendor_a_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_a_notification.child {
            UciVendor_A_NotificationDataChild::Payload(bytes) => &bytes,
            UciVendor_A_NotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_a_notification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_A_NotificationBuilder {
    pub fn build(self) -> UciVendor_A_Notification {
        let ucivendor_a_notification = UciVendor_A_NotificationData {
            child: match self.payload {
                None => UciVendor_A_NotificationDataChild::None,
                Some(bytes) => UciVendor_A_NotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::UciVendor_A_Notification(ucivendor_a_notification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedA,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciVendor_A_Notification::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_A_NotificationBuilder> for UciPacket {
    fn from(builder: UciVendor_A_NotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_A_NotificationBuilder> for UciNotification {
    fn from(builder: UciVendor_A_NotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<UciVendor_A_NotificationBuilder> for UciVendor_A_Notification {
    fn from(builder: UciVendor_A_NotificationBuilder) -> UciVendor_A_Notification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_B_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_B_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_B_NotificationDataChild::Payload(bytes) => bytes.len(),
            UciVendor_B_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_B_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_NotificationData {
    child: UciVendor_B_NotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_b_notification: UciVendor_B_NotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_B_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_B_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_B_NotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_B_NotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_B_NotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_B_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_B_Notification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_B_Notification> for Bytes {
    fn from(packet: UciVendor_B_Notification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_B_Notification> for Vec<u8> {
    fn from(packet: UciVendor_B_Notification) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_B_Notification> for UciPacket {
    fn from(packet: UciVendor_B_Notification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_B_Notification> for UciNotification {
    fn from(packet: UciVendor_B_Notification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_B_Notification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_B_Notification> {
        UciVendor_B_Notification::new(packet.ucipacket)
    }
}
impl UciVendor_B_Notification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_B_NotificationChild {
        match &self.ucivendor_b_notification.child {
            UciVendor_B_NotificationDataChild::Payload(payload) => {
                UciVendor_B_NotificationChild::Payload(payload.clone())
            }
            UciVendor_B_NotificationDataChild::None => UciVendor_B_NotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_b_notification = match &ucinotification.child {
            UciNotificationDataChild::UciVendor_B_Notification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::UciVendor_B_Notification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            ucivendor_b_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_b_notification.child {
            UciVendor_B_NotificationDataChild::Payload(bytes) => &bytes,
            UciVendor_B_NotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_b_notification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_B_NotificationBuilder {
    pub fn build(self) -> UciVendor_B_Notification {
        let ucivendor_b_notification = UciVendor_B_NotificationData {
            child: match self.payload {
                None => UciVendor_B_NotificationDataChild::None,
                Some(bytes) => UciVendor_B_NotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::UciVendor_B_Notification(ucivendor_b_notification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedB,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciVendor_B_Notification::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_B_NotificationBuilder> for UciPacket {
    fn from(builder: UciVendor_B_NotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_B_NotificationBuilder> for UciNotification {
    fn from(builder: UciVendor_B_NotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<UciVendor_B_NotificationBuilder> for UciVendor_B_Notification {
    fn from(builder: UciVendor_B_NotificationBuilder) -> UciVendor_B_Notification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_E_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_E_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_E_NotificationDataChild::Payload(bytes) => bytes.len(),
            UciVendor_E_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_E_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_NotificationData {
    child: UciVendor_E_NotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_e_notification: UciVendor_E_NotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_E_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_E_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_E_NotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_E_NotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_E_NotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_E_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_E_Notification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_E_Notification> for Bytes {
    fn from(packet: UciVendor_E_Notification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_E_Notification> for Vec<u8> {
    fn from(packet: UciVendor_E_Notification) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_E_Notification> for UciPacket {
    fn from(packet: UciVendor_E_Notification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_E_Notification> for UciNotification {
    fn from(packet: UciVendor_E_Notification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_E_Notification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_E_Notification> {
        UciVendor_E_Notification::new(packet.ucipacket)
    }
}
impl UciVendor_E_Notification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_E_NotificationChild {
        match &self.ucivendor_e_notification.child {
            UciVendor_E_NotificationDataChild::Payload(payload) => {
                UciVendor_E_NotificationChild::Payload(payload.clone())
            }
            UciVendor_E_NotificationDataChild::None => UciVendor_E_NotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_e_notification = match &ucinotification.child {
            UciNotificationDataChild::UciVendor_E_Notification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::UciVendor_E_Notification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            ucivendor_e_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_e_notification.child {
            UciVendor_E_NotificationDataChild::Payload(bytes) => &bytes,
            UciVendor_E_NotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_e_notification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_E_NotificationBuilder {
    pub fn build(self) -> UciVendor_E_Notification {
        let ucivendor_e_notification = UciVendor_E_NotificationData {
            child: match self.payload {
                None => UciVendor_E_NotificationDataChild::None,
                Some(bytes) => UciVendor_E_NotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::UciVendor_E_Notification(ucivendor_e_notification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedE,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciVendor_E_Notification::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_E_NotificationBuilder> for UciPacket {
    fn from(builder: UciVendor_E_NotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_E_NotificationBuilder> for UciNotification {
    fn from(builder: UciVendor_E_NotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<UciVendor_E_NotificationBuilder> for UciVendor_E_Notification {
    fn from(builder: UciVendor_E_NotificationBuilder) -> UciVendor_E_Notification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_F_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_F_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_F_NotificationDataChild::Payload(bytes) => bytes.len(),
            UciVendor_F_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UciVendor_F_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_NotificationData {
    child: UciVendor_F_NotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucivendor_f_notification: UciVendor_F_NotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UciVendor_F_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_F_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                UciVendor_F_NotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => UciVendor_F_NotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_F_NotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            UciVendor_F_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for UciVendor_F_Notification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_F_Notification> for Bytes {
    fn from(packet: UciVendor_F_Notification) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_F_Notification> for Vec<u8> {
    fn from(packet: UciVendor_F_Notification) -> Self {
        packet.to_vec()
    }
}
impl From<UciVendor_F_Notification> for UciPacket {
    fn from(packet: UciVendor_F_Notification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<UciVendor_F_Notification> for UciNotification {
    fn from(packet: UciVendor_F_Notification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for UciVendor_F_Notification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<UciVendor_F_Notification> {
        UciVendor_F_Notification::new(packet.ucipacket)
    }
}
impl UciVendor_F_Notification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> UciVendor_F_NotificationChild {
        match &self.ucivendor_f_notification.child {
            UciVendor_F_NotificationDataChild::Payload(payload) => {
                UciVendor_F_NotificationChild::Payload(payload.clone())
            }
            UciVendor_F_NotificationDataChild::None => UciVendor_F_NotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let ucivendor_f_notification = match &ucinotification.child {
            UciNotificationDataChild::UciVendor_F_Notification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::UciVendor_F_Notification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            ucivendor_f_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.ucivendor_f_notification.child {
            UciVendor_F_NotificationDataChild::Payload(bytes) => &bytes,
            UciVendor_F_NotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.ucivendor_f_notification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl UciVendor_F_NotificationBuilder {
    pub fn build(self) -> UciVendor_F_Notification {
        let ucivendor_f_notification = UciVendor_F_NotificationData {
            child: match self.payload {
                None => UciVendor_F_NotificationDataChild::None,
                Some(bytes) => UciVendor_F_NotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::UciVendor_F_Notification(ucivendor_f_notification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::VendorReservedF,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        UciVendor_F_Notification::new(ucipacket).unwrap()
    }
}
impl From<UciVendor_F_NotificationBuilder> for UciPacket {
    fn from(builder: UciVendor_F_NotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<UciVendor_F_NotificationBuilder> for UciNotification {
    fn from(builder: UciVendor_F_NotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<UciVendor_F_NotificationBuilder> for UciVendor_F_Notification {
    fn from(builder: UciVendor_F_NotificationBuilder) -> UciVendor_F_Notification {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestNotificationDataChild {
    Payload(Bytes),
    None,
}
impl TestNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            TestNotificationDataChild::Payload(bytes) => bytes.len(),
            TestNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestNotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestNotificationData {
    child: TestNotificationDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestNotification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucipacket: UciPacketData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    ucinotification: UciNotificationData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    testnotification: TestNotificationData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestNotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl TestNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                TestNotificationDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => TestNotificationDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            TestNotificationDataChild::Payload(payload) => buffer.put_slice(payload),
            TestNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for TestNotification {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.ucipacket.get_size());
        self.ucipacket.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<TestNotification> for Bytes {
    fn from(packet: TestNotification) -> Self {
        packet.to_bytes()
    }
}
impl From<TestNotification> for Vec<u8> {
    fn from(packet: TestNotification) -> Self {
        packet.to_vec()
    }
}
impl From<TestNotification> for UciPacket {
    fn from(packet: TestNotification) -> UciPacket {
        UciPacket::new(packet.ucipacket).unwrap()
    }
}
impl From<TestNotification> for UciNotification {
    fn from(packet: TestNotification) -> UciNotification {
        UciNotification::new(packet.ucipacket).unwrap()
    }
}
impl TryFrom<UciPacket> for TestNotification {
    type Error = Error;
    fn try_from(packet: UciPacket) -> Result<TestNotification> {
        TestNotification::new(packet.ucipacket)
    }
}
impl TestNotification {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = UciPacketData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> TestNotificationChild {
        match &self.testnotification.child {
            TestNotificationDataChild::Payload(payload) => {
                TestNotificationChild::Payload(payload.clone())
            }
            TestNotificationDataChild::None => TestNotificationChild::None,
        }
    }
    fn new(ucipacket: UciPacketData) -> Result<Self> {
        let ucinotification = match &ucipacket.child {
            UciPacketDataChild::UciNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciPacketDataChild::UciNotification),
                    actual: format!("{:?}", &ucipacket.child),
                });
            }
        };
        let testnotification = match &ucinotification.child {
            UciNotificationDataChild::TestNotification(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(UciNotificationDataChild::TestNotification),
                    actual: format!("{:?}", &ucinotification.child),
                });
            }
        };
        Ok(Self {
            ucipacket,
            ucinotification,
            testnotification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.ucipacket.group_id
    }
    pub fn get_message_type(&self) -> MessageType {
        self.ucipacket.message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.ucipacket.opcode
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.ucipacket.packet_boundary_flag
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.testnotification.child {
            TestNotificationDataChild::Payload(bytes) => &bytes,
            TestNotificationDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.testnotification.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.ucipacket.get_size()
    }
}
impl TestNotificationBuilder {
    pub fn build(self) -> TestNotification {
        let testnotification = TestNotificationData {
            child: match self.payload {
                None => TestNotificationDataChild::None,
                Some(bytes) => TestNotificationDataChild::Payload(bytes),
            },
        };
        let ucinotification = UciNotificationData {
            child: UciNotificationDataChild::TestNotification(testnotification),
        };
        let ucipacket = UciPacketData {
            group_id: GroupId::Test,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            child: UciPacketDataChild::UciNotification(ucinotification),
        };
        TestNotification::new(ucipacket).unwrap()
    }
}
impl From<TestNotificationBuilder> for UciPacket {
    fn from(builder: TestNotificationBuilder) -> UciPacket {
        builder.build().into()
    }
}
impl From<TestNotificationBuilder> for UciNotification {
    fn from(builder: TestNotificationBuilder) -> UciNotification {
        builder.build().into()
    }
}
impl From<TestNotificationBuilder> for TestNotification {
    fn from(builder: TestNotificationBuilder) -> TestNotification {
        builder.build().into()
    }
}
