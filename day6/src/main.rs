use std::{fs::File, io::Read};

fn main() {
    //file reading
    let mut file = File::open("/home/panda/Desktop/aoc/day6/src/data.txt").expect("sorry");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("no transfer");
    let mut datavec = Vec::new();
    let mut sum =0;
    
    for i in data.split("") {
        datavec.push(i);
        sum+=1;
        if sum>4 {
            let w1 = *datavec.get(sum-4).unwrap();
            let w2 = *datavec.get(sum-3).unwrap();
            let w3 = *datavec.get(sum-2).unwrap();
            let w4 = *datavec.get(sum-1).unwrap();
            if (w1 != w2) && (w1 != w3)&& (w1 != w4) && (w2 != w3) && (w2 !=w4 ) && (w3 != w4){
                print!("{}", sum);
            }
        }
    }

}
