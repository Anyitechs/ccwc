use std::error::Error;
use std::fs;
use std::convert::TryFrom;

pub struct Config {
    flag: Option<String>,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Incomplete arguments");
        }

        if args.len() == 2 {
            let file_path = args[1].clone();
            return Ok(Self {
                flag: None,
                file_path,
            });
        }

        let flag = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self {
            flag: Some(flag),
            file_path,
        })
    }
}

fn byte_count(file_content: &String) -> usize {
    file_content.len()
}

fn line_count(file_content: &String) -> usize {
    file_content.lines().count()
}

fn word_count(file_content: &String) -> usize {
    let file_content: Vec<&str> = file_content.split_whitespace().collect();
    file_content.len()
}

fn character_count(file_content: &String) -> usize {
    file_content.chars().count()
}

fn no_flag_count(file_content: &String) -> String {
    let byte_count = byte_count(file_content);
    let line_count = line_count(file_content);
    let word_count = word_count(file_content);

    let combine_count = format!(
        "{} {} {}",
        line_count.to_string(),
        word_count.to_string(),
        byte_count.to_string(),
    );

    combine_count
}

enum ValidFlags {
    ByteCount,
    LineCount,
    WordCount,
    CharacterCount,
}

impl TryFrom<&str> for ValidFlags {
    type Error = ();

    fn try_from(flag: &str) -> Result<Self, Self::Error> {
        match flag {
            "-c" => Ok(ValidFlags::ByteCount),
            "-l" => Ok(ValidFlags::LineCount),
            "-w" => Ok(ValidFlags::WordCount),
            "-m" => Ok(ValidFlags::CharacterCount),
            _ => Err(())
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;

    let flag = match config.flag {
        Some(flag) => flag,
        None => "".to_string(),
    };

    let file_len = match ValidFlags::try_from(flag.as_str()) {
        Ok(ValidFlags::ByteCount) => byte_count(&file_content),
        Ok(ValidFlags::LineCount) => line_count(&file_content),
        Ok(ValidFlags::WordCount) => word_count(&file_content),
        Ok(ValidFlags::CharacterCount) => character_count(&file_content),
        Err(_e) => 0,
    };

    if file_len == 0 {
        let all_count = no_flag_count(&file_content);
        let file_len = format!("{} {}", all_count, config.file_path);
        println!("{file_len}");
    } else {
        let file_len = format!("{} {}", file_len, config.file_path);
        println!("{file_len}");
    }

    Ok(())
}
