use std::{
    io::{stdin, stdout, BufRead, Error, Write},
    process::Command,
    time::Duration,
};
pub const SECOND: Duration = Duration::from_secs(1);

pub fn duration() -> Duration {
    let mut h_m_s = Vec::<u32>::new();

    'Source: loop {
        for part in input("Set up duration(%H:%M:%S): ", b'\n').split(':') {
            if (part == "") {
                h_m_s.push(0);
                continue;
            }

            match part.parse() {
                Ok(value) => h_m_s.push(value),
                Err(error) => {
                    print("{}\n", &[error]);
                    h_m_s.clear();
                    continue 'Source;
                },
            }
        }

        if (h_m_s.len() != 3) {
            print::<u8>("Must be three values seperated by ':'\n", &[]);
            h_m_s.clear();
            continue;
        }

        let (h, m, s) = (h_m_s[0], h_m_s[1], h_m_s[2]);
        let (hs, cond1) = h.overflowing_mul(3600);
        let (ms, cond2) = m.overflowing_mul(60);
        let (hms, cond3) = hs.overflowing_add(ms);
        let (seconds, cond4) = hms.overflowing_add(s);

        if (cond1 || cond2 || cond3 || cond4) {
            print::<u8>("Overflow(max 32 bits)\n", &[]);
            continue;
        }

        return seconds * SECOND;
    }
}
//
pub fn input(prompt: &str, delimiter: u8) -> String {
    let mut buffer = Vec::<u8>::new();
    print("{}", &[prompt]);
    stdout().flush();
    stdin().lock().read_until(delimiter, &mut buffer);

    return std::str::from_utf8(&buffer)
        .unwrap()
        .trim_end_matches(delimiter as char)
        .to_owned();
}
//
pub fn execute(commands: &str, sync: bool) -> Result<(), Error> {
    let mut command = Command::new("sh").args(["-c", commands]).spawn()?;

    if (sync) {
        _ = command.wait()?;
    }

    return Ok(());
}
//////////////////////////////////////////////////////////////////////
pub fn print<Type: std::fmt::Display>(format: &str, objects: &[Type]) {
    let mut output = format.to_owned();

    let mut index: usize = 0;
    let placeholders = format.split("{}").count() - 1;
    while (index < placeholders) {
        output = output.replacen("{}", &objects[index].to_string(), 1);

        index += 1;
    }

    stdout().write_all(output.as_bytes());
}
