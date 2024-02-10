use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(file_name: &str) {
    println!("now reading file [{}]", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}\t\t ({} bytes long)", line, line.len());
    }
}
