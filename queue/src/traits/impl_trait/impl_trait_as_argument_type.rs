#![allow(non_snake_case)]

use std::io::BufReader;


fn parseCsvDocument<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines().map(|line| {
        line.map(|line| {
            line.split(",").map(|v| {
                String::from(v)
            }).collect()
        })
    }).collect()
}


fn parseCsvDoc(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines().map(|line| {
        line.map(|line| {
            line.split(",").map(|v| {
                String::from(v)
            }).collect()
        })
    }).collect()
}




fn main() {
    let data = "\
    city,country,pop
    Boston,United States,4628910
    Concord,United States,42695
    ";    
    let src = BufReader::new(data.as_bytes());
    let parsed = parseCsvDocument(src);
    println!("parsed: {:?}", parsed);

    let src = BufReader::new(data.as_bytes());
    let parsed = parseCsvDoc(src);
    println!("parsed: {:?}", parsed);


}