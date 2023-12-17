use std::io::{stdin, stdout, BufRead, Error, Write};
use std::process::Command;
use std::time::Duration;

pub const SECOND: Duration = Duration::from_secs(1);

pub fn duration() -> Duration {
    let mut h_m_s = Vec::new();

    'Source: loop {
        let mut input = String::new();
        print!("Set up duration(%H:%M:%S): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        for part in input.trim_end().split(':') {
            if part.is_empty() {
                h_m_s.push(0);
                continue;
            }

            match part.parse::<u64>() {
                Ok(value) => h_m_s.push(value),
                Err(err) => {
                    println!("{}", err);
                    h_m_s.clear();
                    continue 'Source;
                }
            }
        }

        if h_m_s.len() != 3 {
            println!("Must be three values seperated by ':'");
            h_m_s.clear();
            continue;
        }

        let (h, m, s) = (h_m_s[0], h_m_s[1], h_m_s[2]);
        let (hs, cond1) = h.overflowing_mul(3600);
        let (ms, cond2) = m.overflowing_mul(60);
        let (hms, cond3) = hs.overflowing_add(ms);
        let (seconds, cond4) = hms.overflowing_add(s);

        if cond1 || cond2 || cond3 || cond4 {
            println!("Overflowed");
            continue;
        }

        return Duration::from_secs(seconds);
    }
}

pub fn input(prompt: &str, delimiter: u8) -> String {
    let mut buffer = Vec::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().lock().read_until(delimiter, &mut buffer).unwrap();

    std::str::from_utf8(&buffer)
        .unwrap()
        .trim_end_matches(delimiter as char)
        .to_owned()
}

pub fn execute(commands: &str, sync: bool) -> Result<(), Error> {
    let mut command = Command::new("sh").args(["-c", commands]).spawn()?;

    if sync {
        _ = command.wait()?;
    }

    Ok(())
}
