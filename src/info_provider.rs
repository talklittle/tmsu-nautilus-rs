use nautilus_extension::{FileInfo, InfoProvider};
use std::path::Path;
use std::process::Command;
use url;

pub struct TmsuInfoProvider {

}

impl InfoProvider for TmsuInfoProvider {
    fn should_update_file_info(&self, file_info: &FileInfo) -> bool {
        "file" == file_info.get_uri_scheme()
    }

    fn update_file_info(&self, file_info: &mut FileInfo) {
        let path = get_path(file_info);

        let output = Command::new("tmsu")
                             .arg("tags")
                             .arg(&path)
                             .current_dir(Path::new(&path).parent().unwrap())
                             .output()
                             .expect("could not query tmsu tags");

        let output_string = String::from_utf8(output.stdout.into_iter().collect()).unwrap();
        let output_str = output_string.trim();
        let find_result = output_str.find(": ");

        let attr_name = "tmsu_tags".to_string();
        let attr_value =
            match find_result {
                Some(idx) => String::from(&output_str[idx+2..]),
                None => String::new(),
            };

        file_info.attributes.insert(attr_name, attr_value);
    }
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
}
