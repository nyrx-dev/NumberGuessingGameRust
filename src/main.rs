use rand::random_range;
use std::{cmp::Ordering};
use std::process;

use crate::GuessState::{Correct, TooHigh, TooLow};


fn main() {
    let answer: i32 = create_number();
    let mut tries_left: i32 = 3;
    
    println!("Guess a number, you have 3 tries: ");

    while !is_game_over(tries_left){
        let guess = match player_turn(){
            Ok(number) => number,
            Err(message) => {
                println!("{message}. Enter a whole number from 0 to 10.");
                continue;
            }
        };

        let remaining = update_tries(tries_left);
        match check_guess(guess, answer) {
            Correct => {
                println!("Correct, took you {} guesses", 3 - remaining);
                process::exit(0);
            }
            TooLow if remaining > 0 => {
                println!("Too low, try again, {} guesses left", remaining)
            }
            TooHigh if remaining > 0 => {
                println!("Too high, try again, {} guesses left", remaining)
            }
            TooLow | TooHigh => {}
        }

        tries_left = remaining

    }

    println!("You lost! Failed to guess the number: {}", answer)

}

fn player_turn() ->  Result<i32, String>{
    let mut player_input = String::new();

    std::io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input");

    validate_guess(player_input.trim())
}

fn create_number() -> i32 {
    random_range(0..11)
}

fn check_guess(x: i32, y: i32) -> GuessState {
    match x.cmp(&y) {
        Ordering::Equal => GuessState::Correct,
        Ordering::Greater => GuessState::TooHigh,
        Ordering::Less => GuessState::TooLow,
    }
}

#[derive(Debug, PartialEq)]
enum GuessState {
    Correct,
    TooHigh,
    TooLow,
}


fn string_2_int(x: &str) -> Result<i32, std::num::ParseIntError> {
    x.parse::<i32>()
}

fn validate_guess(guess: &str) -> Result<i32, String> {
    match string_2_int(guess) {
        Ok(number) => {
            if !(0..=10).contains(&number) {
                Err("Out of bound".to_string())
            } else {
                Ok(number)
            }
        }
        Err(_) => Err("Not a number".to_string()),
    }
}

fn update_tries(x: i32) -> i32 {
    x - 1
}

fn is_game_over(tries_left: i32) -> bool {
    tries_left == 0
}


#[cfg(test)]
mod tests {
    use super::*;

    mod randomnumber_test {
        use super::*;

        #[test]
        fn number_range_test() {
            let range = 0..11;

            for _ in 0..100 {
                let number = create_number();

                assert!(range.contains(&number))
            }
        }
    }

    mod check_guess_test {
        use super::*;

        #[test]
        fn check_guess_correct() {
            assert_eq!(check_guess(5, 5), GuessState::Correct)
        }

        #[test]
        fn check_guess_high() {
            assert_eq!(check_guess(8, 5), GuessState::TooHigh)
        }

        #[test]
        fn check_guess_low() {
            assert_eq!(check_guess(2, 5), GuessState::TooLow)
        }
    }

    mod input_test {
        use super::*;

        #[test]
        fn check_input(){
            let input: &str = "5";
            assert_eq!(string_2_int(input), Ok(5));
        }

        #[test]
        fn validate_input_inbound(){
            assert_eq!(
                validate_guess("12"),
                Err("Out of bound".to_string())
            );
        }

        #[test]
        fn validate_input_number(){
            assert_eq!(
                validate_guess("Hello"),
                Err("Not a number".to_string())
            )
        }
     }

    mod tries_test {
        use super::*;

        #[test]
        fn wrong_guess(){
            assert_eq!(update_tries(3), 2)
        }

    }

    mod game_over_test{
        use super::*;

        #[test]
        fn game_lose(){
            assert!(is_game_over(0));
        }

        #[test]
        fn game_conintue(){
            assert!(!is_game_over(1));
        }
    }
}
