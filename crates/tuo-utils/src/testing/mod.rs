use std::path::PathBuf;
use uuid::Uuid;
use crate::datetime::timestamp::now_string;
use crate::fs::file_name_sanitizer::sanitize_file_name;


pub const TEST_FIXTURES_FOLDER: &str = "fixtures";
pub const TEST_TEMP_FOLDER: &str = "temp";
pub const TEST_CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
/// Get the fixture path under `tests/fixtures`
pub fn get_test_fixture_path() -> String {
    let tests_fixtures_path = get_cargo_root_test_folder().join(TEST_FIXTURES_FOLDER);
    tests_fixtures_path.to_str().unwrap().to_string()
}

/// Get the fixture path buf under `tests/fixtures`
pub fn get_test_fixture_path_buf() -> PathBuf {
    get_cargo_root_test_folder().join(TEST_FIXTURES_FOLDER)
}

/// Get the temp folder path buf under `tests/temp`
pub fn get_test_temp_path_buf() -> PathBuf {
    let tests_temp_path = get_cargo_root_test_folder().join(TEST_TEMP_FOLDER);
    // create if not exists
    std::fs::create_dir_all(&tests_temp_path).unwrap();
    tests_temp_path
}

// Get the temp folder path under `tests/temp`
pub fn get_test_temp_path() -> String {
    let tests_temp_path = get_cargo_root_test_folder().join(TEST_TEMP_FOLDER);
    // create if not exists
    std::fs::create_dir_all(&tests_temp_path).unwrap();
    tests_temp_path.to_str().unwrap().to_string()
}

// clear test_temp folder
pub fn clear_test_temp_folder() {
    let tests_temp_path = get_test_temp_path_buf();
    std::fs::remove_dir_all(&tests_temp_path).unwrap();
}

/// Get the path to the test folder under Cargo root folder
pub fn get_cargo_root_test_folder() -> PathBuf {
    let cargo_root_path = std::env::var(TEST_CARGO_MANIFEST_DIR).unwrap();
    let tests_path = format!("{}/tests", cargo_root_path);
    PathBuf::from(tests_path)
}

/// Get a dedicated unit test fixture folder under `tests/fixtures/{MODULE_NAME}`
///
/// This test folder is dedicated to the unit test module.
/// It should be deleted after the test.
pub fn get_test_fixture_module_folder_path_buf(module_name: &str) -> PathBuf {
    let tests_fixtures_path = get_test_fixture_path_buf().join(module_name);
    // create if not exists
    std::fs::create_dir_all(&tests_fixtures_path).unwrap();
    tests_fixtures_path
}

pub fn get_test_fixture_module_folder_path(module_name: &str) -> String {
    let tests_fixtures_path = get_test_fixture_module_folder_path_buf(module_name);
    tests_fixtures_path.to_str().unwrap().to_string()
}

pub fn get_random_unit_test_module_folder() -> String {
    let random_name = Uuid::new_v4().to_string();
    get_test_fixture_module_folder_path(&random_name)
}

/// Get a dedicated unit test fixture folder under `tests/temp/{MODULE_NAME}`
pub fn get_random_test_temp_folder() -> String {
    let path_string = get_random_test_temp_folder_path_buf()
        .to_str()
        .unwrap()
        .to_string();
    // create if not exists
    path_string
}

pub fn get_random_test_temp_folder_path_buf() -> PathBuf {
    let random_name = sanitize_file_name(now_string().as_str());
    let path = get_test_temp_path_buf().join(random_name);
    // create if not exists
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub fn get_test_libs_path_buf() -> PathBuf {
    let test_root = get_cargo_root_test_folder();
    test_root.join("test_libs")
}
