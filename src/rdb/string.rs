use std::io::{Cursor, Error, ErrorKind, Read, Result};
use crate::{Pair, read_key_string,read_string_encoding};



pub fn parse_string(cursor: &mut Cursor<&[u8]>) -> Result<Pair> {
    let key = read_key_string(cursor)?;
    let (encoding, length) = read_string_encoding(cursor)?;

    let value = match encoding {
        0 => {
            // 普通字符串
            let mut buf = vec![0; length];
            cursor.read_exact(&mut buf)?;
            String::from_utf8_lossy(&buf).into_owned()
        },
        1 => {
            // 整数 8
            let mut buf = [0; 1];
            cursor.read_exact(&mut buf)?;
            buf[0].to_string()
        },
        2 => {
            // 整数 16
            let mut buf = [0; 2];
            cursor.read_exact(&mut buf)?;
            i16::from_be_bytes(buf).to_string()
        },
        3 => {
            // 整数 32
            let mut buf = [0; 4];
            cursor.read_exact(&mut buf)?;
            i32::from_be_bytes(buf).to_string()
        },
     //   _ => return Err(Error::new(ErrorKind::InvalidData, "String Unknown string encoding")),
        _ => {
            return Ok(Pair {
                key_name: key,
                val: "".to_string(), // 占位符
                val_type: "String".to_string(),
                size: 0, // 设置大小为 0
                expiry: None,
            });
           }

    };

    Ok(Pair {
        key_name: key,
        val: value,
        val_type: "String".to_string(),
        size: length, //这里内存占用有些问题
        expiry: None,
    })
}


