use kf_emoji_generator::*;
use serial_test::serial;
use std::{fs, path::Path, process::Command};

const EXE_DEBUG: &str = ".//target//debug//kf_emoji_generator";
const GENERATED_INPUT_PATH: &str = "input";

#[test]
#[serial]
// if this panics - let it happen!
fn exe_run_initial() {
    match Command::new(EXE_DEBUG).arg("--size=32").status() {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

fn copy_files() {
    let reference_input_path: &Path = Path::new("tests//input");
    let target_input: &Path = Path::new(GENERATED_INPUT_PATH);

    // println!(
    //     "reference_input_path: {reference_input_path:?}, exist: {}, target_input: {target_input:?} (exists: {})",
    //     reference_input_path.exists(),
    //     target_input.exists()
    // );
    if let Ok(reference_files) = get_dir_files(reference_input_path) {
        reference_files.into_iter().for_each(|file| {
            if let Some(file_name) = file.file_name() {
                // println!("{file_name:?}");
                _ = fs::copy(file.as_path(), target_input.join(file_name));
            }
        });
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
#[serial]
// if this panics - let it happen!
fn exe_run_final() {
    copy_files();
    match Command::new(EXE_DEBUG).arg("--size=32").status() {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
