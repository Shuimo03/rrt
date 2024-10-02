///这里主要实现核心数据类型: https://redis.io/docs/latest/develop/data-types/
/// https://github.com/redis/redis/blob/unstable/src/rdb.h#L75
pub const STRING_ENCODING: u8 = 0;
pub const LIST_ENCODING: u8 = 1;
pub const SET_ENCODING: u8 = 2;
pub const ZSET_ENCODING: u8 = 3;
pub const HASH_ENCODING: u8 = 4;
pub const ZSET_2_ENCODING: u8 = 5;
pub const MODULE_PRE_GA_ENCODING: u8 = 6;
pub const MODULE_2_ENCODING: u8 = 7;
pub const HASH_ZIPMAP_ENCODING: u8 = 9;
pub const LIST_ZIPLIST_ENCODING: u8 = 10;
pub const SET_INTSET_ENCODING: u8 = 11;
pub const ZSET_ZIPLIST_ENCODING: u8 = 12;
pub const HASH_ZIPLIST_ENCODING: u8 = 13;
pub const LIST_QUICKLIST_ENCODING: u8 = 14;

//TODO 未来实现
pub const STREAM_LISTPACKS_ENCODING: u8 = 15;
pub const HASH_LISTPACK_ENCODING: u8 = 16;
pub const ZSET_LISTPACK_ENCODING: u8 = 17;
pub const LIST_QUICKLIST_2_ENCODING: u8 = 18;
pub const STREAM_LISTPACKS_2_ENCODING: u8 = 19;
pub const SET_LISTPACK_ENCODING: u8 = 20;
pub const STREAM_LISTPACKS_3_ENCODING: u8 = 21;
pub const HASH_METADATA_PRE_GA_ENCODING: u8 = 22;
pub const HASH_LISTPACK_EX_PRE_GA_ENCODING: u8 = 23;
pub const HASH_METADATA_ENCODING: u8 = 24;
pub const HASH_LISTPACK_EX_ENCODING: u8 = 25;
