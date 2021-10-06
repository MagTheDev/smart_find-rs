use std::{path::PathBuf, process::exit};

use clap::{App, Arg};

fn main() {
    
    // Initalize argument parser: clap

    let args = App::new("Smart_Find-rs")
                            .version("1.0")
                            .author("MagSG")
                            .about("Streamer272's c-smart_find that i RIIR")
                            .arg(Arg::with_name("query")
                                .short("q")
                                .long("query")
                                .value_name("QUERY")
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

    // Goes through all the lines collected from file, new Content instance will be created if query is found in 
    // line, then pushed to buffer
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


    // Loops through all Content instances from buffer
    for (_index, content) in buffer.iter().enumerate() {

        // Creates the offset of index number 
        let white_line_offset = create_offset(content.index.to_string().encode_utf16().count(), ' ');

        // Creates a string wiht ammount of carrots that correspond to length of query
        let carrots = create_offset(query.encode_utf16().count(), '^');

        // Gets all occurences of query in content's line
        let query_inidices: Vec<_> = content.text.match_indices(&query).collect();

        // Buffer for whitespace offset for carrots
        let mut carrot_offset = String::new();

        // Closure just for aesthetic purposes
        {

            let mut total: u32  = 0;

            for (loc , _) in query_inidices {

                let offset = &create_offset(loc - total as usize, ' ');

                carrot_offset.push_str(offset);
                carrot_offset.push_str(&carrots);
                carrot_offset.push(' ');
                                                                  // Extract this to a variable              
                total += (offset.encode_utf16().count() as u32) + carrots.encode_utf16().count() as u32 + 1;

            }

        }


        println!(" {}--> {}:{}", &white_line_offset, content.location, content.index);
        println!(" {} |", &white_line_offset);
        println!(" {} |  {}", content.index, content.text);
        println!(" {} |  {}", &white_line_offset, carrot_offset);
        println!(" {} |\n", &white_line_offset);


    }

    fn create_offset(length: usize, char: char) -> String {

        let mut buf = String::new();

        for _ in 0..length {
            buf.push(char);
        }
        buf
    }
    
}
