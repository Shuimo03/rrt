use std::io::{Cursor, Error, ErrorKind, Read, Result};
use crate::Parser;
use crate::rdb::constants::{FE,FF};
#[derive(Debug)]
pub struct DbInfo {
    pub id: u8,
    pub num_keys: usize, //每个db中的key
    pub num_expired_keys: usize, //每个db中具有过期时间的key数量
    pub total_keys: usize // db数量总和
}

impl Parser for DbInfo {
    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<DbInfo> {
        println!("db offset: {:?}",cursor.position());
        let mut flag_byte = [0; 1];
        cursor.read_exact(&mut flag_byte)?;
        while cursor.read_exact(&mut flag_byte).is_ok() {
            match flag_byte[0] {
                FE => {
                    let db_number = parse_db_number(cursor)?; // 解析DB编号
                    println!("Selected database: {}", db_number);
                }
                FF => {
                    println!("End of RDB file");
                    break;  // 遇到 FF，结束解析
                }
                _ => {
                    //    println!("position :{:x?}",flag_byte);
                    continue
                },
            }
        }
        

        let db = DbInfo {
            id: 0,
            num_keys: 0,
            num_expired_keys: 0,
            total_keys: 0,
        };
        Ok(db)
    }
}

fn parse_db_number(cursor: &mut Cursor<&[u8]>) -> Result<u8>{
    let mut db_number = [0; 1];
    cursor.read_exact(&mut db_number)?;
    Ok(db_number[0])
}
