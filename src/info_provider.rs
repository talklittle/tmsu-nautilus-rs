use nautilus_extension::{FileInfo, InfoProvider};
use nautilus_ffi::NautilusFileInfo;
use nautilus_ffi::{nautilus_file_info_get_uri, nautilus_file_info_get_uri_scheme};
use std::ffi::CStr;
use std::path::Path;
use std::process::Command;
use url;

pub struct TmsuInfoProvider {

}

impl InfoProvider for TmsuInfoProvider {
    fn should_update_file_info(&self, file_info: &mut FileInfo) -> bool {
        "file" == get_uri_scheme(file_info.raw_file_info)
    }

    fn update_file_info(&self, file_info: &mut FileInfo) {
        let ref mut file = file_info.raw_file_info;
        let path = get_path(*file as *mut NautilusFileInfo);

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

fn get_uri_scheme(file: *mut NautilusFileInfo) -> String {
    unsafe {
        CStr::from_ptr(nautilus_file_info_get_uri_scheme(file)).to_string_lossy().into_owned()
    }
}

fn get_path(file: *mut NautilusFileInfo) -> String {
    unsafe {
        let uri = CStr::from_ptr(nautilus_file_info_get_uri(file)).to_str().unwrap();
        url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
    }
}
