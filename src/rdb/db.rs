use std::io::{Cursor, Error, ErrorKind, Read, Result};
use crate::{KeyType,Pair,ValueType, Parser};
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
        //TODO 有时候FE后面不一定是在0-15这个范围，有可能会是其他的值，就表示有问题。
        let  db_id = db_number[0];



        loop {
            let mut flags  = parser_flag(cursor)?;
            match flags {
                FB => {
                    let fb =  parse_fb(cursor);
                    println!("FB:{:?}",fb);
                    let kv = Pair::parse(cursor)?;
                    println!("Parsed KV: {:?}", kv);
                }
                FC => {
                    parse_fc(cursor)
                }
                FD => {
                    parse_fd(cursor)
                }
                FE =>{
                    break
                }
                //因为flag_byte是读取FE后一位，所以这里是匹配到id，但是这里处理太粗糙了
                _ => {
                    continue;

                },
            }
        }

        let db = DbInfo {
            db_id,
        };
        Ok(db)
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

fn parse_fc(cursor: &mut Cursor<&[u8]>){
    println!("parse_fc FC")
}

fn parse_fd(cursor: &mut Cursor<&[u8]>){
    println!("parse_fc FD")
}


