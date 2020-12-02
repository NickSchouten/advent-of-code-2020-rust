use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let file = File::open("data/2.inp").expect("Could not open file!");
    let reader = BufReader::new(file);
    println!("{}", count_correct_passwords(reader));
}


pub fn count_correct_passwords(reader: BufReader<File>) -> usize {
    return reader.lines().filter(|s| is_correct_password(s.as_ref().expect("line not parsable"))).count();
}

pub fn is_correct_password(s: &String) -> bool {
    let values: Vec<&str> = s.split(" ").collect();
    let amount_range: Vec<&str> = values.get(0).unwrap().split("-").collect();
    let amount_min: usize = amount_range.get(0).unwrap().parse().unwrap();
    let amount_max: usize = amount_range.get(1).unwrap().parse().unwrap();
    let letter: &str = values.get(1).unwrap().trim_end_matches(':');
    let password = values.get(2).unwrap();

    let amount_of_matching_letters = password.matches(letter).count();
    amount_of_matching_letters >= amount_min && amount_of_matching_letters <= amount_max
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_easy() {
        assert_eq!(is_correct_password(&String::from("1-3 a: abcde")), true);
        assert_eq!(is_correct_password(&String::from("1-3 b: cdefg")), false);
        assert_eq!(is_correct_password(&String::from("2-9 c: ccccccccc")), true);
    }
}