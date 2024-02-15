mod cli;

use kf_emoji_generator::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    // get enviroment arguments
    let env_arguments: cli::Options = gumdrop::Options::parse_args_default_or_exit();
    // compose arguments for internal use
    let cli_args: CliArgs = CliArgs {
        dimension: env_arguments.size,
    };

    let directories: Directories = Directories::new();
    let _ = directories
        .validate_directories()
        .inspect_err(|_| std::process::exit(ERR_CANNOT_MAKE as i32));

    let internal_args: InternalArgs = InternalArgs {
        directories,
        cli_args,
    };

    match process_files(&internal_args) {
        Ok(()) => ExitCode::from(ERR_SUCCESS),
        Err(_) => {
            // eprintln!("Terminated with error: {}", e);
            std::process::exit(ERR_CANNOT_MAKE as i32)
        }
    };

    ExitCode::from(ERR_SUCCESS)
}
