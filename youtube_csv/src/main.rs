use std::error::Error;
use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records(){
        let record = result?;
        println!("{:?}",record);
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./tdtype.csv") {
        eprintln!("{}",e);
    }
}
// https://www.youtube.com/watch?v=VQ5cXoAMHQI&list=PL5dTjWUk_cPYuhHm9_QImW7_u4lr5d6zO&index=4