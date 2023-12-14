use std::{io, error::Error};

fn main() {
    loop {
        match next_cmd() {
            Ok(has_next) => {
                if !has_next {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Encountered error: {}", err);
            },
        }
    }
}

fn next_cmd() -> Result<bool, Box<dyn Error>> {
    let mut raw_input = String::new();

    io::stdin().read_line(&mut raw_input)?;

    let trimmed_input = raw_input.trim();

    if trimmed_input.len() == 0 {
        return Ok(true);
    }

    let mut split = trimmed_input.split_whitespace();
    let cmd = split.next().unwrap_or("");
    let args: Vec<&str> = split.collect();

    if cmd.to_lowercase() == "exit" {
        println!("Exiting");
        return Ok(false);
    }

    handle_cmd(cmd, args)?;

    Ok(true)
}

fn handle_cmd(cmd: &str, args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // TODO
    println!("cmd={}, args={:?}", cmd, args);

    Ok(())
}