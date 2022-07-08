# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# File generated from uci_packets.json, with the command:
#  ./scripts/generate_python_backend.py --input uci_packets.json
# /!\ Do not edit by hand.
from dataclasses import dataclass, field, fields
from typing import Optional, List, Tuple
import enum
import inspect
import math


@dataclass
class Packet:
    payload: Optional[bytes] = field(repr=False)

    @classmethod
    def parse_all(cls, span: bytes) -> 'Packet':
        packet, remain = getattr(cls, 'parse')(span)
        if len(remain) > 0:
            raise Exception('Unexpected parsing remainder')
        return packet

    def show(self, prefix: str = ''):
        print(f'{self.__class__.__name__}')

        def print_val(p: str, pp: str, name: str, align: int, typ, val):
            if name == 'payload':
                pass

            # Scalar fields.
            elif typ is int:
                print(f'{p}{name:{align}} = {val} (0x{val:x})')

            # Byte fields.
            elif typ is bytes:
                print(f'{p}{name:{align}} = [', end='')
                line = ''
                n_pp = ''
                for (idx, b) in enumerate(val):
                    if idx > 0 and idx % 8 == 0:
                        print(f'{n_pp}{line}')
                        line = ''
                        n_pp = pp + (' ' * (align + 4))
                    line += f' {b:02x}'
                print(f'{n_pp}{line} ]')

            # Enum fields.
            elif inspect.isclass(typ) and issubclass(typ, enum.IntEnum):
                print(f'{p}{name:{align}} = {typ.__name__}::{val.name} (0x{val:x})')

            # Struct fields.
            elif inspect.isclass(typ) and issubclass(typ, globals().get('Packet')):
                print(f'{p}{name:{align}} = ', end='')
                val.show(prefix=pp)

            # Array fields.
            elif typ.__origin__ == list:
                print(f'{p}{name:{align}}')
                last = len(val) - 1
                align = 5
                for (idx, elt) in enumerate(val):
                    n_p = pp + ('├── ' if idx != last else '└── ')
                    n_pp = pp + ('│   ' if idx != last else '    ')
                    print_val(n_p, n_pp, f'[{idx}]',
                              align, typ.__args__[0], val[idx])

            else:
                print(f'{p}{name:{align}} = ##{typ}##')

        last = len(fields(self)) - 1
        align = max(len(f.name) for f in fields(self) if f.name != 'payload')

        for (idx, f) in enumerate(fields(self)):
            p = prefix + ('├── ' if idx != last else '└── ')
            pp = prefix + ('│   ' if idx != last else '    ')
            val = getattr(self, f.name)

            print_val(p, pp, f.name, align, f.type, val)


class PacketBoundaryFlag(enum.IntEnum):
    COMPLETE = 0x0
    NOT_COMPLETE = 0x1


class GroupId(enum.IntEnum):
    CORE = 0x0
    SESSION_CONFIG = 0x1
    RANGING_SESSION_CONTROL = 0x2
    DATA_CONTROL = 0x3
    TEST = 0xd
    VENDOR_PICA = 0x9
    VENDOR_RESERVED_A = 0xa
    VENDOR_RESERVED_B = 0xb
    VENDOR_ANDROID = 0xe
    VENDOR_RESERVED_E = 0xc
    VENDOR_RESERVED_F = 0xf


class CoreOpCode(enum.IntEnum):
    CORE_DEVICE_RESET = 0x0
    CORE_DEVICE_STATUS_NTF = 0x1
    CORE_GET_DEVICE_INFO = 0x2
    CORE_GET_CAPS_INFO = 0x3
    CORE_SET_CONFIG = 0x4
    CORE_GET_CONFIG = 0x5
    CORE_DEVICE_SUSPEND = 0x6
    CORE_GENERIC_ERROR_NTF = 0x7


class SessionOpCode(enum.IntEnum):
    SESSION_INIT = 0x0
    SESSION_DEINIT = 0x1
    SESSION_STATUS_NTF = 0x2
    SESSION_SET_APP_CONFIG = 0x3
    SESSION_GET_APP_CONFIG = 0x4
    SESSION_GET_COUNT = 0x5
    SESSION_GET_STATE = 0x6
    SESSION_UPDATE_CONTROLLER_MULTICAST_LIST = 0x7


class RangeOpCode(enum.IntEnum):
    RANGE_START = 0x0
    RANGE_STOP = 0x1
    RANGE_INTERVAL_UPDATE_REQ = 0x2
    RANGE_GET_RANGING_COUNT = 0x3


class AppDataOpCode(enum.IntEnum):
    APP_DATA_TX = 0x0
    APP_DATA_RX = 0x1


class PicaOpCode(enum.IntEnum):
    PICA_INIT_DEVICE = 0x0
    PICA_SET_DEVICE_POSITION = 0x1
    PICA_CREATE_BEACON = 0x2
    PICA_SET_BEACON_POSITION = 0x3
    PICA_DESTROY_BEACON = 0x4


class AndroidOpCode(enum.IntEnum):
    ANDROID_GET_POWER_STATS = 0x0
    ANDROID_SET_COUNTRY_CODE = 0x1


class StatusCode(enum.IntEnum):
    UCI_STATUS_OK = 0x0
    UCI_STATUS_REJECTED = 0x1
    UCI_STATUS_FAILED = 0x2
    UCI_STATUS_SYNTAX_ERROR = 0x3
    UCI_STATUS_INVALID_PARAM = 0x4
    UCI_STATUS_INVALID_RANGE = 0x5
    UCI_STATUS_INVALID_MSG_SIZE = 0x6
    UCI_STATUS_UNKNOWN_GID = 0x7
    UCI_STATUS_UNKNOWN_OID = 0x8
    UCI_STATUS_READ_ONLY = 0x9
    UCI_STATUS_COMMAND_RETRY = 0xa
    UCI_STATUS_SESSION_NOT_EXIST = 0x11
    UCI_STATUS_SESSION_DUPLICATE = 0x12
    UCI_STATUS_SESSION_ACTIVE = 0x13
    UCI_STATUS_MAX_SESSIONS_EXCEEDED = 0x14
    UCI_STATUS_SESSION_NOT_CONFIGURED = 0x15
    UCI_STATUS_ACTIVE_SESSION_ONGOING = 0x16
    UCI_STATUS_MULTICAST_LIST_FULL = 0x17
    UCI_STATUS_ADDRESS_NOT_FOUND = 0x18
    UCI_STATUS_ADDRESS_ALREADY_PRESENT = 0x19
    UCI_STATUS_RANGING_TX_FAILED = 0x20
    UCI_STATUS_RANGING_RX_TIMEOUT = 0x21
    UCI_STATUS_RANGING_RX_PHY_DEC_FAILED = 0x22
    UCI_STATUS_RANGING_RX_PHY_TOA_FAILED = 0x23
    UCI_STATUS_RANGING_RX_PHY_STS_FAILED = 0x24
    UCI_STATUS_RANGING_RX_MAC_DEC_FAILED = 0x25
    UCI_STATUS_RANGING_RX_MAC_IE_DEC_FAILED = 0x26
    UCI_STATUS_RANGING_RX_MAC_IE_MISSING = 0x27
    UCI_STATUS_DATA_MAX_TX_PSDU_SIZE_EXCEEDED = 0x30
    UCI_STATUS_DATA_RX_CRC_ERROR = 0x31
    UCI_STATUS_ERROR_CCC_SE_BUSY = 0x50
    UCI_STATUS_ERROR_CCC_LIFECYCLE = 0x51


class ResetConfig(enum.IntEnum):
    UWBS_RESET = 0x0


class DeviceConfigId(enum.IntEnum):
    DEVICE_STATE = 0x0
    LOW_POWER_MODE = 0x1


class AppConfigTlvType(enum.IntEnum):
    DEVICE_TYPE = 0x0
    RANGING_ROUND_USAGE = 0x1
    STS_CONFIG = 0x2
    MULTI_NODE_MODE = 0x3
    CHANNEL_NUMBER = 0x4
    NO_OF_CONTROLEE = 0x5
    DEVICE_MAC_ADDRESS = 0x6
    DST_MAC_ADDRESS = 0x7
    SLOT_DURATION = 0x8
    RANGING_INTERVAL = 0x9
    STS_INDEX = 0xa
    MAC_FCS_TYPE = 0xb
    RANGING_ROUND_CONTROL = 0xc
    AOA_RESULT_REQ = 0xd
    RNG_DATA_NTF = 0xe
    RNG_DATA_NTF_PROXIMITY_NEAR = 0xf
    RNG_DATA_NTF_PROXIMITY_FAR = 0x10
    DEVICE_ROLE = 0x11
    RFRAME_CONFIG = 0x12
    PREAMBLE_CODE_INDEX = 0x14
    SFD_ID = 0x15
    PSDU_DATA_RATE = 0x16
    PREAMBLE_DURATION = 0x17
    RANGING_TIME_STRUCT = 0x1a
    SLOTS_PER_RR = 0x1b
    TX_ADAPTIVE_PAYLOAD_POWER = 0x1c
    RESPONDER_SLOT_INDEX = 0x1e
    PRF_MODE = 0x1f
    SCHEDULED_MODE = 0x22
    KEY_ROTATION = 0x23
    KEY_ROTATION_RATE = 0x24
    SESSION_PRIORITY = 0x25
    MAC_ADDRESS_MODE = 0x26
    VENDOR_ID = 0x27
    STATIC_STS_IV = 0x28
    NUMBER_OF_STS_SEGMENTS = 0x29
    MAX_RR_RETRY = 0x2a
    UWB_INITIATION_TIME = 0x2b
    HOPPING_MODE = 0x2c
    BLOCK_STRIDE_LENGTH = 0x2d
    RESULT_REPORT_CONFIG = 0x2e
    IN_BAND_TERMINATION_ATTEMPT_COUNT = 0x2f
    SUB_SESSION_ID = 0x30
    BPRF_PHR_DATA_RATE = 0x31
    MAX_NUMBER_OF_MEASUREMENTS = 0x32
    STS_LENGTH = 0x35
    CCC_HOP_MODE_KEY = 0xa0
    CCC_UWB_TIME0 = 0xa1
    CCC_RANGING_PROTOCOL_VER = 0xa3
    CCC_UWB_CONFIG_ID = 0xa4
    CCC_PULSESHAPE_COMBO = 0xa5
    CCC_URSK_TTL = 0xa6
    NB_OF_RANGE_MEASUREMENTS = 0xe3
    NB_OF_AZIMUTH_MEASUREMENTS = 0xe4
    NB_OF_ELEVATION_MEASUREMENTS = 0xe5


class CapTlvType(enum.IntEnum):
    SUPPORTED_FIRA_PHY_VERSION_RANGE = 0x0
    SUPPORTED_FIRA_MAC_VERSION_RANGE = 0x1
    SUPPORTED_DEVICE_ROLES = 0x2
    SUPPORTED_RANGING_METHOD = 0x3
    SUPPORTED_STS_CONFIG = 0x4
    SUPPORTED_MULTI_NODE_MODES = 0x5
    SUPPORTED_RANGING_TIME_STRUCT = 0x6
    SUPPORTED_SCHEDULED_MODE = 0x7
    SUPPORTED_HOPPING_MODE = 0x8
    SUPPORTED_BLOCK_STRIDING = 0x9
    SUPPORTED_UWB_INITIATION_TIME = 0xa
    SUPPORTED_CHANNELS = 0xb
    SUPPORTED_RFRAME_CONFIG = 0xc
    SUPPORTED_CC_CONSTRAINT_LENGTH = 0xd
    SUPPORTED_BPRF_PARAMETER_SETS = 0xe
    SUPPORTED_HPRF_PARAMETER_SETS = 0xf
    SUPPORTED_AOA = 0x10
    SUPPORTED_EXTENDED_MAC_ADDRESS = 0x11
    SUPPORTED_AOA_RESULT_REQ_ANTENNA_INTERLEAVING = 0xe3
    SUPPORTED_POWER_STATS = 0xc0
    CCC_SUPPORTED_CHAPS_PER_SLOT = 0xa0
    CCC_SUPPORTED_SYNC_CODES = 0xa1
    CCC_SUPPORTED_HOPPING_CONFIG_MODES_AND_SEQUENCES = 0xa2
    CCC_SUPPORTED_CHANNELS = 0xa3
    CCC_SUPPORTED_VERSIONS = 0xa4
    CCC_SUPPORTED_UWB_CONFIGS = 0xa5
    CCC_SUPPORTED_PULSE_SHAPE_COMBOS = 0xa6
    CCC_SUPPORTED_RAN_MULTIPLIER = 0xa7


class AoaResultReqType(enum.IntEnum):
    AOA_DISABLE = 0x0
    AOA_ENABLE = 0x1
    AOA_ENABLE_AZIMUTH = 0x2
    AOA_ENABLE_ELEVATION = 0x3
    AOA_ENABLE_INTERLEAVED = 0xf0


class DeviceState(enum.IntEnum):
    DEVICE_STATE_READY = 0x1
    DEVICE_STATE_ACTIVE = 0x2
    DEVICE_STATE_ERROR = 0xff


class SessionState(enum.IntEnum):
    SESSION_STATE_INIT = 0x0
    SESSION_STATE_DEINIT = 0x1
    SESSION_STATE_ACTIVE = 0x2
    SESSION_STATE_IDLE = 0x3


class ReasonCode(enum.IntEnum):
    STATE_CHANGE_WITH_SESSION_MANAGEMENT_COMMANDS = 0x0
    MAX_RANGING_ROUND_RETRY_COUNT_REACHED = 0x1
    MAX_NUMBER_OF_MEASUREMENTS_REACHED = 0x2
    ERROR_SLOT_LENGTH_NOT_SUPPORTED = 0x20
    ERROR_INSUFFICIENT_SLOTS_PER_RR = 0x21
    ERROR_MAC_ADDRESS_MODE_NOT_SUPPORTED = 0x22
    ERROR_INVALID_RANGING_INTERVAL = 0x23
    ERROR_INVALID_STS_CONFIG = 0x24
    ERROR_INVALID_RFRAME_CONFIG = 0x25


class MulticastUpdateStatusCode(enum.IntEnum):
    STATUS_OK_MULTICAST_LIST_UPDATE = 0x0
    STATUS_ERROR_MULTICAST_LIST_FULL = 0x1
    STATUS_ERROR_KEY_FETCH_FAIL = 0x2
    STATUS_ERROR_SUB_SESSION_ID_NOT_FOUND = 0x3


class MacAddressIndicator(enum.IntEnum):
    SHORT_ADDRESS = 0x0
    EXTENDED_ADDRESS = 0x1


class SessionType(enum.IntEnum):
    FIRA_RANGING_SESSION = 0x0
    FIRA_DATA_TRANSFER = 0x1
    CCC = 0xa0


class MessageType(enum.IntEnum):
    COMMAND = 0x1
    RESPONSE = 0x2
    NOTIFICATION = 0x3


@dataclass
class UciPacket(Packet):
    group_id: GroupId
    packet_boundary_flag: PacketBoundaryFlag
    message_type: MessageType
    opcode: int

    @staticmethod
    def parse(span: bytes) -> Tuple['UciPacket', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['group_id'] = GroupId((span[0] >> 0) & 0xf)
        fields['packet_boundary_flag'] = PacketBoundaryFlag(
            (span[0] >> 4) & 0x1)
        fields['message_type'] = MessageType((span[0] >> 5) & 0x7)
        fields['opcode'] = (span[1] >> 0) & 0x3f
        _payload__size = span[3]
        span = span[4:]
        if len(span) < _payload__size:
            raise Exception('Invalid packet size')
        payload = span[:_payload__size]
        fields['payload'] = payload
        if fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x0:
            return DeviceResetCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x2:
            return GetDeviceInfoCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x3:
            return GetCapsInfoCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x4:
            return SetConfigCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x5:
            return GetConfigCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x0:
            return SessionInitCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x1:
            return SessionDeinitCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x3:
            return SessionSetAppConfigCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x4:
            return SessionGetAppConfigCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x5:
            return SessionGetCountCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x6:
            return SessionGetStateCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.RANGING_SESSION_CONTROL:
            return RangingCommand.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x0:
            return PicaInitDeviceCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x1:
            return PicaSetDevicePositionCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x2:
            return PicaCreateBeaconCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x3:
            return PicaSetBeaconPositionCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x4:
            return PicaDestroyBeaconCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x0:
            return AndroidGetPowerStatsCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.COMMAND and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x1:
            return AndroidSetCountryCodeCmd.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x0:
            return DeviceResetRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x2:
            return GetDeviceInfoRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x3:
            return GetCapsInfoRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x4:
            return SetConfigRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x5:
            return GetConfigRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x0:
            return SessionInitRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x1:
            return SessionDeinitRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x3:
            return SessionSetAppConfigRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x4:
            return SessionGetAppConfigRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x5:
            return SessionGetCountRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x6:
            return SessionGetStateRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x0:
            return RangeStartRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x1:
            return RangeStopRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x3:
            return RangeGetRangingCountRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x0:
            return PicaInitDeviceRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x1:
            return PicaSetDevicePositionRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x2:
            return PicaCreateBeaconRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x3:
            return PicaSetBeaconPositionRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x4:
            return PicaDestroyBeaconRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x0:
            return AndroidGetPowerStatsRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.RESPONSE and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x1:
            return AndroidSetCountryCodeRsp.parse(fields, payload)
        elif fields['message_type'] == MessageType.NOTIFICATION and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x1:
            return DeviceStatusNtf.parse(fields, payload)
        elif fields['message_type'] == MessageType.NOTIFICATION and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x7:
            return GenericError.parse(fields, payload)
        elif fields['message_type'] == MessageType.NOTIFICATION and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x2:
            return SessionStatusNtf.parse(fields, payload)
        elif fields['message_type'] == MessageType.NOTIFICATION and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListNtf.parse(fields, payload)
        elif fields['message_type'] == MessageType.NOTIFICATION and fields['packet_boundary_flag'] == PacketBoundaryFlag.COMPLETE and fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x0:
            return RangeDataNtf.parse(fields, payload)
        else:
            return UciPacket(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciCommand(UciPacket):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciCommand', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x0:
            return DeviceResetCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x2:
            return GetDeviceInfoCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x3:
            return GetCapsInfoCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x4:
            return SetConfigCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x5:
            return GetConfigCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x0:
            return SessionInitCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x1:
            return SessionDeinitCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x3:
            return SessionSetAppConfigCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x4:
            return SessionGetAppConfigCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x5:
            return SessionGetCountCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x6:
            return SessionGetStateCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.RANGING_SESSION_CONTROL:
            return RangingCommand.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x0:
            return PicaInitDeviceCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x1:
            return PicaSetDevicePositionCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x2:
            return PicaCreateBeaconCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x3:
            return PicaSetBeaconPositionCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x4:
            return PicaDestroyBeaconCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x0:
            return AndroidGetPowerStatsCmd.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x1:
            return AndroidSetCountryCodeCmd.parse(fields, payload)
        else:
            return UciCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciResponse(UciPacket):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x0:
            return DeviceResetRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x2:
            return GetDeviceInfoRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x3:
            return GetCapsInfoRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x4:
            return SetConfigRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x5:
            return GetConfigRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x0:
            return SessionInitRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x1:
            return SessionDeinitRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x3:
            return SessionSetAppConfigRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x4:
            return SessionGetAppConfigRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x5:
            return SessionGetCountRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x6:
            return SessionGetStateRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x0:
            return RangeStartRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x1:
            return RangeStopRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x3:
            return RangeGetRangingCountRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x0:
            return PicaInitDeviceRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x1:
            return PicaSetDevicePositionRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x2:
            return PicaCreateBeaconRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x3:
            return PicaSetBeaconPositionRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_PICA and fields['opcode'] == 0x4:
            return PicaDestroyBeaconRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x0:
            return AndroidGetPowerStatsRsp.parse(fields, payload)
        elif fields['group_id'] == GroupId.VENDOR_ANDROID and fields['opcode'] == 0x1:
            return AndroidSetCountryCodeRsp.parse(fields, payload)
        else:
            return UciResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciNotification(UciPacket):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciNotification', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x1:
            return DeviceStatusNtf.parse(fields, payload)
        elif fields['group_id'] == GroupId.CORE and fields['opcode'] == 0x7:
            return GenericError.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x2:
            return SessionStatusNtf.parse(fields, payload)
        elif fields['group_id'] == GroupId.SESSION_CONFIG and fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListNtf.parse(fields, payload)
        elif fields['group_id'] == GroupId.RANGING_SESSION_CONTROL and fields['opcode'] == 0x0:
            return RangeDataNtf.parse(fields, payload)
        else:
            return UciNotification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class CoreCommand(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreCommand', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return DeviceResetCmd.parse(fields, payload)
        elif fields['opcode'] == 0x2:
            return GetDeviceInfoCmd.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return GetCapsInfoCmd.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return SetConfigCmd.parse(fields, payload)
        elif fields['opcode'] == 0x5:
            return GetConfigCmd.parse(fields, payload)
        else:
            return CoreCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class CoreResponse(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return DeviceResetRsp.parse(fields, payload)
        elif fields['opcode'] == 0x2:
            return GetDeviceInfoRsp.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return GetCapsInfoRsp.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return SetConfigRsp.parse(fields, payload)
        elif fields['opcode'] == 0x5:
            return GetConfigRsp.parse(fields, payload)
        else:
            return CoreResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class CoreNotification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreNotification', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x1:
            return DeviceStatusNtf.parse(fields, payload)
        elif fields['opcode'] == 0x7:
            return GenericError.parse(fields, payload)
        else:
            return CoreNotification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionCommand(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionCommand', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return SessionInitCmd.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return SessionDeinitCmd.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return SessionSetAppConfigCmd.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return SessionGetAppConfigCmd.parse(fields, payload)
        elif fields['opcode'] == 0x5:
            return SessionGetCountCmd.parse(fields, payload)
        elif fields['opcode'] == 0x6:
            return SessionGetStateCmd.parse(fields, payload)
        elif fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListCmd.parse(fields, payload)
        else:
            return SessionCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionResponse(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return SessionInitRsp.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return SessionDeinitRsp.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return SessionSetAppConfigRsp.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return SessionGetAppConfigRsp.parse(fields, payload)
        elif fields['opcode'] == 0x5:
            return SessionGetCountRsp.parse(fields, payload)
        elif fields['opcode'] == 0x6:
            return SessionGetStateRsp.parse(fields, payload)
        elif fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListRsp.parse(fields, payload)
        else:
            return SessionResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionNotification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionNotification', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x2:
            return SessionStatusNtf.parse(fields, payload)
        elif fields['opcode'] == 0x7:
            return SessionUpdateControllerMulticastListNtf.parse(fields, payload)
        else:
            return SessionNotification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangingCommand(UciCommand):
    session_id: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangingCommand', bytes]:
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return RangeStartCmd.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return RangeStopCmd.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return RangeGetRangingCountCmd.parse(fields, payload)
        else:
            return RangingCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangingResponse(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangingResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return RangeStartRsp.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return RangeStopRsp.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return RangeGetRangingCountRsp.parse(fields, payload)
        else:
            return RangingResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangingNotification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangingNotification', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return RangeDataNtf.parse(fields, payload)
        else:
            return RangingNotification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaCommand(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaCommand', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return PicaInitDeviceCmd.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return PicaSetDevicePositionCmd.parse(fields, payload)
        elif fields['opcode'] == 0x2:
            return PicaCreateBeaconCmd.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return PicaSetBeaconPositionCmd.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return PicaDestroyBeaconCmd.parse(fields, payload)
        else:
            return PicaCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaResponse(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return PicaInitDeviceRsp.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return PicaSetDevicePositionRsp.parse(fields, payload)
        elif fields['opcode'] == 0x2:
            return PicaCreateBeaconRsp.parse(fields, payload)
        elif fields['opcode'] == 0x3:
            return PicaSetBeaconPositionRsp.parse(fields, payload)
        elif fields['opcode'] == 0x4:
            return PicaDestroyBeaconRsp.parse(fields, payload)
        else:
            return PicaResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidCommand(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidCommand', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return AndroidGetPowerStatsCmd.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return AndroidSetCountryCodeCmd.parse(fields, payload)
        else:
            return AndroidCommand(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidResponse(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidResponse', bytes]:
        payload = span
        fields['payload'] = payload
        if fields['opcode'] == 0x0:
            return AndroidGetPowerStatsRsp.parse(fields, payload)
        elif fields['opcode'] == 0x1:
            return AndroidSetCountryCodeRsp.parse(fields, payload)
        else:
            return AndroidResponse(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidNotification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidNotification', bytes]:
        payload = span
        fields['payload'] = payload
        return AndroidNotification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class DeviceResetCmd(CoreCommand):
    reset_config: ResetConfig

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceResetCmd', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['reset_config'] = ResetConfig(span[0])
        span = span[1:]
        return DeviceResetCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class DeviceResetRsp(CoreResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceResetRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return DeviceResetRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class DeviceStatusNtf(CoreNotification):
    device_state: DeviceState

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceStatusNtf', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['device_state'] = DeviceState(span[0])
        span = span[1:]
        return DeviceStatusNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetDeviceInfoCmd(CoreCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetDeviceInfoCmd', bytes]:
        return GetDeviceInfoCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetDeviceInfoRsp(CoreResponse):
    status: StatusCode
    uci_version: int
    mac_version: int
    phy_version: int
    uci_test_version: int
    vendor_spec_info: bytes

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetDeviceInfoRsp', bytes]:
        if len(span) < 10:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        value_ = int.from_bytes(span[1:3], byteorder='little')
        fields['uci_version'] = value_
        value_ = int.from_bytes(span[3:5], byteorder='little')
        fields['mac_version'] = value_
        value_ = int.from_bytes(span[5:7], byteorder='little')
        fields['phy_version'] = value_
        value_ = int.from_bytes(span[7:9], byteorder='little')
        fields['uci_test_version'] = value_
        vendor_spec_info_count = span[9]
        span = span[10:]
        if len(span) < vendor_spec_info_count:
            raise Exception('Invalid packet size')
        vendor_spec_info = []
        for n in range(vendor_spec_info_count):
            vendor_spec_info.append(int.from_bytes(
                span[n:n + 1], byteorder='little'))
        fields['vendor_spec_info'] = vendor_spec_info
        span = span[vendor_spec_info_count:]
        return GetDeviceInfoRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetCapsInfoCmd(CoreCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetCapsInfoCmd', bytes]:
        return GetCapsInfoCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class CapTlv(Packet):
    t: CapTlvType
    v: bytes

    @staticmethod
    def parse(span: bytes) -> Tuple['CapTlv', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['t'] = CapTlvType(span[0])
        v_count = span[1]
        span = span[2:]
        if len(span) < v_count:
            raise Exception('Invalid packet size')
        v = []
        for n in range(v_count):
            v.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['v'] = v
        span = span[v_count:]
        return CapTlv(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetCapsInfoRsp(CoreResponse):
    status: StatusCode
    tlvs: List[CapTlv]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetCapsInfoRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        tlvs_count = span[1]
        span = span[2:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = CapTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return GetCapsInfoRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class DeviceParameter(Packet):
    id: int
    value: bytes

    @staticmethod
    def parse(span: bytes) -> Tuple['DeviceParameter', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['id'] = span[0]
        value_count = span[1]
        span = span[2:]
        if len(span) < value_count:
            raise Exception('Invalid packet size')
        value = []
        for n in range(value_count):
            value.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['value'] = value
        span = span[value_count:]
        return DeviceParameter(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SetConfigCmd(CoreCommand):
    parameters: List[DeviceParameter]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SetConfigCmd', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        parameters_count = span[0]
        span = span[1:]
        parameters = []
        for n in range(parameters_count):
            element, span = DeviceParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return SetConfigCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class DeviceConfigStatus(Packet):
    parameter_id: int
    status: StatusCode

    @staticmethod
    def parse(span: bytes) -> Tuple['DeviceConfigStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['parameter_id'] = span[0]
        fields['status'] = StatusCode(span[1])
        span = span[2:]
        return DeviceConfigStatus(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SetConfigRsp(CoreResponse):
    status: StatusCode
    parameters: List[DeviceConfigStatus]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SetConfigRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        parameters_count = span[1]
        span = span[2:]
        if len(span) < parameters_count * 2:
            raise Exception('Invalid packet size')
        parameters = []
        for n in range(parameters_count):
            parameters.append(DeviceConfigStatus.parse_all(
                span[n * 2:(n + 1) * 2]))
        fields['parameters'] = parameters
        span = span[parameters_count * 2:]
        return SetConfigRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetConfigCmd(CoreCommand):
    parameter_ids: bytes

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetConfigCmd', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        parameter_ids_count = span[0]
        span = span[1:]
        if len(span) < parameter_ids_count:
            raise Exception('Invalid packet size')
        parameter_ids = []
        for n in range(parameter_ids_count):
            parameter_ids.append(int.from_bytes(
                span[n:n + 1], byteorder='little'))
        fields['parameter_ids'] = parameter_ids
        span = span[parameter_ids_count:]
        return GetConfigCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GetConfigRsp(CoreResponse):
    status: StatusCode
    parameters: List[DeviceParameter]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetConfigRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        parameters_count = span[1]
        span = span[2:]
        parameters = []
        for n in range(parameters_count):
            element, span = DeviceParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return GetConfigRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class GenericError(CoreNotification):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GenericError', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return GenericError(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionInitCmd(SessionCommand):
    session_id: int
    session_type: SessionType

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitCmd', bytes]:
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        fields['session_type'] = SessionType(span[4])
        span = span[5:]
        return SessionInitCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionInitRsp(SessionResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return SessionInitRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionDeinitCmd(SessionCommand):
    session_id: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitCmd', bytes]:
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        return SessionDeinitCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionDeinitRsp(SessionResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return SessionDeinitRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionStatusNtf(SessionNotification):
    session_id: int
    session_state: SessionState
    reason_code: ReasonCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStatusNtf', bytes]:
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        fields['session_state'] = SessionState(span[4])
        fields['reason_code'] = ReasonCode(span[5])
        span = span[6:]
        return SessionStatusNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AppConfigParameter(Packet):
    id: int
    value: bytes

    @staticmethod
    def parse(span: bytes) -> Tuple['AppConfigParameter', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['id'] = span[0]
        value_count = span[1]
        span = span[2:]
        if len(span) < value_count:
            raise Exception('Invalid packet size')
        value = []
        for n in range(value_count):
            value.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['value'] = value
        span = span[value_count:]
        return AppConfigParameter(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionSetAppConfigCmd(SessionCommand):
    session_id: int
    parameters: List[AppConfigParameter]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigCmd', bytes]:
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        parameters_count = span[4]
        span = span[5:]
        parameters = []
        for n in range(parameters_count):
            element, span = AppConfigParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return SessionSetAppConfigCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AppConfigStatus(Packet):
    config_id: int
    status: StatusCode

    @staticmethod
    def parse(span: bytes) -> Tuple['AppConfigStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['config_id'] = span[0]
        fields['status'] = StatusCode(span[1])
        span = span[2:]
        return AppConfigStatus(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionSetAppConfigRsp(SessionResponse):
    status: StatusCode
    parameters: List[AppConfigStatus]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        parameters_count = span[1]
        span = span[2:]
        if len(span) < parameters_count * 2:
            raise Exception('Invalid packet size')
        parameters = []
        for n in range(parameters_count):
            parameters.append(AppConfigStatus.parse_all(
                span[n * 2:(n + 1) * 2]))
        fields['parameters'] = parameters
        span = span[parameters_count * 2:]
        return SessionSetAppConfigRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetAppConfigCmd(SessionCommand):
    session_id: int
    parameters: bytes

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigCmd', bytes]:
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        parameters_count = span[4]
        span = span[5:]
        if len(span) < parameters_count:
            raise Exception('Invalid packet size')
        parameters = []
        for n in range(parameters_count):
            parameters.append(int.from_bytes(
                span[n:n + 1], byteorder='little'))
        fields['parameters'] = parameters
        span = span[parameters_count:]
        return SessionGetAppConfigCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetAppConfigRsp(SessionResponse):
    status: StatusCode
    parameters: List[AppConfigParameter]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        parameters_count = span[1]
        span = span[2:]
        parameters = []
        for n in range(parameters_count):
            element, span = AppConfigParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return SessionGetAppConfigRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetCountCmd(SessionCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountCmd', bytes]:
        return SessionGetCountCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetCountRsp(SessionResponse):
    status: StatusCode
    session_count: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        fields['session_count'] = span[1]
        span = span[2:]
        return SessionGetCountRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetStateCmd(SessionCommand):
    session_id: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateCmd', bytes]:
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        return SessionGetStateCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionGetStateRsp(SessionResponse):
    status: StatusCode
    session_state: SessionState

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateRsp', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        fields['session_state'] = SessionState(span[1])
        span = span[2:]
        return SessionGetStateRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class Controlee(Packet):
    short_address: int
    subsession_id: int

    @staticmethod
    def parse(span: bytes) -> Tuple['Controlee', bytes]:
        fields = {'payload': None}
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['short_address'] = value_
        value_ = int.from_bytes(span[2:6], byteorder='little')
        fields['subsession_id'] = value_
        span = span[6:]
        return Controlee(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionUpdateControllerMulticastListCmd(SessionCommand):
    session_id: int
    action: int
    controlees: List[Controlee]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmd', bytes]:
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        fields['action'] = span[4]
        controlees_count = span[5]
        span = span[6:]
        if len(span) < controlees_count * 6:
            raise Exception('Invalid packet size')
        controlees = []
        for n in range(controlees_count):
            controlees.append(Controlee.parse_all(span[n * 6:(n + 1) * 6]))
        fields['controlees'] = controlees
        span = span[controlees_count * 6:]
        return SessionUpdateControllerMulticastListCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionUpdateControllerMulticastListRsp(SessionResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return SessionUpdateControllerMulticastListRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class ControleeStatus(Packet):
    mac_address: int
    subsession_id: int
    status: int

    @staticmethod
    def parse(span: bytes) -> Tuple['ControleeStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 7:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['mac_address'] = value_
        value_ = int.from_bytes(span[2:6], byteorder='little')
        fields['subsession_id'] = value_
        fields['status'] = span[6]
        span = span[7:]
        return ControleeStatus(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class SessionUpdateControllerMulticastListNtf(SessionNotification):
    session_id: int
    remaining_multicast_list_size: int
    controlee_status: List[ControleeStatus]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListNtf', bytes]:
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        fields['remaining_multicast_list_size'] = span[4]
        controlee_status_count = span[5]
        span = span[6:]
        if len(span) < controlee_status_count * 7:
            raise Exception('Invalid packet size')
        controlee_status = []
        for n in range(controlee_status_count):
            controlee_status.append(
                ControleeStatus.parse_all(span[n * 7:(n + 1) * 7]))
        fields['controlee_status'] = controlee_status
        span = span[controlee_status_count * 7:]
        return SessionUpdateControllerMulticastListNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeStartCmd(RangingCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeStartCmd', bytes]:
        return RangeStartCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeStartRsp(RangingResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeStartRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return RangeStartRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class ShortAddressTwoWayRangingMeasurement(Packet):
    mac_address: int
    status: StatusCode
    nlos: int
    distance: int
    aoa_azimuth: int
    aoa_azimuth_fom: int
    aoa_elevation: int
    aoa_elevation_fom: int
    aoa_destination_azimuth: int
    aoa_destination_azimuth_fom: int
    aoa_destination_elevation: int
    aoa_destination_elevation_fom: int
    slot_index: int

    @staticmethod
    def parse(span: bytes) -> Tuple['ShortAddressTwoWayRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 31:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode(span[2])
        fields['nlos'] = span[3]
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['distance'] = value_
        value_ = int.from_bytes(span[6:8], byteorder='little')
        fields['aoa_azimuth'] = value_
        fields['aoa_azimuth_fom'] = span[8]
        value_ = int.from_bytes(span[9:11], byteorder='little')
        fields['aoa_elevation'] = value_
        fields['aoa_elevation_fom'] = span[11]
        value_ = int.from_bytes(span[12:14], byteorder='little')
        fields['aoa_destination_azimuth'] = value_
        fields['aoa_destination_azimuth_fom'] = span[14]
        value_ = int.from_bytes(span[15:17], byteorder='little')
        fields['aoa_destination_elevation'] = value_
        fields['aoa_destination_elevation_fom'] = span[17]
        fields['slot_index'] = span[18]
        value_ = int.from_bytes(span[19:31], byteorder='little')
        span = span[31:]
        return ShortAddressTwoWayRangingMeasurement(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class ExtendedAddressTwoWayRangingMeasurement(Packet):
    mac_address: int
    status: StatusCode
    nlos: int
    distance: int
    aoa_azimuth: int
    aoa_azimuth_fom: int
    aoa_elevation: int
    aoa_elevation_fom: int
    aoa_destination_azimuth: int
    aoa_destination_azimuth_fom: int
    aoa_destination_elevation: int
    aoa_destination_elevation_fom: int
    slot_index: int

    @staticmethod
    def parse(span: bytes) -> Tuple['ExtendedAddressTwoWayRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 31:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode(span[8])
        fields['nlos'] = span[9]
        value_ = int.from_bytes(span[10:12], byteorder='little')
        fields['distance'] = value_
        value_ = int.from_bytes(span[12:14], byteorder='little')
        fields['aoa_azimuth'] = value_
        fields['aoa_azimuth_fom'] = span[14]
        value_ = int.from_bytes(span[15:17], byteorder='little')
        fields['aoa_elevation'] = value_
        fields['aoa_elevation_fom'] = span[17]
        value_ = int.from_bytes(span[18:20], byteorder='little')
        fields['aoa_destination_azimuth'] = value_
        fields['aoa_destination_azimuth_fom'] = span[20]
        value_ = int.from_bytes(span[21:23], byteorder='little')
        fields['aoa_destination_elevation'] = value_
        fields['aoa_destination_elevation_fom'] = span[23]
        fields['slot_index'] = span[24]
        value_ = int.from_bytes(span[25:31], byteorder='little')
        span = span[31:]
        return ExtendedAddressTwoWayRangingMeasurement(**fields), span

    def serialize(self) -> bytes:
        pass


class RangingMeasurementType(enum.IntEnum):
    ONE_WAY = 0x0
    TWO_WAY = 0x1


@dataclass
class RangeDataNtf(RangingNotification):
    sequence_number: int
    session_id: int
    rcr_indicator: int
    current_ranging_interval: int
    ranging_measurement_type: RangingMeasurementType
    mac_address_indicator: MacAddressIndicator

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeDataNtf', bytes]:
        if len(span) < 24:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['sequence_number'] = value_
        value_ = int.from_bytes(span[4:8], byteorder='little')
        fields['session_id'] = value_
        fields['rcr_indicator'] = span[8]
        value_ = int.from_bytes(span[9:13], byteorder='little')
        fields['current_ranging_interval'] = value_
        fields['ranging_measurement_type'] = RangingMeasurementType(span[13])
        fields['mac_address_indicator'] = MacAddressIndicator(span[15])
        value_ = int.from_bytes(span[16:24], byteorder='little')
        span = span[24:]
        payload = span
        fields['payload'] = payload
        if fields['ranging_measurement_type'] == RangingMeasurementType.TWO_WAY and fields['mac_address_indicator'] == MacAddressIndicator.SHORT_ADDRESS:
            return ShortMacTwoWayRangeDataNtf.parse(fields, payload)
        elif fields['ranging_measurement_type'] == RangingMeasurementType.TWO_WAY and fields['mac_address_indicator'] == MacAddressIndicator.EXTENDED_ADDRESS:
            return ExtendedMacTwoWayRangeDataNtf.parse(fields, payload)
        else:
            return RangeDataNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class ShortMacTwoWayRangeDataNtf(RangeDataNtf):
    two_way_ranging_measurements: List[ShortAddressTwoWayRangingMeasurement]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacTwoWayRangeDataNtf', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < two_way_ranging_measurements_count * 31:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements = []
        for n in range(two_way_ranging_measurements_count):
            two_way_ranging_measurements.append(
                ShortAddressTwoWayRangingMeasurement.parse_all(span[n * 31:(n + 1) * 31]))
        fields['two_way_ranging_measurements'] = two_way_ranging_measurements
        span = span[two_way_ranging_measurements_count * 31:]
        return ShortMacTwoWayRangeDataNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class ExtendedMacTwoWayRangeDataNtf(RangeDataNtf):
    two_way_ranging_measurements: List[ExtendedAddressTwoWayRangingMeasurement]

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacTwoWayRangeDataNtf', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < two_way_ranging_measurements_count * 31:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements = []
        for n in range(two_way_ranging_measurements_count):
            two_way_ranging_measurements.append(
                ExtendedAddressTwoWayRangingMeasurement.parse_all(span[n * 31:(n + 1) * 31]))
        fields['two_way_ranging_measurements'] = two_way_ranging_measurements
        span = span[two_way_ranging_measurements_count * 31:]
        return ExtendedMacTwoWayRangeDataNtf(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeStopCmd(RangingCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeStopCmd', bytes]:
        return RangeStopCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeStopRsp(RangingResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeStopRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return RangeStopRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeGetRangingCountCmd(RangingCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeGetRangingCountCmd', bytes]:
        return RangeGetRangingCountCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class RangeGetRangingCountRsp(RangingResponse):
    status: StatusCode
    count: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['RangeGetRangingCountRsp', bytes]:
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        value_ = int.from_bytes(span[1:5], byteorder='little')
        fields['count'] = value_
        span = span[5:]
        return RangeGetRangingCountRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaPosition(Packet):
    x: int
    y: int
    z: int
    yaw: int
    pitch: int
    roll: int

    @staticmethod
    def parse(span: bytes) -> Tuple['PicaPosition', bytes]:
        fields = {'payload': None}
        if len(span) < 11:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['x'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['y'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['z'] = value_
        value_ = int.from_bytes(span[6:8], byteorder='little')
        fields['yaw'] = value_
        fields['pitch'] = span[8]
        value_ = int.from_bytes(span[9:11], byteorder='little')
        fields['roll'] = value_
        span = span[11:]
        return PicaPosition(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaInitDeviceCmd(PicaCommand):
    mac_address: int
    position: PicaPosition

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaInitDeviceCmd', bytes]:
        if len(span) < 19:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['position'] = PicaPosition.parse_all(span[8:19])
        span = span[19:]
        return PicaInitDeviceCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaInitDeviceRsp(PicaResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaInitDeviceRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return PicaInitDeviceRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaSetDevicePositionCmd(PicaCommand):
    position: PicaPosition

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaSetDevicePositionCmd', bytes]:
        if len(span) < 11:
            raise Exception('Invalid packet size')
        fields['position'] = PicaPosition.parse_all(span[0:11])
        span = span[11:]
        return PicaSetDevicePositionCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaSetDevicePositionRsp(PicaResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaSetDevicePositionRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return PicaSetDevicePositionRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaCreateBeaconCmd(PicaCommand):
    mac_address: int
    position: PicaPosition

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaCreateBeaconCmd', bytes]:
        if len(span) < 19:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['position'] = PicaPosition.parse_all(span[8:19])
        span = span[19:]
        return PicaCreateBeaconCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaCreateBeaconRsp(PicaResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaCreateBeaconRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return PicaCreateBeaconRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaSetBeaconPositionCmd(PicaCommand):
    mac_address: int
    position: PicaPosition

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaSetBeaconPositionCmd', bytes]:
        if len(span) < 19:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['position'] = PicaPosition.parse_all(span[8:19])
        span = span[19:]
        return PicaSetBeaconPositionCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaSetBeaconPositionRsp(PicaResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaSetBeaconPositionRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return PicaSetBeaconPositionRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaDestroyBeaconCmd(PicaCommand):
    mac_address: int

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaDestroyBeaconCmd', bytes]:
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        span = span[8:]
        return PicaDestroyBeaconCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PicaDestroyBeaconRsp(PicaResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['PicaDestroyBeaconRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return PicaDestroyBeaconRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidGetPowerStatsCmd(AndroidCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsCmd', bytes]:
        return AndroidGetPowerStatsCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class PowerStats(Packet):
    status: StatusCode
    idle_time_ms: int
    tx_time_ms: int
    rx_time_ms: int
    total_wake_count: int

    @staticmethod
    def parse(span: bytes) -> Tuple['PowerStats', bytes]:
        fields = {'payload': None}
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        value_ = int.from_bytes(span[1:5], byteorder='little')
        fields['idle_time_ms'] = value_
        value_ = int.from_bytes(span[5:9], byteorder='little')
        fields['tx_time_ms'] = value_
        value_ = int.from_bytes(span[9:13], byteorder='little')
        fields['rx_time_ms'] = value_
        value_ = int.from_bytes(span[13:17], byteorder='little')
        fields['total_wake_count'] = value_
        span = span[17:]
        return PowerStats(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidGetPowerStatsRsp(AndroidResponse):
    stats: PowerStats

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsRsp', bytes]:
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['stats'] = PowerStats.parse_all(span[0:17])
        span = span[17:]
        return AndroidGetPowerStatsRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidSetCountryCodeCmd(AndroidCommand):
    country_code: bytes

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeCmd', bytes]:
        if len(span) < 2:
            raise Exception('Invalid packet size')
        country_code = []
        for n in range(2):
            country_code.append(int.from_bytes(
                span[n:n + 1], byteorder='little'))
        fields['country_code'] = country_code
        span = span[2:]
        return AndroidSetCountryCodeCmd(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class AndroidSetCountryCodeRsp(AndroidResponse):
    status: StatusCode

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeRsp', bytes]:
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode(span[0])
        span = span[1:]
        return AndroidSetCountryCodeRsp(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_A_Command(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Command', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_A_Command(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_B_Command(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Command', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_B_Command(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_E_Command(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Command', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_E_Command(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_F_Command(UciCommand):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Command', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_F_Command(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_A_Response(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Response', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_A_Response(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_B_Response(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Response', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_B_Response(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_E_Response(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Response', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_E_Response(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_F_Response(UciResponse):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Response', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_F_Response(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_A_Notification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Notification', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_A_Notification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_B_Notification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Notification', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_B_Notification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_E_Notification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Notification', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_E_Notification(**fields), span

    def serialize(self) -> bytes:
        pass


@dataclass
class UciVendor_F_Notification(UciNotification):

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Notification', bytes]:
        payload = span
        fields['payload'] = payload
        return UciVendor_F_Notification(**fields), span

    def serialize(self) -> bytes:
        pass
