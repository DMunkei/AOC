use anyhow::{Ok, Result};
use std::str::FromStr;

#[derive(Debug)]
struct CraneCommands {
    commands: Vec<Command>,
}

impl CraneCommands {
    fn new(commands: Vec<Command>) -> Option<CraneCommands> {
        // println!("{:?}", &commands);
        Some(CraneCommands { commands })
    }
}

#[derive(Debug)]
struct Command {
    amount: u8,
    from: u8,
    to: u8,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Command> {
        let mut rev_s = s
            .chars()
            .rev()
            .into_iter()
            .filter(|x| !x.to_string().is_empty())
            .collect::<String>();
        Ok(Command::new(&mut rev_s).unwrap())
    }
}

impl Command {
    fn new(values: &mut str) -> Option<Command> {
        let cmd = values.split_whitespace().collect::<Vec<&str>>();
        let amount = cmd[1].parse::<u8>().unwrap();
        let from = cmd[3].parse::<u8>().unwrap();
        let to = cmd[5].parse::<u8>().unwrap();

        println!("cmd = {:?}", cmd);

        return Some(Command { amount, from, to, });
    }
}

#[derive(Debug)]
enum Crate {
    Badge(char),
}

impl FromStr for Crate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rev_s = s
            .chars()
            .rev()
            .into_iter()
            .filter(|x| !x.to_string().is_empty())
            .collect::<String>();
        let max_stacks = rev_s
            .lines()
            .next()
            .map(|s| s.split_whitespace())
            .unwrap()
            .max()
            .unwrap()
            .parse::<i32>()?;
        println!("max_stacks = {:?}", max_stacks);
        let badge = rev_s;

        println!("badge = {:?}", badge);
        // let badge_fromstr = badge.parse::<char>()?;
        Ok(Crate::Badge('c'))
    }
}

fn main() -> Result<()> {
    let filename = "data/5.input";
    let mut commands: Vec<Command> = Vec::new();
    let mut crates: Vec<Crate> = Vec::new();
    commands = aoc::reverse_input_string::<Command>(&filename, 1, 4)?;
    crates = aoc::reverse_input_string::<Crate>(&filename, 6, 4)?;
    println!("commands = {:?}", commands);
    println!("crates = {:?}", crates);
    CraneCommands::new(commands);

    Ok(())
}
