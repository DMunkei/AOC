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
    let mut counter: i32 = 0;
    let mut rucksack_1 = HashSet::new();
    let mut rucksack_2 = HashSet::new();
    let mut rucksack_3 = HashSet::new();

    for rucksack in file_reader.lines() {
        let rucksack_items = rucksack?.chars().collect::<Vec<char>>();
        if counter == 1{
            for c in rucksack_items{
                rucksack_1.insert(c);
            }
        }
        else if counter == 2{
            for c in rucksack_items{
                rucksack_2.insert(c);
            }
        }
        else if counter == 3{
            for c in rucksack_items{
                rucksack_3.insert(c);
            }

            let r1 = &rucksack_1;
            let r2 = &rucksack_2;
            let r3 = &rucksack_3;

            let common_item_btw_1_2: _ = r1.intersection(&r2).collect::<HashSet<&char>>();
            let c12 = common_item_btw_1_2;
            let common_item_btw_2_3: _ = c12.intersection(&r3).collect::<HashSet<&char>>();
                        
            println!("{:?} {:?}", common_item_btw_1_2, common_item_btw_2_3);
            println!("-------------");
            counter = 0;
        }


        counter += 1;
    }
    Ok(())
}
