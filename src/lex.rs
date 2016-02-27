extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use regex::Regex;

//let mut file = try!(File::open(""))
pub enum Token { }


fn main(){
    let args: Vec<_> = env::args().collect();
    let re = Regex::new(r"\.rsc$");
    assert!(re.unwrap().is_match(&args[0]));
}
