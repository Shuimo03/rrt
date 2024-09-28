use std::io::{Cursor, Error, ErrorKind, Read, Result};
use crate::{LIST_QUICKLIST_ENCODING, Pair, Parser, STRING_ENCODING};
use crate::rdb::rdb_flag::{FB, FC, FD,FE};
#[derive(Debug)]
pub struct DbInfo {
    pub db_id: u8,
}

#[derive(Debug)]
pub struct ResizeDbInfo{
    pub total_db_key: u8,
    pub total_expire_key: u8,
}



impl Parser for DbInfo {
    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<DbInfo> {
        let mut db_number = [0; 1];
        cursor.read_exact(&mut db_number)?;

        let db_id = db_number[0];
        let mut resize_info: Option<ResizeDbInfo> = None;
        
        loop {
            let flags = parser_flag(cursor)?;
            match flags {
                FB => {
                    match parse_fb(cursor) {
                        Ok(fb) => {
                            println!("FB: {:?}", fb);
                            resize_info = Some(fb);
                        },
                        Err(e) => {
                            return Err(Error::new(ErrorKind::InvalidData, format!("Error parsing FB: {:?}", e)));
                        }
                    }
                }
                FC => {
                    match parse_fc(cursor) {
                        Ok(kv) => println!("Parsed KV with FC: {:?}", kv),
                        Err(e) => return Err(Error::new(ErrorKind::InvalidData, format!("Error parsing FC: {:?}", e))),
                    }
                }
                FD => {
                    match parse_fd(cursor) {
                        Ok(kv) => println!("Parsed KV with FD: {:?}", kv),
                        Err(e) => return Err(Error::new(ErrorKind::InvalidData, format!("Error parsing FD: {:?}", e))),
                    }
                }
                _ => {
                    if is_key_type(cursor) {
                        match Pair::parse(cursor) {
                            Ok(kv) => println!("Parsed KV without expiry: {:?}", kv),
                            Err(e) => return Err(Error::new(ErrorKind::InvalidData, format!("Error parsing KV: {:?}", e))),
                        }
                    } else {
                        println!("position:{:?}",cursor.position());
                        println!("other key: {:x?}",flags);
                        continue
                    }
                },
            }
        }

        Ok(DbInfo { db_id })
    }
}

fn parser_flag(cursor: &mut Cursor<&[u8]>) -> Result<u8>{
    let mut flag_val = [0;1];
    cursor.read_exact(&mut flag_val)?;
    let  flag = flag_val[0];
    Ok(flag)
}

//TODO 也是RDB-version >= 7才会有的字段
fn parse_fb(cursor: &mut Cursor<&[u8]>) -> Result<ResizeDbInfo> {
    let mut resize_db = ResizeDbInfo {
        total_db_key: 0,
        total_expire_key: 0,
    };

    // 读取主哈希表大小
    let total_key = rdb_load_len(cursor)?;
    resize_db.total_db_key = total_key as u8;

    // 读取过期哈希表大小
    let total_expire_key = rdb_load_len(cursor)?;
    resize_db.total_expire_key = total_expire_key as u8;
    //解析key-value
    Ok(resize_db)
}

fn rdb_load_len(cursor: &mut Cursor<&[u8]>) -> Result<usize> {
    let mut len_bytes = [0; 1];
    cursor.read_exact(&mut len_bytes)?;

    let len_type = len_bytes[0] >> 6;
    let len_value = len_bytes[0] & 0x3F; // 提取低6位

    match len_type {
        0 => Ok(len_value as usize), // 6-bit length
        1 => {
            let mut more_bytes = [0; 1];
            cursor.read_exact(&mut more_bytes)?;
            Ok(((len_value as usize) << 8) | (more_bytes[0] as usize))
        }
        2 => {
            let mut more_bytes = [0; 4];
            cursor.read_exact(&mut more_bytes)?;
            let len32 = u32::from_be_bytes([len_value, more_bytes[0], more_bytes[1], more_bytes[2]]);
            Ok(len32 as usize)
        }
        3 => {
            let mut more_bytes = [0; 8];
            cursor.read_exact(&mut more_bytes)?;
            let len64 = u64::from_be_bytes([len_value, more_bytes[0], more_bytes[1], more_bytes[2], more_bytes[3], more_bytes[4], more_bytes[5], more_bytes[6]]);
            Ok(len64 as usize)
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid length type")),
    }
}

fn parse_fc(cursor: &mut Cursor<&[u8]>) -> Result<Pair> {
    println!("parse_fc");
    // 读取过期时间（毫秒）
    let mut expiry_time_ms = [0; 8];
    cursor.read_exact(&mut expiry_time_ms)?;
    println!("expiry_time_ms:{:x?}",expiry_time_ms);
    // 将读取的字节转换为无符号长整型（u64）
    let expiry_time = u64::from_le_bytes(expiry_time_ms);
    let kv = Pair::parse(cursor)?;


    let kv_with_expiry = Pair {
        expiry: Some(expiry_time),
        ..kv // 这里使用 kv 中的其他字段
    };

    Ok(kv_with_expiry)
}
fn parse_fd(cursor: &mut Cursor<&[u8]>)-> Result<Pair>{
    println!("parse_fd FD");
    let mut expiry_time_sec = [0; 4];
    cursor.read_exact(&mut expiry_time_sec)?;
    println!("expiry_time_sec:{:?}",expiry_time_sec);

    let expiry_time_sec = u32::from_le_bytes(expiry_time_sec);

    let expiry_time = expiry_time_sec as u64;
    let kv = Pair::parse(cursor)?;
    // 你可以将过期时间存储在 kv 中，假设 Pair 结构体可以支持过期时间
    let kv_with_expiry = Pair {
        expiry: Some(expiry_time),
        ..kv // 这里使用 kv 中的其他字段
    };

    Ok(kv_with_expiry)
}


fn is_key_type(cursor: &mut Cursor<&[u8]>) -> bool {
    let mut fb_after_value = [0; 1];
    match cursor.read_exact(&mut fb_after_value) {
        Ok(_) => {
            let key_type = fb_after_value[0];
            key_type >= STRING_ENCODING && key_type <= LIST_QUICKLIST_ENCODING
        },
        Err(_) => false // 如果读取失败，返回 false
    }
}