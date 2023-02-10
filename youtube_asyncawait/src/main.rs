// https://www.youtube.com/watch?v=dwEy3OiUCHw&list=PL5dTjWUk_cPYuhHm9_QImW7_u4lr5d6zO&index=9
use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std:io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers: \n{}", res.headers());

    let body = res.text().await?;
    println!("body: \n{}", body);

    Ok(())
}
