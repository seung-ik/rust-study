use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
        {
            "article": "how to work with json in rust",
            "author": "ik",
            "paragraph": [
                {
                    "name": "starting sentence"
                },
                {
                    "name": "middle sentence"
                },
                {
                    "name": "end sentence"
                }
            ]
        }
    "#;

    let parsed:Article = read_json_typed(json);
    for paragraph in parsed.paragraph {
        println!("{}",paragraph.name);
    }
    
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
