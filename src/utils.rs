use std::fmt::Debug;
use std::fs::File;
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

pub(crate) fn get_input_as_matrix<T>(path: &str, mut parse: impl FnMut(u8) -> T) -> Vec<Vec<T>> {
    let file = get_buffered_input(path);
    let mut contents = Vec::with_capacity(128);

    for line in file.lines() {
        contents.push(line.unwrap().bytes().map(|b| parse(b)).collect());
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

    file.read_line(&mut line).unwrap();
    line.trim()
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect()
}

fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}
