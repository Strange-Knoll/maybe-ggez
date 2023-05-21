use std::{path::{PathBuf, self}, fs, io::Read, env, fmt::format};
use core::slice::Iter;
use std::fmt::Write;


// load file from bytes (credit: Bowarc)
pub fn load_file(p: &str) -> Option<Vec<u8>>{
    //build path
    let path = PathBuf::from(format!("{}", p));
    //chech path
    if !path.exists(){
        println!("Path doesn't exist: {path:?}");
        return None;
    }

    //read file from path and write data to bytes
    let mut file = fs::File::open(path).ok()?;
    let mut bytes:Vec<u8> = Vec::new();
    //writes Vec<u8> to bytes
    //_bytes_read returns num of bytes read
    let _bytes_read = file.read_to_end(&mut bytes);
    
/*// <DEBUG>
    println!("DUMP IMG BYTES ...");
    println!("{}", _bytes_read.unwrap());
    //<BoilerPlate>

    //constructs printable string of hex values
    let mut s = String::new();
    for &b in bytes.iter(){
        write!(&mut s, "{:X} ", b).expect("Unable to write");
    }
    //</BoilerPlate>

    // dump bytes to console
    println!("{}", s);
// </DEBUG> */
    
    Some(bytes) // return some bytes :p
}

