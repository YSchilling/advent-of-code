use regex::Match;
use regex::Regex;
use std::fs;

#[allow(dead_code)]
fn first_puzzle() {
    // searched: the sum of all numbers
    let mut sum = 0;
    // read file
    let file_path = "/mnt/e/dev/github/advent-of-code/2023/day1/input.txt";
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = content.split("\n");
    for line in lines {
        // filter line
        let string_nums: Vec<char> = line.chars().filter(|char| char.is_ascii_digit()).collect();
        // take first and last appearing number in string
        // if only one number is present, repeat it
        let first = string_nums.first();
        if first.is_none() {
            continue;
        }
        let first = first.unwrap();
        let last = string_nums.last().unwrap();
        let mut string_num = String::new();
        string_num.push(*first);
        string_num.push(*last);
        let num: u32 = string_num.parse().unwrap();

        // add num to sum
        sum += num;
    }

    println!("{}", sum);
}

fn second_puzzle() {
    fn converter(string: &str) -> i32 {
        match string {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!(),
        }
    }
    //i need to adjust the filtering
    //then match strings to convert all to char numbers
    // searched: the sum of all numbers
    let mut sum = 0;
    // read file
    let file_path = "/mnt/e/dev/github/advent-of-code/2023/day1/input.txt";
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = content.split("\n");

    let re = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine")
        .expect("Regex could not be parsed");
    let reverse_re = Regex::new(r"[0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin")
        .expect("Regex could not be parsed");

    for line in lines {
        // filter line
        let first_match_matches = re.find_iter(line).collect::<Vec<Match>>();
        // if there are no numbers in the line
        if first_match_matches.first().is_none() {
            continue;
        }
        let reversed_line = line.chars().rev().collect::<String>();
        let last_match_matches = reverse_re.find_iter(&reversed_line).collect::<Vec<Match>>();

        let first_str = first_match_matches.first().unwrap().as_str();

        let last_str = last_match_matches
            .first()
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>();

        let first_num = converter(first_str);
        let last_num = converter(&last_str);

        let num = first_num * 10 + last_num;

        // add num to sum
        sum += num;
    }

    println!("{}", sum);
}

fn main() {
    second_puzzle();
}
