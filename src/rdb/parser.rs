use std::io::{Cursor, Read, Result, Error, ErrorKind};
use crate::{AuxInfo, BaseInfo,DbInfo};
use crate::rdb::constants::{FA,FE, FF};


pub trait Parser {
    fn parse(cursor: &mut Cursor<&[u8]>) ->  Result<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct RDBInfo {
    pub base_info: BaseInfo,
    pub aux_info: AuxInfo,
    //   pub db_info: DbInfo,

}
pub struct ParserFactory;

///使用动态注册的解析器：通过为每个解析器实现一个 Parser trait，将所有解析器注册到 ParserFactory 中。工厂在运行时可以根据上下文自动选择合适的解析器。
///责任链模式：为每个 opcode 注册一个处理器链，按顺序处理每个块。
impl ParserFactory {
    pub fn parse(rdb_context: &[u8]) -> Result<RDBInfo> {
        let mut cursor = Cursor::new(rdb_context);
        let base_info = BaseInfo::parse(&mut cursor)?;
        let rdb_version:usize = base_info.rdb_version.parse().expect("Not a valid number");
        let mut aux_info = None;
        let mut db_info=  None;

        // aux只有rdb版本大于等于7才引入
        if rdb_version < 7 {
            aux_info = None
        };

        loop {
            // 读取标志位
            let mut flag_byte = [0;1];
            if cursor.read_exact(&mut flag_byte).is_err(){
                break
            }
            match flag_byte[0] {
                FA => {
                    aux_info = Some(AuxInfo::parse(&mut cursor)?);
                }
                FE => {
                    db_info = Some(DbInfo::parse(&mut cursor)?);
                }

                FF => {
                    println!("parse done.");
                    break
                }
                _ =>{
                    continue;
                }
            }
        }

        // 组合 RDBInfo
        let rdb_info = RDBInfo {
            base_info,
            aux_info: aux_info.unwrap_or_else(|| AuxInfo {
                redis_server_version: String::new(),
                used_mem:0,
            }),
        };

        Ok(rdb_info)
    }

}


