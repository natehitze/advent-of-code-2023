mod input;

use std::fmt::{Display, Formatter};

fn main() {
    let i = input::input::get_day_one_input();
    let mut sum = 0;
    let mut part_two_sum = 0;
    for line in i.lines() {
        sum += pair_to_int(get_first_last_in_line(line));
        part_two_sum += pair_to_int(get_first_last_in_line_part_two(line));
    }

    println!("Part 1 result is {sum}");
    println!("Part 2 result is {part_two_sum}");
}

fn pair_to_int(pair: (i32, i32)) -> i32 {
    if pair.0 == -1 || pair.1 == -1 {
        panic!("-1 is in this pair");
    }

    pair.0 * 10 + pair.1
}

fn get_first_last_in_line(line: &str) -> (i32, i32) {
    let mut first = -1i32;
    let mut second = -1i32;
    for c in line.chars() {
        if let Some(number) = c.to_digit(10) {
            if first == -1 {
                first = number as i32;
            }
            else {
                second = number as i32;
            }
        }
    }

    if second == -1 {
        second = first;
    }

    (first, second)
}

fn get_first_last_in_line_part_two(line: &str) -> (i32, i32) {
    let mut first = -1i32;
    let mut second = -1i32;

    let mut numbers = [
        StringStateMachine::new("one", 1),
        StringStateMachine::new("two", 2),
        StringStateMachine::new("three", 3),
        StringStateMachine::new("four", 4),
        StringStateMachine::new("five", 5),
        StringStateMachine::new("six", 6),
        StringStateMachine::new("seven", 7),
        StringStateMachine::new("eight", 8),
        StringStateMachine::new("nine", 9)
    ];

    for c in line.chars() {
        let mut needs_reset = false;

        //println!("{c} ---------------");
        if let Some(number) = c.to_digit(10) {
            if first == -1 {
                first = number as i32;
            }
            else {
                second = number as i32;
            }

            needs_reset = true;
        }
        else {
            for n in &mut numbers {
                let outcome = n.advance(c);
                //println!("{0}: {1} @ {2}", n.value, n.current_state, n.pos);
                match outcome {
                    AdvanceResult::Completed => {
                        if first == -1 {
                            first = n.value;
                        }
                        else {
                            second = n.value;
                        }

                        n.reset();
                    }
                    AdvanceResult::Mismatch => {
                        n.reset();
                    }
                    AdvanceResult::Continuing => {}
                    AdvanceResult::Initial => {}
                }
            }
        }

        if needs_reset {
            for n in &mut numbers {
                n.reset();
            }
        }
    }

    if second == -1 {
        second = first;
    }

    (first, second)
}

struct StringStateMachine {
    s: Vec<char>,
    value: i32,
    pos: usize,
    current_state: AdvanceResult,
}

#[derive(Clone)]
enum AdvanceResult {
    Completed,
    Mismatch,
    Continuing,
    Initial,
}

impl Display for AdvanceResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            AdvanceResult::Completed => { "Completed" }
            AdvanceResult::Mismatch => { "Mismatch" }
            AdvanceResult::Continuing => { "Continuing" }
            AdvanceResult::Initial => { "Initial" }
        };

        f.write_str(r)
    }
}

impl StringStateMachine {
    fn new(s: &str, value: i32) -> StringStateMachine {
        StringStateMachine {
            s: s.chars().collect(),
            pos: 0,
            value,
            current_state: AdvanceResult::Initial,
        }
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.current_state = AdvanceResult::Initial;
    }

    fn advance(&mut self, current: char) -> AdvanceResult {
        if self.s[self.pos] == current {
            self.pos += 1;
            if self.pos == self.s.len() {
                self.current_state = AdvanceResult::Completed;
            }
            else {
                self.current_state = AdvanceResult::Continuing;
            }
        }
        else if self.s[0] == current {
            self.pos = 1;
            self.current_state = AdvanceResult::Continuing;
        }
        else {
            self.current_state = AdvanceResult::Mismatch;
        }

        return self.current_state.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_first_last_in_line, get_first_last_in_line_part_two, pair_to_int};

    #[test]
    fn can_get_first_last_simple() {
        let (first, last) = get_first_last_in_line("1bc2");
        assert_eq!(first, 1);
        assert_eq!(last, 2);
    }

    #[test]
    fn can_get_first_last_middle() {
        let (first, last) = get_first_last_in_line("pqr3stu8vwx");
        assert_eq!(first, 3);
        assert_eq!(last, 8);
    }

    #[test]
    fn can_get_first_last_multi() {
        let (first, last) = get_first_last_in_line("a1b2c3d4e5f");
        assert_eq!(first, 1);
        assert_eq!(last, 5);
    }

    #[test]
    fn can_get_first_last_single() {
        let (first, last) = get_first_last_in_line("treb7uchet");
        assert_eq!(first, 7);
        assert_eq!(last, 7);
    }

    #[test]
    fn can_get_first_last_part2_simple() {
        let (first, last) = get_first_last_in_line_part_two("two1nine");
        assert_eq!(first, 2);
        assert_eq!(last, 9);
    }

    #[test]
    fn can_get_first_last_part2_only_words() {
        let (first, last) = get_first_last_in_line_part_two("eightwothree");
        assert_eq!(first, 8);
        assert_eq!(last, 3);
    }

    #[test]
    fn can_get_first_last_part2_simple_with_more_letters() {
        let (first, last) = get_first_last_in_line_part_two("abcone2threexyz");
        assert_eq!(first, 1);
        assert_eq!(last, 3);
    }

    #[test]
    fn can_get_first_last_part2_words_share_letters() {
        let (first, last) = get_first_last_in_line_part_two("xtwone3four");
        assert_eq!(first, 2);
        assert_eq!(last, 4);
    }

    #[test]
    fn can_get_first_last_part2_numbers() {
        let (first, last) = get_first_last_in_line_part_two("4nineeightseven2");
        assert_eq!(first, 4);
        assert_eq!(last, 2);
    }

    #[test]
    fn can_get_first_last_part2_words_share_letters_again() {
        let (first, last) = get_first_last_in_line_part_two("zoneight234");
        assert_eq!(first, 1);
        assert_eq!(last, 4);
    }

    #[test]
    fn can_get_first_last_part2_as_if_i_can_parse_big_numbers() {
        let (first, last) = get_first_last_in_line_part_two("7pqrstsixteen");
        assert_eq!(first, 7);
        assert_eq!(last, 6);
    }

    #[test]
    fn can_get_first_last_part2_one_off() {
        let (first, last) = get_first_last_in_line_part_two("sevenine");
        assert_eq!((first, last), (7,9));
    }

    #[test]
    fn can_get_first_last_part2_two_off() {
        let (first, last) = get_first_last_in_line_part_two("oneight");
        assert_eq!((first, last), (1,8));
    }

    #[test]
    fn can_get_first_last_part2_three_off() {
        let (first, last) = get_first_last_in_line_part_two("twoneight");
        assert_eq!((first, last), (2,8));
    }

    #[test]
    fn can_get_first_last_part2_reset() {
        let (first, last) = get_first_last_in_line_part_two("1ffive");
        assert_eq!((first, last), (1,5));
    }

    #[test]
    fn can_get_first_last_part2_spot_checks() {
        let (first, last) = get_first_last_in_line_part_two("onesevenlbsp6eightsixtwoninelpfl");
        assert_eq!((first, last), (1,9));
        let (first, last) = get_first_last_in_line_part_two("k3");
        assert_eq!((first, last), (3,3));
        let (first, last) = get_first_last_in_line_part_two("sevenllscmf6sjqbjvdqzd8khxvpninezctzf8");
        assert_eq!((first, last), (7,8));
        let (first, last) = get_first_last_in_line_part_two("jmbqfour1eightwot");
        assert_eq!((first, last), (4,2));
        let (first, last) = get_first_last_in_line_part_two("pksixseven9vthrzfouroneightlvr");
        assert_eq!((first, last), (6,8));
        let (first, last) = get_first_last_in_line_part_two("1ffive");
        assert_eq!((first, last), (1,5));
    }

    #[test]
    fn can_get_first_last_part2_can_read() {
        let numbers = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ];

        for n in 1..=9i32 {
            let (first, last) = get_first_last_in_line_part_two(numbers[(n - 1) as usize].0);
            assert_eq!((first, last), (n, n))
        }
    }

    #[test]
    fn pair_can_convert() {
        let result = pair_to_int((3, 8));
        assert_eq!(result, 38);
    }
}