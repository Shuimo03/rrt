use std::fs::File;
use std::io::{self, Read, Error, ErrorKind};
use parserRDB::rdb::parser;
use parserRDB::cli::Cli;
use log::{error, info, warn};

fn main() -> io::Result<()> {
    // 初始化日志记录器
    env_logger::init();

    let command = Cli::new();
    let rdb_file_path = command.parse_args();
    let content = match read_rdb_file(&rdb_file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read RDB file: {}", e);
            return Err(e);  // 返回错误，而不是panic
        }
    };

    // 解析内容，并处理错误
    match parser::ParserFactory::parse(&content) {
        Ok(info) => {
            info!("Successfully parsed RDB file.");
            println!("{:?}", info);
        }
        Err(e) => {
            error!("Error parsing RDB file: {}", e);

            if e.kind() == ErrorKind::InvalidData {
                warn!("Invalid data encountered, skipping this section...");
                // 继续逻辑，可以跳过当前数据块
            } else {
                // 处理其他错误或停止
                return Err(e);
            }
        }
    }

    Ok(())
}

fn read_rdb_file(path: &str) -> io::Result<Vec<u8>> {
    let mut rdb_file = File::open(path)?;
    let mut content = Vec::new();
    rdb_file.read_to_end(&mut content)?;
    Ok(content)
}
