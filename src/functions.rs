use std::io::{stdin, stdout, Write};
use std::process::Command;

pub fn input(prompt: &str, delimiter: &str) -> String {
    fn print<Type: std::fmt::Display>(object: Type, end: &str) {
        stdout().write((object.to_string() + end).as_bytes());
    }
    //
    let mut result = String::new();

    if (delimiter == "\n") {
        print(prompt, "");
        stdout().flush();
        stdin().read_line(&mut result);

        return result[..result.len() - 1].to_string();
    }

    print(prompt, "\n\n");

    loop {
        let mut text = String::new();

        stdin().read_line(&mut text);

        result += &text;

        if (text == "\n") {
            continue;
        }

        let length = text.len();
        if (&text[length - 2..length - 1] == delimiter) {
            result = result[..result.len() - 2].to_string();

            return result;
        }
    }
}
//
pub fn duration() -> std::time::Duration {
    let mut result = Vec::<u64>::new();

    'Source: loop {
        for part in input("\nSet up duration(%H:%M:%S): ", "\n").split(":") {
            if (part == "") {
                result.push(0);
                continue;
            }
            match part.parse() {
                Result::Ok(value) => result.push(value),
                Result::Err(error) => {
                    println!("{}", error);
                    result.clear();
                    continue 'Source;
                }
            }
        }

        if (result.len() != 3) {
            println!("Must be three values seperated by ':'");
            result.clear();
            continue;
        }

        return std::time::Duration::from_secs(result[2] + result[1] * 60 + result[0] * 3600);
    }
}
//
pub fn execute(commands: &str) {
    Command::new("sh")
        .args(["-c", commands])
        .spawn()
        .unwrap()
        .wait();
}
