use std::path::Path;
use std::process::Command;

pub fn tags(path: &str) -> String {
    let output = Command::new("tmsu")
                         .arg("tags")
                         .arg(&path)
                         .current_dir(Path::new(&path).parent().unwrap())
                         .output()
                         .expect("could not query tmsu tags");

    let output_string = String::from_utf8(output.stdout.into_iter().collect()).unwrap();
    let output_str = output_string.trim();
    let find_result = output_str.find(": ");

    let tags =
        match find_result {
            Some(idx) => String::from(&output_str[idx+2..]),
            None => String::new(),
        };

    tags
}

pub fn add_tags(filenames: &Vec<String>, tags: &Vec<String>) {
    for tag in tags {
        Command::new("tmsu")
                .arg("tag")
                .arg(format!("--tags=\"{}\"", tag))
                .args(&filenames)
                .current_dir(Path::new(&filenames[0]).parent().unwrap())
                .output()
                .expect("failed to tag files");
    }
}

pub fn untag(path: &str, tag: &str) {
    Command::new("tmsu")
        .arg("untag")
        .arg(path)
        .arg(tag)
        .current_dir(Path::new(&path).parent().unwrap())
        .output()
        .expect("could not untag");
}