use std::{fs::File, io::Read};

const MAXIMA: u64 = 34; 

fn main() {
    // implenting range of python
    let _num = 1..45;
    //implemnting vector
    let animals = vec!["dog", "cat", "zebra"];
    
    // impletmenting enum
    // let p_direc: Direc = Direc::left;

    //implementing for loop

    // for i in animals.iter() {
    //     println!("{}",i);
    // }
    //     for i in animals.iter() {
    //     println!("{}",i);
    // }

    //implementing match
    // match p_direc {
    //     Direc::down => print!("hola down"),
    //     Direc::left => print!("hola left"),
    //     Direc::up => print!("hola up"),
    //     Direc::right => print!("hola right")
    // }

    //implementing constants
    // print!("{}",MAXIMA);

    //implementing Tuples
        let tup1 = ("abs",  "ffds", 12132, (12,1,22,12,3,1,3,11));
        // println!("{}{}{}", tup1.0, tup1.3.0,(tup1.3).2)
        //another method to access tuples
        let (a,b,c,d) = tup1; 

    //implentation of function
        // print_num_to(1000);
    //implementation of returning func
    // print!("{}", tell_even(32));

    //implementation of codeblocks
    // let vv = 34;
    // {
    //     //just isolated part of a function
    //     let cc = 3222;
    //     print!("{}{}", vv,cc);

    // }
//    print!("{}{}", vv,cc);//not working
//implementation of shadowing
let mut cc = 43;
        {
            let cc =45;
            //by redeclaring varible inside the value doesn't change, you chaange the data type too
        }

        //implementation 0f references, pointer of rust

        let mut x = 883;
        let xr = &x;
        // println!("{}", xr);//not need to depointerize the variable
        //but to do operation we habe to declare mmutable reference and use *
        //ex 
        let xrr = &mut x;
        *xrr+=1;
        // println!("{}", xrr)

        //impletation of file handling
        let mut file = File::open("data.txt").expect("sorry!!");

        let mut data = String::new();

        file.read_to_string(&mut data)
            .expect("nhi hora ab!!");

        let mut datavec = Vec::new();
        for i in data.split('\n'){
            let mut x:u32 = i.parse().unwrap();
            x+=1;
            datavec.push(x); 
        }
        for i in &datavec{
            println!("{}", i);
        }
}
//normal function returning nothijng
fn print_num_to( num: u32){
    
    for i in 1..(num+1){
        println!("{}",i);
    } 
}

//function retruning bool value

fn tell_even (num: u128) -> bool
{
    return num%2 == 0 ;
}
// enum  Direc {
//     up,
//     down,
//     right,
//     left,
// }