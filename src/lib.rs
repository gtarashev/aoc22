use std::fs;

// ===========================================
// DAY 1
// ==========================================

pub fn day1(path: &std::path::PathBuf) {
    // open and read the file
    let content = fs::read_to_string(path)
        .expect("Could not read file");

    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
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
    println!("part 1 answer: {}", biggest);
}

fn part2(content: &String) {
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
    println!("part 2 answer: {}", biggest.iter().sum::<i32>());
}
