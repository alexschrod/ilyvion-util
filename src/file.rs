use std::path::Path;

pub fn get_file_name_from_file_path<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    path.file_name().unwrap().to_string_lossy().into_owned()
}
