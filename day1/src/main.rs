mod top3;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/home/panda/Desktop/aoc/day1/src/data.txt").expect("sorry");
    let mut data_in_str = String::new();
    let mut int_in_vec = Vec::new();
    file.read_to_string(&mut data_in_str).expect("sorry can set");
    let mut sum = 0;
    let mut max = 0;
    for i in data_in_str.split("\n"){
        if i != "" {
            let x:u32 = i.parse().unwrap();
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
    // println!("{}", max);
    // for i in &int_in_vec {
    //     println!("{}", i);
    // }
    let mut clone = int_in_vec.clone();
    clone.sort_by(|a,b| a.partial_cmp(b).unwrap());
    println!("{}", max);
    for i in &clone {
        println!("{}", i);
    }
    // println!("{}", clone.get(0)+clone.get(1)+clone.get(2));
}