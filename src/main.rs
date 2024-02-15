#![warn(clippy::all, clippy::pedantic)]
mod cli;
mod types;

use image::imageops::FilterType::Lanczos3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::{self, File},
    io::{self, BufWriter, Error, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};
use types::*;

fn main() -> ExitCode {
    // get enviroment arguments
    let env_arguments: cli::Options = gumdrop::Options::parse_args_default_or_exit();
    // compose arguments for internal use
    let cli_args: CliArgs = CliArgs {
        dimension: env_arguments.size,
    };

    let directories: Directories = Directories::new();
    if let Err(_) = directories.validate_directories() {
        std::process::exit(i32::from(ERR_CANNOT_MAKE))
    };

    let internal_args: InternalArgs = InternalArgs {
        directories,
        cli_args,
    };

    match process_files(&internal_args) {
        Ok(()) => ExitCode::from(ERR_SUCCESS),
        Err(e) => {
            eprintln!("Terminated with error: {}", e);
            std::process::exit(i32::from(ERR_CANNOT_MAKE))
        }
    };

    ExitCode::from(ERR_SUCCESS)
}

fn process_files(internal_args: &InternalArgs) -> io::Result<()> {
    let now: Instant = Instant::now();

    convert_to_tga_par(&internal_args)?;
    let converted_files = get_dir_files(&internal_args.directories.output)?;
    create_kf_files(&converted_files, &internal_args.directories)?;

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    Ok(())
}

fn create_kf_files(files: &Vec<PathBuf>, directories: &Directories) -> io::Result<()> {
    let uc_file: File = File::create(directories.classes.join(format!("{}.uc", US_FILE_NAME)))?;
    let mut uc_writer: BufWriter<File> = BufWriter::new(uc_file);

    let ini_file: File = File::create(directories.configs.join(format!("{}.ini", SP_CONFIG_NAME)))?;
    let mut ini_writer: BufWriter<File> = BufWriter::new(ini_file);

    writeln!(uc_writer, "class {} extends Actor;\n", US_FILE_NAME)?;
    writeln!(ini_writer, "[ServerPerksMut.ServerPerksMut]")?;

    for file in files {
        if let Some(file_stem) = file.file_stem() {
            if let Some(file_stem_str) = file_stem.to_str() {
                writeln!(
                    uc_writer,
                    r#"#exec TEXTURE IMPORT FILE={} NAME={} MIPS=0 MASKED=1 DXT=3"#,
                    file.display(),
                    file_stem_str
                )?;

                writeln!(
                    ini_writer,
                    r#"SmileyTags=(iconTexture="KFEmojiPack.{}",IconTag="{}",bCaseInsensitive=False)"#,
                    file_stem_str, file_stem_str
                )?;
            };
        };
    }

    Ok(())
}

fn convert_to_tga_par(internal_args: &InternalArgs) -> io::Result<()> {
    let entries = get_dir_files(&internal_args.directories.input)?;

    entries.into_par_iter().for_each(|mut entry| {
        if let Ok(mut img) = image::open(entry.as_path()) {
            img = img.resize_exact(
                internal_args.cli_args.dimension as u32,
                internal_args.cli_args.dimension as u32,
                Lanczos3,
            );

            entry.set_extension("TGA");
            if let Some(new_file_name) = entry.file_name() {
                let dest = internal_args.directories.output.join(new_file_name);
                let _ = img.save(dest).inspect_err(|err| eprintln!("{err}"));
            };
        }
    });

    Ok(())
}

// get input file
// https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
fn get_dir_files(input: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<std::path::PathBuf> = fs::read_dir(input)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    files.sort();

    if files.is_empty() {
        eprintln!("There are no files in {input:?}! Aborting.");
        Err(Error::from(io::ErrorKind::NotFound))
    } else {
        Ok(files)
    }
}
