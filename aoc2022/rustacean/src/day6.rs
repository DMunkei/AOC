use std::collections::HashSet;

fn main() -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string("data/6.input")?
        .chars()
        .collect::<Vec<char>>();
    let mut res = 0;
    let message_length = 14;
    for (i, win) in input.windows(message_length).enumerate() {
        let mut elements: HashSet<char> = HashSet::new();
        if win.iter().all(|&el| elements.insert(el)) {
            res = i + message_length;
            break;
        }
    }
    println!("{}", res);
    Ok(())
}
