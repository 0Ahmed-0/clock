pub fn input(prompt: &str, delimiter: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut result = String::new();

    if (delimiter == "\n") {
        print("{}", &[prompt]);
        stdout().flush();
        stdin().read_line(&mut result);

        return result[..result.len() - 1].to_string();
    }

    print("{}\n\n", &[prompt]);
    loop {
        let mut text = String::new();

        stdin().read_line(&mut text);

        result += &text;

        if (text == "\n") {
            continue;
        }

        let length = text.len();
        if (&text[length - 2..length - 1] == delimiter) {
            return result[..result.len() - 2].to_string();
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
                Ok(value) => result.push(value),
                Err(error) => {
                    print("{}\n", &[error]);
                    result.clear();
                    continue 'Source;
                }
            }
        }

        if (result.len() != 3) {
            print("Must be three values seperated by ':'\n", &[""]);
            result.clear();
            continue;
        }

        return std::time::Duration::from_secs(result[0] * 3600 + result[1] * 60 + result[2]);
    }
}
//
pub fn execute(commands: &str, sync: bool) {
    let mut binding = std::process::Command::new("sh");
    let command = binding.args(["-c", commands]);

    if sync {
        command.spawn().unwrap().wait();
    } else {
        command.spawn();
    }
}
//////////////////////////////////////////////////////////////////////
pub fn print<Type: std::fmt::Display>(format: &str, objects: &[Type]) {
    use std::io::{stdout, Write};

    let mut content = String::from(format);

    let mut index: usize = 0;
    let placeholders = format.split("{}").count() - 1;
    while (index < placeholders) {
        content = content.replacen("{}", &(objects[index].to_string()), 1);
        index += 1;
    }

    stdout().write(content.as_bytes());
}
