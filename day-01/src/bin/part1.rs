// Trebuchet?!
// Solution:
// 1. Read input file
// 2. Iterate over each line
// 3. Iterate from the start of the line, until it finds a number
// 4. Iterate from the end of the line, until it finds a number
// 5. Concatenate the two numbers
// 6. Push the 2-digit number to a vec (a list)
// 7. Sum the vec

fn main() {
    let input =
        std::fs::read_to_string("././input/input.txt")
            .unwrap();

    let lines = input
        .lines()
        .collect::<Vec<&str>>();

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));
}

fn part_1(lines: &[&str]) -> u32 {
    let mut total = 0;

    for line in lines {
        let line = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>();

        let number = line[0].to_digit(10).unwrap() * 10
            + line[line.len() - 1]
                .to_digit(10)
                .unwrap();

        total += number;
    }

    total
}

fn replace_prefix(line: &str) -> (char, bool) {
    let prefixes = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for (prefix, character) in prefixes {
        if line.starts_with(prefix) {
            return (character, true);
        }
    }

    (' ', false)
}

fn part_2(lines: &[&str]) -> u32 {
    let mut total = 0;

    for line in lines {
        let mut first = ' ';
        let mut last = ' ';

        let chars = line
            .chars()
            .collect::<Vec<char>>();

        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                first = chars[i];
                break;
            }
            let (c, found) = replace_prefix(&line[i..]);
            if found {
                first = c;
                break;
            }
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_ascii_digit() {
                last = chars[i];
                break;
            }
            let (c, found) = replace_prefix(&line[i..]);
            if found {
                last = c;
                break;
            }
        }
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        let number = first * 10 + last;
        total += number;
    }

    total
}
