use std::error::Error;
use std::fs;
pub struct Config {
    flag: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Incomplete arguments")
        }

        let flag = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { flag, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;
    let file_len = file_content.len();

    let file_wc = format!("{} {}", file_len, config.file_path);
    println!("{file_wc}");
    Ok(())
}