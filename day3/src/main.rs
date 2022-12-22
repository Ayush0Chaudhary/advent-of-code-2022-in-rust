use std::{fs::File, io::Read};

fn main(){
    let mut file = File::open("/home/panda/Desktop/aoc/day3/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum = 0 ;
    for i in data.split("\n") {
        datavec.push(i);
    }
    let mut counter:i32 = -1; 
    while counter < 300 {
        counter+=1;
        let wd1 = datavec.get(counter as usize).unwrap().to_string();
        counter+=1;
        let wd2 = datavec.get(counter as usize).unwrap().to_string();
        counter+=1;
        let wd3 = datavec.get(counter as usize).unwrap().to_string();
        let common:String = wd1.chars().filter(|c| wd2.contains(*c)).collect();
        let common2:String = wd3.chars().filter(|c| common.contains(*c)).collect();
        sum = sum + find_sc(common2.chars().next().unwrap());

    println!("{}", sum);
    }
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
