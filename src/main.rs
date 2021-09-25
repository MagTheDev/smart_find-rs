use std::{path::PathBuf, process::exit};

use clap::{App, Arg};

fn main() {
    
    let args = App::new("Smart_Find-rs")
                            .version("1.0")
                            .author("MagSG")
                            .about("Streamer272's c-smart_find that i RIIR")
                            .arg(Arg::with_name("text")
                                .short("q")
                                .long("query")
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
                                .long("file-extension")
                                .value_name("FILE_EXTENSION")
                                .long_help("Filetypes you want to search in\nUse with recursive search\nDefault: None")
                                .takes_value(true))
                            .get_matches();

                            
    find_in_file(args.value_of("text").unwrap().to_string(), args.value_of("file").unwrap().to_string())

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


fn find_in_file(query: String, filename: String) {

    let file_ent = PathBuf::from(&filename);


    let content = match std::fs::read_to_string(file_ent) {
        Ok(k) => k,
        Err(e) => {eprintln!("[ERROR] {}", e); exit(1)}
    };
    let lines = content.lines();
    
    // Stores all the lines, so that we can have nice formatting
    let mut buffer: Vec<Content> = Vec::new();


    for (index, line) in lines.enumerate() {

        if line.contains(&query) {
            buffer.push(                                    // Reeeeaaaly dont like this
                Content::new(line.to_string(), filename.clone(), index as u128)
            );
        }

    }


    if buffer.is_empty() {
        println!("No occurences found!");
        exit(0);
    }

    println!("Matches found!!\n\n");

    for (_index, line) in buffer.iter().enumerate() {

        let white_line_offset = create_offset(line.index.to_string().encode_utf16().count(), ' ');
        let carrots = create_offset(query.encode_utf16().count(), '^');

        let query_inidices: Vec<_> = line.text.match_indices(&query).collect();
        let mut carrot_offset = String::new();

        {

            let mut total: u32  = 0;

            for (loc , _) in query_inidices {

                let offset = &create_offset(loc - total as usize, ' ');

                carrot_offset.push_str(offset);
                carrot_offset.push_str(&carrots);
                carrot_offset.push_str(" ");

                total += (offset.encode_utf16().count() as u32) + carrots.encode_utf16().count() as u32 + 1;

            }

        }


        print!(" {}--> {}:{}\n", &white_line_offset, line.location, line.index);
        print!(" {} |\n", &white_line_offset);
        print!(" {} |  {}\n", line.index, line.text);
        print!(" {} |  {}\n", &white_line_offset, carrot_offset);
        print!(" {} |\n\n", &white_line_offset);


    }

    fn create_offset(length: usize, char: char) -> String {

        let mut buf = String::new();

        for _ in 0..length {
            buf.push(char);
        }
        buf
    }
    
}
