use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct Config {
    flag: Option<String>,
    file_path: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        const MIN_ARGS: usize = 2;

        if args.len() < MIN_ARGS {
            return Err("Incomplete arguments. Expected at least one flag or file path.");
        }

        let (flag, file_path) = if args.len() == MIN_ARGS {
            let flag_or_file = &args[1];
            if Path::new(flag_or_file).is_file() {
                (None, Some(flag_or_file.clone()))
            } else {
                (Some(flag_or_file.clone()), None)
            }
        } else {
            let flag = args[1].clone();
            let file_path = args[2].clone();
            (Some(flag), Some(file_path))
        };

        Ok(Self { flag, file_path })
    }
}

mod flag_method {
    pub fn byte_count(file_content: &String) -> usize {
        file_content.len()
    }

    pub fn line_count(file_content: &String) -> usize {
        file_content.lines().count()
    }

    pub fn word_count(file_content: &String) -> usize {
        let file_content: Vec<&str> = file_content.split_whitespace().collect();
        file_content.len()
    }

    pub fn character_count(file_content: &String) -> usize {
        file_content.chars().count()
    }

    pub fn no_flag_count(file_content: &String) -> String {
        let byte_count = byte_count(file_content);
        let line_count = line_count(file_content);
        let word_count = word_count(file_content);

        format!(
            "{} {} {}",
            line_count.to_string(),
            word_count.to_string(),
            byte_count.to_string(),
        )
    }
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
            _ => Err(()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    use flag_method::*;

    let file_path = config.file_path.unwrap_or_default();
    let flag = config.flag.unwrap_or_default();

    let content = if Path::new(&file_path).is_file() {
        fs::read_to_string(&file_path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let file_len = match ValidFlags::try_from(flag.as_str()) {
        Ok(ValidFlags::ByteCount) => byte_count(&content),
        Ok(ValidFlags::LineCount) => line_count(&content),
        Ok(ValidFlags::WordCount) => word_count(&content),
        Ok(ValidFlags::CharacterCount) => character_count(&content),
        Err(_) => {
            let combined_count = no_flag_count(&content);
            println!("{combined_count} {file_path}");
            return Ok(());
        }
    };

    let result = format!("{} {}", file_len, file_path);
    println!("{result}");

    Ok(())
}
