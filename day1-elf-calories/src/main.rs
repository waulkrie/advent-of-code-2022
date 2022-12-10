use std::fs;


fn main() {
    let file_path = "src/elf_cals.txt";

    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_path).unwrap();

    // Split the string on empty lines to get the individual groups of numbers
    let groups: Vec<&str> = contents.split("\r\n\r\n").collect();
    let groups_size = groups.len();

    // For each group, split the group on newlines to get the individual numbers
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for group in groups {
        let group_numbers: Vec<i32> = group
            .split("\n")
            .map(|s| s.trim().parse().unwrap_or(0))
            .collect();
        numbers.push(group_numbers);
    }

    // For each group, find the sum of the numbers in the group and store it in a list
    let mut sums: Vec<i32> = Vec::new();
    for group in numbers {
        let sum: i32 = group.iter().sum();
        sums.push(sum);
        println!("{}", sum);
    }

    // Find the maximum value in the list of sums and print it
    sums.sort_by(|a, b| b.cmp(a)); // sort biggest to smallest
    let max_sum: i32 = *sums.iter().max().unwrap();
    println!("most cals: {} in {} groups", max_sum, groups_size);
    
    // Find the sum of the top 3
    let mut top_three: i32 = 0;
    let mut iterator = sums.iter();

    for _index in 0..3 {

        top_three += iterator.next().unwrap();
        
    }
    println!("top three sum: {}", top_three);
}