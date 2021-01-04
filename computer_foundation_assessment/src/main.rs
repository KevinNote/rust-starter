use std::io::{stdout, Write, stdin};
use rand::Rng;
use std::process::exit;

const ENTER_OPTION: &str = "PLZ ENTER OPTION: ";
const WRONG_OPTION_RANGE: &str = "WRONG OPTION RANGE DETECTED!";

fn main() {
    loop {
        println!("1. Multiplication Table");
        println!("2. Test");
        println!("3. Exit");
        let option = get_option(1, 3, ENTER_OPTION, WRONG_OPTION_RANGE);
        match option {
            1 => print_multiplication_table(),
            2 => test(),
            _ => {
                println!("Bye!");
                exit(0);
            }
        }
    }
}

fn test() {
    println!("Type selection");
    println!("1. +");
    println!("2. -");
    println!("3. *");
    println!("4. /");
    println!("5. mixed");
    secure_print(ENTER_OPTION);
    let opera = get_option(1, 5, ENTER_OPTION, WRONG_OPTION_RANGE);
    println!("Range selection");
    println!("1. 0-10");
    println!("2. 0-100");
    println!("3. 0-1000");
    let range = (10 as i32).pow(get_option(1, 3, ENTER_OPTION, WRONG_OPTION_RANGE) as u32);
    let mut correct_counter = 0;
    for _i in 0..10 {
        let answer = get_test_question(opera, range);

        let user_answer: i32;
        loop {
            let mut t = String::new();
            stdin().read_line(&mut t).unwrap();
            let user_answer_result = t.trim().parse();
            user_answer = match user_answer_result {
                Ok(v) => v,
                _ =>
                    {
                        secure_print("WRONG VAR TYPE! ENTER AGAIN: ");
                        continue;
                    }
            };
            break;
        }
        if user_answer == answer {
            println!("YOU ARE RIGHT!");
            correct_counter += 1;
        } else {
            println!("YOU ARE WRONG! ANSWER IS {}", answer);
        }
    }
    println!("FINISHED! YOUR SCORE IS {}/10", correct_counter);
}

fn get_test_question(option: i16, range: i32) -> i32 {
    let mut rng = rand::thread_rng();

    if option == 5 {
        get_test_question(rng.gen_range(1..5), range)
    } else {
        let mut first_number = rng.gen_range(0..(range + 1));
        let mut second_number = rng.gen_range(0..(range + 1));
        let result: i32;
        match option {
            1 => {
                print!("{} + {} = ", first_number, second_number);
                result = first_number + second_number;
            }
            2 => {
                print!("{} - {} = ", first_number, second_number);
                result = first_number - second_number;
            }
            3 => {
                print!("{} * {} = ", first_number, second_number);
                result = first_number * second_number;
            }
            4 => {
                while second_number == 0 || first_number % second_number != 0 {
                    first_number = rng.gen_range(0..(range + 1));
                    second_number = rng.gen_range(0..(range + 1));
                }
                print!("{} / {} = ", first_number, second_number);
                result = first_number / second_number;
            }
            _ => {
                panic!("{} IS OUT OF EXPECT!", option);
            }
        };
        stdout().flush().unwrap();
        result
    }
}

fn get_option(min: i16, max: i16, out: &str, err_out: &str) -> i16 {
    let mut result: i16;
    loop {
        let mut input = String::new();
        secure_print(out);
        stdin().read_line(&mut input).unwrap();
        let parse_result = input.trim().parse::<i16>();
        result = match parse_result {
            Ok(v) => v,
            _ => min - 1
        };
        if result > max || result < min {
            println!("{}", err_out);
        } else {
            break;
        }
    };
    result
}

// print! does not flush automatically, so a secure_print is required!
fn secure_print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}

fn print_multiplication_table() {
    let lines = get_option(5, 15, ENTER_OPTION, WRONG_OPTION_RANGE);
    //let lines: i16 = 15;
    secure_print("x\t");
    for i in 1..(lines + 1) {
        print!("{}\t", i);
    }
    println!();
    for i in 1..(lines + 1) {
        print!("{}\t", i);
        for j in 1..(lines + 1) {
            print!("{}\t", i * j);
        }
        println!();
    }
}