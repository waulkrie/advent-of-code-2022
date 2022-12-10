use std::fs;
use std::collections::HashMap;


fn main() {
    let file_path: &str = "src/input.txt";
    // let file_path = "src/test.txt";
    let file: String = fs::read_to_string(file_path).expect("ERROR reading in file");
    let map: HashMap<char, i32> = init_map();
    let sum = first_part(&file, &map);

    // Print the results
    println!("Sums: {:?}", sum);
    let sum = second_part(&file, &map);

    // Print the results
    println!("Sums: {:?}", sum);

}

fn init_map() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    let upper_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let lower_alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    for (i, c) in lower_alphabet.enumerate() {
        map.insert(c, i as i32 + 1);
    }
    for (i, c) in upper_alphabet.enumerate() {
        map.insert(c, i as i32 + 27);
    }

    return map;
}

fn first_part(file: &String, map: &HashMap<char, i32>) -> i32 {

    let sum = file
        .split("\r\n")
        .map(|line| {
            let first_half: Vec<char> = line.chars().take(line.len() / 2).collect();
            let second_half: Vec<char> = line.chars().skip(line.len() / 2).collect();
            let mut sum = 0;
            for c in first_half.iter() {
                if second_half.contains(c) {
                    // println!("found {} as {} from \n{:?} in \n{:?}", c, map.get(c).unwrap(), first_half, second_half);
                    sum = *map.get(c).unwrap_or(&0);
                }
            }
            sum
        }).sum::<i32>();
    return sum;
}

fn second_part(file: &String, map: &HashMap<char, i32>) -> i32 {

    let mut sum:i32 = 0;
    let lines:Vec<&str> = file.split("\r\n").collect();
    for i in lines.chunks_exact(3) {
        // println!("{:?}", i);
        let first = i.get(0).unwrap();
        let second = i.get(1).unwrap();
        let third = i.get(2).unwrap();
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                sum += *map.get(&c).unwrap_or(&0);
                break;
            }
        }
    }
    sum
}