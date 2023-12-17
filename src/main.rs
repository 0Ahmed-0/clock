use std::io::{stdout, Error, Write};
use std::process;
use std::thread;
mod functions;
use functions::{execute, input, SECOND};

use time::OffsetDateTime;

fn main() {
    loop {
        match &input("What to do('alarm', 'timer', 'stopwatch'): ", b'\n')[..] {
            "alarm" => {
                alarm();
                break;
            }
            "timer" => {
                timer();
                break;
            }
            "stopwatch" => stopwatch(),
            _ => {
                println!("Type one of ('alarm', 'timer', 'stopwatch')");
                continue;
            }
        }
    }
}

fn alarm() {
    let alarm_time = loop {
        match time::Time::parse(
            &input("Set up alarm(%H:%M): ", b'\n'),
            &(time::format_description::parse_borrowed::<2>("[hour]:[minute]").unwrap()),
        ) {
            Ok(value) => break value,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }
    };

    let mut minutes = (alarm_time - OffsetDateTime::now_local().unwrap().time()).whole_minutes();

    if minutes < 0 {
        minutes += 24 * 60;
    }

    let h = minutes / 60;
    let m = minutes % 60;
    println!("Duration left: {}h {}m", h, m);
    let commands = &input("\nEnter commands(delimiter is '#')...\n", b'#');
    println!("\nAwaiting...");

    thread::sleep((60 - OffsetDateTime::now_local().unwrap().second() as u32) * SECOND);
    while alarm_time
        != OffsetDateTime::now_local()
            .unwrap()
            .time()
            .replace_nanosecond(0)
            .unwrap()
    {
        thread::sleep(60 * SECOND);
    }

    to_happen(commands).unwrap();
}

fn timer() {
    let mut secs = functions::duration().as_secs();
    let commands = &input("\nEnter commands(delimiter is '#')...\n", b'#');
    println!("\nAwaiting...");

    while secs > 0 {
        thread::sleep(SECOND);
        secs -= 1;
        let h = secs / 3600;
        let m = secs / 60 - 60 * h;
        let s = secs % 60;
        print!("\r{}h {}m {}s ", h, m, s);
        stdout().flush().unwrap();
    }

    to_happen(commands).unwrap();
}

fn stopwatch() {
    _ = thread::spawn(|| {
        _ = input("\nPress Enter to exit\n", b'\n');
        process::exit(0);
    });

    let mut seconds: u64 = 0;
    loop {
        thread::sleep(SECOND);
        seconds += 1;
        print!("\r{}s", seconds);
        stdout().flush().unwrap();
    }
}

fn to_happen(commands: &str) -> Result<(), Error> {
    std::fs::File::create("/tmp/temp.mp3")?.write_all(include_bytes!("../assets/sound.mp3"))?;

    let handle = thread::spawn(|| {
        for _ in 1..=3 {
            execute("paplay /tmp/temp.mp3", true).unwrap();
        }
    });

    println!("\n\nExecuting...");

    if commands != "#" {
        execute(commands, true)?;
    }

    handle.join().unwrap();

    Ok(())
}
