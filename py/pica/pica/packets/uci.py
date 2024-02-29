# File generated from <stdin>, with the command:
#  ./pdl-compiler/scripts/generate_python_backend.py
# /!\ Do not edit by hand.
from dataclasses import dataclass, field, fields
from typing import Optional, List, Tuple, Union
import enum
import inspect
import math

@dataclass
class Packet:
    payload: Optional[bytes] = field(repr=False, default_factory=bytes, compare=False)

    @classmethod
    def parse_all(cls, span: bytes) -> 'Packet':
        packet, remain = getattr(cls, 'parse')(span)
        if len(remain) > 0:
            raise Exception('Unexpected parsing remainder')
        return packet

    @property
    def size(self) -> int:
        pass

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
            elif getattr(typ, '__origin__', None) == list:
                print(f'{p}{name:{align}}')
                last = len(val) - 1
                align = 5
                for (idx, elt) in enumerate(val):
                    n_p  = pp + ('├── ' if idx != last else '└── ')
                    n_pp = pp + ('│   ' if idx != last else '    ')
                    print_val(n_p, n_pp, f'[{idx}]', align, typ.__args__[0], val[idx])

            # Custom fields.
            elif inspect.isclass(typ):
                print(f'{p}{name:{align}} = {repr(val)}')

            else:
                print(f'{p}{name:{align}} = ##{typ}##')

        last = len(fields(self)) - 1
        align = max(len(f.name) for f in fields(self) if f.name != 'payload')

        for (idx, f) in enumerate(fields(self)):
            p  = prefix + ('├── ' if idx != last else '└── ')
            pp = prefix + ('│   ' if idx != last else '    ')
            val = getattr(self, f.name)

            print_val(p, pp, f.name, align, f.type, val)

class PacketBoundaryFlag(enum.IntEnum):
    COMPLETE = 0x0
    NOT_COMPLETE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'PacketBoundaryFlag']:
        try:
            return PacketBoundaryFlag(v)
        except ValueError as exn:
            raise exn


class GroupId(enum.IntEnum):
    CORE = 0x0
    SESSION_CONFIG = 0x1
    SESSION_CONTROL = 0x2
    DATA_CONTROL = 0x3
    TEST = 0xd
    VENDOR_RESERVED_9 = 0x9
    VENDOR_RESERVED_A = 0xa
    VENDOR_RESERVED_B = 0xb
    VENDOR_ANDROID = 0xc
    VENDOR_RESERVED_E = 0xe
    VENDOR_RESERVED_F = 0xf

    @staticmethod
    def from_int(v: int) -> Union[int, 'GroupId']:
        try:
            return GroupId(v)
        except ValueError as exn:
            raise exn


class DataPacketFormat(enum.IntEnum):
    DATA_SND = 0x1
    DATA_RCV = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'DataPacketFormat']:
        try:
            return DataPacketFormat(v)
        except ValueError as exn:
            raise exn


class GroupIdOrDataPacketFormat(enum.IntEnum):
    CORE = 0x0
    SESSION_CONFIG_OR_DATA_SND = 0x1
    SESSION_CONTROL_OR_DATA_RCV = 0x2
    DATA_CONTROL = 0x3
    TEST = 0xd
    VENDOR_RESERVED_9 = 0x9
    VENDOR_RESERVED_A = 0xa
    VENDOR_RESERVED_B = 0xb
    VENDOR_ANDROID = 0xc
    VENDOR_RESERVED_E = 0xe
    VENDOR_RESERVED_F = 0xf

    @staticmethod
    def from_int(v: int) -> Union[int, 'GroupIdOrDataPacketFormat']:
        try:
            return GroupIdOrDataPacketFormat(v)
        except ValueError as exn:
            raise exn


class CoreOpCode(enum.IntEnum):
    CORE_DEVICE_RESET = 0x0
    CORE_DEVICE_STATUS_NTF = 0x1
    CORE_DEVICE_INFO = 0x2
    CORE_GET_CAPS_INFO = 0x3
    CORE_SET_CONFIG = 0x4
    CORE_GET_CONFIG = 0x5
    CORE_DEVICE_SUSPEND = 0x6
    CORE_GENERIC_ERROR_NTF = 0x7
    CORE_QUERY_UWBS_TIMESTAMP = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'CoreOpCode']:
        try:
            return CoreOpCode(v)
        except ValueError as exn:
            raise exn


class SessionConfigOpCode(enum.IntEnum):
    SESSION_INIT = 0x0
    SESSION_DEINIT = 0x1
    SESSION_STATUS_NTF = 0x2
    SESSION_SET_APP_CONFIG = 0x3
    SESSION_GET_APP_CONFIG = 0x4
    SESSION_GET_COUNT = 0x5
    SESSION_GET_STATE = 0x6
    SESSION_UPDATE_CONTROLLER_MULTICAST_LIST = 0x7
    SESSION_UPDATE_ACTIVE_ROUNDS_ANCHOR = 0x8
    SESSION_UPDATE_ACTIVE_ROUNDS_DT_TAG = 0x9
    SESSION_SET_INITIATOR_DT_ANCHOR_RR_RDM_LIST = 0xa
    SESSION_QUERY_DATA_SIZE_IN_RANGING = 0xb
    SESSION_SET_HUS_CONFIG = 0xc

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionConfigOpCode']:
        try:
            return SessionConfigOpCode(v)
        except ValueError as exn:
            raise exn


class SessionControlOpCode(enum.IntEnum):
    SESSION_START = 0x0
    SESSION_STOP = 0x1
    SESSION_RESERVED = 0x2
    SESSION_GET_RANGING_COUNT = 0x3
    SESSION_DATA_CREDIT_NTF = 0x4
    SESSION_DATA_TRANSFER_STATUS_NTF = 0x5

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionControlOpCode']:
        try:
            return SessionControlOpCode(v)
        except ValueError as exn:
            raise exn


class AppDataOpCode(enum.IntEnum):
    APP_DATA_TX = 0x0
    APP_DATA_RX = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'AppDataOpCode']:
        try:
            return AppDataOpCode(v)
        except ValueError as exn:
            raise exn


class AndroidOpCode(enum.IntEnum):
    ANDROID_GET_POWER_STATS = 0x0
    ANDROID_SET_COUNTRY_CODE = 0x1
    ANDROID_FIRA_RANGE_DIAGNOSTICS = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'AndroidOpCode']:
        try:
            return AndroidOpCode(v)
        except ValueError as exn:
            raise exn


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
    UCI_STATUS_UNKNOWN = 0xb
    UCI_STATUS_NOT_APPLICABLE = 0xc
    UCI_STATUS_SESSION_NOT_EXIST = 0x11
    UCI_STATUS_SESSION_DUPLICATE = 0x12
    UCI_STATUS_SESSION_ACTIVE = 0x13
    UCI_STATUS_MAX_SESSIONS_EXCEEDED = 0x14
    UCI_STATUS_SESSION_NOT_CONFIGURED = 0x15
    UCI_STATUS_ACTIVE_SESSIONS_ONGOING = 0x16
    UCI_STATUS_MULTICAST_LIST_FULL = 0x17
    UCI_STATUS_ADDRESS_NOT_FOUND = 0x18
    UCI_STATUS_ADDRESS_ALREADY_PRESENT = 0x19
    UCI_STATUS_ERROR_UWB_INITIATION_TIME_TOO_OLD = 0x1a
    UCI_STATUS_OK_NEGATIVE_DISTANCE_REPORT = 0x1b
    UCI_STATUS_RANGING_TX_FAILED = 0x20
    UCI_STATUS_RANGING_RX_TIMEOUT = 0x21
    UCI_STATUS_RANGING_RX_PHY_DEC_FAILED = 0x22
    UCI_STATUS_RANGING_RX_PHY_TOA_FAILED = 0x23
    UCI_STATUS_RANGING_RX_PHY_STS_FAILED = 0x24
    UCI_STATUS_RANGING_RX_MAC_DEC_FAILED = 0x25
    UCI_STATUS_RANGING_RX_MAC_IE_DEC_FAILED = 0x26
    UCI_STATUS_RANGING_RX_MAC_IE_MISSING = 0x27
    UCI_STATUS_ERROR_ROUND_INDEX_NOT_ACTIVATED = 0x28
    UCI_STATUS_ERROR_NUMBER_OF_ACTIVE_RANGING_ROUNDS_EXCEEDED = 0x29
    UCI_STATUS_ERROR_DL_TDOA_DEVICE_ADDRESS_NOT_MATCHING_IN_REPLY_TIME_LIST = 0x2a
    UCI_STATUS_DATA_MAX_TX_PSDU_SIZE_EXCEEDED = 0x30
    UCI_STATUS_DATA_RX_CRC_ERROR = 0x31
    VENDOR_SPECIFIC_STATUS_CODE_2 = 0xff

    @staticmethod
    def from_int(v: int) -> Union[int, 'StatusCode']:
        try:
            return StatusCode(v)
        except ValueError as exn:
            return v


class DataRcvStatusCode(enum.IntEnum):
    UCI_STATUS_SUCCESS = 0x0
    UCI_STATUS_ERROR = 0x1
    UCI_STATUS_UNKNOWN = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'DataRcvStatusCode']:
        try:
            return DataRcvStatusCode(v)
        except ValueError as exn:
            raise exn


class CreditAvailability(enum.IntEnum):
    CREDIT_NOT_AVAILABLE = 0x0
    CREDIT_AVAILABLE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'CreditAvailability']:
        try:
            return CreditAvailability(v)
        except ValueError as exn:
            raise exn


class DataTransferNtfStatusCode(enum.IntEnum):
    UCI_DATA_TRANSFER_STATUS_REPETITION_OK = 0x0
    UCI_DATA_TRANSFER_STATUS_OK = 0x1
    UCI_DATA_TRANSFER_STATUS_ERROR_DATA_TRANSFER = 0x2
    UCI_DATA_TRANSFER_STATUS_ERROR_NO_CREDIT_AVAILABLE = 0x3
    UCI_DATA_TRANSFER_STATUS_ERROR_REJECTED = 0x4
    UCI_DATA_TRANSFER_STATUS_SESSION_TYPE_NOT_SUPPORTED = 0x5
    UCI_DATA_TRANSFER_STATUS_ERROR_DATA_TRANSFER_IS_ONGOING = 0x6
    UCI_DATA_TRANSFER_STATUS_INVALID_FORMAT = 0x7

    @staticmethod
    def from_int(v: int) -> Union[int, 'DataTransferNtfStatusCode']:
        try:
            return DataTransferNtfStatusCode(v)
        except ValueError as exn:
            raise exn


class ResetConfig(enum.IntEnum):
    UWBS_RESET = 0x0

    @staticmethod
    def from_int(v: int) -> Union[int, 'ResetConfig']:
        try:
            return ResetConfig(v)
        except ValueError as exn:
            raise exn


class DeviceConfigId(enum.IntEnum):
    DEVICE_STATE = 0x0
    LOW_POWER_MODE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'DeviceConfigId']:
        try:
            return DeviceConfigId(v)
        except ValueError as exn:
            raise exn


class AppConfigTlvType(enum.IntEnum):
    DEVICE_TYPE = 0x0
    RANGING_ROUND_USAGE = 0x1
    STS_CONFIG = 0x2
    MULTI_NODE_MODE = 0x3
    CHANNEL_NUMBER = 0x4
    NUMBER_OF_CONTROLEES = 0x5
    DEVICE_MAC_ADDRESS = 0x6
    DST_MAC_ADDRESS = 0x7
    SLOT_DURATION = 0x8
    RANGING_DURATION = 0x9
    STS_INDEX = 0xa
    MAC_FCS_TYPE = 0xb
    RANGING_ROUND_CONTROL = 0xc
    AOA_RESULT_REQ = 0xd
    SESSION_INFO_NTF_CONFIG = 0xe
    NEAR_PROXIMITY_CONFIG = 0xf
    FAR_PROXIMITY_CONFIG = 0x10
    DEVICE_ROLE = 0x11
    RFRAME_CONFIG = 0x12
    RSSI_REPORTING = 0x13
    PREAMBLE_CODE_INDEX = 0x14
    SFD_ID = 0x15
    PSDU_DATA_RATE = 0x16
    PREAMBLE_DURATION = 0x17
    LINK_LAYER_MODE = 0x18
    DATA_REPETITION_COUNT = 0x19
    RANGING_TIME_STRUCT = 0x1a
    SLOTS_PER_RR = 0x1b
    AOA_BOUND_CONFIG = 0x1d
    PRF_MODE = 0x1f
    CAP_SIZE_RANGE = 0x20
    TX_JITTER_WINDOW_SIZE = 0x21
    SCHEDULE_MODE = 0x22
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
    MIN_FRAMES_PER_RR = 0x3a
    MTU_SIZE = 0x3b
    INTER_FRAME_INTERVAL = 0x3c
    DL_TDOA_RANGING_METHOD = 0x3d
    DL_TDOA_TX_TIMESTAMP_CONF = 0x3e
    DL_TDOA_HOP_COUNT = 0x3f
    DL_TDOA_ANCHOR_CFO = 0x40
    DL_TDOA_ANCHOR_LOCATION = 0x41
    DL_TDOA_TX_ACTIVE_RANGING_ROUNDS = 0x42
    DL_TDOA_BLOCK_SKIPPING = 0x43
    DL_TDOA_TIME_REFERENCE_ANCHOR = 0x44
    SESSION_KEY = 0x45
    SUB_SESSION_KEY = 0x46
    SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG = 0x47
    SESSION_TIME_BASE = 0x48
    DL_TDOA_RESPONDER_TOF = 0x49
    SECURE_RANGING_NEFA_LEVEL = 0x4a
    SECURE_RANGING_CSW_LENGTH = 0x4b
    APPLICATION_DATA_ENDPOINT = 0x4c
    OWR_AOA_MEASUREMENT_NTF_PERIOD = 0x4d

    @staticmethod
    def from_int(v: int) -> Union[int, 'AppConfigTlvType']:
        try:
            return AppConfigTlvType(v)
        except ValueError as exn:
            return v


class DeviceType(enum.IntEnum):
    CONTROLEE = 0x0
    CONTROLLER = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'DeviceType']:
        try:
            return DeviceType(v)
        except ValueError as exn:
            raise exn


class RangingRoundUsage(enum.IntEnum):
    SS_TWR_DEFERRED_MODE = 0x1
    DS_TWR_DEFERRED_MODE = 0x2
    SS_TWR_NON_DEFERRED_MODE = 0x3
    DS_TWR_NON_DEFERRED_MODE = 0x4
    ON_WAY_RANGING_DL_TDOA = 0x5
    OWR_AOA_MEASUREMENT = 0x6
    ESS_TWR_NON_DEFERRED = 0x7
    ADS_TWR_NON_DEFERRED = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'RangingRoundUsage']:
        try:
            return RangingRoundUsage(v)
        except ValueError as exn:
            raise exn


class StsConfig(enum.IntEnum):
    STATIC = 0x0
    DYNAMIC = 0x1
    DYNAMIC_FOR_RESPONDER_SUB_SESSION_KEY = 0x2
    PROVISIONED = 0x3
    PROVISIONED_FOR_RESPONDER_SUB_SESSION_KEY = 0x4

    @staticmethod
    def from_int(v: int) -> Union[int, 'StsConfig']:
        try:
            return StsConfig(v)
        except ValueError as exn:
            raise exn


class MultiNodeMode(enum.IntEnum):
    ONE_TO_ONE = 0x0
    ONE_TO_MANY = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'MultiNodeMode']:
        try:
            return MultiNodeMode(v)
        except ValueError as exn:
            raise exn


class ChannelNumber(enum.IntEnum):
    CHANNEL_NUMBER_5 = 0x5
    CHANNEL_NUMBER_6 = 0x6
    CHANNEL_NUMBER_8 = 0x8
    CHANNEL_NUMBER_9 = 0x9
    CHANNEL_NUMBER_10 = 0xa
    CHANNEL_NUMBER_12 = 0xc
    CHANNEL_NUMBER_13 = 0xd
    CHANNEL_NUMBER_14 = 0xe

    @staticmethod
    def from_int(v: int) -> Union[int, 'ChannelNumber']:
        try:
            return ChannelNumber(v)
        except ValueError as exn:
            raise exn


class MacFcsType(enum.IntEnum):
    CRC_16 = 0x0
    CRC_32 = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'MacFcsType']:
        try:
            return MacFcsType(v)
        except ValueError as exn:
            raise exn


@dataclass
class RangingRoundControl(Packet):
    rrrm: int = field(kw_only=True, default=0)
    rcp: int = field(kw_only=True, default=0)
    mrp: int = field(kw_only=True, default=0)
    mrm: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['RangingRoundControl', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['rrrm'] = (span[0] >> 0) & 0x1
        if (span[0] >> 1) & 0x1 != 0x1:
            raise Exception('Unexpected fixed field value')
        fields['rcp'] = (span[0] >> 2) & 0x1
        fields['mrp'] = (span[0] >> 6) & 0x1
        fields['mrm'] = (span[0] >> 7) & 0x1
        span = span[1:]
        return RangingRoundControl(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.rrrm > 1:
            print(f"Invalid value for field RangingRoundControl::rrrm: {self.rrrm} > 1; the value will be truncated")
            self.rrrm &= 1
        if self.rcp > 1:
            print(f"Invalid value for field RangingRoundControl::rcp: {self.rcp} > 1; the value will be truncated")
            self.rcp &= 1
        if self.mrp > 1:
            print(f"Invalid value for field RangingRoundControl::mrp: {self.mrp} > 1; the value will be truncated")
            self.mrp &= 1
        if self.mrm > 1:
            print(f"Invalid value for field RangingRoundControl::mrm: {self.mrm} > 1; the value will be truncated")
            self.mrm &= 1
        _value = (
            (self.rrrm << 0) |
            (1 << 1) |
            (self.rcp << 2) |
            (self.mrp << 6) |
            (self.mrm << 7)
        )
        _span.append(_value)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class AoaResultReq(enum.IntEnum):
    AOA_DISABLED = 0x0
    AOA_ENABLED = 0x1
    AOA_ENABLED_AZIMUTH_ONLY = 0x2
    AOA_ENABLED_ELEVATION_ONLY = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'AoaResultReq']:
        try:
            return AoaResultReq(v)
        except ValueError as exn:
            raise exn


class SessionInfoNtfConfig(enum.IntEnum):
    DISABLE = 0x0
    ENABLE = 0x1
    ENABLE_PROXIMITY_TRIGGER = 0x2
    ENABLE_AOA_TRIGGER = 0x3
    ENABLE_PROXIMITY_AOA_TRIGGER = 0x4
    ENABLE_PROXIMITY_EDGE_TRIGGER = 0x5
    ENABLE_AOA_EDGE_TRIGGER = 0x6
    ENABLE_PROXIMITY_AOA_EDGE_TRIGGER = 0x7

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionInfoNtfConfig']:
        try:
            return SessionInfoNtfConfig(v)
        except ValueError as exn:
            raise exn


class DeviceRole(enum.IntEnum):
    RESPONDER = 0x0
    INITIATOR = 0x1
    ADVERTISER = 0x5
    OBSERVER = 0x6
    DT_ANCHOR = 0x7
    DT_TAG = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'DeviceRole']:
        try:
            return DeviceRole(v)
        except ValueError as exn:
            raise exn


class RframeConfig(enum.IntEnum):
    SP0 = 0x0
    SP1 = 0x1
    SP3 = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'RframeConfig']:
        try:
            return RframeConfig(v)
        except ValueError as exn:
            raise exn


class RssiReporting(enum.IntEnum):
    DISABLE = 0x0
    ENABLE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'RssiReporting']:
        try:
            return RssiReporting(v)
        except ValueError as exn:
            raise exn


class PsduDataRate(enum.IntEnum):
    DATA_RATE_6M81 = 0x0
    DATA_RATE_7M80 = 0x1
    DATA_RATE_27M2 = 0x2
    DATA_RATE_31M2 = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'PsduDataRate']:
        try:
            return PsduDataRate(v)
        except ValueError as exn:
            raise exn


class PreambleDuration(enum.IntEnum):
    DURATION_32_SYMBOLS = 0x0
    DURATION_64_SYMBOLS = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'PreambleDuration']:
        try:
            return PreambleDuration(v)
        except ValueError as exn:
            raise exn


class LinkLayerMode(enum.IntEnum):
    BYPASS_MODE = 0x0

    @staticmethod
    def from_int(v: int) -> Union[int, 'LinkLayerMode']:
        try:
            return LinkLayerMode(v)
        except ValueError as exn:
            raise exn


class RangingTimeStruct(enum.IntEnum):
    BLOCK_BASED_SCHEDULING = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'RangingTimeStruct']:
        try:
            return RangingTimeStruct(v)
        except ValueError as exn:
            raise exn


class PrfMode(enum.IntEnum):
    BPRF_MODE = 0x0
    HPRF_MODE_124M8 = 0x1
    HPRF_MODE_249M6 = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'PrfMode']:
        try:
            return PrfMode(v)
        except ValueError as exn:
            raise exn


class ScheduleMode(enum.IntEnum):
    CONTENTION_BASED = 0x0
    TIME_SCHEDULED = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'ScheduleMode']:
        try:
            return ScheduleMode(v)
        except ValueError as exn:
            raise exn


class KeyRotation(enum.IntEnum):
    DISABLE = 0x0
    ENABLE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'KeyRotation']:
        try:
            return KeyRotation(v)
        except ValueError as exn:
            raise exn


class MacAddressMode(enum.IntEnum):
    MODE_0 = 0x0
    MODE_1 = 0x1
    MODE_2 = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'MacAddressMode']:
        try:
            return MacAddressMode(v)
        except ValueError as exn:
            raise exn


class HoppingMode(enum.IntEnum):
    DISABLE = 0x0
    ENABLE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'HoppingMode']:
        try:
            return HoppingMode(v)
        except ValueError as exn:
            raise exn


@dataclass
class ResultReportConfig(Packet):
    tof: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_fom: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ResultReportConfig', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['tof'] = (span[0] >> 0) & 0x1
        fields['aoa_azimuth'] = (span[0] >> 1) & 0x1
        fields['aoa_elevation'] = (span[0] >> 2) & 0x1
        fields['aoa_fom'] = (span[0] >> 3) & 0x1
        span = span[1:]
        return ResultReportConfig(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.tof > 1:
            print(f"Invalid value for field ResultReportConfig::tof: {self.tof} > 1; the value will be truncated")
            self.tof &= 1
        if self.aoa_azimuth > 1:
            print(f"Invalid value for field ResultReportConfig::aoa_azimuth: {self.aoa_azimuth} > 1; the value will be truncated")
            self.aoa_azimuth &= 1
        if self.aoa_elevation > 1:
            print(f"Invalid value for field ResultReportConfig::aoa_elevation: {self.aoa_elevation} > 1; the value will be truncated")
            self.aoa_elevation &= 1
        if self.aoa_fom > 1:
            print(f"Invalid value for field ResultReportConfig::aoa_fom: {self.aoa_fom} > 1; the value will be truncated")
            self.aoa_fom &= 1
        _value = (
            (self.tof << 0) |
            (self.aoa_azimuth << 1) |
            (self.aoa_elevation << 2) |
            (self.aoa_fom << 3)
        )
        _span.append(_value)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class BprfPhrDataRate(enum.IntEnum):
    DATA_RATE_850K = 0x0
    DATA_RATE_6M81 = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'BprfPhrDataRate']:
        try:
            return BprfPhrDataRate(v)
        except ValueError as exn:
            raise exn


class StsLength(enum.IntEnum):
    LENGTH_32_SYMBOLS = 0x0
    LENGTH_64_SYMBOLS = 0x1
    LENGTH_128_SYMBOLS = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'StsLength']:
        try:
            return StsLength(v)
        except ValueError as exn:
            raise exn


class DlTdoaRangingMethod(enum.IntEnum):
    SS_TWR = 0x0
    DS_TWR = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'DlTdoaRangingMethod']:
        try:
            return DlTdoaRangingMethod(v)
        except ValueError as exn:
            raise exn


class DlTdoaAnchorCfo(enum.IntEnum):
    ANCHOR_CFO_NOT_INCLUDED = 0x0
    ANCHOR_CFO_INCLUDED = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'DlTdoaAnchorCfo']:
        try:
            return DlTdoaAnchorCfo(v)
        except ValueError as exn:
            raise exn


class SessionDataTransferStatusNtfConfig(enum.IntEnum):
    DISABLE = 0x0
    ENABLE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionDataTransferStatusNtfConfig']:
        try:
            return SessionDataTransferStatusNtfConfig(v)
        except ValueError as exn:
            raise exn


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
    SUPPORTED_MAX_MESSAGE_SIZE = 0x12
    SUPPORTED_MAX_DATA_PACKET_PAYLOAD_SIZE = 0x13
    SUPPORTED_POWER_STATS = 0xc0

    @staticmethod
    def from_int(v: int) -> Union[int, 'CapTlvType']:
        try:
            return CapTlvType(v)
        except ValueError as exn:
            return v


class AoaResultReqType(enum.IntEnum):
    AOA_DISABLE = 0x0
    AOA_ENABLE = 0x1
    AOA_ENABLE_AZIMUTH = 0x2
    AOA_ENABLE_ELEVATION = 0x3
    AOA_ENABLE_INTERLEAVED = 0xf0

    @staticmethod
    def from_int(v: int) -> Union[int, 'AoaResultReqType']:
        try:
            return AoaResultReqType(v)
        except ValueError as exn:
            raise exn


class DeviceState(enum.IntEnum):
    DEVICE_STATE_READY = 0x1
    DEVICE_STATE_ACTIVE = 0x2
    DEVICE_STATE_ERROR = 0xff

    @staticmethod
    def from_int(v: int) -> Union[int, 'DeviceState']:
        try:
            return DeviceState(v)
        except ValueError as exn:
            raise exn


class SessionState(enum.IntEnum):
    SESSION_STATE_INIT = 0x0
    SESSION_STATE_DEINIT = 0x1
    SESSION_STATE_ACTIVE = 0x2
    SESSION_STATE_IDLE = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionState']:
        try:
            return SessionState(v)
        except ValueError as exn:
            raise exn


class ReasonCode(enum.IntEnum):
    STATE_CHANGE_WITH_SESSION_MANAGEMENT_COMMANDS = 0x0
    MAX_RANGING_ROUND_RETRY_COUNT_REACHED = 0x1
    MAX_NUMBER_OF_MEASUREMENTS_REACHED = 0x2
    SESSION_SUSPENDED_DUE_TO_INBAND_SIGNAL = 0x3
    SESSION_RESUMED_DUE_TO_INBAND_SIGNAL = 0x4
    SESSION_STOPPED_DUE_TO_INBAND_SIGNAL = 0x5
    ERROR_INVALID_UL_TDOA_RANDOM_WINDOW = 0x1d
    ERROR_MIN_RFRAMES_PER_RR_NOT_SUPPORTED = 0x1e
    ERROR_TX_DELAY_NOT_SUPPORTED = 0x1f
    ERROR_SLOT_LENGTH_NOT_SUPPORTED = 0x20
    ERROR_INSUFFICIENT_SLOTS_PER_RR = 0x21
    ERROR_MAC_ADDRESS_MODE_NOT_SUPPORTED = 0x22
    ERROR_INVALID_RANGING_DURATION = 0x23
    ERROR_INVALID_STS_CONFIG = 0x24
    ERROR_INVALID_RFRAME_CONFIG = 0x25
    ERROR_HUS_NOT_ENOUGH_SLOTS = 0x26
    ERROR_HUS_CFP_PHASE_TOO_SHORT = 0x27
    ERROR_HUS_CAP_PHASE_TOO_SHORT = 0x28
    ERROR_HUS_OTHERS = 0x29
    ERROR_STATUS_SESSION_KEY_NOT_FOUND = 0x2a
    ERROR_STATUS_SUB_SESSION_KEY_NOT_FOUND = 0x2b
    ERROR_INVALID_PREAMBLE_CODE_INDEX = 0x2c
    ERROR_INVALID_SFD_ID = 0x2d
    ERROR_INVALID_PSDU_DATA_RATE = 0x2e
    ERROR_INVALID_PHR_DATA_RATE = 0x2f
    ERROR_INVALID_PREAMBLE_DURATION = 0x30
    ERROR_INVALID_STS_LENGTH = 0x31
    ERROR_INVALID_NUM_OF_STS_SEGMENTS = 0x32
    ERROR_INVALID_NUM_OF_CONTROLEES = 0x33
    ERROR_MAX_RANGING_REPLY_TIME_EXCEEDED = 0x34
    ERROR_INVALID_DST_ADDRESS_LIST = 0x35
    ERROR_INVALID_OR_NOT_FOUND_SUB_SESSION_ID = 0x36
    ERROR_INVALID_RESULT_REPORT_CONFIG = 0x37
    ERROR_INVALID_RANGING_ROUND_CONTROL_CONFIG = 0x38
    ERROR_INVALID_RANGING_ROUND_USAGE = 0x39
    ERROR_INVALID_MULTI_NODE_MODE = 0x3a
    ERROR_RDS_FETCH_FAILURE = 0x3b
    ERROR_REF_UWB_SESSION_DOES_NOT_EXIST = 0x3c
    ERROR_REF_UWB_SESSION_RANGING_DURATION_MISMATCH = 0x3d
    ERROR_REF_UWB_SESSION_INVALID_OFFSET_TIME = 0x3e
    ERROR_REF_UWB_SESSION_LOST = 0x3f
    VENDOR_SPECIFIC_REASON_CODE_2 = 0xff

    @staticmethod
    def from_int(v: int) -> Union[int, 'ReasonCode']:
        try:
            return ReasonCode(v)
        except ValueError as exn:
            return v


class MulticastUpdateStatusCode(enum.IntEnum):
    STATUS_OK_MULTICAST_LIST_UPDATE = 0x0
    STATUS_ERROR_MULTICAST_LIST_FULL = 0x1
    STATUS_ERROR_KEY_FETCH_FAIL = 0x2
    STATUS_ERROR_SUB_SESSION_ID_NOT_FOUND = 0x3
    STATUS_ERROR_SUB_SESSION_KEY_NOT_FOUND = 0x5
    STATUS_ERROR_SUB_SESSION_KEY_NOT_APPLICABLE = 0x6
    STATUS_ERROR_SESSION_KEY_NOT_FOUND = 0x7
    STATUS_ERROR_ADDRESS_ALREADY_PRESENT = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'MulticastUpdateStatusCode']:
        try:
            return MulticastUpdateStatusCode(v)
        except ValueError as exn:
            raise exn


class MacAddressIndicator(enum.IntEnum):
    SHORT_ADDRESS = 0x0
    EXTENDED_ADDRESS = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'MacAddressIndicator']:
        try:
            return MacAddressIndicator(v)
        except ValueError as exn:
            raise exn


class SessionType(enum.IntEnum):
    FIRA_RANGING_SESSION = 0x0
    FIRA_RANGING_AND_IN_BAND_DATA_SESSION = 0x1
    FIRA_DATA_TRANSFER_SESSION = 0x2
    FIRA_RANGING_ONLY_PHASE = 0x3
    FIRA_IN_BAND_DATA_PHASE = 0x4
    FIRA_RANGING_WITH_DATA_PHASE = 0x5
    CCC = 0xa0
    DEVICE_TEST_MODE = 0xd0

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionType']:
        try:
            return SessionType(v)
        except ValueError as exn:
            raise exn


class MessageType(enum.IntEnum):
    DATA = 0x0
    COMMAND = 0x1
    RESPONSE = 0x2
    NOTIFICATION = 0x3
    RESERVED_FOR_TESTING_1 = 0x4
    RESERVED_FOR_TESTING_2 = 0x5

    @staticmethod
    def from_int(v: int) -> Union[int, 'MessageType']:
        try:
            return MessageType(v)
        except ValueError as exn:
            raise exn


@dataclass
class CommonPacketHeader(Packet):
    pbf: PacketBoundaryFlag = field(kw_only=True, default=PacketBoundaryFlag.COMPLETE)
    mt: MessageType = field(kw_only=True, default=MessageType.DATA)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['CommonPacketHeader', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['pbf'] = PacketBoundaryFlag.from_int((span[0] >> 4) & 0x1)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        span = span[1:]
        return CommonPacketHeader(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _value = (
            (self.pbf << 4) |
            (self.mt << 5)
        )
        _span.append(_value)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

@dataclass
class ControlPacketHeader(Packet):
    pbf: PacketBoundaryFlag = field(kw_only=True, default=PacketBoundaryFlag.COMPLETE)
    mt: MessageType = field(kw_only=True, default=MessageType.DATA)
    payload_length: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ControlPacketHeader', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['pbf'] = PacketBoundaryFlag.from_int((span[0] >> 4) & 0x1)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        value_ = int.from_bytes(span[1:3], byteorder='little')
        fields['payload_length'] = span[3]
        span = span[4:]
        return ControlPacketHeader(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _value = (
            (self.pbf << 4) |
            (self.mt << 5)
        )
        _span.append(_value)
        _span.extend([0] * 2)
        if self.payload_length > 255:
            print(f"Invalid value for field ControlPacketHeader::payload_length: {self.payload_length} > 255; the value will be truncated")
            self.payload_length &= 255
        _span.append((self.payload_length << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class DataPacketHeader(Packet):
    pbf: PacketBoundaryFlag = field(kw_only=True, default=PacketBoundaryFlag.COMPLETE)
    mt: MessageType = field(kw_only=True, default=MessageType.DATA)
    payload_length: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['DataPacketHeader', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['pbf'] = PacketBoundaryFlag.from_int((span[0] >> 4) & 0x1)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['payload_length'] = value_
        span = span[4:]
        return DataPacketHeader(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _value = (
            (self.pbf << 4) |
            (self.mt << 5)
        )
        _span.append(_value)
        _span.extend([0] * 1)
        if self.payload_length > 65535:
            print(f"Invalid value for field DataPacketHeader::payload_length: {self.payload_length} > 65535; the value will be truncated")
            self.payload_length &= 65535
        _span.extend(int.to_bytes((self.payload_length << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class ControlPacket(Packet):
    gid: GroupId = field(kw_only=True, default=GroupId.CORE)
    mt: MessageType = field(kw_only=True, default=MessageType.DATA)
    opcode: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ControlPacket', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['gid'] = GroupId.from_int((span[0] >> 0) & 0xf)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        fields['opcode'] = (span[1] >> 0) & 0x3f
        value_ = int.from_bytes(span[2:4], byteorder='little')
        span = span[4:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceResetCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionControlCommand.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidGetPowerStatsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DeviceResetRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitRsp_V2.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStartRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStopRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetRangingCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidGetPowerStatsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DeviceStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GenericError.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataCreditNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataTransferStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidRangeDiagnosticsNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return ControlPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _value = (
            (self.gid << 0) |
            (self.mt << 5)
        )
        _span.append(_value)
        if self.opcode > 63:
            print(f"Invalid value for field ControlPacket::opcode: {self.opcode} > 63; the value will be truncated")
            self.opcode &= 63
        _span.append((self.opcode << 0))
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 4

@dataclass
class DataPacket(Packet):
    dpf: DataPacketFormat = field(kw_only=True, default=DataPacketFormat.DATA_SND)
    pbf: PacketBoundaryFlag = field(kw_only=True, default=PacketBoundaryFlag.COMPLETE)
    mt: MessageType = field(kw_only=True, default=MessageType.DATA)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['DataPacket', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['dpf'] = DataPacketFormat.from_int((span[0] >> 0) & 0xf)
        fields['pbf'] = PacketBoundaryFlag.from_int((span[0] >> 4) & 0x1)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        value_ = int.from_bytes(span[2:4], byteorder='little')
        span = span[4:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DataMessageSnd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataMessageRcv.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return DataPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _value = (
            (self.dpf << 0) |
            (self.pbf << 4) |
            (self.mt << 5)
        )
        _span.append(_value)
        _span.extend([0] * 1)
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 4

@dataclass
class DataMessageSnd(DataPacket):
    session_handle: int = field(kw_only=True, default=0)
    destination_address: int = field(kw_only=True, default=0)
    data_sequence_number: int = field(kw_only=True, default=0)
    application_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.dpf = DataPacketFormat.DATA_SND
        self.mt = MessageType.DATA

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DataMessageSnd', bytes]:
        if fields['dpf'] != DataPacketFormat.DATA_SND or fields['mt'] != MessageType.DATA:
            raise Exception("Invalid constraint field values")
        if len(span) < 16:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_handle'] = value_
        value_ = int.from_bytes(span[4:12], byteorder='little')
        fields['destination_address'] = value_
        value_ = int.from_bytes(span[12:14], byteorder='little')
        fields['data_sequence_number'] = value_
        value_ = int.from_bytes(span[14:16], byteorder='little')
        application_data_size = value_
        span = span[16:]
        if len(span) < application_data_size:
            raise Exception('Invalid packet size')
        fields['application_data'] = list(span[:application_data_size])
        span = span[application_data_size:]
        return DataMessageSnd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_handle > 4294967295:
            print(f"Invalid value for field DataMessageSnd::session_handle: {self.session_handle} > 4294967295; the value will be truncated")
            self.session_handle &= 4294967295
        _span.extend(int.to_bytes((self.session_handle << 0), length=4, byteorder='little'))
        if self.destination_address > 18446744073709551615:
            print(f"Invalid value for field DataMessageSnd::destination_address: {self.destination_address} > 18446744073709551615; the value will be truncated")
            self.destination_address &= 18446744073709551615
        _span.extend(int.to_bytes((self.destination_address << 0), length=8, byteorder='little'))
        if self.data_sequence_number > 65535:
            print(f"Invalid value for field DataMessageSnd::data_sequence_number: {self.data_sequence_number} > 65535; the value will be truncated")
            self.data_sequence_number &= 65535
        _span.extend(int.to_bytes((self.data_sequence_number << 0), length=2, byteorder='little'))
        _span.extend(int.to_bytes(((len(self.application_data) * 1) << 0), length=2, byteorder='little'))
        _span.extend(self.application_data)
        return DataPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.application_data) * 1 + 16

@dataclass
class DataMessageRcv(DataPacket):
    session_handle: int = field(kw_only=True, default=0)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    source_address: int = field(kw_only=True, default=0)
    data_sequence_number: int = field(kw_only=True, default=0)
    application_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.dpf = DataPacketFormat.DATA_RCV
        self.mt = MessageType.DATA

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DataMessageRcv', bytes]:
        if fields['dpf'] != DataPacketFormat.DATA_RCV or fields['mt'] != MessageType.DATA:
            raise Exception("Invalid constraint field values")
        if len(span) < 17:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_handle'] = value_
        fields['status'] = StatusCode.from_int(span[4])
        value_ = int.from_bytes(span[5:13], byteorder='little')
        fields['source_address'] = value_
        value_ = int.from_bytes(span[13:15], byteorder='little')
        fields['data_sequence_number'] = value_
        value_ = int.from_bytes(span[15:17], byteorder='little')
        application_data_size = value_
        span = span[17:]
        if len(span) < application_data_size:
            raise Exception('Invalid packet size')
        fields['application_data'] = list(span[:application_data_size])
        span = span[application_data_size:]
        return DataMessageRcv(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_handle > 4294967295:
            print(f"Invalid value for field DataMessageRcv::session_handle: {self.session_handle} > 4294967295; the value will be truncated")
            self.session_handle &= 4294967295
        _span.extend(int.to_bytes((self.session_handle << 0), length=4, byteorder='little'))
        _span.append((self.status << 0))
        if self.source_address > 18446744073709551615:
            print(f"Invalid value for field DataMessageRcv::source_address: {self.source_address} > 18446744073709551615; the value will be truncated")
            self.source_address &= 18446744073709551615
        _span.extend(int.to_bytes((self.source_address << 0), length=8, byteorder='little'))
        if self.data_sequence_number > 65535:
            print(f"Invalid value for field DataMessageRcv::data_sequence_number: {self.data_sequence_number} > 65535; the value will be truncated")
            self.data_sequence_number &= 65535
        _span.extend(int.to_bytes((self.data_sequence_number << 0), length=2, byteorder='little'))
        _span.extend(int.to_bytes(((len(self.application_data) * 1) << 0), length=2, byteorder='little'))
        _span.extend(self.application_data)
        return DataPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.application_data) * 1 + 17

@dataclass
class UciCommand(ControlPacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciCommand', bytes]:
        if fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceResetCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionControlCommand.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidGetPowerStatsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return UciCommand(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciResponse(ControlPacket):
    

    def __post_init__(self):
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciResponse', bytes]:
        if fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceResetRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitRsp_V2.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStartRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStopRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetRangingCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidGetPowerStatsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return UciResponse(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciNotification(ControlPacket):
    

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciNotification', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GenericError.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataCreditNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataTransferStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidRangeDiagnosticsNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return UciNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class CoreCommand(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreCommand', bytes]:
        if fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceResetCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return CoreCommand(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class CoreResponse(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreResponse', bytes]:
        if fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceResetRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetDeviceInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetCapsInfoRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GetConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return CoreQueryTimeStampRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return CoreResponse(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class CoreNotification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.CORE
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreNotification', bytes]:
        if fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DeviceStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return GenericError.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return CoreNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class SessionConfigCommand(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionConfigCommand', bytes]:
        if fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return SessionInitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionConfigCommand(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class SessionConfigResponse(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionConfigResponse', bytes]:
        if fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return SessionInitRsp_V2.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionDeinitRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetAppConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetStateRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateDtTagRangingRoundsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionSetHybridConfigRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionQueryMaxDataSizeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionConfigResponse(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class SessionConfigNotification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionConfigNotification', bytes]:
        if fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return SessionStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionUpdateControllerMulticastListNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionConfigNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class SessionControlCommand(UciCommand):
    session_id: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionControlCommand', bytes]:
        if fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return SessionStartCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStopCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetRangingCountCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionControlCommand(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_id > 4294967295:
            print(f"Invalid value for field SessionControlCommand::session_id: {self.session_id} > 4294967295; the value will be truncated")
            self.session_id &= 4294967295
        _span.extend(int.to_bytes((self.session_id << 0), length=4, byteorder='little'))
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 4

@dataclass
class SessionControlResponse(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionControlResponse', bytes]:
        if fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return SessionStartRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionStopRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionGetRangingCountRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionControlResponse(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class SessionControlNotification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionControlNotification', bytes]:
        if fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return DataCreditNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return DataTransferStatusNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return SessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionControlNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class AndroidCommand(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidCommand', bytes]:
        if fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return AndroidGetPowerStatsCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeCmd.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return AndroidCommand(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class AndroidResponse(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidResponse', bytes]:
        if fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return AndroidGetPowerStatsRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return AndroidSetCountryCodeRsp.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return AndroidResponse(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class AndroidNotification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidNotification', bytes]:
        if fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return AndroidRangeDiagnosticsNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return AndroidNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class DeviceResetCmd(CoreCommand):
    reset_config: ResetConfig = field(kw_only=True, default=ResetConfig.UWBS_RESET)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceResetCmd', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['reset_config'] = ResetConfig.from_int(span[0])
        span = span[1:]
        return DeviceResetCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.reset_config << 0))
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class DeviceResetRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceResetRsp', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return DeviceResetRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class DeviceStatusNtf(CoreNotification):
    device_state: DeviceState = field(kw_only=True, default=DeviceState.DEVICE_STATE_READY)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.CORE
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DeviceStatusNtf', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['device_state'] = DeviceState.from_int(span[0])
        span = span[1:]
        return DeviceStatusNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.device_state << 0))
        return CoreNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class GetDeviceInfoCmd(CoreCommand):
    

    def __post_init__(self):
        self.opcode = 2
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetDeviceInfoCmd', bytes]:
        if fields['opcode'] != 0x2 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return GetDeviceInfoCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class GetDeviceInfoRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    uci_version: int = field(kw_only=True, default=0)
    mac_version: int = field(kw_only=True, default=0)
    phy_version: int = field(kw_only=True, default=0)
    uci_test_version: int = field(kw_only=True, default=0)
    vendor_spec_info: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.opcode = 2
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetDeviceInfoRsp', bytes]:
        if fields['opcode'] != 0x2 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 10:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
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
        fields['vendor_spec_info'] = list(span[:vendor_spec_info_count])
        span = span[vendor_spec_info_count:]
        return GetDeviceInfoRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.uci_version > 65535:
            print(f"Invalid value for field GetDeviceInfoRsp::uci_version: {self.uci_version} > 65535; the value will be truncated")
            self.uci_version &= 65535
        _span.extend(int.to_bytes((self.uci_version << 0), length=2, byteorder='little'))
        if self.mac_version > 65535:
            print(f"Invalid value for field GetDeviceInfoRsp::mac_version: {self.mac_version} > 65535; the value will be truncated")
            self.mac_version &= 65535
        _span.extend(int.to_bytes((self.mac_version << 0), length=2, byteorder='little'))
        if self.phy_version > 65535:
            print(f"Invalid value for field GetDeviceInfoRsp::phy_version: {self.phy_version} > 65535; the value will be truncated")
            self.phy_version &= 65535
        _span.extend(int.to_bytes((self.phy_version << 0), length=2, byteorder='little'))
        if self.uci_test_version > 65535:
            print(f"Invalid value for field GetDeviceInfoRsp::uci_test_version: {self.uci_test_version} > 65535; the value will be truncated")
            self.uci_test_version &= 65535
        _span.extend(int.to_bytes((self.uci_test_version << 0), length=2, byteorder='little'))
        if len(self.vendor_spec_info) > 255:
            print(f"Invalid length for field GetDeviceInfoRsp::vendor_spec_info:  {len(self.vendor_spec_info)} > 255; the array will be truncated")
            del self.vendor_spec_info[255:]
        _span.append((len(self.vendor_spec_info) << 0))
        _span.extend(self.vendor_spec_info)
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.vendor_spec_info) * 1 + 10

@dataclass
class GetCapsInfoCmd(CoreCommand):
    

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetCapsInfoCmd', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return GetCapsInfoCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class CapTlv(Packet):
    t: CapTlvType = field(kw_only=True, default=CapTlvType.SUPPORTED_FIRA_PHY_VERSION_RANGE)
    v: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['CapTlv', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['t'] = CapTlvType.from_int(span[0])
        v_count = span[1]
        span = span[2:]
        if len(span) < v_count:
            raise Exception('Invalid packet size')
        fields['v'] = list(span[:v_count])
        span = span[v_count:]
        return CapTlv(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.t << 0))
        if len(self.v) > 255:
            print(f"Invalid length for field CapTlv::v:  {len(self.v)} > 255; the array will be truncated")
            del self.v[255:]
        _span.append((len(self.v) << 0))
        _span.extend(self.v)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.v) * 1 + 2

@dataclass
class GetCapsInfoRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    tlvs: List[CapTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetCapsInfoRsp', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        tlvs_count = span[1]
        span = span[2:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = CapTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return GetCapsInfoRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.tlvs) > 255:
            print(f"Invalid length for field GetCapsInfoRsp::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 2

@dataclass
class DeviceConfigTlv(Packet):
    cfg_id: DeviceConfigId = field(kw_only=True, default=DeviceConfigId.DEVICE_STATE)
    v: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['DeviceConfigTlv', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = DeviceConfigId.from_int(span[0])
        v_count = span[1]
        span = span[2:]
        if len(span) < v_count:
            raise Exception('Invalid packet size')
        fields['v'] = list(span[:v_count])
        span = span[v_count:]
        return DeviceConfigTlv(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.cfg_id << 0))
        if len(self.v) > 255:
            print(f"Invalid length for field DeviceConfigTlv::v:  {len(self.v)} > 255; the array will be truncated")
            del self.v[255:]
        _span.append((len(self.v) << 0))
        _span.extend(self.v)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.v) * 1 + 2

@dataclass
class SetConfigCmd(CoreCommand):
    tlvs: List[DeviceConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 4
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SetConfigCmd', bytes]:
        if fields['opcode'] != 0x4 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        tlvs_count = span[0]
        span = span[1:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = DeviceConfigTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return SetConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.tlvs) > 255:
            print(f"Invalid length for field SetConfigCmd::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 1

@dataclass
class DeviceConfigStatus(Packet):
    cfg_id: DeviceConfigId = field(kw_only=True, default=DeviceConfigId.DEVICE_STATE)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['DeviceConfigStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = DeviceConfigId.from_int(span[0])
        fields['status'] = StatusCode.from_int(span[1])
        span = span[2:]
        return DeviceConfigStatus(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.cfg_id << 0))
        _span.append((self.status << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class SetConfigRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    cfg_status: List[DeviceConfigStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 4
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SetConfigRsp', bytes]:
        if fields['opcode'] != 0x4 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        cfg_status_count = span[1]
        span = span[2:]
        if len(span) < cfg_status_count * 2:
            raise Exception('Invalid packet size')
        cfg_status = []
        for n in range(cfg_status_count):
            cfg_status.append(DeviceConfigStatus.parse_all(span[n * 2:(n + 1) * 2]))
        fields['cfg_status'] = cfg_status
        span = span[cfg_status_count * 2:]
        return SetConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.cfg_status) > 255:
            print(f"Invalid length for field SetConfigRsp::cfg_status:  {len(self.cfg_status)} > 255; the array will be truncated")
            del self.cfg_status[255:]
        _span.append((len(self.cfg_status) << 0))
        for _elt in self.cfg_status:
            _span.extend(_elt.serialize())
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.cfg_status]) + 2

@dataclass
class GetConfigCmd(CoreCommand):
    cfg_id: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.opcode = 5
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetConfigCmd', bytes]:
        if fields['opcode'] != 0x5 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        cfg_id_count = span[0]
        span = span[1:]
        if len(span) < cfg_id_count:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = list(span[:cfg_id_count])
        span = span[cfg_id_count:]
        return GetConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.cfg_id) > 255:
            print(f"Invalid length for field GetConfigCmd::cfg_id:  {len(self.cfg_id)} > 255; the array will be truncated")
            del self.cfg_id[255:]
        _span.append((len(self.cfg_id) << 0))
        _span.extend(self.cfg_id)
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.cfg_id) * 1 + 1

@dataclass
class GetConfigRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    tlvs: List[DeviceConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 5
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GetConfigRsp', bytes]:
        if fields['opcode'] != 0x5 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        tlvs_count = span[1]
        span = span[2:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = DeviceConfigTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return GetConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.tlvs) > 255:
            print(f"Invalid length for field GetConfigRsp::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 2

@dataclass
class GenericError(CoreNotification):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 7
        self.gid = GroupId.CORE
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['GenericError', bytes]:
        if fields['opcode'] != 0x7 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return GenericError(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return CoreNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class CoreQueryTimeStampCmd(CoreCommand):
    

    def __post_init__(self):
        self.opcode = 8
        self.gid = GroupId.CORE
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreQueryTimeStampCmd', bytes]:
        if fields['opcode'] != 0x8 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return CoreQueryTimeStampCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CoreCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class CoreQueryTimeStampRsp(CoreResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    timeStamp: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 8
        self.gid = GroupId.CORE
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreQueryTimeStampRsp', bytes]:
        if fields['opcode'] != 0x8 or fields['gid'] != GroupId.CORE or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 9:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        value_ = int.from_bytes(span[1:9], byteorder='little')
        fields['timeStamp'] = value_
        span = span[9:]
        return CoreQueryTimeStampRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.timeStamp > 18446744073709551615:
            print(f"Invalid value for field CoreQueryTimeStampRsp::timeStamp: {self.timeStamp} > 18446744073709551615; the value will be truncated")
            self.timeStamp &= 18446744073709551615
        _span.extend(int.to_bytes((self.timeStamp << 0), length=8, byteorder='little'))
        return CoreResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 9

@dataclass
class SessionInitCmd(SessionConfigCommand):
    session_id: int = field(kw_only=True, default=0)
    session_type: SessionType = field(kw_only=True, default=SessionType.FIRA_RANGING_SESSION)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitCmd', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        fields['session_type'] = SessionType.from_int(span[4])
        span = span[5:]
        return SessionInitCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_id > 4294967295:
            print(f"Invalid value for field SessionInitCmd::session_id: {self.session_id} > 4294967295; the value will be truncated")
            self.session_id &= 4294967295
        _span.extend(int.to_bytes((self.session_id << 0), length=4, byteorder='little'))
        _span.append((self.session_type << 0))
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class SessionInitRsp_V2(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    session_handle: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitRsp_V2', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        value_ = int.from_bytes(span[1:5], byteorder='little')
        fields['session_handle'] = value_
        span = span[5:]
        return SessionInitRsp_V2(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.session_handle > 4294967295:
            print(f"Invalid value for field SessionInitRsp_V2::session_handle: {self.session_handle} > 4294967295; the value will be truncated")
            self.session_handle &= 4294967295
        _span.extend(int.to_bytes((self.session_handle << 0), length=4, byteorder='little'))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class SessionInitRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitRsp', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionInitRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionDeinitCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitCmd', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        span = span[4:]
        return SessionDeinitCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionDeinitCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionDeinitRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitRsp', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionDeinitRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionStatusNtf(SessionConfigNotification):
    session_token: int = field(kw_only=True, default=0)
    session_state: SessionState = field(kw_only=True, default=SessionState.SESSION_STATE_INIT)
    reason_code: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 2
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStatusNtf', bytes]:
        if fields['opcode'] != 0x2 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['session_state'] = SessionState.from_int(span[4])
        fields['reason_code'] = span[5]
        span = span[6:]
        return SessionStatusNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionStatusNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        _span.append((self.session_state << 0))
        if self.reason_code > 255:
            print(f"Invalid value for field SessionStatusNtf::reason_code: {self.reason_code} > 255; the value will be truncated")
            self.reason_code &= 255
        _span.append((self.reason_code << 0))
        return SessionConfigNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 6

@dataclass
class AppConfigTlv(Packet):
    cfg_id: AppConfigTlvType = field(kw_only=True, default=AppConfigTlvType.DEVICE_TYPE)
    v: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['AppConfigTlv', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = AppConfigTlvType.from_int(span[0])
        v_count = span[1]
        span = span[2:]
        if len(span) < v_count:
            raise Exception('Invalid packet size')
        fields['v'] = list(span[:v_count])
        span = span[v_count:]
        return AppConfigTlv(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.cfg_id << 0))
        if len(self.v) > 255:
            print(f"Invalid length for field AppConfigTlv::v:  {len(self.v)} > 255; the array will be truncated")
            del self.v[255:]
        _span.append((len(self.v) << 0))
        _span.extend(self.v)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.v) * 1 + 2

@dataclass
class SessionSetAppConfigCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)
    tlvs: List[AppConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigCmd', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        tlvs_count = span[4]
        span = span[5:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = AppConfigTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return SessionSetAppConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionSetAppConfigCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if len(self.tlvs) > 255:
            print(f"Invalid length for field SessionSetAppConfigCmd::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 5

@dataclass
class AppConfigStatus(Packet):
    cfg_id: AppConfigTlvType = field(kw_only=True, default=AppConfigTlvType.DEVICE_TYPE)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['AppConfigStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = AppConfigTlvType.from_int(span[0])
        fields['status'] = StatusCode.from_int(span[1])
        span = span[2:]
        return AppConfigStatus(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.cfg_id << 0))
        _span.append((self.status << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class SessionSetAppConfigRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    cfg_status: List[AppConfigStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigRsp', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        cfg_status_count = span[1]
        span = span[2:]
        if len(span) < cfg_status_count * 2:
            raise Exception('Invalid packet size')
        cfg_status = []
        for n in range(cfg_status_count):
            cfg_status.append(AppConfigStatus.parse_all(span[n * 2:(n + 1) * 2]))
        fields['cfg_status'] = cfg_status
        span = span[cfg_status_count * 2:]
        return SessionSetAppConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.cfg_status) > 255:
            print(f"Invalid length for field SessionSetAppConfigRsp::cfg_status:  {len(self.cfg_status)} > 255; the array will be truncated")
            del self.cfg_status[255:]
        _span.append((len(self.cfg_status) << 0))
        for _elt in self.cfg_status:
            _span.extend(_elt.serialize())
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.cfg_status]) + 2

@dataclass
class SessionGetAppConfigCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)
    app_cfg: List[AppConfigTlvType] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 4
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigCmd', bytes]:
        if fields['opcode'] != 0x4 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        app_cfg_count = span[4]
        span = span[5:]
        if len(span) < app_cfg_count:
            raise Exception('Invalid packet size')
        app_cfg = []
        for n in range(app_cfg_count):
            app_cfg.append(AppConfigTlvType(int.from_bytes(span[n:n + 1], byteorder='little')))
        fields['app_cfg'] = app_cfg
        span = span[app_cfg_count:]
        return SessionGetAppConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionGetAppConfigCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if len(self.app_cfg) > 255:
            print(f"Invalid length for field SessionGetAppConfigCmd::app_cfg:  {len(self.app_cfg)} > 255; the array will be truncated")
            del self.app_cfg[255:]
        _span.append((len(self.app_cfg) << 0))
        for _elt in self.app_cfg:
            _span.append(_elt)
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.app_cfg) * 8 + 5

@dataclass
class SessionGetAppConfigRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    tlvs: List[AppConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 4
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigRsp', bytes]:
        if fields['opcode'] != 0x4 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        tlvs_count = span[1]
        span = span[2:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = AppConfigTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return SessionGetAppConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.tlvs) > 255:
            print(f"Invalid length for field SessionGetAppConfigRsp::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 2

@dataclass
class SessionGetCountCmd(SessionConfigCommand):
    

    def __post_init__(self):
        self.opcode = 5
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountCmd', bytes]:
        if fields['opcode'] != 0x5 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return SessionGetCountCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionGetCountRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    session_count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 5
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountRsp', bytes]:
        if fields['opcode'] != 0x5 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        fields['session_count'] = span[1]
        span = span[2:]
        return SessionGetCountRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.session_count > 255:
            print(f"Invalid value for field SessionGetCountRsp::session_count: {self.session_count} > 255; the value will be truncated")
            self.session_count &= 255
        _span.append((self.session_count << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class SessionGetStateCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 6
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateCmd', bytes]:
        if fields['opcode'] != 0x6 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        span = span[4:]
        return SessionGetStateCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionGetStateCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionGetStateRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    session_state: SessionState = field(kw_only=True, default=SessionState.SESSION_STATE_INIT)

    def __post_init__(self):
        self.opcode = 6
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateRsp', bytes]:
        if fields['opcode'] != 0x6 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        fields['session_state'] = SessionState.from_int(span[1])
        span = span[2:]
        return SessionGetStateRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        _span.append((self.session_state << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class SessionUpdateDtTagRangingRoundsCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)
    ranging_round_indexes: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.opcode = 9
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtTagRangingRoundsCmd', bytes]:
        if fields['opcode'] != 0x9 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        ranging_round_indexes_count = span[4]
        span = span[5:]
        if len(span) < ranging_round_indexes_count:
            raise Exception('Invalid packet size')
        fields['ranging_round_indexes'] = list(span[:ranging_round_indexes_count])
        span = span[ranging_round_indexes_count:]
        return SessionUpdateDtTagRangingRoundsCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionUpdateDtTagRangingRoundsCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if len(self.ranging_round_indexes) > 255:
            print(f"Invalid length for field SessionUpdateDtTagRangingRoundsCmd::ranging_round_indexes:  {len(self.ranging_round_indexes)} > 255; the array will be truncated")
            del self.ranging_round_indexes[255:]
        _span.append((len(self.ranging_round_indexes) << 0))
        _span.extend(self.ranging_round_indexes)
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.ranging_round_indexes) * 1 + 5

@dataclass
class SessionUpdateDtTagRangingRoundsRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    ranging_round_indexes: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.opcode = 9
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtTagRangingRoundsRsp', bytes]:
        if fields['opcode'] != 0x9 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        ranging_round_indexes_count = span[1]
        span = span[2:]
        if len(span) < ranging_round_indexes_count:
            raise Exception('Invalid packet size')
        fields['ranging_round_indexes'] = list(span[:ranging_round_indexes_count])
        span = span[ranging_round_indexes_count:]
        return SessionUpdateDtTagRangingRoundsRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.ranging_round_indexes) > 255:
            print(f"Invalid length for field SessionUpdateDtTagRangingRoundsRsp::ranging_round_indexes:  {len(self.ranging_round_indexes)} > 255; the array will be truncated")
            del self.ranging_round_indexes[255:]
        _span.append((len(self.ranging_round_indexes) << 0))
        _span.extend(self.ranging_round_indexes)
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.ranging_round_indexes) * 1 + 2

@dataclass
class Controlee(Packet):
    short_address: bytearray = field(kw_only=True, default_factory=bytearray)
    subsession_id: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Controlee', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['short_address'] = list(span[:2])
        span = span[2:]
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['subsession_id'] = value_
        span = span[4:]
        return Controlee(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.short_address)
        if self.subsession_id > 4294967295:
            print(f"Invalid value for field Controlee::subsession_id: {self.subsession_id} > 4294967295; the value will be truncated")
            self.subsession_id &= 4294967295
        _span.extend(int.to_bytes((self.subsession_id << 0), length=4, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 6

@dataclass
class Controlee_V2_0_16_Byte_Version(Packet):
    short_address: bytearray = field(kw_only=True, default_factory=bytearray)
    subsession_id: int = field(kw_only=True, default=0)
    subsession_key: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Controlee_V2_0_16_Byte_Version', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['short_address'] = list(span[:2])
        span = span[2:]
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['subsession_id'] = value_
        span = span[4:]
        if len(span) < 16:
            raise Exception('Invalid packet size')
        fields['subsession_key'] = list(span[:16])
        span = span[16:]
        return Controlee_V2_0_16_Byte_Version(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.short_address)
        if self.subsession_id > 4294967295:
            print(f"Invalid value for field Controlee_V2_0_16_Byte_Version::subsession_id: {self.subsession_id} > 4294967295; the value will be truncated")
            self.subsession_id &= 4294967295
        _span.extend(int.to_bytes((self.subsession_id << 0), length=4, byteorder='little'))
        _span.extend(self.subsession_key)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 22

@dataclass
class Controlee_V2_0_32_Byte_Version(Packet):
    short_address: bytearray = field(kw_only=True, default_factory=bytearray)
    subsession_id: int = field(kw_only=True, default=0)
    subsession_key: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Controlee_V2_0_32_Byte_Version', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['short_address'] = list(span[:2])
        span = span[2:]
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['subsession_id'] = value_
        span = span[4:]
        if len(span) < 32:
            raise Exception('Invalid packet size')
        fields['subsession_key'] = list(span[:32])
        span = span[32:]
        return Controlee_V2_0_32_Byte_Version(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.short_address)
        if self.subsession_id > 4294967295:
            print(f"Invalid value for field Controlee_V2_0_32_Byte_Version::subsession_id: {self.subsession_id} > 4294967295; the value will be truncated")
            self.subsession_id &= 4294967295
        _span.extend(int.to_bytes((self.subsession_id << 0), length=4, byteorder='little'))
        _span.extend(self.subsession_key)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 38

class UpdateMulticastListAction(enum.IntEnum):
    ADD_CONTROLEE = 0x0
    REMOVE_CONTROLEE = 0x1
    ADD_CONTROLEE_WITH_SHORT_SUB_SESSION_KEY = 0x2
    ADD_CONTROLEE_WITH_EXTENDED_SUB_SESSION_KEY = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'UpdateMulticastListAction']:
        try:
            return UpdateMulticastListAction(v)
        except ValueError as exn:
            raise exn


@dataclass
class SessionUpdateControllerMulticastListCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)
    action: UpdateMulticastListAction = field(kw_only=True, default=UpdateMulticastListAction.ADD_CONTROLEE)

    def __post_init__(self):
        self.opcode = 7
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmd', bytes]:
        if fields['opcode'] != 0x7 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['action'] = UpdateMulticastListAction.from_int(span[4])
        span = span[5:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return SessionUpdateControllerMulticastListCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionUpdateControllerMulticastListCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        _span.append((self.action << 0))
        _span.extend(payload or self.payload or [])
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 5

@dataclass
class PhaseList(Packet):
    session_token: int = field(kw_only=True, default=0)
    start_slot_index: int = field(kw_only=True, default=0)
    end_slot_index: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['PhaseList', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['start_slot_index'] = value_
        value_ = int.from_bytes(span[6:8], byteorder='little')
        fields['end_slot_index'] = value_
        span = span[8:]
        return PhaseList(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field PhaseList::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.start_slot_index > 65535:
            print(f"Invalid value for field PhaseList::start_slot_index: {self.start_slot_index} > 65535; the value will be truncated")
            self.start_slot_index &= 65535
        _span.extend(int.to_bytes((self.start_slot_index << 0), length=2, byteorder='little'))
        if self.end_slot_index > 65535:
            print(f"Invalid value for field PhaseList::end_slot_index: {self.end_slot_index} > 65535; the value will be truncated")
            self.end_slot_index &= 65535
        _span.extend(int.to_bytes((self.end_slot_index << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class SessionSetHybridConfigCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)
    number_of_phases: int = field(kw_only=True, default=0)
    update_time: bytearray = field(kw_only=True, default_factory=bytearray)
    phase_list: List[PhaseList] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 12
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetHybridConfigCmd', bytes]:
        if fields['opcode'] != 0xc or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['number_of_phases'] = span[4]
        span = span[5:]
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['update_time'] = list(span[:8])
        span = span[8:]
        if len(span) % 8 != 0:
            raise Exception('Array size is not a multiple of the element size')
        phase_list_count = int(len(span) / 8)
        phase_list = []
        for n in range(phase_list_count):
            phase_list.append(PhaseList.parse_all(span[n * 8:(n + 1) * 8]))
        fields['phase_list'] = phase_list
        span = bytes()
        return SessionSetHybridConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionSetHybridConfigCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.number_of_phases > 255:
            print(f"Invalid value for field SessionSetHybridConfigCmd::number_of_phases: {self.number_of_phases} > 255; the value will be truncated")
            self.number_of_phases &= 255
        _span.append((self.number_of_phases << 0))
        _span.extend(self.update_time)
        for _elt in self.phase_list:
            _span.extend(_elt.serialize())
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.phase_list]) + 13

@dataclass
class SessionSetHybridConfigRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 12
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetHybridConfigRsp', bytes]:
        if fields['opcode'] != 0xc or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionSetHybridConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionUpdateControllerMulticastListCmdPayload(Packet):
    controlees: List[Controlee] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmdPayload', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        controlees_count = span[0]
        span = span[1:]
        if len(span) < controlees_count * 6:
            raise Exception('Invalid packet size')
        controlees = []
        for n in range(controlees_count):
            controlees.append(Controlee.parse_all(span[n * 6:(n + 1) * 6]))
        fields['controlees'] = controlees
        span = span[controlees_count * 6:]
        return SessionUpdateControllerMulticastListCmdPayload(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.controlees) > 255:
            print(f"Invalid length for field SessionUpdateControllerMulticastListCmdPayload::controlees:  {len(self.controlees)} > 255; the array will be truncated")
            del self.controlees[255:]
        _span.append((len(self.controlees) << 0))
        for _elt in self.controlees:
            _span.extend(_elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.controlees]) + 1

@dataclass
class SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload(Packet):
    controlees: List[Controlee_V2_0_16_Byte_Version] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        controlees_count = span[0]
        span = span[1:]
        if len(span) < controlees_count * 22:
            raise Exception('Invalid packet size')
        controlees = []
        for n in range(controlees_count):
            controlees.append(Controlee_V2_0_16_Byte_Version.parse_all(span[n * 22:(n + 1) * 22]))
        fields['controlees'] = controlees
        span = span[controlees_count * 22:]
        return SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.controlees) > 255:
            print(f"Invalid length for field SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload::controlees:  {len(self.controlees)} > 255; the array will be truncated")
            del self.controlees[255:]
        _span.append((len(self.controlees) << 0))
        for _elt in self.controlees:
            _span.extend(_elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.controlees]) + 1

@dataclass
class SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload(Packet):
    controlees: List[Controlee_V2_0_32_Byte_Version] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        controlees_count = span[0]
        span = span[1:]
        if len(span) < controlees_count * 38:
            raise Exception('Invalid packet size')
        controlees = []
        for n in range(controlees_count):
            controlees.append(Controlee_V2_0_32_Byte_Version.parse_all(span[n * 38:(n + 1) * 38]))
        fields['controlees'] = controlees
        span = span[controlees_count * 38:]
        return SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.controlees) > 255:
            print(f"Invalid length for field SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload::controlees:  {len(self.controlees)} > 255; the array will be truncated")
            del self.controlees[255:]
        _span.append((len(self.controlees) << 0))
        for _elt in self.controlees:
            _span.extend(_elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.controlees]) + 1

@dataclass
class SessionUpdateControllerMulticastListRsp(SessionConfigResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 7
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListRsp', bytes]:
        if fields['opcode'] != 0x7 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionUpdateControllerMulticastListRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class ControleeStatus(Packet):
    mac_address: bytearray = field(kw_only=True, default_factory=bytearray)
    subsession_id: int = field(kw_only=True, default=0)
    status: MulticastUpdateStatusCode = field(kw_only=True, default=MulticastUpdateStatusCode.STATUS_OK_MULTICAST_LIST_UPDATE)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ControleeStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['mac_address'] = list(span[:2])
        span = span[2:]
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['subsession_id'] = value_
        fields['status'] = MulticastUpdateStatusCode.from_int(span[4])
        span = span[5:]
        return ControleeStatus(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.mac_address)
        if self.subsession_id > 4294967295:
            print(f"Invalid value for field ControleeStatus::subsession_id: {self.subsession_id} > 4294967295; the value will be truncated")
            self.subsession_id &= 4294967295
        _span.extend(int.to_bytes((self.subsession_id << 0), length=4, byteorder='little'))
        _span.append((self.status << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 7

@dataclass
class SessionUpdateControllerMulticastListNtf(SessionConfigNotification):
    session_token: int = field(kw_only=True, default=0)
    remaining_multicast_list_size: int = field(kw_only=True, default=0)
    controlee_status: List[ControleeStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 7
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListNtf', bytes]:
        if fields['opcode'] != 0x7 or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['remaining_multicast_list_size'] = span[4]
        controlee_status_count = span[5]
        span = span[6:]
        if len(span) < controlee_status_count * 7:
            raise Exception('Invalid packet size')
        controlee_status = []
        for n in range(controlee_status_count):
            controlee_status.append(ControleeStatus.parse_all(span[n * 7:(n + 1) * 7]))
        fields['controlee_status'] = controlee_status
        span = span[controlee_status_count * 7:]
        return SessionUpdateControllerMulticastListNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionUpdateControllerMulticastListNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.remaining_multicast_list_size > 255:
            print(f"Invalid value for field SessionUpdateControllerMulticastListNtf::remaining_multicast_list_size: {self.remaining_multicast_list_size} > 255; the value will be truncated")
            self.remaining_multicast_list_size &= 255
        _span.append((self.remaining_multicast_list_size << 0))
        if len(self.controlee_status) > 255:
            print(f"Invalid length for field SessionUpdateControllerMulticastListNtf::controlee_status:  {len(self.controlee_status)} > 255; the array will be truncated")
            del self.controlee_status[255:]
        _span.append((len(self.controlee_status) << 0))
        for _elt in self.controlee_status:
            _span.extend(_elt.serialize())
        return SessionConfigNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.controlee_status]) + 6

@dataclass
class DataCreditNtf(SessionControlNotification):
    session_token: int = field(kw_only=True, default=0)
    credit_availability: CreditAvailability = field(kw_only=True, default=CreditAvailability.CREDIT_NOT_AVAILABLE)

    def __post_init__(self):
        self.opcode = 4
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DataCreditNtf', bytes]:
        if fields['opcode'] != 0x4 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['credit_availability'] = CreditAvailability.from_int(span[4])
        span = span[5:]
        return DataCreditNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field DataCreditNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        _span.append((self.credit_availability << 0))
        return SessionControlNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class DataTransferStatusNtf(SessionControlNotification):
    session_token: int = field(kw_only=True, default=0)
    uci_sequence_number: int = field(kw_only=True, default=0)
    status: DataTransferNtfStatusCode = field(kw_only=True, default=DataTransferNtfStatusCode.UCI_DATA_TRANSFER_STATUS_REPETITION_OK)
    tx_count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 5
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['DataTransferStatusNtf', bytes]:
        if fields['opcode'] != 0x5 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 7:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['uci_sequence_number'] = span[4]
        fields['status'] = DataTransferNtfStatusCode.from_int(span[5])
        fields['tx_count'] = span[6]
        span = span[7:]
        return DataTransferStatusNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field DataTransferStatusNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.uci_sequence_number > 255:
            print(f"Invalid value for field DataTransferStatusNtf::uci_sequence_number: {self.uci_sequence_number} > 255; the value will be truncated")
            self.uci_sequence_number &= 255
        _span.append((self.uci_sequence_number << 0))
        _span.append((self.status << 0))
        if self.tx_count > 255:
            print(f"Invalid value for field DataTransferStatusNtf::tx_count: {self.tx_count} > 255; the value will be truncated")
            self.tx_count &= 255
        _span.append((self.tx_count << 0))
        return SessionControlNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 7

@dataclass
class SessionQueryMaxDataSizeCmd(SessionConfigCommand):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 11
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionQueryMaxDataSizeCmd', bytes]:
        if fields['opcode'] != 0xb or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        span = span[4:]
        return SessionQueryMaxDataSizeCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionQueryMaxDataSizeCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        return SessionConfigCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionQueryMaxDataSizeRsp(SessionConfigResponse):
    session_token: int = field(kw_only=True, default=0)
    max_data_size: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 11
        self.gid = GroupId.SESSION_CONFIG
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionQueryMaxDataSizeRsp', bytes]:
        if fields['opcode'] != 0xb or fields['gid'] != GroupId.SESSION_CONFIG or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['max_data_size'] = value_
        span = span[6:]
        return SessionQueryMaxDataSizeRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionQueryMaxDataSizeRsp::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.max_data_size > 65535:
            print(f"Invalid value for field SessionQueryMaxDataSizeRsp::max_data_size: {self.max_data_size} > 65535; the value will be truncated")
            self.max_data_size &= 65535
        _span.extend(int.to_bytes((self.max_data_size << 0), length=2, byteorder='little'))
        return SessionConfigResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 6

@dataclass
class SessionStartCmd(SessionControlCommand):
    

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStartCmd', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return SessionStartCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionControlCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionStartRsp(SessionControlResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStartRsp', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionStartRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionControlResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class ShortAddressTwoWayRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    nlos: int = field(kw_only=True, default=0)
    distance: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_elevation_fom: int = field(kw_only=True, default=0)
    aoa_destination_azimuth: int = field(kw_only=True, default=0)
    aoa_destination_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_destination_elevation: int = field(kw_only=True, default=0)
    aoa_destination_elevation_fom: int = field(kw_only=True, default=0)
    slot_index: int = field(kw_only=True, default=0)
    rssi: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ShortAddressTwoWayRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 31:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode.from_int(span[2])
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
        fields['rssi'] = span[19]
        value_ = int.from_bytes(span[20:28], byteorder='little')
        value_ = int.from_bytes(span[28:31], byteorder='little')
        span = span[31:]
        return ShortAddressTwoWayRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::mac_address: {self.mac_address} > 65535; the value will be truncated")
            self.mac_address &= 65535
        _span.extend(int.to_bytes((self.mac_address << 0), length=2, byteorder='little'))
        _span.append((self.status << 0))
        if self.nlos > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::nlos: {self.nlos} > 255; the value will be truncated")
            self.nlos &= 255
        _span.append((self.nlos << 0))
        if self.distance > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::distance: {self.distance} > 65535; the value will be truncated")
            self.distance &= 65535
        _span.extend(int.to_bytes((self.distance << 0), length=2, byteorder='little'))
        if self.aoa_azimuth > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_azimuth: {self.aoa_azimuth} > 65535; the value will be truncated")
            self.aoa_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_azimuth_fom > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_azimuth_fom: {self.aoa_azimuth_fom} > 255; the value will be truncated")
            self.aoa_azimuth_fom &= 255
        _span.append((self.aoa_azimuth_fom << 0))
        if self.aoa_elevation > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_elevation: {self.aoa_elevation} > 65535; the value will be truncated")
            self.aoa_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_elevation << 0), length=2, byteorder='little'))
        if self.aoa_elevation_fom > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_elevation_fom: {self.aoa_elevation_fom} > 255; the value will be truncated")
            self.aoa_elevation_fom &= 255
        _span.append((self.aoa_elevation_fom << 0))
        if self.aoa_destination_azimuth > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_destination_azimuth: {self.aoa_destination_azimuth} > 65535; the value will be truncated")
            self.aoa_destination_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_destination_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_destination_azimuth_fom > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_destination_azimuth_fom: {self.aoa_destination_azimuth_fom} > 255; the value will be truncated")
            self.aoa_destination_azimuth_fom &= 255
        _span.append((self.aoa_destination_azimuth_fom << 0))
        if self.aoa_destination_elevation > 65535:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_destination_elevation: {self.aoa_destination_elevation} > 65535; the value will be truncated")
            self.aoa_destination_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_destination_elevation << 0), length=2, byteorder='little'))
        if self.aoa_destination_elevation_fom > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::aoa_destination_elevation_fom: {self.aoa_destination_elevation_fom} > 255; the value will be truncated")
            self.aoa_destination_elevation_fom &= 255
        _span.append((self.aoa_destination_elevation_fom << 0))
        if self.slot_index > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::slot_index: {self.slot_index} > 255; the value will be truncated")
            self.slot_index &= 255
        _span.append((self.slot_index << 0))
        if self.rssi > 255:
            print(f"Invalid value for field ShortAddressTwoWayRangingMeasurement::rssi: {self.rssi} > 255; the value will be truncated")
            self.rssi &= 255
        _span.append((self.rssi << 0))
        _span.extend([0] * 8)
        _span.extend([0] * 3)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 31

@dataclass
class ExtendedAddressTwoWayRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    nlos: int = field(kw_only=True, default=0)
    distance: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_elevation_fom: int = field(kw_only=True, default=0)
    aoa_destination_azimuth: int = field(kw_only=True, default=0)
    aoa_destination_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_destination_elevation: int = field(kw_only=True, default=0)
    aoa_destination_elevation_fom: int = field(kw_only=True, default=0)
    slot_index: int = field(kw_only=True, default=0)
    rssi: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ExtendedAddressTwoWayRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 31:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode.from_int(span[8])
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
        fields['rssi'] = span[25]
        value_ = int.from_bytes(span[26:31], byteorder='little')
        span = span[31:]
        return ExtendedAddressTwoWayRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 18446744073709551615:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::mac_address: {self.mac_address} > 18446744073709551615; the value will be truncated")
            self.mac_address &= 18446744073709551615
        _span.extend(int.to_bytes((self.mac_address << 0), length=8, byteorder='little'))
        _span.append((self.status << 0))
        if self.nlos > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::nlos: {self.nlos} > 255; the value will be truncated")
            self.nlos &= 255
        _span.append((self.nlos << 0))
        if self.distance > 65535:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::distance: {self.distance} > 65535; the value will be truncated")
            self.distance &= 65535
        _span.extend(int.to_bytes((self.distance << 0), length=2, byteorder='little'))
        if self.aoa_azimuth > 65535:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_azimuth: {self.aoa_azimuth} > 65535; the value will be truncated")
            self.aoa_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_azimuth_fom > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_azimuth_fom: {self.aoa_azimuth_fom} > 255; the value will be truncated")
            self.aoa_azimuth_fom &= 255
        _span.append((self.aoa_azimuth_fom << 0))
        if self.aoa_elevation > 65535:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_elevation: {self.aoa_elevation} > 65535; the value will be truncated")
            self.aoa_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_elevation << 0), length=2, byteorder='little'))
        if self.aoa_elevation_fom > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_elevation_fom: {self.aoa_elevation_fom} > 255; the value will be truncated")
            self.aoa_elevation_fom &= 255
        _span.append((self.aoa_elevation_fom << 0))
        if self.aoa_destination_azimuth > 65535:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_destination_azimuth: {self.aoa_destination_azimuth} > 65535; the value will be truncated")
            self.aoa_destination_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_destination_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_destination_azimuth_fom > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_destination_azimuth_fom: {self.aoa_destination_azimuth_fom} > 255; the value will be truncated")
            self.aoa_destination_azimuth_fom &= 255
        _span.append((self.aoa_destination_azimuth_fom << 0))
        if self.aoa_destination_elevation > 65535:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_destination_elevation: {self.aoa_destination_elevation} > 65535; the value will be truncated")
            self.aoa_destination_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_destination_elevation << 0), length=2, byteorder='little'))
        if self.aoa_destination_elevation_fom > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::aoa_destination_elevation_fom: {self.aoa_destination_elevation_fom} > 255; the value will be truncated")
            self.aoa_destination_elevation_fom &= 255
        _span.append((self.aoa_destination_elevation_fom << 0))
        if self.slot_index > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::slot_index: {self.slot_index} > 255; the value will be truncated")
            self.slot_index &= 255
        _span.append((self.slot_index << 0))
        if self.rssi > 255:
            print(f"Invalid value for field ExtendedAddressTwoWayRangingMeasurement::rssi: {self.rssi} > 255; the value will be truncated")
            self.rssi &= 255
        _span.append((self.rssi << 0))
        _span.extend([0] * 5)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 31

@dataclass
class ShortAddressOwrAoaRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    nlos: int = field(kw_only=True, default=0)
    frame_sequence_number: int = field(kw_only=True, default=0)
    block_index: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_elevation_fom: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ShortAddressOwrAoaRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 13:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode.from_int(span[2])
        fields['nlos'] = span[3]
        fields['frame_sequence_number'] = span[4]
        value_ = int.from_bytes(span[5:7], byteorder='little')
        fields['block_index'] = value_
        value_ = int.from_bytes(span[7:9], byteorder='little')
        fields['aoa_azimuth'] = value_
        fields['aoa_azimuth_fom'] = span[9]
        value_ = int.from_bytes(span[10:12], byteorder='little')
        fields['aoa_elevation'] = value_
        fields['aoa_elevation_fom'] = span[12]
        span = span[13:]
        return ShortAddressOwrAoaRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 65535:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::mac_address: {self.mac_address} > 65535; the value will be truncated")
            self.mac_address &= 65535
        _span.extend(int.to_bytes((self.mac_address << 0), length=2, byteorder='little'))
        _span.append((self.status << 0))
        if self.nlos > 255:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::nlos: {self.nlos} > 255; the value will be truncated")
            self.nlos &= 255
        _span.append((self.nlos << 0))
        if self.frame_sequence_number > 255:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::frame_sequence_number: {self.frame_sequence_number} > 255; the value will be truncated")
            self.frame_sequence_number &= 255
        _span.append((self.frame_sequence_number << 0))
        if self.block_index > 65535:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::block_index: {self.block_index} > 65535; the value will be truncated")
            self.block_index &= 65535
        _span.extend(int.to_bytes((self.block_index << 0), length=2, byteorder='little'))
        if self.aoa_azimuth > 65535:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::aoa_azimuth: {self.aoa_azimuth} > 65535; the value will be truncated")
            self.aoa_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_azimuth_fom > 255:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::aoa_azimuth_fom: {self.aoa_azimuth_fom} > 255; the value will be truncated")
            self.aoa_azimuth_fom &= 255
        _span.append((self.aoa_azimuth_fom << 0))
        if self.aoa_elevation > 65535:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::aoa_elevation: {self.aoa_elevation} > 65535; the value will be truncated")
            self.aoa_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_elevation << 0), length=2, byteorder='little'))
        if self.aoa_elevation_fom > 255:
            print(f"Invalid value for field ShortAddressOwrAoaRangingMeasurement::aoa_elevation_fom: {self.aoa_elevation_fom} > 255; the value will be truncated")
            self.aoa_elevation_fom &= 255
        _span.append((self.aoa_elevation_fom << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 13

@dataclass
class ExtendedAddressOwrAoaRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    nlos: int = field(kw_only=True, default=0)
    frame_sequence_number: int = field(kw_only=True, default=0)
    block_index: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_elevation_fom: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ExtendedAddressOwrAoaRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 19:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        fields['status'] = StatusCode.from_int(span[8])
        fields['nlos'] = span[9]
        fields['frame_sequence_number'] = span[10]
        value_ = int.from_bytes(span[11:13], byteorder='little')
        fields['block_index'] = value_
        value_ = int.from_bytes(span[13:15], byteorder='little')
        fields['aoa_azimuth'] = value_
        fields['aoa_azimuth_fom'] = span[15]
        value_ = int.from_bytes(span[16:18], byteorder='little')
        fields['aoa_elevation'] = value_
        fields['aoa_elevation_fom'] = span[18]
        span = span[19:]
        return ExtendedAddressOwrAoaRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 18446744073709551615:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::mac_address: {self.mac_address} > 18446744073709551615; the value will be truncated")
            self.mac_address &= 18446744073709551615
        _span.extend(int.to_bytes((self.mac_address << 0), length=8, byteorder='little'))
        _span.append((self.status << 0))
        if self.nlos > 255:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::nlos: {self.nlos} > 255; the value will be truncated")
            self.nlos &= 255
        _span.append((self.nlos << 0))
        if self.frame_sequence_number > 255:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::frame_sequence_number: {self.frame_sequence_number} > 255; the value will be truncated")
            self.frame_sequence_number &= 255
        _span.append((self.frame_sequence_number << 0))
        if self.block_index > 65535:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::block_index: {self.block_index} > 65535; the value will be truncated")
            self.block_index &= 65535
        _span.extend(int.to_bytes((self.block_index << 0), length=2, byteorder='little'))
        if self.aoa_azimuth > 65535:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::aoa_azimuth: {self.aoa_azimuth} > 65535; the value will be truncated")
            self.aoa_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_azimuth_fom > 255:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::aoa_azimuth_fom: {self.aoa_azimuth_fom} > 255; the value will be truncated")
            self.aoa_azimuth_fom &= 255
        _span.append((self.aoa_azimuth_fom << 0))
        if self.aoa_elevation > 65535:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::aoa_elevation: {self.aoa_elevation} > 65535; the value will be truncated")
            self.aoa_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_elevation << 0), length=2, byteorder='little'))
        if self.aoa_elevation_fom > 255:
            print(f"Invalid value for field ExtendedAddressOwrAoaRangingMeasurement::aoa_elevation_fom: {self.aoa_elevation_fom} > 255; the value will be truncated")
            self.aoa_elevation_fom &= 255
        _span.append((self.aoa_elevation_fom << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 19

class RangingMeasurementType(enum.IntEnum):
    ONE_WAY = 0x0
    TWO_WAY = 0x1
    DL_TDOA = 0x2
    OWR_AOA = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'RangingMeasurementType']:
        try:
            return RangingMeasurementType(v)
        except ValueError as exn:
            raise exn


@dataclass
class SessionInfoNtf(SessionControlNotification):
    sequence_number: int = field(kw_only=True, default=0)
    session_token: int = field(kw_only=True, default=0)
    rcr_indicator: int = field(kw_only=True, default=0)
    current_ranging_interval: int = field(kw_only=True, default=0)
    ranging_measurement_type: RangingMeasurementType = field(kw_only=True, default=RangingMeasurementType.ONE_WAY)
    mac_address_indicator: MacAddressIndicator = field(kw_only=True, default=MacAddressIndicator.SHORT_ADDRESS)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInfoNtf', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 24:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['sequence_number'] = value_
        value_ = int.from_bytes(span[4:8], byteorder='little')
        fields['session_token'] = value_
        fields['rcr_indicator'] = span[8]
        value_ = int.from_bytes(span[9:13], byteorder='little')
        fields['current_ranging_interval'] = value_
        fields['ranging_measurement_type'] = RangingMeasurementType.from_int(span[13])
        fields['mac_address_indicator'] = MacAddressIndicator.from_int(span[15])
        value_ = int.from_bytes(span[16:24], byteorder='little')
        span = span[24:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            return ShortMacTwoWaySessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return ExtendedMacTwoWaySessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return ShortMacDlTDoASessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return ExtendedMacDlTDoASessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return ShortMacOwrAoaSessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return ExtendedMacOwrAoaSessionInfoNtf.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return SessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.sequence_number > 4294967295:
            print(f"Invalid value for field SessionInfoNtf::sequence_number: {self.sequence_number} > 4294967295; the value will be truncated")
            self.sequence_number &= 4294967295
        _span.extend(int.to_bytes((self.sequence_number << 0), length=4, byteorder='little'))
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionInfoNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.rcr_indicator > 255:
            print(f"Invalid value for field SessionInfoNtf::rcr_indicator: {self.rcr_indicator} > 255; the value will be truncated")
            self.rcr_indicator &= 255
        _span.append((self.rcr_indicator << 0))
        if self.current_ranging_interval > 4294967295:
            print(f"Invalid value for field SessionInfoNtf::current_ranging_interval: {self.current_ranging_interval} > 4294967295; the value will be truncated")
            self.current_ranging_interval &= 4294967295
        _span.extend(int.to_bytes((self.current_ranging_interval << 0), length=4, byteorder='little'))
        _span.append((self.ranging_measurement_type << 0))
        _span.extend([0] * 1)
        _span.append((self.mac_address_indicator << 0))
        _span.extend([0] * 8)
        _span.extend(payload or self.payload or [])
        return SessionControlNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 24

@dataclass
class ShortMacTwoWaySessionInfoNtf(SessionInfoNtf):
    two_way_ranging_measurements: List[ShortAddressTwoWayRangingMeasurement] = field(kw_only=True, default_factory=list)
    vendor_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.TWO_WAY
        self.mac_address_indicator = MacAddressIndicator.SHORT_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacTwoWaySessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.TWO_WAY or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < two_way_ranging_measurements_count * 31:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements = []
        for n in range(two_way_ranging_measurements_count):
            two_way_ranging_measurements.append(ShortAddressTwoWayRangingMeasurement.parse_all(span[n * 31:(n + 1) * 31]))
        fields['two_way_ranging_measurements'] = two_way_ranging_measurements
        span = span[two_way_ranging_measurements_count * 31:]
        fields['vendor_data'] = list(span)
        span = bytes()
        return ShortMacTwoWaySessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.two_way_ranging_measurements) > 255:
            print(f"Invalid length for field ShortMacTwoWaySessionInfoNtf::two_way_ranging_measurements:  {len(self.two_way_ranging_measurements)} > 255; the array will be truncated")
            del self.two_way_ranging_measurements[255:]
        _span.append((len(self.two_way_ranging_measurements) << 0))
        for _elt in self.two_way_ranging_measurements:
            _span.extend(_elt.serialize())
        _span.extend(self.vendor_data)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1 + (
        sum([elt.size for elt in self.two_way_ranging_measurements]) +
            len(self.vendor_data) * 1
        )

@dataclass
class ExtendedMacTwoWaySessionInfoNtf(SessionInfoNtf):
    two_way_ranging_measurements: List[ExtendedAddressTwoWayRangingMeasurement] = field(kw_only=True, default_factory=list)
    vendor_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.TWO_WAY
        self.mac_address_indicator = MacAddressIndicator.EXTENDED_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacTwoWaySessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.TWO_WAY or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < two_way_ranging_measurements_count * 31:
            raise Exception('Invalid packet size')
        two_way_ranging_measurements = []
        for n in range(two_way_ranging_measurements_count):
            two_way_ranging_measurements.append(ExtendedAddressTwoWayRangingMeasurement.parse_all(span[n * 31:(n + 1) * 31]))
        fields['two_way_ranging_measurements'] = two_way_ranging_measurements
        span = span[two_way_ranging_measurements_count * 31:]
        fields['vendor_data'] = list(span)
        span = bytes()
        return ExtendedMacTwoWaySessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.two_way_ranging_measurements) > 255:
            print(f"Invalid length for field ExtendedMacTwoWaySessionInfoNtf::two_way_ranging_measurements:  {len(self.two_way_ranging_measurements)} > 255; the array will be truncated")
            del self.two_way_ranging_measurements[255:]
        _span.append((len(self.two_way_ranging_measurements) << 0))
        for _elt in self.two_way_ranging_measurements:
            _span.extend(_elt.serialize())
        _span.extend(self.vendor_data)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1 + (
        sum([elt.size for elt in self.two_way_ranging_measurements]) +
            len(self.vendor_data) * 1
        )

@dataclass
class ShortMacDlTDoASessionInfoNtf(SessionInfoNtf):
    no_of_ranging_measurements: int = field(kw_only=True, default=0)
    dl_tdoa_measurements: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.DL_TDOA
        self.mac_address_indicator = MacAddressIndicator.SHORT_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacDlTDoASessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.DL_TDOA or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['no_of_ranging_measurements'] = span[0]
        span = span[1:]
        fields['dl_tdoa_measurements'] = list(span)
        span = bytes()
        return ShortMacDlTDoASessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.no_of_ranging_measurements > 255:
            print(f"Invalid value for field ShortMacDlTDoASessionInfoNtf::no_of_ranging_measurements: {self.no_of_ranging_measurements} > 255; the value will be truncated")
            self.no_of_ranging_measurements &= 255
        _span.append((self.no_of_ranging_measurements << 0))
        _span.extend(self.dl_tdoa_measurements)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.dl_tdoa_measurements) * 1 + 1

@dataclass
class ExtendedMacDlTDoASessionInfoNtf(SessionInfoNtf):
    no_of_ranging_measurements: int = field(kw_only=True, default=0)
    dl_tdoa_measurements: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.DL_TDOA
        self.mac_address_indicator = MacAddressIndicator.EXTENDED_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacDlTDoASessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.DL_TDOA or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['no_of_ranging_measurements'] = span[0]
        span = span[1:]
        fields['dl_tdoa_measurements'] = list(span)
        span = bytes()
        return ExtendedMacDlTDoASessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.no_of_ranging_measurements > 255:
            print(f"Invalid value for field ExtendedMacDlTDoASessionInfoNtf::no_of_ranging_measurements: {self.no_of_ranging_measurements} > 255; the value will be truncated")
            self.no_of_ranging_measurements &= 255
        _span.append((self.no_of_ranging_measurements << 0))
        _span.extend(self.dl_tdoa_measurements)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.dl_tdoa_measurements) * 1 + 1

@dataclass
class ShortMacOwrAoaSessionInfoNtf(SessionInfoNtf):
    owr_aoa_ranging_measurements: List[ShortAddressOwrAoaRangingMeasurement] = field(kw_only=True, default_factory=list)
    vendor_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.OWR_AOA
        self.mac_address_indicator = MacAddressIndicator.SHORT_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacOwrAoaSessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.OWR_AOA or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        owr_aoa_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < owr_aoa_ranging_measurements_count * 13:
            raise Exception('Invalid packet size')
        owr_aoa_ranging_measurements = []
        for n in range(owr_aoa_ranging_measurements_count):
            owr_aoa_ranging_measurements.append(ShortAddressOwrAoaRangingMeasurement.parse_all(span[n * 13:(n + 1) * 13]))
        fields['owr_aoa_ranging_measurements'] = owr_aoa_ranging_measurements
        span = span[owr_aoa_ranging_measurements_count * 13:]
        fields['vendor_data'] = list(span)
        span = bytes()
        return ShortMacOwrAoaSessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.owr_aoa_ranging_measurements) > 255:
            print(f"Invalid length for field ShortMacOwrAoaSessionInfoNtf::owr_aoa_ranging_measurements:  {len(self.owr_aoa_ranging_measurements)} > 255; the array will be truncated")
            del self.owr_aoa_ranging_measurements[255:]
        _span.append((len(self.owr_aoa_ranging_measurements) << 0))
        for _elt in self.owr_aoa_ranging_measurements:
            _span.extend(_elt.serialize())
        _span.extend(self.vendor_data)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1 + (
        sum([elt.size for elt in self.owr_aoa_ranging_measurements]) +
            len(self.vendor_data) * 1
        )

@dataclass
class ExtendedMacOwrAoaSessionInfoNtf(SessionInfoNtf):
    owr_aoa_ranging_measurements: List[ExtendedAddressOwrAoaRangingMeasurement] = field(kw_only=True, default_factory=list)
    vendor_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.OWR_AOA
        self.mac_address_indicator = MacAddressIndicator.EXTENDED_ADDRESS
        self.opcode = 0
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacOwrAoaSessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.OWR_AOA or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['opcode'] != 0x0 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        owr_aoa_ranging_measurements_count = span[0]
        span = span[1:]
        if len(span) < owr_aoa_ranging_measurements_count * 19:
            raise Exception('Invalid packet size')
        owr_aoa_ranging_measurements = []
        for n in range(owr_aoa_ranging_measurements_count):
            owr_aoa_ranging_measurements.append(ExtendedAddressOwrAoaRangingMeasurement.parse_all(span[n * 19:(n + 1) * 19]))
        fields['owr_aoa_ranging_measurements'] = owr_aoa_ranging_measurements
        span = span[owr_aoa_ranging_measurements_count * 19:]
        fields['vendor_data'] = list(span)
        span = bytes()
        return ExtendedMacOwrAoaSessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.owr_aoa_ranging_measurements) > 255:
            print(f"Invalid length for field ExtendedMacOwrAoaSessionInfoNtf::owr_aoa_ranging_measurements:  {len(self.owr_aoa_ranging_measurements)} > 255; the array will be truncated")
            del self.owr_aoa_ranging_measurements[255:]
        _span.append((len(self.owr_aoa_ranging_measurements) << 0))
        for _elt in self.owr_aoa_ranging_measurements:
            _span.extend(_elt.serialize())
        _span.extend(self.vendor_data)
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1 + (
        sum([elt.size for elt in self.owr_aoa_ranging_measurements]) +
            len(self.vendor_data) * 1
        )

@dataclass
class SessionStopCmd(SessionControlCommand):
    

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStopCmd', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return SessionStopCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionControlCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionStopRsp(SessionControlResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStopRsp', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return SessionStopRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionControlResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionGetRangingCountCmd(SessionControlCommand):
    

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetRangingCountCmd', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return SessionGetRangingCountCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionControlCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionGetRangingCountRsp(SessionControlResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.opcode = 3
        self.gid = GroupId.SESSION_CONTROL
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetRangingCountRsp', bytes]:
        if fields['opcode'] != 0x3 or fields['gid'] != GroupId.SESSION_CONTROL or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        value_ = int.from_bytes(span[1:5], byteorder='little')
        fields['count'] = value_
        span = span[5:]
        return SessionGetRangingCountRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.count > 4294967295:
            print(f"Invalid value for field SessionGetRangingCountRsp::count: {self.count} > 4294967295; the value will be truncated")
            self.count &= 4294967295
        _span.extend(int.to_bytes((self.count << 0), length=4, byteorder='little'))
        return SessionControlResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class AndroidGetPowerStatsCmd(AndroidCommand):
    

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsCmd', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        return AndroidGetPowerStatsCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return AndroidCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class PowerStats(Packet):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)
    idle_time_ms: int = field(kw_only=True, default=0)
    tx_time_ms: int = field(kw_only=True, default=0)
    rx_time_ms: int = field(kw_only=True, default=0)
    total_wake_count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['PowerStats', bytes]:
        fields = {'payload': None}
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
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

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.idle_time_ms > 4294967295:
            print(f"Invalid value for field PowerStats::idle_time_ms: {self.idle_time_ms} > 4294967295; the value will be truncated")
            self.idle_time_ms &= 4294967295
        _span.extend(int.to_bytes((self.idle_time_ms << 0), length=4, byteorder='little'))
        if self.tx_time_ms > 4294967295:
            print(f"Invalid value for field PowerStats::tx_time_ms: {self.tx_time_ms} > 4294967295; the value will be truncated")
            self.tx_time_ms &= 4294967295
        _span.extend(int.to_bytes((self.tx_time_ms << 0), length=4, byteorder='little'))
        if self.rx_time_ms > 4294967295:
            print(f"Invalid value for field PowerStats::rx_time_ms: {self.rx_time_ms} > 4294967295; the value will be truncated")
            self.rx_time_ms &= 4294967295
        _span.extend(int.to_bytes((self.rx_time_ms << 0), length=4, byteorder='little'))
        if self.total_wake_count > 4294967295:
            print(f"Invalid value for field PowerStats::total_wake_count: {self.total_wake_count} > 4294967295; the value will be truncated")
            self.total_wake_count &= 4294967295
        _span.extend(int.to_bytes((self.total_wake_count << 0), length=4, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class AndroidGetPowerStatsRsp(AndroidResponse):
    stats: PowerStats = field(kw_only=True, default_factory=PowerStats)

    def __post_init__(self):
        self.opcode = 0
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsRsp', bytes]:
        if fields['opcode'] != 0x0 or fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['stats'] = PowerStats.parse_all(span[0:17])
        span = span[17:]
        return AndroidGetPowerStatsRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.stats.serialize())
        return AndroidResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 17

@dataclass
class AndroidSetCountryCodeCmd(AndroidCommand):
    country_code: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeCmd', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['country_code'] = list(span[:2])
        span = span[2:]
        return AndroidSetCountryCodeCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.country_code)
        return AndroidCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class AndroidSetCountryCodeRsp(AndroidResponse):
    status: StatusCode = field(kw_only=True, default=StatusCode.UCI_STATUS_OK)

    def __post_init__(self):
        self.opcode = 1
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeRsp', bytes]:
        if fields['opcode'] != 0x1 or fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = StatusCode.from_int(span[0])
        span = span[1:]
        return AndroidSetCountryCodeRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return AndroidResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

class FrameReportTlvType(enum.IntEnum):
    RSSI = 0x0
    AOA = 0x1
    CIR = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'FrameReportTlvType']:
        try:
            return FrameReportTlvType(v)
        except ValueError as exn:
            raise exn


@dataclass
class FrameReportTlv(Packet):
    t: FrameReportTlvType = field(kw_only=True, default=FrameReportTlvType.RSSI)
    v: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['FrameReportTlv', bytes]:
        fields = {'payload': None}
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['t'] = FrameReportTlvType.from_int(span[0])
        value_ = int.from_bytes(span[1:3], byteorder='little')
        v_size = value_
        span = span[3:]
        if len(span) < v_size:
            raise Exception('Invalid packet size')
        fields['v'] = list(span[:v_size])
        span = span[v_size:]
        return FrameReportTlv(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.t << 0))
        _span.extend(int.to_bytes(((len(self.v) * 1) << 0), length=2, byteorder='little'))
        _span.extend(self.v)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.v) * 1 + 3

@dataclass
class FrameReportTlvPacket(Packet):
    t: FrameReportTlvType = field(kw_only=True, default=FrameReportTlvType.RSSI)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['FrameReportTlvPacket', bytes]:
        fields = {'payload': None}
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['t'] = FrameReportTlvType.from_int(span[0])
        value_ = int.from_bytes(span[1:3], byteorder='little')
        _body__size = value_
        span = span[3:]
        if len(span) < _body__size:
            raise Exception('Invalid packet size')
        payload = span[:_body__size]
        span = span[_body__size:]
        fields['payload'] = payload
        try:
            return Rssi.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return Aoa.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        try:
            return Cir.parse(fields.copy(), payload)
        except Exception as exn:
            pass
        return FrameReportTlvPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.t << 0))
        _payload_size = len(payload or self.payload or [])
        if _payload_size > 65535:
            print(f"Invalid length for payload field:  {_payload_size} > 65535; the packet cannot be generated")
            raise Exception("Invalid payload length")
        _span.extend(int.to_bytes((_payload_size << 0), length=2, byteorder='little'))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class Rssi(FrameReportTlvPacket):
    rssi: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.t = FrameReportTlvType.RSSI

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['Rssi', bytes]:
        if fields['t'] != FrameReportTlvType.RSSI:
            raise Exception("Invalid constraint field values")
        fields['rssi'] = list(span)
        span = bytes()
        return Rssi(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.rssi)
        return FrameReportTlvPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.rssi) * 1

@dataclass
class AoaMeasurement(Packet):
    tdoa: int = field(kw_only=True, default=0)
    pdoa: int = field(kw_only=True, default=0)
    aoa: int = field(kw_only=True, default=0)
    fom: int = field(kw_only=True, default=0)
    t: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['AoaMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['tdoa'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['pdoa'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['aoa'] = value_
        fields['fom'] = span[6]
        fields['t'] = span[7]
        span = span[8:]
        return AoaMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.tdoa > 65535:
            print(f"Invalid value for field AoaMeasurement::tdoa: {self.tdoa} > 65535; the value will be truncated")
            self.tdoa &= 65535
        _span.extend(int.to_bytes((self.tdoa << 0), length=2, byteorder='little'))
        if self.pdoa > 65535:
            print(f"Invalid value for field AoaMeasurement::pdoa: {self.pdoa} > 65535; the value will be truncated")
            self.pdoa &= 65535
        _span.extend(int.to_bytes((self.pdoa << 0), length=2, byteorder='little'))
        if self.aoa > 65535:
            print(f"Invalid value for field AoaMeasurement::aoa: {self.aoa} > 65535; the value will be truncated")
            self.aoa &= 65535
        _span.extend(int.to_bytes((self.aoa << 0), length=2, byteorder='little'))
        if self.fom > 255:
            print(f"Invalid value for field AoaMeasurement::fom: {self.fom} > 255; the value will be truncated")
            self.fom &= 255
        _span.append((self.fom << 0))
        if self.t > 255:
            print(f"Invalid value for field AoaMeasurement::t: {self.t} > 255; the value will be truncated")
            self.t &= 255
        _span.append((self.t << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Aoa(FrameReportTlvPacket):
    aoa: List[AoaMeasurement] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.t = FrameReportTlvType.AOA

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['Aoa', bytes]:
        if fields['t'] != FrameReportTlvType.AOA:
            raise Exception("Invalid constraint field values")
        if len(span) % 8 != 0:
            raise Exception('Array size is not a multiple of the element size')
        aoa_count = int(len(span) / 8)
        aoa = []
        for n in range(aoa_count):
            aoa.append(AoaMeasurement.parse_all(span[n * 8:(n + 1) * 8]))
        fields['aoa'] = aoa
        span = bytes()
        return Aoa(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        for _elt in self.aoa:
            _span.extend(_elt.serialize())
        return FrameReportTlvPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.aoa])

@dataclass
class CirValue(Packet):
    first_path_index: int = field(kw_only=True, default=0)
    first_path_snr: int = field(kw_only=True, default=0)
    first_path_ns: int = field(kw_only=True, default=0)
    peak_path_index: int = field(kw_only=True, default=0)
    peak_path_snr: int = field(kw_only=True, default=0)
    peak_path_ns: int = field(kw_only=True, default=0)
    first_path_sample_offset: int = field(kw_only=True, default=0)
    samples_number: int = field(kw_only=True, default=0)
    sample_window: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['CirValue', bytes]:
        fields = {'payload': None}
        if len(span) < 16:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['first_path_index'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['first_path_snr'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['first_path_ns'] = value_
        value_ = int.from_bytes(span[6:8], byteorder='little')
        fields['peak_path_index'] = value_
        value_ = int.from_bytes(span[8:10], byteorder='little')
        fields['peak_path_snr'] = value_
        value_ = int.from_bytes(span[10:12], byteorder='little')
        fields['peak_path_ns'] = value_
        fields['first_path_sample_offset'] = span[12]
        fields['samples_number'] = span[13]
        value_ = int.from_bytes(span[14:16], byteorder='little')
        sample_window_size = value_
        span = span[16:]
        if len(span) < sample_window_size:
            raise Exception('Invalid packet size')
        fields['sample_window'] = list(span[:sample_window_size])
        span = span[sample_window_size:]
        return CirValue(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.first_path_index > 65535:
            print(f"Invalid value for field CirValue::first_path_index: {self.first_path_index} > 65535; the value will be truncated")
            self.first_path_index &= 65535
        _span.extend(int.to_bytes((self.first_path_index << 0), length=2, byteorder='little'))
        if self.first_path_snr > 65535:
            print(f"Invalid value for field CirValue::first_path_snr: {self.first_path_snr} > 65535; the value will be truncated")
            self.first_path_snr &= 65535
        _span.extend(int.to_bytes((self.first_path_snr << 0), length=2, byteorder='little'))
        if self.first_path_ns > 65535:
            print(f"Invalid value for field CirValue::first_path_ns: {self.first_path_ns} > 65535; the value will be truncated")
            self.first_path_ns &= 65535
        _span.extend(int.to_bytes((self.first_path_ns << 0), length=2, byteorder='little'))
        if self.peak_path_index > 65535:
            print(f"Invalid value for field CirValue::peak_path_index: {self.peak_path_index} > 65535; the value will be truncated")
            self.peak_path_index &= 65535
        _span.extend(int.to_bytes((self.peak_path_index << 0), length=2, byteorder='little'))
        if self.peak_path_snr > 65535:
            print(f"Invalid value for field CirValue::peak_path_snr: {self.peak_path_snr} > 65535; the value will be truncated")
            self.peak_path_snr &= 65535
        _span.extend(int.to_bytes((self.peak_path_snr << 0), length=2, byteorder='little'))
        if self.peak_path_ns > 65535:
            print(f"Invalid value for field CirValue::peak_path_ns: {self.peak_path_ns} > 65535; the value will be truncated")
            self.peak_path_ns &= 65535
        _span.extend(int.to_bytes((self.peak_path_ns << 0), length=2, byteorder='little'))
        if self.first_path_sample_offset > 255:
            print(f"Invalid value for field CirValue::first_path_sample_offset: {self.first_path_sample_offset} > 255; the value will be truncated")
            self.first_path_sample_offset &= 255
        _span.append((self.first_path_sample_offset << 0))
        if self.samples_number > 255:
            print(f"Invalid value for field CirValue::samples_number: {self.samples_number} > 255; the value will be truncated")
            self.samples_number &= 255
        _span.append((self.samples_number << 0))
        _span.extend(int.to_bytes(((len(self.sample_window) * 1) << 0), length=2, byteorder='little'))
        _span.extend(self.sample_window)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.sample_window) * 1 + 16

@dataclass
class Cir(FrameReportTlvPacket):
    cir_value: List[CirValue] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.t = FrameReportTlvType.CIR

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['Cir', bytes]:
        if fields['t'] != FrameReportTlvType.CIR:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        cir_value_count = span[0]
        span = span[1:]
        cir_value = []
        for n in range(cir_value_count):
            element, span = CirValue.parse(span)
            cir_value.append(element)
        fields['cir_value'] = cir_value
        return Cir(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.cir_value) > 255:
            print(f"Invalid length for field Cir::cir_value:  {len(self.cir_value)} > 255; the array will be truncated")
            del self.cir_value[255:]
        _span.append((len(self.cir_value) << 0))
        for _elt in self.cir_value:
            _span.extend(_elt.serialize())
        return FrameReportTlvPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.cir_value]) + 1

@dataclass
class FrameReport(Packet):
    uwb_msg_id: int = field(kw_only=True, default=0)
    action: int = field(kw_only=True, default=0)
    antenna_set: int = field(kw_only=True, default=0)
    frame_report_tlvs: List[FrameReportTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['FrameReport', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['uwb_msg_id'] = span[0]
        fields['action'] = span[1]
        fields['antenna_set'] = span[2]
        frame_report_tlvs_count = span[3]
        span = span[4:]
        frame_report_tlvs = []
        for n in range(frame_report_tlvs_count):
            element, span = FrameReportTlv.parse(span)
            frame_report_tlvs.append(element)
        fields['frame_report_tlvs'] = frame_report_tlvs
        return FrameReport(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.uwb_msg_id > 255:
            print(f"Invalid value for field FrameReport::uwb_msg_id: {self.uwb_msg_id} > 255; the value will be truncated")
            self.uwb_msg_id &= 255
        _span.append((self.uwb_msg_id << 0))
        if self.action > 255:
            print(f"Invalid value for field FrameReport::action: {self.action} > 255; the value will be truncated")
            self.action &= 255
        _span.append((self.action << 0))
        if self.antenna_set > 255:
            print(f"Invalid value for field FrameReport::antenna_set: {self.antenna_set} > 255; the value will be truncated")
            self.antenna_set &= 255
        _span.append((self.antenna_set << 0))
        if len(self.frame_report_tlvs) > 255:
            print(f"Invalid length for field FrameReport::frame_report_tlvs:  {len(self.frame_report_tlvs)} > 255; the array will be truncated")
            del self.frame_report_tlvs[255:]
        _span.append((len(self.frame_report_tlvs) << 0))
        for _elt in self.frame_report_tlvs:
            _span.extend(_elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.frame_report_tlvs]) + 4

@dataclass
class AndroidRangeDiagnosticsNtf(AndroidNotification):
    session_token: int = field(kw_only=True, default=0)
    sequence_number: int = field(kw_only=True, default=0)
    frame_reports: List[FrameReport] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.opcode = 2
        self.gid = GroupId.VENDOR_ANDROID
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidRangeDiagnosticsNtf', bytes]:
        if fields['opcode'] != 0x2 or fields['gid'] != GroupId.VENDOR_ANDROID or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        if len(span) < 9:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        value_ = int.from_bytes(span[4:8], byteorder='little')
        fields['sequence_number'] = value_
        frame_reports_count = span[8]
        span = span[9:]
        frame_reports = []
        for n in range(frame_reports_count):
            element, span = FrameReport.parse(span)
            frame_reports.append(element)
        fields['frame_reports'] = frame_reports
        return AndroidRangeDiagnosticsNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field AndroidRangeDiagnosticsNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.sequence_number > 4294967295:
            print(f"Invalid value for field AndroidRangeDiagnosticsNtf::sequence_number: {self.sequence_number} > 4294967295; the value will be truncated")
            self.sequence_number &= 4294967295
        _span.extend(int.to_bytes((self.sequence_number << 0), length=4, byteorder='little'))
        if len(self.frame_reports) > 255:
            print(f"Invalid length for field AndroidRangeDiagnosticsNtf::frame_reports:  {len(self.frame_reports)} > 255; the array will be truncated")
            del self.frame_reports[255:]
        _span.append((len(self.frame_reports) << 0))
        for _elt in self.frame_reports:
            _span.extend(_elt.serialize())
        return AndroidNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.frame_reports]) + 9

@dataclass
class UciVendor_9_Command(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_9
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_9_Command', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_9 or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_9_Command(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_A_Command(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_A
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Command', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_A or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_A_Command(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_B_Command(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_B
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Command', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_B or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_B_Command(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_E_Command(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_E
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Command', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_E or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_E_Command(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_F_Command(UciCommand):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_F
        self.mt = MessageType.COMMAND

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Command', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_F or fields['mt'] != MessageType.COMMAND:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_F_Command(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciCommand.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_9_Response(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_9
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_9_Response', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_9 or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_9_Response(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_A_Response(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_A
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Response', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_A or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_A_Response(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_B_Response(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_B
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Response', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_B or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_B_Response(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_E_Response(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_E
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Response', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_E or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_E_Response(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_F_Response(UciResponse):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_F
        self.mt = MessageType.RESPONSE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Response', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_F or fields['mt'] != MessageType.RESPONSE:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_F_Response(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciResponse.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_9_Notification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_9
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_9_Notification', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_9 or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_9_Notification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_A_Notification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_A
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_A_Notification', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_A or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_A_Notification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_B_Notification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_B
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_B_Notification', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_B or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_B_Notification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_E_Notification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_E
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_E_Notification', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_E or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_E_Notification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class UciVendor_F_Notification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.VENDOR_RESERVED_F
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['UciVendor_F_Notification', bytes]:
        if fields['gid'] != GroupId.VENDOR_RESERVED_F or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return UciVendor_F_Notification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class TestNotification(UciNotification):
    

    def __post_init__(self):
        self.gid = GroupId.TEST
        self.mt = MessageType.NOTIFICATION

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['TestNotification', bytes]:
        if fields['gid'] != GroupId.TEST or fields['mt'] != MessageType.NOTIFICATION:
            raise Exception("Invalid constraint field values")
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return TestNotification(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return UciNotification.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)
