use std::time::{Instant};
use std::io;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum InputError {
    EmptyInput,
}

impl Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty string!"),
        }
    }
}

struct StopWatch;

fn capitalize_first_letter(string: &mut String) -> &String {
    string.get_mut(0..1).map(|letter| {
        letter.make_ascii_uppercase()
    });

    string
}

fn get_user_input() -> Result<String, InputError> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to readline");

    let mut buffer = buffer.trim().to_string();


    if buffer.is_empty() {
        return Err(InputError::EmptyInput)
    }

    if buffer.starts_with("s") {
        capitalize_first_letter(&mut buffer);
    }


    Ok(buffer)
}

impl StopWatch {

    fn start() -> Result<(), InputError> {
        println!("Stopwatch: Type in \"Start\" to begin and \"Stop\" to end!");
        let mut buffer = get_user_input()?;

        while buffer.contains("Start") {
            let now = Instant::now();

            buffer = get_user_input()?;

            match buffer.contains("Stop") {
                true => {
                    println!("Total time elapsed: {} seconds!", now.elapsed().as_secs());
                    break
                }
                false => panic!("Invalid Input: Please type in the correct action."),
            }
        }
        Ok(())
    }

}

fn main() {

   let _ = match StopWatch::start() {
       Ok(stop_watch) => stop_watch,
       Err(e) => println!("Error: {}", e),
   };

}
