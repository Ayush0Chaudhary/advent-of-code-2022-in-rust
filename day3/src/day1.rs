use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/home/panda/Desktop/aoc/day3/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum = 0 ;
    for i in data.split("\n"){
        datavec.push(i);
        let (first_half, second_half) = i.split_at(i.len()/2);
        let mut common: Vec<char> = first_half.chars().filter(|c| second_half.contains(*c)).collect(); 
        common.dedup();
        let char = common.get(0).unwrap();
        println!("{}{}", find_sc(*char), *char);
        sum += find_sc(*char);
    }
    println!("{}", sum);
}

fn find_common(a:&str, b:&str) -> String{
    let common = a.chars().filter(|c| b.contains(*c)).collect();
    return common;
}

fn find_sc(ff : char) -> u32 {
    let ascii_value = ff as u32;
    if ascii_value <=  90{
        return ascii_value - 64+ 26;
    }
    else{
        return ascii_value - 96 ;
    }
}
