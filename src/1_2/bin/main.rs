use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


fn main() {
    let mut lines = lines_from_file("data/1.inp");
    lines.sort_unstable();

    println!("{}", find_2020_multiplication(lines));
}


pub fn find_2020_multiplication(vec: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut k = 1;
    let mut j = vec.len() - 1;
    loop {
        let addition = vec.get(i).unwrap() + vec.get(j).unwrap() + vec.get(k).unwrap();
        if addition > 2020 {
            j = j - 1;
            k = i + 1;
        } else if addition < 2020 {
            k = k + 1;
            if k == j {
                i = i + 1;
                k = i + 1;
            }
        } else {
            return vec.get(i).unwrap() * vec.get(j).unwrap() * vec.get(k).unwrap();
        }
    }
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_easy() {
        let mut v = vec![1721, 979, 366, 299, 675, 1456];
        v.sort_unstable();
        assert_eq!(find_2020_multiplication(v), 241861950);
    }

    #[test]
    fn test_hard() {
        let mut v = vec![1721, 979, 366, 299, 675, 1456, 11, 22, 33, 23000, 34000];
        v.sort_unstable();
        assert_eq!(find_2020_multiplication(v), 241861950);
    }
}