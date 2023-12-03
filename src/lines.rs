use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// pub example()
// {
//     if let Ok(lines) = read("./file/txt") {
//         for line in lines {
//             if let Ok(ip) = line {
//                 println!("{}", ip);
//             }
//         }
//     }
// }

pub fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
