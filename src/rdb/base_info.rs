use std::io::{Cursor, Error, ErrorKind, Read, Result};
use std::str;
use log::{error, info};
use crate::parser::Parser;

const RDB_HEADER_LENGTH: usize = 9;


#[derive(Debug)]
pub struct BaseInfo {
    pub magic: String,
    pub rdb_version: String,
}

#[derive(Debug)]
pub struct AuxInfo {
    pub redis_server_version: String,
    pub used_mem: usize,
}

impl Parser for BaseInfo {
    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<BaseInfo> {
        if cursor.get_ref().len() < RDB_HEADER_LENGTH {
            return Err(Error::new(ErrorKind::InvalidData, "RDB file is too short to contain base info"));
        }

        let mut magic_bytes = [0; 5];
        cursor.read_exact(&mut magic_bytes)?;
        let magic = str::from_utf8(&magic_bytes)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid magic string"))?
            .to_string();


        let mut version_bytes = [0; 4];
        cursor.read_exact(&mut version_bytes)?;
        let rdb_version = str::from_utf8(&version_bytes)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid RDB version"))?
            .to_string();

        let base_info = BaseInfo {
            magic,
            rdb_version,
        };
        Ok(base_info)
    }
}

impl Parser for AuxInfo {
    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<AuxInfo> {
        let mut aux_info = AuxInfo {
            redis_server_version: String::new(),
            used_mem: 0,
        };

        let aux_name = parser_aux_name(cursor)?;
        let aux_value = parser_aux_value(cursor)?;

        match aux_name.as_str() {
            "redis-ver" => aux_info.redis_server_version = aux_value,
            "used-mem" => {
                if aux_value.is_empty() {
                    aux_info.used_mem = 0;
                } else {
                    aux_info.used_mem = aux_value.parse::<usize>().unwrap_or_else(|_| {
                        println!("Failed to parse used-mem, setting it to 0");
                        0
                    });
                }
            },
            _ => {
                // 忽略其他未知字段
                println!("Unknown aux_name: {}", aux_name);
            }
        }


        Ok(aux_info)
    }
}


fn parser_aux_name(cursor: &mut Cursor<&[u8]>)-> Result<String>{
    let mut aux_name_after = [0;1];
    cursor.read_exact(&mut aux_name_after)?;
    if aux_name_after[0] < 0 || aux_name_after[0] > 127 {
        println!("aux_name_after: {:x?}",aux_name_after);
        return Err(Error::new(ErrorKind::InvalidData, "parser_aux_name Value contains non-ASCII characters"));
    }

    let aux_name_length = aux_name_after[0] as usize;
    let mut  aux_name = vec![0;aux_name_length];


    cursor.read_exact(&mut aux_name)
        .map_err(|_| Error::new(ErrorKind::UnexpectedEof, "Failed to read key bytes"))?;


    let aux_value_str = String::from_utf8(aux_name)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to convert bytes to string"))?;

    Ok(aux_value_str)
}



fn parser_aux_value(cursor: &mut Cursor<&[u8]>) -> Result<String>{
    //每次读取到的position是可以匹配到cursor的，也就是FA本身
    //判断是否会在ASCII码内
    let mut fa_after = [0;1];
    cursor.read_exact(&mut fa_after)?;

    if fa_after[0] < 0 || fa_after[0] > 127{
        log::warn!("Non-ASCII characters detected, setting value to an empty string.");
        return Ok(String::new());
    }

    let aux_value_length = fa_after[0] as usize;
    let mut  aux_value = vec![0;aux_value_length];


    cursor.read_exact(&mut aux_value)
        .map_err(|_| Error::new(ErrorKind::UnexpectedEof, "Failed to read key bytes"))?;


    let aux_name_str = String::from_utf8(aux_value)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to convert bytes to string"))?;


    Ok(aux_name_str)
}
