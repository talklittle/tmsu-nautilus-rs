use nautilus_extension::{FileInfo, InfoProvider};
use percent_encoding;
use tmsu_commands;

pub struct TmsuInfoProvider {

}

impl InfoProvider for TmsuInfoProvider {
    fn should_update_file_info(&self, file_info: &FileInfo) -> bool {
        "file" == file_info.get_uri_scheme()
    }

    fn update_file_info(&self, file_info: &mut FileInfo) {
        let path = get_path(file_info);
        let attr_name = "tmsu_tags";
        let attr_value = tmsu_commands::tags(&path).join(" ");

        file_info.add_attribute(attr_name, &attr_value);
    }
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
}
