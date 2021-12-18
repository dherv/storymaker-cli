use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Novel {
    id: Uuid,
    title: String,
    // TODO: add Page struct with lines
    pages: Vec<Vec<String>>,
}

impl Novel {
    fn new(title: String, pages: Vec<Vec<String>>) -> Novel {
        let id = Uuid::new_v4();
        Novel { title, id, pages }
    }
}

fn run() {
    let filename = String::from("test");
    let path = format!("./novels/text/{}.txt", filename);
    let file = fs::read_to_string(path).expect("file read");
    let lines: Vec<String> = file
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let pages: Vec<Vec<String>> = lines.chunks(20).map(|chunk| chunk.to_vec()).collect();
    let novel = Novel::new(filename, pages);
    let result = serde_json::to_string(&novel);
    match result {
        Ok(data) => {
            println!("{}", data);
            let path = format!("./novels/json/{}.json", novel.title);
            fs::remove_dir_all("./novels/json");
            fs::create_dir_all("./novels/json").expect("could not create the folder");
            fs::write(path, &data).expect("Unable to write file");

            // TODO: move to S3 after
            let path_server = format!("../server/novels/json/{}.json", novel.id);
            fs::remove_dir_all("../server/novels/json");
            fs::create_dir_all("../server/novels/json").expect("could not create the folder");
            fs::write(path_server, data).expect("Unable to write file");
        }
        Err(err) => println!("{}", err),
    }
}
fn main() {
    run()
}
