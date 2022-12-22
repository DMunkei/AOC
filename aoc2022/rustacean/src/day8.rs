#[allow(dead_code, unused)]
fn main() {
    let filename = "data/8.prod";
    let lines = std::fs::read_to_string(filename).unwrap();
    let mut location: Vec<Vec<i32>> = Vec::new();
    for line in lines.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<i32>().unwrap());
        }
        location.push(row);
    }
    // part 1
    let mut visibility_counter = 0;
    for row in 1..location.len() - 1 {
        for col in 1..location[0].len() - 1 {
            let tree_height = location[row][col];
            //left            //left
            if location[row][col - 1] < tree_height {
                let mut seen: bool = true;
                for col_scan in 0..col - 1 {
                    if location[row][col_scan] >= tree_height {
                        seen = false;
                    }
                }
                if seen {
                    visibility_counter += 1;
                    continue;
                }
            }
            // right
            if location[row][col + 1] < tree_height {
                let mut seen: bool = true;
                for col_scan in col + 1..location[0].len() {
                    if location[row][col_scan] >= tree_height {
                        seen = false;
                    }
                }
                if seen {
                    visibility_counter += 1;
                    continue;
                }
            }
            // // up
            if location[row - 1][col] < tree_height {
                let mut seen: bool = true;
                for row_scan in 0..row - 1 {
                    if location[row_scan][col] >= tree_height {
                        seen = false;
                    }
                }
                if seen {
                    visibility_counter += 1;
                    continue;
                }
            }
            // // // down
            if location[row + 1][col] < tree_height {
                let mut seen: bool = true;
                for row_scan in row + 1..location.len() {
                    if location[row_scan][col] >= tree_height {
                        seen = false;
                    }
                }
                if seen {
                    visibility_counter += 1;
                    continue;
                }
            }
        }
    }
    // visibility_counter += location.len() * 2 + location[0].len() * 2 - 4;
    //part 2
    //println!("visibility_counter = {:?}", visibility_counter);
    let mut scenenic_score: Vec<i32> = Vec::new();
    for row in 1..location.len() - 1 {
        for col in 1..location[0].len() - 1 {
            let tree_height = location[row][col];
            println!("~~~~START~~~~~~~~~~~");
            println!("row = {:?} col = {:?} CURRENT TREE = {:?} ", row, col, tree_height);
            //left
            let mut left_score = 0;
            let mut right_score = 0;
            let mut up_score = 0;
            let mut down_score = 0;
            for col_scan in  (0..col).into_iter().rev(){
                // println!("COL_SCAN = {:?} LEFT TREE = {:?}", col_scan,location[row][col_scan]);
                if location[row][col_scan] > tree_height{
                    left_score +=1;
                    // println!("BIGGER THAN ME ONI CHAN~~~~");
                    break;
                }
                else if location[row][col_scan]==tree_height{
                    left_score += 1;
                    // println!("SAME HEIGHT left_score = {:?}", left_score);
                    break;
                }
                left_score += 1;
                // println!("SMALLER ADD TO LEFT SCORE = {:?}", left_score);
            }
            for row_scan in (0..row).into_iter().rev(){
                // println!("ROW_SCAN = {:?} UP TREE = {:?}", row_scan,location[row_scan][col]);
                if location[row_scan][col] > tree_height{
                    up_score += 1;
                    // println!("BIGGER THAN ME ONI CHAN~~~~ UPSCORE:{up_score}");
                    break;
                }
                else if location[row_scan][col] == tree_height{
                    up_score += 1;
                    // println!("SAME HEIGHT up_score = {:?}", up_score);
                    break;
                }
                up_score += 1;
                // println!("SMALLER ADD TO UP SCORE = {:?}", up_score);
            }
            for col_scan in (col + 1..location[0].len()).into_iter(){
                // println!("col_scan = {:?} RIGHT TREE = {:?}", col_scan,location[row][col_scan]);
                if location[row][col_scan] > tree_height{
                    right_score += 1;
                    // println!("BIGGER THAN ME ONI CHAN~~~~ RIGHTSCORE:{right_score}");
                    break;
                }
                else if location[row][col_scan] == tree_height{
                    right_score += 1;
                    // println!("SAME HEIGHT right_score = {:?}", right_score);
                    break;
                }
                right_score += 1;
                // println!("SMALLER ADD TO RIGHT SCORE = {:?}", right_score);
            }
            for row_scan in (row + 1..location.len()).into_iter(){
                // println!("row_scan = {:?} DOWN TREE = {:?}", row_scan, location[row_scan][col]);
                if location[row_scan][col] > tree_height{
                    down_score += 1;
                    // println!("BIGGER THAN ME ONI CHAN~~~~ downSCORE:{down_score}");
                    break;
                }
                else if location[row_scan][col] == tree_height{
                    down_score += 1;
                    // println!("SAME HEIGHT down_score = {:?}", down_score);
                    break;
                }
                down_score += 1;
                // println!("SMALLER ADD TO DOWN SCORE = {:?}", down_score);
            }
            println!("~~~~END~~~~~~~~~~~");
            scenenic_score.push(left_score * right_score * up_score * down_score);
            // println!("----------------------------- = ");
        }
    }
    println!("scenenic_score = {:?}", scenenic_score);
    println!( "score = {:?}", scenenic_score.iter().fold(-1, |a, b| a.max(*b)));
}
