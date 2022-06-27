use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader,BufRead};
use regex::Regex;
use std::io;
use log::{ info, error, debug, warn,trace };

mod cmd_line;
use crate::cmd_line::CommandArgs;

mod log_string_vec;
use crate::log_string_vec::{info_vec,debug_vec};



fn main() {

    env_logger::init();

    let cmd_line = CommandArgs::new();

    debug!("Command Line, {:?}!",cmd_line);

    info!("Determining the distances for {} clusters",cmd_line.num_clusters);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

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
    }

 }
