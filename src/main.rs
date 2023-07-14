use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;

pub struct Records {
    win:  i32,
    loss: i32,
}
    
fn main() {

    //let mut team_map = HashMap::new();

    if let Ok(lines) = read_lines("data/records.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("inserting {}", ip);
                let parts: Vec<&str> = ip.split(":").collect();
                let record = Records {
                    win:  parts[1].parse().unwrap(),
                    loss: parts[2].parse().unwrap(),
                };
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
