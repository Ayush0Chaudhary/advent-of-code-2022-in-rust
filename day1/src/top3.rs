use std::{fs::File, io::Read};

fn top3() {
    let mut file = File::open("/home/panda/Desktop/aoc/day2/src/data.txt").expect("sorry");
    let mut data_in_str = String::new();
    let mut int_in_vec = Vec::new();
    file.read_to_string(&mut data_in_str).expect("sorry can set");
    let mut sum = 0;
    let mut max = 0;
    let mut counter = 0;
    for i in data_in_str.split("\n"){
        if i != "" {
            let x:u32 = i.parse().unwrap();
            // println!("{}", x);
            sum = x + sum;
            if sum > max {
                
                max = sum;

            }
        }
        else {
            int_in_vec.push(sum);
            sum = 0;
        }
        
    }
    println!("{}", max)

}