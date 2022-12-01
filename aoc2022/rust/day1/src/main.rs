use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    let mut max: i32 = 0;
    let mut sum: i32 = 0;
    let mut elves: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    elves.push(sum);
                    if max < sum{
                        max = sum;
                    }
                    sum = 0;
                }
                else {
                    sum += value.parse::<i32>().unwrap();
                }
            }
        }
        elves.sort();
        let mut res: i32 = 0;
        for snack in elves[elves.len()-3..].iter().rev(){
            println!("{:?}", snack);
            res += snack;
        }
        let res2: i32 = elves.iter().rev().take(3).sum();
        println!("{:?}", res);
        println!("{:?}", res2);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
