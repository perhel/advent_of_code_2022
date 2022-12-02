use std::{error::Error, fs::File, io::{ Lines, BufReader, BufRead}, path::Path};

use models::Input;

pub trait Load {
    fn load(path: &str) -> Input;
}

impl Load for Input {
    
    fn load(file_path: &str) -> Input {
        let mut output: Vec<String> = Vec::new();

        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                if let Ok(content) = line {
                    output.push(content);
                }
            }
        }
        Input { lines: output }
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, Box<dyn Error + 'static>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
