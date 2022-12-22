#[allow(dead_code, unused)]
fn main() {
    let filename = "data/8.test";
    let lines = std::fs::read_to_string(filename).unwrap();

    const UP:(i32, i32)=(-1,0);
    const DOWN:(i32, i32)=(1,0);
    const LEFT:(i32, i32)=(0,-1);
    const RIGHT:(i32, i32)=(0,1);

    let mut forest: Vec<Vec<i32>> = Vec::new();
    for line in lines.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<i32>().unwrap());
        }
        forest.push(row);
    }
    let width = forest[0].len() as i32;
    let height = forest.len() as i32;


    for (start, step, search) in [
        ((0,0), RIGHT, DOWN),
        ((0,0), DOWN, RIGHT),
        ((height -1 , width - 1), DOWN, RIGHT),
        ((height - 1, width - 1), DOWN, RIGHT),
    ]{
        let mut walk = start;
        let mut b = 0;
        while walk.0 >= 0 && walk.0 < height || walk.1 >= 0 && walk.1 < width{
            b += 1;

            println!("walk0 = {:?} walk1 = {:?}", walk.0,walk.1);
            walk.0 += step.0;
            walk.1 += step.1;
            if b == 5{break;}
        }
    }
}
