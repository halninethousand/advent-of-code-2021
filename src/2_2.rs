use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> std::io::Result<()> {

    let path = env::current_dir()?;
    println!("{}", path.display());

    let file = File::open("../data/input_2.txt")?;
    let reader = BufReader::new(file);
    let mut instructions_horizontal = Vec::new();
    let mut instructions_vertical = Vec::new();
    let mut current_aim: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split(" ");
        let split = split.collect::<Vec<&str>>();
        
        match &split[0].chars().next() {
            Some('f') => {
                let hor = split[1].parse::<i32>().unwrap();
                instructions_vertical.push(&hor * current_aim);
                instructions_horizontal.push(hor);
            },
            Some('u') => current_aim -= split[1].parse::<i32>().unwrap(),
            Some('d') => current_aim += split[1].parse::<i32>().unwrap(),
            _ => println!("unknown instruction")
        }
        
    }
    let horizontal_sum: i32 = instructions_horizontal.iter().sum();
    let vertical_sum: i32 = instructions_vertical.iter().sum();
    println!("{:?}", instructions_horizontal);
    println!("{:?}", instructions_vertical);
    println!("{:?}", horizontal_sum * vertical_sum);
    
    Ok(())

}