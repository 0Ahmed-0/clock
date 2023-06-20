use std::{
    io::{stdout, Error, Write},
    process::exit,
    thread::{self, sleep},
};

use time::OffsetDateTime;
mod functions;
use functions::{execute, input, print, SECOND};

fn main() {
    loop {
        match &input("What to do('alarm', 'timer', 'stopwatch'): ", b'\n')[0..] {
            "alarm" => alarm(),
            "timer" => timer(),
            "stopwatch" => stopwatch(),
            _ => {
                print::<u8>("Type one of ('alarm', 'timer', 'stopwatch')\n", &[]);
                continue;
            },
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
                print("{}\n", &[error]);
                continue;
            },
        }
    };

    let mut minutes = (alarm_time - OffsetDateTime::now_local().unwrap().time()).whole_minutes();

    if (minutes < 0) {
        minutes += 24 * 60;
    }

    let h = minutes / 60;
    let m = minutes % 60;
    print("Duration left: {}h {}m\n", &[h, m]);
    let commands = &input("\nEnter commands(delimiter is '#')...\n", b'#');
    print::<u8>("\nAwaiting...\n", &[]);

    sleep((60 - OffsetDateTime::now_local().unwrap().second() as u32) * SECOND);
    while (alarm_time
        != OffsetDateTime::now_local()
            .unwrap()
            .time()
            .replace_nanosecond(0)
            .unwrap())
    {
        sleep(60 * SECOND);
    }

    to_happen(commands);
}

fn timer() {
    let mut secs = functions::duration().as_secs();
    let commands = &input("\nEnter commands(delimiter is '#')...\n", b'#');
    print::<u8>("\nAwaiting...\n", &[]);

    while (secs > 0) {
        sleep(SECOND);
        secs -= 1;
        let h = secs / 3600;
        let m = secs / 60 - 60 * h;
        let s = secs % 60;
        print("\r{}h {}m {}s ", &[h, m, s]);
        stdout().flush();
    }

    to_happen(commands);
}

fn stopwatch() {
    _ = thread::spawn(|| {
        _ = input("\nPress Enter to exit\n", b'\n');
        exit(0);
    });

    let mut seconds: u64 = 0;
    loop {
        sleep(SECOND);
        seconds += 1;
        print("\r{}s", &[seconds]);
        stdout().flush();
    }
}

fn to_happen(commands: &str) -> Result<(), Error> {
    std::fs::File::create("/tmp/temp.mp3")?.write_all(include_bytes!("../assets/sound.mp3"))?;
    execute(
        "for _ in {1..3}; do ffplay -autoexit -nodisp -loglevel error /tmp/temp.mp3; done;",
        false,
    )?;
    print::<u8>("\n\nExecuting...\n", &[]);

    if (commands != "#") {
        execute(commands, true)?;
    }

    exit(0);
}
