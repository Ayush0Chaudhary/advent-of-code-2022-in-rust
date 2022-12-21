use std::{fs::File, io::Read, iter::Skip};

fn main() {
    let mut file = File::open("/home/panda/Desktop/aoc/day2/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    for i in data.split("\n"){
        // xunwrap().to_string()
        datavec.push(i);

    }
    let mut sum  = 0 ;
    for i in 0..datavec.len(){
        let a = find_first(datavec.get(i));
        let b = find_second(datavec.get(i));
        let mut gg1 : rps1 = rps1::A;
        let mut gg2 : rps2 = rps2::X;
        let mut gg3 = rps2::X;
        if a == "B" {
            gg1 = rps1::B;
        }
        else if a == "C" {
            gg1 = rps1::C;
        }

        if b == "Y"{
            gg2 = rps2::Y;
            gg3 = rps2::Y;


        }
        else if b == "Z" {
            gg2 = rps2::Z;
            gg3 = rps2::Z;

        }
        
        // let ddc = rps1::a;
        let sc1 = find_score(gg1, gg2);
        
        let sc2 = tool_score(gg3);
        println!("{}", sc1+sc2);
        sum = sum +sc1+sc2;
    }
    println!("{}",sum);
}

fn find_first(str : Option<&&str>) -> String{
    return str.unwrap().to_string().chars().nth(0).unwrap().to_string();
}
fn find_second(str : Option<&&str>) -> String{
    return str.unwrap().to_string().chars().nth(2).unwrap().to_string();
}

fn tool_score (b :rps2) -> i32 {
    match b {
        rps2::Y =>  2,
        rps2::Z =>  3,
        rps2::X =>  1,
    }
}

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
fn find_score(a:rps1, b :rps2) -> i32{
    let mut answer = 0;
    if a == rps1::A {
        match b {
            rps2::X =>  3,
            rps2::Y =>  6,
            rps2::Z =>  0,
            
        }
    }
    else if a == rps1::B {
        match b {
            rps2::X => return 0,
            rps2::Y => return 3,
            rps2::Z => return 6,
        }
    }
    else {
        match b {
            rps2::X => return 6,
            rps2::Y => return 0,
            rps2::Z => return 3,
        }
    }

}
#[derive(PartialEq)]
enum rps1 {
    A,
    B,
    C,
}
#[derive(PartialEq)]
enum rps2 {
    X,
    Y,
    Z,
}

