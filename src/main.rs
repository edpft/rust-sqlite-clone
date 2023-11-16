use std::io::{self, Write};

fn main() -> io::Result<()> {
    print_prompt();
    let lines = io::stdin().lines();
    for line in lines {
        let Ok(line) = line else {
            let error = io::Error::new(io::ErrorKind::InvalidInput, "Error reading input");
            return Err(error);
        };
        let command: Result<Command, String> = line.try_into();
        match command {
            Ok(Command::Exit) => {
                return Ok(());
            }
            Err(unrecognized_command) => {
                eprintln!("Unrecognized command: {}", unrecognized_command)
            }
        };
        print_prompt();
    }
    Ok(())
}

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

enum Command {
    Exit,
}

impl TryFrom<String> for Command {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            ".exit" => Ok(Self::Exit),
            _ => Err(value),
        }
    }
}
