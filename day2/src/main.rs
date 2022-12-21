use std::{fs::File, io::Read};

fn main(){
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
        let mut gg3 : rps2 = rps2::X;
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
        let sc1 = f_sc(gg2);
        let sc2 = s_sc(gg3, gg1);
        
        sum = sum +sc1+sc2;
    }
    println!("{}", sum);
}
fn find_first(str : Option<&&str>) -> String{
    return str.unwrap().to_string().chars().nth(0).unwrap().to_string();
}
fn find_second(str : Option<&&str>) -> String{
    return str.unwrap().to_string().chars().nth(2).unwrap().to_string();
}

fn f_sc(dig : rps2) -> i32{
    match dig {
        rps2::X => return 0,
        rps2::Y => return 3,
        rps2::Z => return 6,
    }
}

fn s_sc(res : rps2, tool : rps1) -> i32 {
    if res == rps2::X {
        match tool {
            rps1::A => return 3,
            rps1::B => return 1,
            rps1::C => return 2,
        }
    }
    else if res == rps2::Y {
        match tool {
            rps1::A => return 1,
            rps1::B => return 2,
            rps1::C => return 3,
        }
    }
    else {
        match tool {
            rps1::A => return 2,
            rps1::B => return 3,
            rps1::C => return 1,
        }
    }
    
}
// A1 for Rock, B2 for Paper, and C3 for Scissors
// X means lose, Y means draw, Z means  win
// A for Rock, B for Paper, and C for Scissors

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
