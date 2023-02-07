use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize,Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work with json"),
        author: String::from("seungik"),
        paragraph: vec![
            Paragraph {
                name: String::from("the first sentence")
            },
            Paragraph {
                name: String::from("the second sentence")
            },
            Paragraph {
                name: String::from("the third sentence")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("{:#?}",json);
}
