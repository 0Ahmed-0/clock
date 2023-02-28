use std::thread::sleep;
use std::time::Duration;

mod functions;

use time::{format_description, OffsetDateTime, Time};

fn main() {
    loop {
        let mode: &str = &functions::input("What to do('alarm', 'timer'): ", "\n");

        match mode {
            "alarm" => {
                alarm();
                break;
            }
            "timer" => {
                timer();
                break;
            }
            _other => {
                println!("Type one of ('alarm', 'timer')\n");
                continue;
            }
        }
    }
}

fn alarm() {
    let format = format_description::parse("[hour]:[minute]:[second]").unwrap();

    let alarm_hms = loop {
        match Time::parse(
            &functions::input("\nSet up alarm(%H:%M:%S): ", "\n"),
            &format,
        ) {
            Ok(value) => break value.as_hms(),
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }
    };

    let commands = &functions::input("\nEnter commands(Delimiter is '#')...", "#");

    println!("\n\nAwating...");
    loop {
        if (alarm_hms == OffsetDateTime::now_local().unwrap().time().as_hms()) {
            to_happen(commands);
            break;
        }

        sleep(Duration::from_secs(1));
    }
}

fn timer() {
    let duration = functions::duration();

    let commands = &functions::input("\nEnter commands(Delimiter is '#')...", "#");

    println!("\n\nAwating...");
    sleep(duration);

    to_happen(commands);
}

fn to_happen(commands: &str) {
    if (commands != "#") {
        for _ in 1..=3 {
            functions::execute("espeak 'Time is up'");
            sleep(Duration::from_secs(1));
        }
    }

    println!("\n*******");
    functions::execute(commands);
    println!("*******");
}
