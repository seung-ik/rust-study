extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() !=3 {
        eprintln!("Usage: cargo run `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression:: default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}",output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

// unwrap : ok 인경우에는 그냥 지나가고 err 인 경우에는 패닉이 나게됨 비슷한 케이스로 unwrap_or_else panic shortcut 이며 Result를 반환하는 곳에서 사용가능하다.