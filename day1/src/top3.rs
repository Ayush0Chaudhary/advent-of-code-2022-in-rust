use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/home/panda/Desktop/aoc/day1/src/data.txt").expect("sorry");
    let mut data_in_str = String::new();
    // let mut int_in_vec
    file.read_to_string(&mut data_in_str).expect("sorry can set");
    let mut sum = 0;
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
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
            sum = 0;
        }
        
    }
    println!("{}", max)

}