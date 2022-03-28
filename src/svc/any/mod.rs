#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum ErrDefs {
    ///基本错误：范围0x1 - 0xff
    SUCCESS = 0x0,
    FAILED = 0x1,
    SUB_ERROR = 0x2,
    BUSY = 0x3,
    TIMEOUT = 0x4,
    UNKNOWN_ERROR = 0x5,
    UNDEFINED_ERROR = 0x6,
    NOT_IMPLEMENTED = 0x7,
    UNSUPPORTED = 0x8,

    ///系统相关：范围0x100 - 0x1ff
    ERR_SYS = 0x100,

    ///参数相关：范围0x200 - 0x2ff
    ERR_ARG = 0x200,
    ERR_ARG_CANNOT_EMPTY = 0x201,

    ///表单相关：范围0x300 - 0x3ff
    ERR_FORM = 0x300,

    ///配置相关：范围0x400 - 0x4ff
    ERR_CONFIG = 0x400,

    ///认证相关：范围0x500 - 0x5ff
    ERR_AUTH = 0x500,
    ERR_AUTH_UNAUTHORIZED = 0x501,
    ERR_AUTH_FORBIDDEN = 0x502,
    ERR_AUTH_TOKEN_NOT_ACTIVE = 0x503,
    ERR_AUTH_TOKEN_TIMEOUT = 0x504,
    ERR_AUTH_TOKEN_INVALID_ISSUER = 0x505,
    ERR_AERR_TOKEN_FORMAT_WRONG = 0x506,
    ERR_AUTH_TOKEN_SIGNATURE_WRONG = 0x507,

    ///用户相关：范围0x600 - 0x6ff
    ERR_USER = 0x600,
    ///数据库相关：范围0x700 - 0x7ff
    ERR_DB = 0x700,
    ///消息队列相关：范围0x800 - 0x8ff
    ERR_MQ = 0x800,
    ///数据字典相关：范围0x900 - 0x9ff
    ERR_DICT = 0x900,
    ///网络相关：范围0x1000 - 0x10ff
    ERR_NETWORK = 0x1000,
    ///业务的错误码段，范围0x10000 - 0xfffff
    ERR_BUSINESS = 0x10000,
}

impl std::convert::Into<u32> for ErrDefs {
    fn into(self) -> u32 {
        self as u32
    }
}