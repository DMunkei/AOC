use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let file_reader = BufReader::new(file);
    let ascii_values: HashMap<char, i32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let mut sum = 0;
    for rucksack in file_reader.lines() {
        let rucksack_items = rucksack.unwrap().chars().collect::<Vec<char>>();
        let midpoint: usize = &rucksack_items.len() / 2;

        let mut compartment_1 = HashSet::new();
        let mut compartment_2 = HashSet::new();
        for c in rucksack_items[0..midpoint].to_vec(){
            compartment_1.insert(c);
        }
        for c in rucksack_items[midpoint..rucksack_items.len()].to_vec(){
            compartment_2.insert(c);
        }

        let common_item:HashSet<_> = compartment_1.intersection(&compartment_2).collect();
        for x in common_item {
            sum += ascii_values[x];
        }
    }
    println!("{}", sum);
    Ok(())
}
