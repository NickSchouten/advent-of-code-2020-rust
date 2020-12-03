use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


fn main() {
    let lines = lines_from_file("data/3.inp");

    println!("{}", find_multiplied_nb_of_trees(&lines));
}

pub fn find_multiplied_nb_of_trees(vec: &Vec<Vec<bool>>) -> u128 {
    find_nb_of_trees(vec, 3, 1) * find_nb_of_trees(vec, 1, 1) * find_nb_of_trees( vec, 5, 1) * find_nb_of_trees(vec, 7, 1) * find_nb_of_trees(vec, 1, 2)
}


pub fn find_nb_of_trees(vec: &Vec<Vec<bool>>, right: usize, down: usize) -> u128 {
    let mut r = 0;
    let mut d = 0;
    let mut amount_of_trees = 0;
    let modulo = vec.get(0).unwrap().len();
    loop {
        amount_of_trees = amount_of_trees + (*vec.get(d).unwrap().get(r).unwrap() as u128);
        if d + down >= vec.len() {
            return amount_of_trees;
        }

        r = (r + right) % modulo;
        d = d + down;
    }
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<bool>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().chars()
            .map(|char| tree_to_bool(char))
            .collect())
        .collect()
}

fn tree_to_bool(char: char) -> bool {
    char == '#'
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tree_to_bool() {
        assert_eq!(tree_to_bool('#'), true);
        assert_eq!(tree_to_bool('.'), false);
    }

    #[test]
    fn test_1() {
        let v = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        let r = v.iter().map(|l| l.chars()
            .map(|char| tree_to_bool(char))
            .collect())
            .collect();
        assert_eq!(find_nb_of_trees(&r, 3, 1), 7);
    }

    #[test]
    fn test_2() {
        let v = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        let r = v.iter().map(|l| l.chars()
            .map(|char| tree_to_bool(char))
            .collect())
            .collect();
        assert_eq!(find_nb_of_trees(&r, 1, 1), 2);
    }

    #[test]
    fn test_3() {
        let v = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        let r: Vec<Vec<bool>> = v.iter().map(|l| l.chars()
            .map(|char| tree_to_bool(char))
            .collect())
            .collect();
        assert_eq!(find_nb_of_trees(&r, 5, 1), 3);
    }

    #[test]
    fn test_full() {
        let v = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ];
        let r: Vec<Vec<bool>> = v.iter().map(|l| l.chars()
            .map(|char| tree_to_bool(char))
            .collect())
            .collect();
        assert_eq!(find_multiplied_nb_of_trees(&r), 336);
    }
}