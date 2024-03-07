use std::path::Path;

pub fn get_folder_name_from_path(path: &str) -> String {
    let path = Path::new(path);
    let folder_name = path.file_name().unwrap().to_str().unwrap();
    folder_name.to_string()
}
