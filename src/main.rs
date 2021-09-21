use std::{path::PathBuf, process::exit};

use clap::{App, Arg};

fn main() {
    
    let args = App::new("Smart_Find-rs")
                            .version("1.0")
                            .author("MagSG")
                            .about("Streamer272's c-smart_find that i RIIR")
                            .arg(Arg::with_name("text")
                                .short("t")
                                .long("text")
                                .value_name("TEXT")
                                .required(false)
                                .help("The text you want to search for")
                                .takes_value(true))
                            .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .value_name("FILE.EXTENSION")
                                .required(false)
                                .help("The file you want to search in")
                                .takes_value(true))
                            .arg(Arg::with_name("file_extension")
                                .short("e")
                                .long("file-extensions")
                                .value_name("FILE_EXTENSION")
                                .long_help("Filetypes you want to search in\nUse with recursive search\nDefault: None")
                                .takes_value(true))
                            .get_matches();

                            
                            find_in_file("key".to_string(), args.value_of("file").unwrap_or("cargo.toml").to_string())

}

struct Content {

    text: String,
    index: u128,

}

impl Content {

    pub fn new(text: String, index: u128) -> Self {
        Content {
            text,
            index
        }
    }

}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    
        write!(f, "{}: {}", self.index, self.text)
    }
}


fn find_in_file(text: String, filename: String) {

    let file_ent = PathBuf::from(&filename);


    let content = match std::fs::read_to_string(file_ent) {
        Ok(k) => k,
        Err(e) => {eprintln!("[ERROR] {}", e); exit(1)}
    };
    let lines = content.lines();
    
    // Stores all the lines, so that we can have nice formatting
    let mut buffer: Vec<Content> = Vec::new();


    for (index, line) in lines.enumerate() {

        if line.contains(&text) {
            buffer.push(

                Content::new(line.to_string(), index as u128)
            )
        }

    }


    if buffer.is_empty() {
        println!("No occurences found!");
        exit(0);
    }

    for line in buffer {

        println!("Matches Found!!\n");
        println!("{}", line);

    }

}
