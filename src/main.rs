use std::fs::File;
use std::io::{self, Read};
use parserRDB::rdb::parser;
use parserRDB::cli::Cli;


fn main() -> io::Result<()> {

    let command = Cli::new();
    let rdb_file_path = command.parse_args();
    let content = read_rdb_file(&rdb_file_path)?;
    ///context是一个二进制数组，但是 Rust 默认的数字表示方式是十进制
    match parser::ParserFactory::parse(&content) {
        Ok(info) => println!("{:?}", info),
        Err(e) => println!("Error parsing RDB file: {}", e),
    }
    Ok(())

}

fn read_rdb_file(path: &str) -> io::Result<Vec<u8>>{
    let mut rdb_file = File::open(path)?;
    let  mut content = Vec::new();
    rdb_file.read_to_end(&mut content)?;
    Ok(content)
}

