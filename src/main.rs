use std::path::Path;
use std::fs::File;

use std::io::{prelude::*, BufReader,BufRead};
use regex::Regex;
//use std::io;

use log::{ info, /*error, */ debug, /*warn, trace*/ };

mod cmd_line;
use crate::cmd_line::CommandArgs;

//mod log_string_vec;
//use crate::log_string_vec::{info_vec,debug_vec};


mod huffman;
use crate::huffman::HuffmanInfo;

fn process_huffman(file: &mut File) {

    let mut reader = BufReader::new(file);

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();
    debug!("First Input Line is \'{}\'",line);
    let first_line_regex = Regex::new(r"\s*(?P<num_vertex>\d+)\s+.*$").unwrap();
    let _first_line = first_line_regex.captures(&line).unwrap();
    
    let mut h = HuffmanInfo::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let mut line_data = line.unwrap();
        debug!("Processing {}",line_data);
        // remove all the 
        line_data.retain(|c| !c.is_whitespace());
        let weight = line_data.parse::<u64>().unwrap();
        h.add_simple_vertex(weight);
    }
    h.process();
}

fn main() {

    env_logger::init();

    let cmd_line = CommandArgs::new();

    debug!("Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    process_huffman(&mut file);

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let _h = HuffmanInfo::new();
        debug!("Testing");
    }

 }
