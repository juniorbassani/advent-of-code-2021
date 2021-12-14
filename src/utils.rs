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

pub(crate) fn split_map<T, E, F1, F2>(
    path: &str,
    sep: &str,
    mut f1: F1,
    mut f2: F2,
) -> (Vec<T>, Vec<E>)
where
    F1: FnMut(u8) -> T,
    F2: FnMut(&str) -> E,
{
    let file = fs::read_to_string(path).unwrap();
    let (lhs, rhs) = file.split_once(sep).unwrap();
    let (lhs, rhs) = (lhs.trim(), rhs.trim());

    (lhs.bytes().map(f1).collect(), rhs.lines().map(f2).collect())
}

pub(crate) fn get_input_as_matrix<T>(
    path: &str,
    mut parse: impl FnMut(u8) -> T + Copy,
) -> Vec<Vec<T>> {
    get_buffered_input(path)
        .lines()
        .map(|line| line.unwrap().bytes().map(parse).collect())
        .collect()
}

pub(crate) fn parse_one_line<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let line = get_buffered_input(path).lines().next().unwrap().unwrap();
    line.trim()
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect()
}

fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}
