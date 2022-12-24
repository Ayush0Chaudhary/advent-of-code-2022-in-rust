use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/home/panda/Desktop/aoc/day4/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum = 0 ;
    for i in data.split("\n"){
        datavec.push(i);
        let (r1,r2) = i.split_once(",").unwrap();
        let (sf1, sf2)= r1.split_once("-").unwrap();
        // let (r1,r2) = i.split_once(",").unwrap();
        let (ss1, ss2)= r2.split_once("-").unwrap();
        let f1:i32 = sf1.parse().unwrap();
        let f2:i32 = sf2.parse().unwrap();
        let s1:i32 = ss1.parse().unwrap();
        let s2:i32 = ss2.parse().unwrap();
         
        println!("{}-{},{}-{}", f1,f2,s1,s2);
        if f1 > s1 {
            if s2 > f2 {
                sum+=1;
                print!("onwe");
            }
            else if s2 == f2 {
                sum+=1;
                print!("onwe2");
            }         
        }
        else if f1 < s1 {
            if s2 < f2 {
                sum+=1;
                print!("two");
            }
            else if s2 == f2 {
                sum+=1;
                print!("twotwo");
            }         
        }
        else {
            sum+=1;
            print!("three");
        }

    }        
    println!("{}", sum);
}

