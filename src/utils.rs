#![allow(unused)]

use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) fn get_input_as_vec<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    get_input_as_vec_with(path, |line| line.parse().unwrap())
}

pub(crate) fn get_input_as_vec_with<T>(path: &str, mut f: impl FnMut(&str) -> T) -> Vec<T> {
    get_buffered_input(path)
        .lines()
        .map(|line| f(&line.unwrap()))
        .collect()
}

pub(crate) fn get_input_as_vec_with_skipping<T>(
    path: &str,
    mut f: impl FnMut(&str) -> T,
    to_skip: usize,
) -> Vec<T> {
    get_buffered_input(path)
        .lines()
        .skip(to_skip)
        .map(|line| f(&line.unwrap()))
        .collect()
}

pub(crate) fn get_input_as_matrix(path: &str) -> Vec<Vec<char>> {
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(128);

    for line in file.lines() {
        contents.push(line.unwrap().chars().collect());
    }

    contents
}

pub(crate) fn parse_one_line<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut file = get_buffered_input(path);
    let mut line = String::new();

    file.read_line(&mut line);
    line.trim()
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect()
}

fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}
