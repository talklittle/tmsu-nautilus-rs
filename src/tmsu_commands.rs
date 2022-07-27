use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn tags(path: &str) -> Vec<String> {
    let output_result = Command::new("tmsu")
                         .arg("tags")
                         .arg("-1")  // one tag per line
                         .arg(&path)
                         .current_dir(Path::new(&path).parent().unwrap())
                         .output();

    match output_result {
        Ok(output) => {
            let output_string = String::from_utf8(output.stdout).unwrap();
            output_string.lines().skip(1).map(|s| s.to_owned()).collect()
        },
        Err(err) => {
            eprintln!("could not query tmsu tags: {}", err);
            Vec::new()
        }
    }
}

pub fn add_tags<S: AsRef<OsStr>, T: AsRef<str>>(filenames: &[S], tags: &[T]) {
    for tag in tags {
        Command::new("tmsu")
                .arg("tag")
                .arg(format!("--tags=\"{}\"", tag.as_ref()))
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
