use kf_emoji_generator::{util::process_files, *};
use std::process::ExitCode;

fn main() -> ExitCode {
    // get enviroment arguments
    let env_arguments: MyOptions = gumdrop::Options::parse_args_default_or_exit();

    let directories: Directories = Directories::new();
    match directories.validate_directories() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let internal_args: InternalArgs = InternalArgs {
        directories,
        cli_args: env_arguments,
    };

    match process_files(&internal_args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Terminated with error: {err}");
            ExitCode::FAILURE
        }
    }
}
