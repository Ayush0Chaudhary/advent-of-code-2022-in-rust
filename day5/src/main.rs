use std::{fs::File, io::Read};



fn main() {
    //defining the vector
    
    let mut v =vec![vec!["V","J", "B", "D"],vec!["F","D", "R", "W", "B", "V", "P"],vec!["Q","W", "C", "D", "L", "F", "G", "R"],
    vec!["B","D", "N", "L", "M", "P", "J", "W"],vec!["Q","S", "C", "P", "B", "N", "H"],vec!["G","N", "S", "B", "D", "R"]
    ,vec!["H","S", "F", "Q", "M", "P", "B","Z"],vec!["F","L", "W"],vec!["R","M", "F", "V", "S"]];

    //file reading
    let mut file = File::open("/home/panda/Desktop/aoc/day5/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum = 0 ;
    for i in data.split("\n"){
        datavec.push(i);
        let (f_from,s_form)   = i.split_once("from").unwrap();
        let quantity_str:Vec<&str> = f_from.split_whitespace().collect();
        let quantity = quantity_str.get(1).unwrap().parse::<i32>().unwrap();
        let loc = s_form.chars().nth(1).unwrap() as i32;
        let fin = s_form.chars().nth(6).unwrap() as i32;
        println!("{}",v[1].get(0).unwrap());


    }

     
}


// let mut v1:Vec<&str> =vec!["V","J", "B", "D"];
// let mut v2:Vec<&str> =vec!["F","D", "R", "W", "B", "V", "P"];
// let mut v3:Vec<&str> =vec!["Q","W", "C", "D", "L", "F", "G", "R"];
// let mut v4:Vec<&str> =vec!["B","D", "N", "L", "M", "P", "J", "W"];
// let mut v5:Vec<&str> =vec!["Q","S", "C", "P", "B", "N", "H"];
// let mut v6:Vec<&str> =vec!["G","N", "S", "B", "D", "R"];
// let mut v7:Vec<&str> =vec!["H","S", "F", "Q", "M", "P", "B","Z"];
// let mut v8:Vec<&str> =vec!["F","L", "W"];
// let mut v9:Vec<&str> =vec!["R","M", "F", "V", "S"];
