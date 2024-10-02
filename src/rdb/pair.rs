use std::io::{Cursor, Error, ErrorKind, Read, Result};
use crate::{Parser};
pub use crate::rdb::data_type::*;


#[derive(Debug)]
pub struct Pair {
    pub key_name: String, //这里需要对key进行编码处理，是Redis字符串相关编码
    pub val: String,
    pub val_type: String, //redis是 使用一个字节来表示，比如0表示string
    pub size: usize,
    pub expiry: Option<u64>,
}



impl Parser for Pair  {
     fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Pair>{

         let mut kv = Pair{
             key_name: String::new(),
             val: String::new(),
             val_type: String::new(),
             size: 0,
             expiry: None,
         };

             let mut val_type = parse_val_type(cursor)?;

             match  val_type{
                STRING_ENCODING =>{
                    kv = crate::parse_string(cursor)?;
                }
                 LIST_ENCODING => {
                     println!("LIST_ENCODING");
                 }
                 SET_ENCODING =>{
                     println!("SET_ENCODING");
                 }
                 ZSET_ENCODING =>{
                     println!("ZSET_ENCODING");
                 }
                 HASH_ENCODING => {
                     println!("HASH_ENCODING");
                 }

                 _ => {
                     return Ok(Pair {
                         key_name: String::new(),
                         val: String::new(),
                         val_type: "Unknown".to_string(),
                         size: 0,
                         expiry: None,
                     });
                 },

         }
         Ok(kv)
     }
}

pub fn read_key_string(cursor: &mut Cursor<&[u8]>) -> Result<String> {
    let (encoding, length) = read_string_encoding(cursor)?;

    match encoding {
        0 => {
            // Length prefixed string
            let mut buf = vec![0; length];
            cursor.read_exact(&mut buf)?;
            String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
        },
        1 => {
            // 8 bit integer
            let mut buf = [0; 1];
            cursor.read_exact(&mut buf)?;
            Ok(i8::from_be_bytes(buf).to_string())
        },
        2 => {
            // 16 bit integer
            let mut buf = [0; 2];
            cursor.read_exact(&mut buf)?;
            Ok(i16::from_be_bytes(buf).to_string())
        },
        3 => {
            // 32 bit integer
            let mut buf = [0; 4];
            cursor.read_exact(&mut buf)?;
            Ok(i32::from_be_bytes(buf).to_string())
        },
        4 => {
            // LZF compressed string
            let clen = read_length(cursor)?;
            let ulen = read_length(cursor)?;
            let mut compressed = vec![0; clen];
            cursor.read_exact(&mut compressed)?;

            // 这里应该使用 LZF 解压缩
            // 由于我们没有 LZF 解压缩的实现，暂时返回一个占位符
            Ok(format!("Compressed string (compressed: {}, uncompressed: {})", clen, ulen))
        },
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown string encoding")),
    }
}

pub fn read_string_encoding(cursor: &mut Cursor<&[u8]>) -> Result<(u8, usize)> {
    let mut first_byte = [0; 1];
    cursor.read_exact(&mut first_byte)?;

    let first = first_byte[0];
    match first >> 6 {
        0b00 => Ok((0, (first & 0x3F) as usize)),
        0b01 => {
            let mut second_byte = [0; 1];
            cursor.read_exact(&mut second_byte)?;
            Ok((0, (((first & 0x3F) as usize) << 8) | second_byte[0] as usize))
        },
        0b10 => {
            let mut length_bytes = [0; 4];
            cursor.read_exact(&mut length_bytes)?;
            Ok((0, u32::from_be_bytes(length_bytes) as usize))
        },
        0b11 => {
            match first & 0x3F {
                0 => Ok((1, 1)), // 8 bit integer
                1 => Ok((2, 2)), // 16 bit integer
                2 => Ok((3, 4)), // 32 bit integer
                3 => Ok((4, 0)), // LZF compressed string (length will be read separately)
                _ => Err(Error::new(ErrorKind::InvalidData, "Invalid string encoding")),
            }
        },
        _ => unreachable!(),
    }
}

pub fn read_length(cursor: &mut Cursor<&[u8]>) -> Result<usize> {
    let (_, length) = read_string_encoding(cursor)?;
    Ok(length)
}


pub fn parse_val_type(cursor: &mut Cursor<&[u8]>)-> Result<u8>{
    let mut flag_val = [0;1];
    cursor.read_exact(&mut flag_val)?;
    let  flag = flag_val[0];
    Ok(flag)
}