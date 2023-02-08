// https://www.youtube.com/watch?v=gADVfB37QVo&list=PL5dTjWUk_cPYuhHm9_QImW7_u4lr5d6zO&index=8
use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status:{}",res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}
