use std::{fs, ops::Range};


fn main() {
    let file_path: &str = "src/input.txt";
    // let file_path = "src/test.txt";
    let file: String = fs::read_to_string(file_path).expect("ERROR reading in file");

    let num_overlaps: i32 = first_part(&file);

    // Print the results
    println!("Sums: {:?}", num_overlaps);
}


fn first_part(file: &String) -> i32 {

    let mut sum: i32 = 0;
    let lines: Vec<&str> = file.split("\n").collect();
    for i in lines.chunks_exact(1) {
        if i.is_empty() || i.len() <1 { continue; }
        println!("line {:?}", i);
        let first = i.get(0).unwrap();
        // let second = i.get(1).unwrap();
        let expanded: Vec<Range<i32>> = expand_section(first);
        if expanded.len() == 0 { continue; }
        let f = expanded.get(0).unwrap();
        let s = expanded.get(1).unwrap();
        for i in f.start..f.end {
            if s.contains(&i.to_owned()) {
                sum += 1;
            }
        }        
        for i in s.start..s.end {
            if f.contains(&i.to_owned()) {
                sum += 1;
            }
        }

    }
    sum
}

fn expand_section(section: &str) -> Vec<Range<i32>> {

    let mut result: Vec<Range<i32>> = Vec::new();
    let pairs: Vec<&str> = section.split(",").collect();
    for single in pairs{
        let mut first = String::from("");
        let mut last = String::from("");
        for i in single.chars(){
            if i == '-'{
                break;
            } else {
                first.push(i);
            }
        }
        for i in single.chars(){
            if i == '-'{
                break;
            } else {
                last.push(i);
            }
        }
        // ty openGPT
            let first_num = first.to_string().parse::<i32>().unwrap();
            if last.len() > 0 {
                let last_num = last.to_string().parse::<i32>().unwrap();
                result.push(first_num..last_num);
            }else{
                result.push(first_num..first_num);
            }
            // for i in first..last + 1 {
            //     result.push(i);
            // }

        println!("{:?}",result)
    }

    result
}