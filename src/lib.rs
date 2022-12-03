use std::fs;

// ===========================================
// DAY 1
// ==========================================

pub fn day1(path: &std::path::PathBuf) {
    // open and read the file
    let content = fs::read_to_string(path)
        .expect("Could not read file");

    day1part1(&content);
    day1part2(&content);
}

fn day1part1(content: &String) {
    // create variable for the biggest inventory
    let mut biggest = 0;
    // create variable for current elf's total 
    let mut total = 0;
    for line in content.lines() {
        // if line is blank = end of elf inventory
        if line == "" {
            // adjust biggest if needed
            if total > biggest {
                biggest = total;
            }
            // reset total
            total = 0;
            continue;
        }
        
        // add current line to total
        total += line.parse::<i32>().unwrap();
    }
    println!("Part 1: {}", biggest);
}

fn day1part2(content: &String) {
    // create array since 3 biggest
    let mut biggest: [i32; 3] = [0; 3];
    // create variable for current elf's total
    let mut total = 0;
    for line in content.lines() {
        // if line is blank = end of elf inventory
        if line == "" {
            // loop through to check and replace when needed
            for i in 0..3 {
                if total > biggest[i] {
                    total += biggest[i];
                    biggest[i] = total - biggest[i];
                    total = total - biggest[i];
                }
            }
            // reset total
            total = 0;
            continue;
        }
        
        // add current line to total
        total += line.parse::<i32>().unwrap();
    }
    
    // return the sum of the biggest 3 
    println!("Part 2: {}", biggest.iter().sum::<i32>());
}

// ===============================================
// DAY 2
// ===============================================

pub fn day2(path: &std::path::PathBuf) {
    // open the file
    let content = fs::read_to_string(path)
        .expect("Could not open file");

    day2part1(&content);
    day2part2(&content);
}

fn day2part1(content: &String) {
    // rock = A = X = 1
    // paper = B = Y = 2
    // scissors = C = Z = 3
    let mut total = 0;
    for i in content.lines() {
        match i {
            "A X" => total += 4,
            "A Y" => total += 8,
            "A Z" => total += 3,
            "B X" => total += 1,
            "B Y" => total += 5,
            "B Z" => total += 9,
            "C X" => total += 7,
            "C Y" => total += 2,
            "C Z" => total += 6,
            _     => unreachable!(),
        }
    }

    println!("Part 1: {}", total);
}

fn day2part2(content: &String) {
    // rock = A = 1
    // paper = B = 2
    // scissors = C = 3
    //
    // lose = X = 0
    // draw = Y = 3
    // win = Z = 6
    let mut total = 0;
    for i in content.lines() {
        match i {
            "A X" => total += 3,
            "B X" => total += 1,
            "C X" => total += 2,
            "A Y" => total += 4,
            "B Y" => total += 5,
            "C Y" => total += 6,
            "A Z" => total += 8,
            "B Z" => total += 9,
            "C Z" => total += 7,
            _     => unreachable!(),
        }
    }
    println!("Part 2: {}", total);
}
