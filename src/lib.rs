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

fn byte_count(file_content: String) -> usize {
    file_content.len()
}

fn line_count(file_content: String) -> usize {
    file_content.lines().count()
}

fn word_count(file_content: String) -> usize {
    let file_content: Vec<&str> = file_content.split_whitespace().collect();
    file_content.len()
}

fn character_count(file_content: String) -> usize {
    file_content.chars().count()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;

    let file_len = match config.flag.as_str() {
        "-c" => byte_count(file_content),
        "-l" => line_count(file_content),
        "-w" => word_count(file_content),
        "-m" => character_count(file_content),
        _ => 0,
    };

    let file_wc = format!("{} {}", file_len, config.file_path);
    println!("{file_wc}");
    Ok(())
}