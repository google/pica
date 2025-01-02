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


class MessageType(enum.IntEnum):
    DATA = 0x0
    COMMAND = 0x1
    RESPONSE = 0x2
    NOTIFICATION = 0x3

    @staticmethod
    def from_int(v: int) -> Union[int, 'MessageType']:
        try:
            return MessageType(v)
        except ValueError as exn:
            raise exn


class GroupId(enum.IntEnum):
    CORE = 0x0
    SESSION_CONFIG = 0x1
    SESSION_CONTROL = 0x2
    DATA_CONTROL = 0x3
    VENDOR_RESERVED_9 = 0x9
    VENDOR_RESERVED_A = 0xa
    VENDOR_RESERVED_B = 0xb
    VENDOR_ANDROID = 0xc
    TEST = 0xd
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


class CoreOpcodeId(enum.IntEnum):
    DEVICE_RESET = 0x0
    DEVICE_STATUS = 0x1
    GET_DEVICE_INFO = 0x2
    GET_CAPS_INFO = 0x3
    SET_CONFIG = 0x4
    GET_CONFIG = 0x5
    GENERIC_ERROR = 0x7
    QUERY_UWBS_TIMESTAMP = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'CoreOpcodeId']:
        try:
            return CoreOpcodeId(v)
        except ValueError as exn:
            raise exn


class SessionConfigOpcodeId(enum.IntEnum):
    INIT = 0x0
    DEINIT = 0x1
    STATUS = 0x2
    SET_APP_CONFIG = 0x3
    GET_APP_CONFIG = 0x4
    GET_COUNT = 0x5
    GET_STATE = 0x6
    UPDATE_CONTROLLER_MULTICAST_LIST = 0x7
    UPDATE_DT_ANCHOR_RANGING_ROUNDS = 0x8
    UPDATE_DT_TAG_RANGING_ROUNDS = 0x9
    QUERY_DATA_SIZE_IN_RANGING = 0xb

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionConfigOpcodeId']:
        try:
            return SessionConfigOpcodeId(v)
        except ValueError as exn:
            raise exn


class SessionControlOpcodeId(enum.IntEnum):
    START = 0x0
    STOP = 0x1
    GET_RANGING_COUNT = 0x3
    DATA_CREDIT = 0x4
    DATA_TRANSFER_STATUS = 0x5

    @staticmethod
    def from_int(v: int) -> Union[int, 'SessionControlOpcodeId']:
        try:
            return SessionControlOpcodeId(v)
        except ValueError as exn:
            raise exn


class AndroidOpcodeId(enum.IntEnum):
    GET_POWER_STATS = 0x0
    SET_COUNTRY_CODE = 0x1
    FIRA_RANGE_DIAGNOSTICS = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'AndroidOpcodeId']:
        try:
            return AndroidOpcodeId(v)
        except ValueError as exn:
            raise exn


class Status(enum.IntEnum):
    OK = 0x0
    REJECTED = 0x1
    FAILED = 0x2
    SYNTAX_ERROR = 0x3
    INVALID_PARAM = 0x4
    INVALID_RANGE = 0x5
    INVALID_MESSAGE_SIZE = 0x6
    UNKNOWN_GID = 0x7
    UNKNOWN_OID = 0x8
    READ_ONLY = 0x9
    UCI_MESSAGE_RETRY = 0xa
    UNKNOWN = 0xb
    NOT_APPLICABLE = 0xc
    ERROR_SESSION_NOT_EXIST = 0x11
    ERROR_SESSION_DUPLICATE = 0x12
    ERROR_SESSION_ACTIVE = 0x13
    ERROR_MAX_SESSIONS_EXCEEDED = 0x14
    ERROR_SESSION_NOT_CONFIGURED = 0x15
    ERROR_ACTIVE_SESSIONS_ONGOING = 0x16
    ERROR_MULTICAST_LIST_FULL = 0x17
    ERROR_UWB_INITIATION_TIME_TOO_OLD = 0x1a
    OK_NEGATIVE_DISTANCE_REPORT = 0x1b
    RANGING_TX_FAILED = 0x20
    RANGING_RX_TIMEOUT = 0x21
    RANGING_RX_PHY_DEC_FAILED = 0x22
    RANGING_RX_PHY_TOA_FAILED = 0x23
    RANGING_RX_PHY_STS_FAILED = 0x24
    RANGING_RX_MAC_DEC_FAILED = 0x25
    RANGING_RX_MAC_IE_DEC_FAILED = 0x26
    RANGING_RX_MAC_IE_MISSING = 0x27
    ERROR_ROUND_INDEX_NOT_ACTIVATED = 0x28
    ERROR_NUMBER_OF_ACTIVE_RANGING_ROUNDS_EXCEEDED = 0x29
    ERROR_DL_TDOA_DEVICE_ADDRESS_NOT_MATCHING_IN_REPLY_TIME_LIST = 0x2a

    @staticmethod
    def from_int(v: int) -> Union[int, 'Status']:
        try:
            return Status(v)
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


class MulticastUpdateStatus(enum.IntEnum):
    OK_MULTICAST_LIST_UPDATE = 0x0
    ERROR_MULTICAST_LIST_FULL = 0x1
    ERROR_KEY_FETCH_FAIL = 0x2
    ERROR_SUB_SESSION_ID_NOT_FOUND = 0x3
    ERROR_SUB_SESSION_KEY_NOT_FOUND = 0x4
    ERROR_SUB_SESSION_KEY_NOT_APPLICABLE = 0x5
    ERROR_SESSION_KEY_NOT_FOUND = 0x6
    ERROR_ADDRESS_NOT_FOUND = 0x7
    ERROR_ADDRESS_ALREADY_PRESENT = 0x8

    @staticmethod
    def from_int(v: int) -> Union[int, 'MulticastUpdateStatus']:
        try:
            return MulticastUpdateStatus(v)
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

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ControlPacket', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['gid'] = GroupId.from_int((span[0] >> 0) & 0xf)
        fields['mt'] = MessageType.from_int((span[0] >> 5) & 0x7)
        span = span[1:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = CorePacket.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionConfigPacket.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionControlPacket.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = AndroidPacket.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
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
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 1

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
            child, remainder = DataMessageSnd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = DataMessageRcv.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
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
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[4])
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
class CorePacket(ControlPacket):
    oid: CoreOpcodeId = field(kw_only=True, default=CoreOpcodeId.DEVICE_RESET)

    def __post_init__(self):
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CorePacket', bytes]:
        if fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['oid'] = CoreOpcodeId.from_int((span[0] >> 0) & 0x3f)
        value_ = int.from_bytes(span[1:3], byteorder='little')
        span = span[3:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = CoreDeviceResetCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreDeviceResetRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreDeviceStatusNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetDeviceInfoCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetDeviceInfoRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetCapsInfoCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetCapsInfoRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreSetConfigCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreSetConfigRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetConfigCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGetConfigRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreGenericErrorNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreQueryTimeStampCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = CoreQueryTimeStampRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        return CorePacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.oid << 0))
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class SessionConfigPacket(ControlPacket):
    oid: SessionConfigOpcodeId = field(kw_only=True, default=SessionConfigOpcodeId.INIT)

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionConfigPacket', bytes]:
        if fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['oid'] = SessionConfigOpcodeId.from_int((span[0] >> 0) & 0x3f)
        value_ = int.from_bytes(span[1:3], byteorder='little')
        span = span[3:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = SessionInitCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionInitRsp_V2.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionInitRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionDeinitCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionDeinitRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionStatusNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionSetAppConfigCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionSetAppConfigRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetAppConfigCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetAppConfigRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetCountCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetCountRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetStateCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetStateRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateDtAnchorRangingRoundsCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateDtAnchorRangingRoundsRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateDtTagRangingRoundsCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateDtTagRangingRoundsRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateControllerMulticastListCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateControllerMulticastListRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionUpdateControllerMulticastListNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionQueryMaxDataSizeInRangingCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionQueryMaxDataSizeInRangingRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        return SessionConfigPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.oid << 0))
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class SessionControlPacket(ControlPacket):
    oid: SessionControlOpcodeId = field(kw_only=True, default=SessionControlOpcodeId.START)

    def __post_init__(self):
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionControlPacket', bytes]:
        if fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['oid'] = SessionControlOpcodeId.from_int((span[0] >> 0) & 0x3f)
        value_ = int.from_bytes(span[1:3], byteorder='little')
        span = span[3:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = SessionDataCreditNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionDataTransferStatusNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionStartCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionStartRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionStopCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionStopRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetRangingCountCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = SessionGetRangingCountRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        return SessionControlPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.oid << 0))
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class AndroidPacket(ControlPacket):
    oid: AndroidOpcodeId = field(kw_only=True, default=AndroidOpcodeId.GET_POWER_STATS)

    def __post_init__(self):
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidPacket', bytes]:
        if fields['gid'] != GroupId.VENDOR_ANDROID:
            raise Exception("Invalid constraint field values")
        if len(span) < 3:
            raise Exception('Invalid packet size')
        fields['oid'] = AndroidOpcodeId.from_int((span[0] >> 0) & 0x3f)
        value_ = int.from_bytes(span[1:3], byteorder='little')
        span = span[3:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = AndroidGetPowerStatsCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = AndroidGetPowerStatsRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = AndroidSetCountryCodeCmd.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = AndroidSetCountryCodeRsp.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = AndroidRangeDiagnosticsNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        return AndroidPacket(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.oid << 0))
        _span.extend([0] * 2)
        _span.extend(payload or self.payload or [])
        return ControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class CoreDeviceResetCmd(CorePacket):
    reset_config: ResetConfig = field(kw_only=True, default=ResetConfig.UWBS_RESET)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.DEVICE_RESET
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreDeviceResetCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.DEVICE_RESET or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['reset_config'] = ResetConfig.from_int(span[0])
        span = span[1:]
        return CoreDeviceResetCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.reset_config << 0))
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class CoreDeviceResetRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.DEVICE_RESET
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreDeviceResetRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.DEVICE_RESET or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return CoreDeviceResetRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class CoreDeviceStatusNtf(CorePacket):
    device_state: DeviceState = field(kw_only=True, default=DeviceState.DEVICE_STATE_READY)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = CoreOpcodeId.DEVICE_STATUS
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreDeviceStatusNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != CoreOpcodeId.DEVICE_STATUS or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['device_state'] = DeviceState.from_int(span[0])
        span = span[1:]
        return CoreDeviceStatusNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.device_state << 0))
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class CoreGetDeviceInfoCmd(CorePacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.GET_DEVICE_INFO
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetDeviceInfoCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.GET_DEVICE_INFO or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        return CoreGetDeviceInfoCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class CoreGetDeviceInfoRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)
    uci_version: int = field(kw_only=True, default=0)
    mac_version: int = field(kw_only=True, default=0)
    phy_version: int = field(kw_only=True, default=0)
    uci_test_version: int = field(kw_only=True, default=0)
    vendor_spec_info: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.GET_DEVICE_INFO
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetDeviceInfoRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.GET_DEVICE_INFO or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 10:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return CoreGetDeviceInfoRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.uci_version > 65535:
            print(f"Invalid value for field CoreGetDeviceInfoRsp::uci_version: {self.uci_version} > 65535; the value will be truncated")
            self.uci_version &= 65535
        _span.extend(int.to_bytes((self.uci_version << 0), length=2, byteorder='little'))
        if self.mac_version > 65535:
            print(f"Invalid value for field CoreGetDeviceInfoRsp::mac_version: {self.mac_version} > 65535; the value will be truncated")
            self.mac_version &= 65535
        _span.extend(int.to_bytes((self.mac_version << 0), length=2, byteorder='little'))
        if self.phy_version > 65535:
            print(f"Invalid value for field CoreGetDeviceInfoRsp::phy_version: {self.phy_version} > 65535; the value will be truncated")
            self.phy_version &= 65535
        _span.extend(int.to_bytes((self.phy_version << 0), length=2, byteorder='little'))
        if self.uci_test_version > 65535:
            print(f"Invalid value for field CoreGetDeviceInfoRsp::uci_test_version: {self.uci_test_version} > 65535; the value will be truncated")
            self.uci_test_version &= 65535
        _span.extend(int.to_bytes((self.uci_test_version << 0), length=2, byteorder='little'))
        if len(self.vendor_spec_info) > 255:
            print(f"Invalid length for field CoreGetDeviceInfoRsp::vendor_spec_info:  {len(self.vendor_spec_info)} > 255; the array will be truncated")
            del self.vendor_spec_info[255:]
        _span.append((len(self.vendor_spec_info) << 0))
        _span.extend(self.vendor_spec_info)
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.vendor_spec_info) * 1 + 10

@dataclass
class CoreGetCapsInfoCmd(CorePacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.GET_CAPS_INFO
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetCapsInfoCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.GET_CAPS_INFO or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        return CoreGetCapsInfoCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CorePacket.serialize(self, payload = bytes(_span))

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
class CoreGetCapsInfoRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)
    tlvs: List[CapTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.GET_CAPS_INFO
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetCapsInfoRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.GET_CAPS_INFO or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        tlvs_count = span[1]
        span = span[2:]
        tlvs = []
        for n in range(tlvs_count):
            element, span = CapTlv.parse(span)
            tlvs.append(element)
        fields['tlvs'] = tlvs
        return CoreGetCapsInfoRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.tlvs) > 255:
            print(f"Invalid length for field CoreGetCapsInfoRsp::tlvs:  {len(self.tlvs)} > 255; the array will be truncated")
            del self.tlvs[255:]
        _span.append((len(self.tlvs) << 0))
        for _elt in self.tlvs:
            _span.extend(_elt.serialize())
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 2

class ConfigParameterId(enum.IntEnum):
    DEVICE_STATE = 0x0
    LOW_POWER_MODE = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'ConfigParameterId']:
        try:
            return ConfigParameterId(v)
        except ValueError as exn:
            return v


@dataclass
class ConfigParameter(Packet):
    id: ConfigParameterId = field(kw_only=True, default=ConfigParameterId.DEVICE_STATE)
    value: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ConfigParameter', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['id'] = ConfigParameterId.from_int(span[0])
        value_size = span[1]
        span = span[2:]
        if len(span) < value_size:
            raise Exception('Invalid packet size')
        fields['value'] = list(span[:value_size])
        span = span[value_size:]
        return ConfigParameter(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.id << 0))
        _span.append(((len(self.value) * 1) << 0))
        _span.extend(self.value)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.value) * 1 + 2

@dataclass
class CoreSetConfigCmd(CorePacket):
    parameters: List[ConfigParameter] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.SET_CONFIG
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreSetConfigCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.SET_CONFIG or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        parameters_count = span[0]
        span = span[1:]
        parameters = []
        for n in range(parameters_count):
            element, span = ConfigParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return CoreSetConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.parameters) > 255:
            print(f"Invalid length for field CoreSetConfigCmd::parameters:  {len(self.parameters)} > 255; the array will be truncated")
            del self.parameters[255:]
        _span.append((len(self.parameters) << 0))
        for _elt in self.parameters:
            _span.extend(_elt.serialize())
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.parameters]) + 1

@dataclass
class ConfigParameterStatus(Packet):
    id: ConfigParameterId = field(kw_only=True, default=ConfigParameterId.DEVICE_STATE)
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ConfigParameterStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['id'] = ConfigParameterId.from_int(span[0])
        fields['status'] = Status.from_int(span[1])
        span = span[2:]
        return ConfigParameterStatus(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.id << 0))
        _span.append((self.status << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class CoreSetConfigRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)
    parameters: List[ConfigParameterStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.SET_CONFIG
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreSetConfigRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.SET_CONFIG or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        parameters_count = span[1]
        span = span[2:]
        if len(span) < parameters_count * 2:
            raise Exception('Invalid packet size')
        parameters = []
        for n in range(parameters_count):
            parameters.append(ConfigParameterStatus.parse_all(span[n * 2:(n + 1) * 2]))
        fields['parameters'] = parameters
        span = span[parameters_count * 2:]
        return CoreSetConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.parameters) > 255:
            print(f"Invalid length for field CoreSetConfigRsp::parameters:  {len(self.parameters)} > 255; the array will be truncated")
            del self.parameters[255:]
        _span.append((len(self.parameters) << 0))
        for _elt in self.parameters:
            _span.extend(_elt.serialize())
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.parameters]) + 2

@dataclass
class CoreGetConfigCmd(CorePacket):
    parameter_ids: List[ConfigParameterId] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.GET_CONFIG
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetConfigCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.GET_CONFIG or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        parameter_ids_count = span[0]
        span = span[1:]
        if len(span) < parameter_ids_count:
            raise Exception('Invalid packet size')
        parameter_ids = []
        for n in range(parameter_ids_count):
            parameter_ids.append(ConfigParameterId(int.from_bytes(span[n:n + 1], byteorder='little')))
        fields['parameter_ids'] = parameter_ids
        span = span[parameter_ids_count:]
        return CoreGetConfigCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.parameter_ids) > 255:
            print(f"Invalid length for field CoreGetConfigCmd::parameter_ids:  {len(self.parameter_ids)} > 255; the array will be truncated")
            del self.parameter_ids[255:]
        _span.append((len(self.parameter_ids) << 0))
        for _elt in self.parameter_ids:
            _span.append(_elt)
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.parameter_ids) * 8 + 1

@dataclass
class CoreGetConfigRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)
    parameters: List[ConfigParameter] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.GET_CONFIG
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGetConfigRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.GET_CONFIG or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        parameters_count = span[1]
        span = span[2:]
        parameters = []
        for n in range(parameters_count):
            element, span = ConfigParameter.parse(span)
            parameters.append(element)
        fields['parameters'] = parameters
        return CoreGetConfigRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if len(self.parameters) > 255:
            print(f"Invalid length for field CoreGetConfigRsp::parameters:  {len(self.parameters)} > 255; the array will be truncated")
            del self.parameters[255:]
        _span.append((len(self.parameters) << 0))
        for _elt in self.parameters:
            _span.extend(_elt.serialize())
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.parameters]) + 2

@dataclass
class CoreGenericErrorNtf(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = CoreOpcodeId.GENERIC_ERROR
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreGenericErrorNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != CoreOpcodeId.GENERIC_ERROR or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return CoreGenericErrorNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class CoreQueryTimeStampCmd(CorePacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = CoreOpcodeId.QUERY_UWBS_TIMESTAMP
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreQueryTimeStampCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != CoreOpcodeId.QUERY_UWBS_TIMESTAMP or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        return CoreQueryTimeStampCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class CoreQueryTimeStampRsp(CorePacket):
    status: Status = field(kw_only=True, default=Status.OK)
    timeStamp: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = CoreOpcodeId.QUERY_UWBS_TIMESTAMP
        self.gid = GroupId.CORE

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['CoreQueryTimeStampRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != CoreOpcodeId.QUERY_UWBS_TIMESTAMP or fields['gid'] != GroupId.CORE:
            raise Exception("Invalid constraint field values")
        if len(span) < 9:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return CorePacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 9

@dataclass
class SessionInitCmd(SessionConfigPacket):
    session_id: int = field(kw_only=True, default=0)
    session_type: SessionType = field(kw_only=True, default=SessionType.FIRA_RANGING_SESSION)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.INIT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.INIT or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class SessionInitRsp_V2(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    session_handle: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.INIT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitRsp_V2', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.INIT or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class SessionInitRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.INIT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInitRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.INIT or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return SessionInitRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionDeinitCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.DEINIT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.DEINIT or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionDeinitRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.DEINIT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDeinitRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.DEINIT or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return SessionDeinitRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionStatusNtf(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    session_state: SessionState = field(kw_only=True, default=SessionState.SESSION_STATE_INIT)
    reason_code: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionConfigOpcodeId.STATUS
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStatusNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionConfigOpcodeId.STATUS or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

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
class SessionSetAppConfigCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    tlvs: List[AppConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.SET_APP_CONFIG
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.SET_APP_CONFIG or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 5

@dataclass
class AppConfigStatus(Packet):
    cfg_id: AppConfigTlvType = field(kw_only=True, default=AppConfigTlvType.DEVICE_TYPE)
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['AppConfigStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['cfg_id'] = AppConfigTlvType.from_int(span[0])
        fields['status'] = Status.from_int(span[1])
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
class SessionSetAppConfigRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    cfg_status: List[AppConfigStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.SET_APP_CONFIG
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionSetAppConfigRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.SET_APP_CONFIG or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.cfg_status]) + 2

@dataclass
class SessionGetAppConfigCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    app_cfg: List[AppConfigTlvType] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.GET_APP_CONFIG
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.GET_APP_CONFIG or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.app_cfg) * 8 + 5

@dataclass
class SessionGetAppConfigRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    tlvs: List[AppConfigTlv] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.GET_APP_CONFIG
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetAppConfigRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.GET_APP_CONFIG or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.tlvs]) + 2

@dataclass
class SessionGetCountCmd(SessionConfigPacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.GET_COUNT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.GET_COUNT or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        return SessionGetCountCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionGetCountRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    session_count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.GET_COUNT
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetCountRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.GET_COUNT or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class SessionGetStateCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.GET_STATE
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.GET_STATE or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionGetStateRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    session_state: SessionState = field(kw_only=True, default=SessionState.SESSION_STATE_INIT)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.GET_STATE
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetStateRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.GET_STATE or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        fields['session_state'] = SessionState.from_int(span[1])
        span = span[2:]
        return SessionGetStateRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        _span.append((self.session_state << 0))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class SessionUpdateDtAnchorRangingRoundsCmd(SessionConfigPacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.UPDATE_DT_ANCHOR_RANGING_ROUNDS
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtAnchorRangingRoundsCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.UPDATE_DT_ANCHOR_RANGING_ROUNDS or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        return SessionUpdateDtAnchorRangingRoundsCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionUpdateDtAnchorRangingRoundsRsp(SessionConfigPacket):
    

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.UPDATE_DT_ANCHOR_RANGING_ROUNDS
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtAnchorRangingRoundsRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.UPDATE_DT_ANCHOR_RANGING_ROUNDS or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        return SessionUpdateDtAnchorRangingRoundsRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class SessionUpdateDtTagRangingRoundsCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    ranging_round_indexes: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.UPDATE_DT_TAG_RANGING_ROUNDS
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtTagRangingRoundsCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.UPDATE_DT_TAG_RANGING_ROUNDS or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.ranging_round_indexes) * 1 + 5

@dataclass
class SessionUpdateDtTagRangingRoundsRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    ranging_round_indexes: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.UPDATE_DT_TAG_RANGING_ROUNDS
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateDtTagRangingRoundsRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.UPDATE_DT_TAG_RANGING_ROUNDS or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

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
class SessionUpdateControllerMulticastListCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    action: UpdateMulticastListAction = field(kw_only=True, default=UpdateMulticastListAction.ADD_CONTROLEE)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST or fields['gid'] != GroupId.SESSION_CONFIG:
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
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload) + 5

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
class SessionUpdateControllerMulticastListRsp(SessionConfigPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return SessionUpdateControllerMulticastListRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class ControleeStatus(Packet):
    mac_address: bytearray = field(kw_only=True, default_factory=bytearray)
    status: MulticastUpdateStatus = field(kw_only=True, default=MulticastUpdateStatus.OK_MULTICAST_LIST_UPDATE)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ControleeStatus', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['mac_address'] = list(span[:2])
        span = span[2:]
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = MulticastUpdateStatus.from_int(span[0])
        span = span[1:]
        return ControleeStatus(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.mac_address)
        _span.append((self.status << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 3

@dataclass
class SessionUpdateControllerMulticastListNtf(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    controlee_status: List[ControleeStatus] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionUpdateControllerMulticastListNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionConfigOpcodeId.UPDATE_CONTROLLER_MULTICAST_LIST or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        controlee_status_count = span[4]
        span = span[5:]
        if len(span) < controlee_status_count * 3:
            raise Exception('Invalid packet size')
        controlee_status = []
        for n in range(controlee_status_count):
            controlee_status.append(ControleeStatus.parse_all(span[n * 3:(n + 1) * 3]))
        fields['controlee_status'] = controlee_status
        span = span[controlee_status_count * 3:]
        return SessionUpdateControllerMulticastListNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionUpdateControllerMulticastListNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if len(self.controlee_status) > 255:
            print(f"Invalid length for field SessionUpdateControllerMulticastListNtf::controlee_status:  {len(self.controlee_status)} > 255; the array will be truncated")
            del self.controlee_status[255:]
        _span.append((len(self.controlee_status) << 0))
        for _elt in self.controlee_status:
            _span.extend(_elt.serialize())
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.controlee_status]) + 5

@dataclass
class SessionDataCreditNtf(SessionControlPacket):
    session_token: int = field(kw_only=True, default=0)
    credit_availability: CreditAvailability = field(kw_only=True, default=CreditAvailability.CREDIT_NOT_AVAILABLE)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.DATA_CREDIT
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDataCreditNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.DATA_CREDIT or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['credit_availability'] = CreditAvailability.from_int(span[4])
        span = span[5:]
        return SessionDataCreditNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionDataCreditNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        _span.append((self.credit_availability << 0))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class SessionDataTransferStatusNtf(SessionControlPacket):
    session_token: int = field(kw_only=True, default=0)
    uci_sequence_number: int = field(kw_only=True, default=0)
    status: DataTransferNtfStatusCode = field(kw_only=True, default=DataTransferNtfStatusCode.UCI_DATA_TRANSFER_STATUS_REPETITION_OK)
    tx_count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.DATA_TRANSFER_STATUS
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionDataTransferStatusNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.DATA_TRANSFER_STATUS or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 7:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        fields['uci_sequence_number'] = span[4]
        fields['status'] = DataTransferNtfStatusCode.from_int(span[5])
        fields['tx_count'] = span[6]
        span = span[7:]
        return SessionDataTransferStatusNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionDataTransferStatusNtf::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.uci_sequence_number > 255:
            print(f"Invalid value for field SessionDataTransferStatusNtf::uci_sequence_number: {self.uci_sequence_number} > 255; the value will be truncated")
            self.uci_sequence_number &= 255
        _span.append((self.uci_sequence_number << 0))
        _span.append((self.status << 0))
        if self.tx_count > 255:
            print(f"Invalid value for field SessionDataTransferStatusNtf::tx_count: {self.tx_count} > 255; the value will be truncated")
            self.tx_count &= 255
        _span.append((self.tx_count << 0))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 7

@dataclass
class SessionQueryMaxDataSizeInRangingCmd(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionConfigOpcodeId.QUERY_DATA_SIZE_IN_RANGING
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionQueryMaxDataSizeInRangingCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionConfigOpcodeId.QUERY_DATA_SIZE_IN_RANGING or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        span = span[4:]
        return SessionQueryMaxDataSizeInRangingCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionQueryMaxDataSizeInRangingCmd::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionQueryMaxDataSizeInRangingRsp(SessionConfigPacket):
    session_token: int = field(kw_only=True, default=0)
    max_data_size: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionConfigOpcodeId.QUERY_DATA_SIZE_IN_RANGING
        self.gid = GroupId.SESSION_CONFIG

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionQueryMaxDataSizeInRangingRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionConfigOpcodeId.QUERY_DATA_SIZE_IN_RANGING or fields['gid'] != GroupId.SESSION_CONFIG:
            raise Exception("Invalid constraint field values")
        if len(span) < 6:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_token'] = value_
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['max_data_size'] = value_
        span = span[6:]
        return SessionQueryMaxDataSizeInRangingRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_token > 4294967295:
            print(f"Invalid value for field SessionQueryMaxDataSizeInRangingRsp::session_token: {self.session_token} > 4294967295; the value will be truncated")
            self.session_token &= 4294967295
        _span.extend(int.to_bytes((self.session_token << 0), length=4, byteorder='little'))
        if self.max_data_size > 65535:
            print(f"Invalid value for field SessionQueryMaxDataSizeInRangingRsp::max_data_size: {self.max_data_size} > 65535; the value will be truncated")
            self.max_data_size &= 65535
        _span.extend(int.to_bytes((self.max_data_size << 0), length=2, byteorder='little'))
        return SessionConfigPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 6

@dataclass
class SessionStartCmd(SessionControlPacket):
    session_id: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStartCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        return SessionStartCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_id > 4294967295:
            print(f"Invalid value for field SessionStartCmd::session_id: {self.session_id} > 4294967295; the value will be truncated")
            self.session_id &= 4294967295
        _span.extend(int.to_bytes((self.session_id << 0), length=4, byteorder='little'))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionStartRsp(SessionControlPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStartRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return SessionStartRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class ShortAddressTwoWayRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[2])
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
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[8])
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
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[2])
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
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[8])
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

@dataclass
class Wgs84Location(Packet):
    data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Wgs84Location', bytes]:
        fields = {'payload': None}
        if len(span) < 12:
            raise Exception('Invalid packet size')
        fields['data'] = list(span[:12])
        span = span[12:]
        return Wgs84Location(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.data)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 12

@dataclass
class RelativeLocation(Packet):
    x: int = field(kw_only=True, default=0)
    y: int = field(kw_only=True, default=0)
    z: int = field(kw_only=True, default=0)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['RelativeLocation', bytes]:
        fields = {'payload': None}
        if len(span) < 10:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:7], byteorder='little')
        fields['x'] = (value_ >> 0) & 0xfffffff
        fields['y'] = (value_ >> 28) & 0xfffffff
        value_ = int.from_bytes(span[7:10], byteorder='little')
        fields['z'] = value_
        span = span[10:]
        return RelativeLocation(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.x > 268435455:
            print(f"Invalid value for field RelativeLocation::x: {self.x} > 268435455; the value will be truncated")
            self.x &= 268435455
        if self.y > 268435455:
            print(f"Invalid value for field RelativeLocation::y: {self.y} > 268435455; the value will be truncated")
            self.y &= 268435455
        _value = (
            (self.x << 0) |
            (self.y << 28)
        )
        _span.extend(int.to_bytes(_value, length=7, byteorder='little'))
        if self.z > 16777215:
            print(f"Invalid value for field RelativeLocation::z: {self.z} > 16777215; the value will be truncated")
            self.z &= 16777215
        _span.extend(int.to_bytes((self.z << 0), length=3, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 10

@dataclass
class DlTdoaRangingMeasurement(Packet):
    status: Status = field(kw_only=True, default=Status.OK)
    message_type: int = field(kw_only=True, default=0)
    tx_timestamp_type: int = field(kw_only=True, default=0)
    block_index: int = field(kw_only=True, default=0)
    round_index: int = field(kw_only=True, default=0)
    nlos: int = field(kw_only=True, default=0)
    aoa_azimuth: int = field(kw_only=True, default=0)
    aoa_azimuth_fom: int = field(kw_only=True, default=0)
    aoa_elevation: int = field(kw_only=True, default=0)
    aoa_elevation_fom: int = field(kw_only=True, default=0)
    rssi: int = field(kw_only=True, default=0)
    tx_timestamp_40: Optional[int] = field(kw_only=True, default=None)
    tx_timestamp_64: Optional[int] = field(kw_only=True, default=None)
    rx_timestamp_40: Optional[int] = field(kw_only=True, default=None)
    rx_timestamp_64: Optional[int] = field(kw_only=True, default=None)
    anchor_cfo: int = field(kw_only=True, default=0)
    cfo: int = field(kw_only=True, default=0)
    initiator_reply_time: int = field(kw_only=True, default=0)
    responder_reply_time: int = field(kw_only=True, default=0)
    initiator_responder_tof: int = field(kw_only=True, default=0)
    wgs84_location: List[Wgs84Location] = field(kw_only=True, default_factory=list)
    relative_location: List[RelativeLocation] = field(kw_only=True, default_factory=list)
    active_ranging_rounds: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['DlTdoaRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 15:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        fields['message_type'] = span[1]
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['tx_timestamp_type'] = (value_ >> 0) & 0x1
        tx_timestamp_length = (value_ >> 1) & 0x1
        rx_timestamp_length = (value_ >> 3) & 0x1
        wgs84_location_count = (value_ >> 5) & 0x1
        relative_location_count = (value_ >> 6) & 0x1
        active_ranging_rounds_count = (value_ >> 7) & 0xf
        value_ = int.from_bytes(span[4:6], byteorder='little')
        fields['block_index'] = value_
        fields['round_index'] = span[6]
        fields['nlos'] = span[7]
        value_ = int.from_bytes(span[8:10], byteorder='little')
        fields['aoa_azimuth'] = value_
        fields['aoa_azimuth_fom'] = span[10]
        value_ = int.from_bytes(span[11:13], byteorder='little')
        fields['aoa_elevation'] = value_
        fields['aoa_elevation_fom'] = span[13]
        fields['rssi'] = span[14]
        span = span[15:]
        
        if tx_timestamp_length == 0:
            if len(span) < 5:
                raise Exception('Invalid packet size')
            fields['tx_timestamp_40'] = int.from_bytes(span[:5], byteorder='little')
            span = span[5:]
        
        
        if tx_timestamp_length == 1:
            if len(span) < 8:
                raise Exception('Invalid packet size')
            fields['tx_timestamp_64'] = int.from_bytes(span[:8], byteorder='little')
            span = span[8:]
        
        
        if rx_timestamp_length == 0:
            if len(span) < 5:
                raise Exception('Invalid packet size')
            fields['rx_timestamp_40'] = int.from_bytes(span[:5], byteorder='little')
            span = span[5:]
        
        
        if rx_timestamp_length == 1:
            if len(span) < 8:
                raise Exception('Invalid packet size')
            fields['rx_timestamp_64'] = int.from_bytes(span[:8], byteorder='little')
            span = span[8:]
        
        if len(span) < 14:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['anchor_cfo'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['cfo'] = value_
        value_ = int.from_bytes(span[4:8], byteorder='little')
        fields['initiator_reply_time'] = value_
        value_ = int.from_bytes(span[8:12], byteorder='little')
        fields['responder_reply_time'] = value_
        value_ = int.from_bytes(span[12:14], byteorder='little')
        fields['initiator_responder_tof'] = value_
        span = span[14:]
        if len(span) < wgs84_location_count * 12:
            raise Exception('Invalid packet size')
        wgs84_location = []
        for n in range(wgs84_location_count):
            wgs84_location.append(Wgs84Location.parse_all(span[n * 12:(n + 1) * 12]))
        fields['wgs84_location'] = wgs84_location
        span = span[wgs84_location_count * 12:]
        if len(span) < relative_location_count * 10:
            raise Exception('Invalid packet size')
        relative_location = []
        for n in range(relative_location_count):
            relative_location.append(RelativeLocation.parse_all(span[n * 10:(n + 1) * 10]))
        fields['relative_location'] = relative_location
        span = span[relative_location_count * 10:]
        if len(span) < active_ranging_rounds_count:
            raise Exception('Invalid packet size')
        fields['active_ranging_rounds'] = list(span[:active_ranging_rounds_count])
        span = span[active_ranging_rounds_count:]
        return DlTdoaRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        if self.message_type > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::message_type: {self.message_type} > 255; the value will be truncated")
            self.message_type &= 255
        _span.append((self.message_type << 0))
        if self.tx_timestamp_type > 1:
            print(f"Invalid value for field DlTdoaRangingMeasurement::tx_timestamp_type: {self.tx_timestamp_type} > 1; the value will be truncated")
            self.tx_timestamp_type &= 1
        if len(self.wgs84_location) > 1:
            print(f"Invalid length for field DlTdoaRangingMeasurement::wgs84_location:  {len(self.wgs84_location)} > 1; the array will be truncated")
            del self.wgs84_location[1:]
        if len(self.relative_location) > 1:
            print(f"Invalid length for field DlTdoaRangingMeasurement::relative_location:  {len(self.relative_location)} > 1; the array will be truncated")
            del self.relative_location[1:]
        if len(self.active_ranging_rounds) > 15:
            print(f"Invalid length for field DlTdoaRangingMeasurement::active_ranging_rounds:  {len(self.active_ranging_rounds)} > 15; the array will be truncated")
            del self.active_ranging_rounds[15:]
        _value = (
            (self.tx_timestamp_type << 0) |
            ((0 if self.tx_timestamp_64 is None else 1) << 1) |
            ((0 if self.rx_timestamp_64 is None else 1) << 3) |
            (len(self.wgs84_location) << 5) |
            (len(self.relative_location) << 6) |
            (len(self.active_ranging_rounds) << 7)
        )
        _span.extend(int.to_bytes(_value, length=2, byteorder='little'))
        if self.block_index > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::block_index: {self.block_index} > 65535; the value will be truncated")
            self.block_index &= 65535
        _span.extend(int.to_bytes((self.block_index << 0), length=2, byteorder='little'))
        if self.round_index > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::round_index: {self.round_index} > 255; the value will be truncated")
            self.round_index &= 255
        _span.append((self.round_index << 0))
        if self.nlos > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::nlos: {self.nlos} > 255; the value will be truncated")
            self.nlos &= 255
        _span.append((self.nlos << 0))
        if self.aoa_azimuth > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::aoa_azimuth: {self.aoa_azimuth} > 65535; the value will be truncated")
            self.aoa_azimuth &= 65535
        _span.extend(int.to_bytes((self.aoa_azimuth << 0), length=2, byteorder='little'))
        if self.aoa_azimuth_fom > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::aoa_azimuth_fom: {self.aoa_azimuth_fom} > 255; the value will be truncated")
            self.aoa_azimuth_fom &= 255
        _span.append((self.aoa_azimuth_fom << 0))
        if self.aoa_elevation > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::aoa_elevation: {self.aoa_elevation} > 65535; the value will be truncated")
            self.aoa_elevation &= 65535
        _span.extend(int.to_bytes((self.aoa_elevation << 0), length=2, byteorder='little'))
        if self.aoa_elevation_fom > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::aoa_elevation_fom: {self.aoa_elevation_fom} > 255; the value will be truncated")
            self.aoa_elevation_fom &= 255
        _span.append((self.aoa_elevation_fom << 0))
        if self.rssi > 255:
            print(f"Invalid value for field DlTdoaRangingMeasurement::rssi: {self.rssi} > 255; the value will be truncated")
            self.rssi &= 255
        _span.append((self.rssi << 0))
        
        if self.tx_timestamp_40 is not None:
            _span.extend(int.to_bytes(self.tx_timestamp_40, length=5, byteorder='little'))
        
        
        if self.tx_timestamp_64 is not None:
            _span.extend(int.to_bytes(self.tx_timestamp_64, length=8, byteorder='little'))
        
        
        if self.rx_timestamp_40 is not None:
            _span.extend(int.to_bytes(self.rx_timestamp_40, length=5, byteorder='little'))
        
        
        if self.rx_timestamp_64 is not None:
            _span.extend(int.to_bytes(self.rx_timestamp_64, length=8, byteorder='little'))
        
        if self.anchor_cfo > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::anchor_cfo: {self.anchor_cfo} > 65535; the value will be truncated")
            self.anchor_cfo &= 65535
        _span.extend(int.to_bytes((self.anchor_cfo << 0), length=2, byteorder='little'))
        if self.cfo > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::cfo: {self.cfo} > 65535; the value will be truncated")
            self.cfo &= 65535
        _span.extend(int.to_bytes((self.cfo << 0), length=2, byteorder='little'))
        if self.initiator_reply_time > 4294967295:
            print(f"Invalid value for field DlTdoaRangingMeasurement::initiator_reply_time: {self.initiator_reply_time} > 4294967295; the value will be truncated")
            self.initiator_reply_time &= 4294967295
        _span.extend(int.to_bytes((self.initiator_reply_time << 0), length=4, byteorder='little'))
        if self.responder_reply_time > 4294967295:
            print(f"Invalid value for field DlTdoaRangingMeasurement::responder_reply_time: {self.responder_reply_time} > 4294967295; the value will be truncated")
            self.responder_reply_time &= 4294967295
        _span.extend(int.to_bytes((self.responder_reply_time << 0), length=4, byteorder='little'))
        if self.initiator_responder_tof > 65535:
            print(f"Invalid value for field DlTdoaRangingMeasurement::initiator_responder_tof: {self.initiator_responder_tof} > 65535; the value will be truncated")
            self.initiator_responder_tof &= 65535
        _span.extend(int.to_bytes((self.initiator_responder_tof << 0), length=2, byteorder='little'))
        for _elt in self.wgs84_location:
            _span.extend(_elt.serialize())
        for _elt in self.relative_location:
            _span.extend(_elt.serialize())
        _span.extend(self.active_ranging_rounds)
        return bytes(_span)

    @property
    def size(self) -> int:
        (0 if self.tx_timestamp_40 is None else 40)

@dataclass
class ShortAddressDlTdoaRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    measurement: DlTdoaRangingMeasurement = field(kw_only=True, default_factory=DlTdoaRangingMeasurement)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ShortAddressDlTdoaRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['mac_address'] = value_
        span = span[2:]
        measurement, span = DlTdoaRangingMeasurement.parse(span)
        fields['measurement'] = measurement
        return ShortAddressDlTdoaRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 65535:
            print(f"Invalid value for field ShortAddressDlTdoaRangingMeasurement::mac_address: {self.mac_address} > 65535; the value will be truncated")
            self.mac_address &= 65535
        _span.extend(int.to_bytes((self.mac_address << 0), length=2, byteorder='little'))
        _span.extend(self.measurement.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.measurement.size + 2

@dataclass
class ExtendedAddressDlTdoaRangingMeasurement(Packet):
    mac_address: int = field(kw_only=True, default=0)
    measurement: DlTdoaRangingMeasurement = field(kw_only=True, default_factory=DlTdoaRangingMeasurement)

    def __post_init__(self):
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ExtendedAddressDlTdoaRangingMeasurement', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['mac_address'] = value_
        span = span[8:]
        measurement, span = DlTdoaRangingMeasurement.parse(span)
        fields['measurement'] = measurement
        return ExtendedAddressDlTdoaRangingMeasurement(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.mac_address > 18446744073709551615:
            print(f"Invalid value for field ExtendedAddressDlTdoaRangingMeasurement::mac_address: {self.mac_address} > 18446744073709551615; the value will be truncated")
            self.mac_address &= 18446744073709551615
        _span.extend(int.to_bytes((self.mac_address << 0), length=8, byteorder='little'))
        _span.extend(self.measurement.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.measurement.size + 8

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
class SessionInfoNtf(SessionControlPacket):
    sequence_number: int = field(kw_only=True, default=0)
    session_token: int = field(kw_only=True, default=0)
    rcr_indicator: int = field(kw_only=True, default=0)
    current_ranging_interval: int = field(kw_only=True, default=0)
    ranging_measurement_type: RangingMeasurementType = field(kw_only=True, default=RangingMeasurementType.ONE_WAY)
    mac_address_indicator: MacAddressIndicator = field(kw_only=True, default=MacAddressIndicator.SHORT_ADDRESS)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionInfoNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
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
            child, remainder = ShortMacTwoWaySessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = ExtendedMacTwoWaySessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = ShortMacDlTDoASessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = ExtendedMacDlTDoASessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = ShortMacOwrAoaSessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = ExtendedMacOwrAoaSessionInfoNtf.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
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
        return SessionControlPacket.serialize(self, payload = bytes(_span))

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
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacTwoWaySessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.TWO_WAY or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
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
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacTwoWaySessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.TWO_WAY or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
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
    dl_tdoa_measurements: List[ShortAddressDlTdoaRangingMeasurement] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.DL_TDOA
        self.mac_address_indicator = MacAddressIndicator.SHORT_ADDRESS
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacDlTDoASessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.DL_TDOA or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        dl_tdoa_measurements_count = span[0]
        span = span[1:]
        dl_tdoa_measurements = []
        for n in range(dl_tdoa_measurements_count):
            element, span = ShortAddressDlTdoaRangingMeasurement.parse(span)
            dl_tdoa_measurements.append(element)
        fields['dl_tdoa_measurements'] = dl_tdoa_measurements
        return ShortMacDlTDoASessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.dl_tdoa_measurements) > 255:
            print(f"Invalid length for field ShortMacDlTDoASessionInfoNtf::dl_tdoa_measurements:  {len(self.dl_tdoa_measurements)} > 255; the array will be truncated")
            del self.dl_tdoa_measurements[255:]
        _span.append((len(self.dl_tdoa_measurements) << 0))
        for _elt in self.dl_tdoa_measurements:
            _span.extend(_elt.serialize())
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.dl_tdoa_measurements]) + 1

@dataclass
class ExtendedMacDlTDoASessionInfoNtf(SessionInfoNtf):
    dl_tdoa_measurements: List[ExtendedAddressDlTdoaRangingMeasurement] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.DL_TDOA
        self.mac_address_indicator = MacAddressIndicator.EXTENDED_ADDRESS
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacDlTDoASessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.DL_TDOA or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        dl_tdoa_measurements_count = span[0]
        span = span[1:]
        dl_tdoa_measurements = []
        for n in range(dl_tdoa_measurements_count):
            element, span = ExtendedAddressDlTdoaRangingMeasurement.parse(span)
            dl_tdoa_measurements.append(element)
        fields['dl_tdoa_measurements'] = dl_tdoa_measurements
        return ExtendedMacDlTDoASessionInfoNtf(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if len(self.dl_tdoa_measurements) > 255:
            print(f"Invalid length for field ExtendedMacDlTDoASessionInfoNtf::dl_tdoa_measurements:  {len(self.dl_tdoa_measurements)} > 255; the array will be truncated")
            del self.dl_tdoa_measurements[255:]
        _span.append((len(self.dl_tdoa_measurements) << 0))
        for _elt in self.dl_tdoa_measurements:
            _span.extend(_elt.serialize())
        return SessionInfoNtf.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.dl_tdoa_measurements]) + 1

@dataclass
class ShortMacOwrAoaSessionInfoNtf(SessionInfoNtf):
    owr_aoa_ranging_measurements: List[ShortAddressOwrAoaRangingMeasurement] = field(kw_only=True, default_factory=list)
    vendor_data: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.ranging_measurement_type = RangingMeasurementType.OWR_AOA
        self.mac_address_indicator = MacAddressIndicator.SHORT_ADDRESS
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ShortMacOwrAoaSessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.OWR_AOA or fields['mac_address_indicator'] != MacAddressIndicator.SHORT_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
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
        self.mt = MessageType.NOTIFICATION
        self.oid = SessionControlOpcodeId.START
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ExtendedMacOwrAoaSessionInfoNtf', bytes]:
        if fields['ranging_measurement_type'] != RangingMeasurementType.OWR_AOA or fields['mac_address_indicator'] != MacAddressIndicator.EXTENDED_ADDRESS or fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != SessionControlOpcodeId.START or fields['gid'] != GroupId.SESSION_CONTROL:
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
class SessionStopCmd(SessionControlPacket):
    session_id: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionControlOpcodeId.STOP
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStopCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionControlOpcodeId.STOP or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        return SessionStopCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_id > 4294967295:
            print(f"Invalid value for field SessionStopCmd::session_id: {self.session_id} > 4294967295; the value will be truncated")
            self.session_id &= 4294967295
        _span.extend(int.to_bytes((self.session_id << 0), length=4, byteorder='little'))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionStopRsp(SessionControlPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionControlOpcodeId.STOP
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionStopRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionControlOpcodeId.STOP or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return SessionStopRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class SessionGetRangingCountCmd(SessionControlPacket):
    session_id: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = SessionControlOpcodeId.GET_RANGING_COUNT
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetRangingCountCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != SessionControlOpcodeId.GET_RANGING_COUNT or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 4:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:4], byteorder='little')
        fields['session_id'] = value_
        span = span[4:]
        return SessionGetRangingCountCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        if self.session_id > 4294967295:
            print(f"Invalid value for field SessionGetRangingCountCmd::session_id: {self.session_id} > 4294967295; the value will be truncated")
            self.session_id &= 4294967295
        _span.extend(int.to_bytes((self.session_id << 0), length=4, byteorder='little'))
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 4

@dataclass
class SessionGetRangingCountRsp(SessionControlPacket):
    status: Status = field(kw_only=True, default=Status.OK)
    count: int = field(kw_only=True, default=0)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = SessionControlOpcodeId.GET_RANGING_COUNT
        self.gid = GroupId.SESSION_CONTROL

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['SessionGetRangingCountRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != SessionControlOpcodeId.GET_RANGING_COUNT or fields['gid'] != GroupId.SESSION_CONTROL:
            raise Exception("Invalid constraint field values")
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
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
        return SessionControlPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 5

@dataclass
class AndroidGetPowerStatsCmd(AndroidPacket):
    

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = AndroidOpcodeId.GET_POWER_STATS
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != AndroidOpcodeId.GET_POWER_STATS or fields['gid'] != GroupId.VENDOR_ANDROID:
            raise Exception("Invalid constraint field values")
        return AndroidGetPowerStatsCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        return AndroidPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 0

@dataclass
class PowerStats(Packet):
    status: Status = field(kw_only=True, default=Status.OK)
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
        fields['status'] = Status.from_int(span[0])
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
class AndroidGetPowerStatsRsp(AndroidPacket):
    stats: PowerStats = field(kw_only=True, default_factory=PowerStats)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = AndroidOpcodeId.GET_POWER_STATS
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidGetPowerStatsRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != AndroidOpcodeId.GET_POWER_STATS or fields['gid'] != GroupId.VENDOR_ANDROID:
            raise Exception("Invalid constraint field values")
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['stats'] = PowerStats.parse_all(span[0:17])
        span = span[17:]
        return AndroidGetPowerStatsRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.stats.serialize())
        return AndroidPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 17

@dataclass
class AndroidSetCountryCodeCmd(AndroidPacket):
    country_code: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self):
        self.mt = MessageType.COMMAND
        self.oid = AndroidOpcodeId.SET_COUNTRY_CODE
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeCmd', bytes]:
        if fields['mt'] != MessageType.COMMAND or fields['oid'] != AndroidOpcodeId.SET_COUNTRY_CODE or fields['gid'] != GroupId.VENDOR_ANDROID:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['country_code'] = list(span[:2])
        span = span[2:]
        return AndroidSetCountryCodeCmd(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.extend(self.country_code)
        return AndroidPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class AndroidSetCountryCodeRsp(AndroidPacket):
    status: Status = field(kw_only=True, default=Status.OK)

    def __post_init__(self):
        self.mt = MessageType.RESPONSE
        self.oid = AndroidOpcodeId.SET_COUNTRY_CODE
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidSetCountryCodeRsp', bytes]:
        if fields['mt'] != MessageType.RESPONSE or fields['oid'] != AndroidOpcodeId.SET_COUNTRY_CODE or fields['gid'] != GroupId.VENDOR_ANDROID:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['status'] = Status.from_int(span[0])
        span = span[1:]
        return AndroidSetCountryCodeRsp(**fields), span

    def serialize(self, payload: bytes = None) -> bytes:
        _span = bytearray()
        _span.append((self.status << 0))
        return AndroidPacket.serialize(self, payload = bytes(_span))

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
            child, remainder = Rssi.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = Aoa.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception as exn:
            pass
        try:
            child, remainder = Cir.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
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
class AndroidRangeDiagnosticsNtf(AndroidPacket):
    session_token: int = field(kw_only=True, default=0)
    sequence_number: int = field(kw_only=True, default=0)
    frame_reports: List[FrameReport] = field(kw_only=True, default_factory=list)

    def __post_init__(self):
        self.mt = MessageType.NOTIFICATION
        self.oid = AndroidOpcodeId.FIRA_RANGE_DIAGNOSTICS
        self.gid = GroupId.VENDOR_ANDROID

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AndroidRangeDiagnosticsNtf', bytes]:
        if fields['mt'] != MessageType.NOTIFICATION or fields['oid'] != AndroidOpcodeId.FIRA_RANGE_DIAGNOSTICS or fields['gid'] != GroupId.VENDOR_ANDROID:
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
        return AndroidPacket.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.frame_reports]) + 9
