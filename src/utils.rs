#![allow(unused)]

use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}

pub(crate) fn get_input_as_string(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't open file")
}

pub(crate) fn get_input_as_vec<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(512);

    for line in file.lines() {
        contents.push(line.unwrap().parse().unwrap());
    }

    contents
}

pub(crate) fn get_input_as_matrix(path: &str) -> Vec<Vec<char>> {
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(128);

    for line in file.lines() {
        contents.push(line.unwrap().chars().collect());
    }

    contents
}
