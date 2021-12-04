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

pub(crate) fn get_input_as_vec_with<T>(path: &str, f: impl Fn(&str) -> T) -> Vec<T> {
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(512);

    for line in file.lines() {
        contents.push(f(&line.unwrap()));
    }

    contents
}

pub(crate) fn get_input_as_vec_with_skipping<T>(
    path: &str,
    f: impl Fn(&str) -> T,
    to_skip: usize,
) -> Vec<T> {
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(512);

    for line in file.lines().skip(to_skip) {
        contents.push(f(&line.unwrap()));
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

pub(crate) fn parse_line<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut file = get_buffered_input(path);
    let mut line = String::new();
    file.read_line(&mut line);
    let line = line.trim_end();
    line.split(',').map(|elem| elem.parse().unwrap()).collect()
}
