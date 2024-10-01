use std::io::{Cursor, Error, ErrorKind, Read};
use crate::{KeyType, Pair, ValueType};

pub fn parse_string(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Pair> {
    // 解析可选的过期时间
    let expiry = match read_expiry(cursor)? {
        Some(expiry) => Some(expiry),
        None => None,
    };

    // 解析键值类型
    let value_type = read_value_type(cursor)?;

    // 解析键
    let key = read_string(cursor)?;

    // 解析值
    let value = match value_type {
        KeyType::String => ValueType::String(read_string(cursor)?),
        _ => return Err(Error::new(ErrorKind::InvalidData, "Unsupported value type")),
    };

    Ok(Pair {
        name: key,
        val: value,
        kv_type: value_type,
        size: cursor.position() as usize,
        expiry,
    })
}

fn read_expiry(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Option<u64>> {
    let mut flag = [0; 1];
    cursor.read_exact(&mut flag)?;
    match flag[0] {
        0xFD => {
            // 读取 4 字节的秒级过期时间
            let mut buf = [0; 4];
            cursor.read_exact(&mut buf)?;
            let expiry = u32::from_be_bytes(buf) as u64;
            Ok(Some(expiry))
        },
        0xFC => {
            // 读取 8 字节的毫秒级过期时间
            let mut buf = [0; 8];
            cursor.read_exact(&mut buf)?;
            let expiry = u64::from_be_bytes(buf);
            Ok(Some(expiry))
        },
        _ => {
            // 不是过期时间标志，可能是其他数据，重置读取位置
            cursor.set_position(cursor.position() - 1);
            Ok(None)
        }
    }
}

// 读取键值类型
fn read_value_type(cursor: &mut Cursor<&[u8]>) -> std::io::Result<KeyType> {
    let mut type_flag = [0; 1];
    cursor.read_exact(&mut type_flag)?;
    match type_flag[0] {
        0 => Ok(KeyType::String),
        1 => Ok(KeyType::List),
        2 => Ok(KeyType::Set),
        3 => Ok(KeyType::ZSet),
        4 => Ok(KeyType::Hash),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown value type")),
    }
}

// 读取 Redis 字符串
fn read_string(cursor: &mut Cursor<&[u8]>) -> std::io::Result<String> {
    let length = read_length(cursor)?;

    // 根据长度读取相应的字节数
    let mut buf = vec![0; length as usize];
    cursor.read_exact(&mut buf)?;

    // 将字节转换为字符串
    String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

// 读取长度编码（变长编码）
fn read_length(cursor: &mut Cursor<&[u8]>) -> std::io::Result<u64> {
    let mut first_byte = [0; 1];
    cursor.read_exact(&mut first_byte)?;

    let length = match first_byte[0] >> 6 {
        0 => (first_byte[0] & 0x3F) as u64, // 前 6 位为长度
        1 => {
            // 读取一个额外字节，组合 14 位长度
            let mut second_byte = [0; 1];
            cursor.read_exact(&mut second_byte)?;
            ((first_byte[0] & 0x3F) as u64) << 8 | second_byte[0] as u64
        }
        2 => {
            // 读取后 4 字节作为长度
            let mut length_bytes = [0; 4];
            cursor.read_exact(&mut length_bytes)?;
            u32::from_be_bytes(length_bytes) as u64
        }
        3 => {
            return Err(Error::new(ErrorKind::InvalidData, "Special encoding not supported"));
        }
        _ => unreachable!(),
    };

    Ok(length)
}