use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

fn main() -> Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let rest_of_the_first_group: Result<Vec<_>> = reader
        .lines()
        .enumerate()
        .take_while(|(_, line)| match line{
            Ok(l) => !l.is_empty(),
            _ => true, 
        })
        .map(|(index, line)|{
            line.map_err(|e|{
                Error::new(ErrorKind::Other, format!("Error reading line {}: {}", index + 1, e ))
            })
        })
        .collect();

    match rest_of_the_first_group{
        Ok(lines) =>{
            println!("Rest of the first group");
            for line in &lines{
                println!("PRINT: {:?}\n", line);
            }
        } 

        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
    
}
