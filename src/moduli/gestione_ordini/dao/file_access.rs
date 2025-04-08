use std::fs;
use std::{fs::File, io::Write};
use std::io::Result;

pub fn crea_files(){
    //TODO CONTINUARE
    match File::create("/db/Ordini.txt"){
        Ok(_) =>{},
        Err(a) => println!("{}",a),
    }
}