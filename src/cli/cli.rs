use clap::{Arg, Command};

pub struct Cli {
    command: Command<>,
}

impl Cli{
    pub fn new() -> Self{
        let command = Command::new("RDB Analyzer")
            .version("0.1")
            .author("wu1998102@gmail.com")
            .about("Analyzes Redis RDB files")
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("rdb file")
                    .help("Specifies the RDB file path")
                    .required(true),
            );

        Cli{command }
    }

    pub fn parse_args(&self) -> String{
        let matches = self.command.clone().get_matches();
        matches
            .get_one::<String>("file")
            .expect("RDB file path is required")
            .to_string()
    }
}
