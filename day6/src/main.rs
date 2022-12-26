use std::{fs::File, io::Read, collections::HashSet};
fn main(){
    //file reading
    let mut file = File::open("/home/panda/Desktop/aoc/day6/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum =0;

    for i in data.split("") {
        datavec.push(i);
        sum+=1;
        if sum>14 {
            let mut vec = Vec::new();
            for i in 0..14 {
                vec.push(*datavec.get(sum- i -1).unwrap());
            }
            // for i in vec{
            //     print!("{}",i);
            // }
            // break;
            println!("{}",vec.len());
            if vec.iter().collect::<HashSet<_>>().len() == vec.len() {
                print!("{}", sum);
                break;
            }
            // vec.de
            
        }
    }
}