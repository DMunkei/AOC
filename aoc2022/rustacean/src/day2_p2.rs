use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let file_reader = BufReader::new(file);
    let mut game_score: i32 = 0;
    let shape_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let elf_counter = HashMap::from([("C","X"), ("A","Y"), ("B","Z")]);
    let elf_winning_condition = HashMap::from([("A","Z"), ("B","X"), ("C","Y")]);
    let draw_condition = HashMap::from([("A","X"), ("B","Y"), ("C","Z")]);
    let mut _res = 0;
    for (_, round) in file_reader.lines().enumerate() {
        let s = String::from(round?);
        let elf_move = &s[..1];
        let my_move = &s[2..];
        if my_move == "X"{
            game_score += 0 + shape_score[elf_winning_condition[elf_move]];
        }
        else if my_move == "Z" {
            game_score += 6 + shape_score[elf_counter[elf_move]];
        }
        else {
            game_score += 3 + shape_score[draw_condition[elf_move]];
        }
    }
         println!("{:?}",  game_score);
    Ok(())
}
