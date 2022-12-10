use std::fs;


fn main() {
    // let file_path: &str = "src/input.txt";
    let file_path = "src/test.txt";
    let file: String = fs::read_to_string(file_path).expect("ERROR reading in file");

    let num_overlaps: i32 = first_part(&file);

    // Print the results
    println!("Sums: {:?}", num_overlaps);
}


fn first_part(file: &String) -> i32 {

    let mut sum: i32 = 0;
    let lines: Vec<&str> = file.split("\n").collect();
    for i in lines.chunks_exact(2) {
        println!("{:?}", i);
        let first = i.get(0).unwrap();
        let second = i.get(1).unwrap();
        expand_section(first);
        expand_section(second);
    }
    0
}

fn expand_section(section: &str) -> Vec<i32> {

    let mut result: Vec<i32> = Vec::new();
    let pairs: Vec<&str> = section.split(",").collect();
    for single in pairs{
        let first = single.chars().nth(0);
        let last = single.chars().nth(2);
        //ty openGPT
        if let (Some(first), Some(last)) = (first, last) {
            let first = first.to_string().parse::<i32>().unwrap();
            let last = last.to_string().parse::<i32>().unwrap();
            for i in first..last + 1 {
                result.push(i);
            }
        }
        println!("{:?}",result)
    }

    result
}