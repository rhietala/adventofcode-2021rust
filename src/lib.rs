use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::io::Lines;
use std::fs::File;

fn read_file(filename: &str) -> Result<Lines<BufReader<File>>, Error> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

pub fn read_file_i32(filename: &str) -> Vec<i32> {
    read_file(filename).unwrap()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect()
}

pub fn read_file_string(filename: &str) -> Vec<String> {
    read_file(filename).unwrap()
        .map(|l| l.unwrap())
        .collect()
}
