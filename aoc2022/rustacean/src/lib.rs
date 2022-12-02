use std::str::FromStr;

pub fn read_one_line_at_a_time<T>(path: &str) -> Result<Vec<T>, anyhow::Error>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split("\n")
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn reverse_input_string<T>(path: &str, skip: usize, take: usize) -> Result<Vec<T>, anyhow::Error>
where
    T: FromStr,
{
    println!("path = {:?}", path);
    
    Ok(std::fs::read_to_string(path)?
        .chars()
        .rev()
        .collect::<String>()
        .split("\n")
        .skip(skip)
        .take(take)
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
