use std::{
    io::{stdout, Write},
    process::exit,
    thread::{self, sleep},
};
const second: std::time::Duration = std::time::Duration::from_secs(1);

mod functions;
use functions::{execute, input, print};

use time::OffsetDateTime;

fn main() {
    loop {
        match input("What to do('alarm', 'timer', 'stopwatch'): ", "\n").as_str() {
            "alarm" => alarm(),
            "timer" => timer(),
            "stopwatch" => stopwatch(),
            _ => {
                print("Type one of ('alarm', 'timer', 'stopwatch')\n\n", &[""]);
                continue;
            }
        }
    }
}

fn alarm() {
    let alarm_time = loop {
        match time::Time::parse(
            &input("\nSet up alarm(%H:%M): ", "\n"),
            &(time::format_description::parse("[hour]:[minute]").unwrap()),
        ) {
            Ok(value) => break value,
            Err(error) => {
                print("{}\n", &[error]);
                continue;
            }
        }
    };

    let mut minutes = (alarm_time - OffsetDateTime::now_local().unwrap().time()).whole_minutes();
    if (minutes < 0) {
        minutes += 60 * 24;
    }
    let (h, m) = (minutes / 60, minutes % 60);
    print("Duration left: {}h {}m\n", &[h, m]);

    let commands = &input("\nEnter commands(Delimiter is '#')...", "#");

    print("\n\nAwaiting...\n", &[""]);
    sleep(second * (60 - OffsetDateTime::now_local().unwrap().second() as u32));
    while (alarm_time
        != OffsetDateTime::now_local()
            .unwrap()
            .time()
            .replace_nanosecond(0)
            .unwrap())
    {
        sleep(second * 60);
    }

    to_happen(commands);
}

fn timer() {
    let mut secs = functions::duration().as_secs();

    let commands = &input("\nEnter commands(Delimiter is '#')...", "#");

    print("\n\nAwaiting...\n", &[""]);
    while (secs > 0) {
        sleep(second);
        secs -= 1;
        let h = secs / 3600;
        let m = secs / 60 - h * 60;
        let s = secs % 60;
        print("\r{}h {}m {}s ", &[h, m, s]);
        stdout().flush();
    }

    to_happen(commands);
}

fn stopwatch() {
    thread::spawn(|| {
        input("\nPress Enter to exit\n", "\n");
        exit(0);
    });

    let mut seconds: u64 = 0;
    loop {
        sleep(second);
        seconds += 1;
        print("\r{}s", &[seconds]);
        stdout().flush();
    }
}

fn to_happen(commands: &str) {
    std::fs::File::create("/tmp/temp.mp3")
        .unwrap()
        .write_all(include_bytes!("../assets/sound.mp3"));
    execute(
        "for _ in {1..3}; do ffplay -autoexit -nodisp -loglevel error /tmp/temp.mp3; done;",
        false,
    );

    print("\nExecuting...\n*******\n", &[""]);
    execute(commands, true);

    exit(0);
}
