use std::path::Path;
use std::process::Command;

pub fn tags(path: &str) -> Vec<String> {
    let output = Command::new("tmsu")
                         .arg("tags")
                         .arg("-1")  // one tag per line
                         .arg(&path)
                         .current_dir(Path::new(&path).parent().unwrap())
                         .output()
                         .expect("could not query tmsu tags");

    let output_string = String::from_utf8(output.stdout).unwrap();
    output_string.lines().skip(1).map(|s| s.to_owned()).collect()
}

pub fn add_tags(filenames: &Vec<String>, tags: &Vec<String>) {
    for tag in tags {
        Command::new("tmsu")
                .arg("tag")
                .arg(format!("--tags=\"{}\"", tag))
                .args(filenames)
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
