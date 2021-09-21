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
    location: String,
    index: u128,

}

impl Content {

    pub fn new(text: String, location:String, index: u128) -> Self {
        Content {
            text,
            location,
            index
        }
    }

}



// Will scrap this in the future 

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    
        write!(f, "{} |  {}", self.index, self.text)
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
            buffer.push(                                    // Reeeeaaaly dont like this
                Content::new(line.to_string(), filename.clone(), index as u128)
            )
        }

    }


    if buffer.is_empty() {
        println!("No occurences found!");
        exit(0);
    }

    println!("Matches found!!\n\n");

    for (index, line) in buffer.iter().enumerate() {

        // This is the format i want!

        //   --> src\main.rs:47:12
        //    |
        // 47 |     pub fn new(text: String, index: u128) -> Self {
        //    |            ^^^
        //    |
        
                      // EWWW gross
        let offset = create_whitespace_offset(line.index.to_string().encode_utf16().count());
        
        
        print!(" {}--> {}\n", &offset, line.location);
        print!(" {} |\n", &offset);
        print!(" {} |  {}\n", line.index, line.text);
        print!(" {} |\n", &offset);
        print!(" {} |\n", &offset);

    }

    fn create_whitespace_offset(lenth: usize) -> String {

        let mut buf = String::new();

        for _ in [0..lenth] {
            buf.push(' ')
        }

        buf
    }

}
