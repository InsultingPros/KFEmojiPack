#![warn(clippy::all, clippy::pedantic)]

mod cli;

use image::imageops::FilterType::Lanczos3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    env,
    fs::{self, File},
    io::{self, BufWriter, Error, Write},
    path::{Path, PathBuf},
    time::Instant,
};

pub const ERR_SUCCESS: u8 = 0;
pub const ERR_CANNOT_MAKE: u8 = 82;

pub const INPUT_DIR_NAME: &str = "input";
pub const OUTPUT_DIR_NAME: &str = "output";
pub const CLASS_DIR_NAME: &str = "classes";
pub const CONFIG_DIR_NAME: &str = "configs";

pub const US_FILE_NAME: &str = "GenerateTexture";
pub const SP_CONFIG_NAME: &str = "ServerPerks_Template";
pub const OUTPUT_EXTENSION: &str = "TGA";

pub struct InternalArgs {
    pub directories: Directories,
    pub cli_args: CliArgs,
}

pub struct Directories {
    pub input: std::path::PathBuf,
    pub output: std::path::PathBuf,
    pub classes: std::path::PathBuf,
    pub configs: std::path::PathBuf,
}

pub struct CliArgs {
    pub dimension: u32,
}

impl Directories {
    pub fn new() -> Self {
        Self {
            input: std::path::PathBuf::from(INPUT_DIR_NAME),
            output: std::path::PathBuf::from(OUTPUT_DIR_NAME),
            classes: std::path::PathBuf::from(CLASS_DIR_NAME),
            configs: std::path::PathBuf::from(CONFIG_DIR_NAME),
        }
    }

    pub fn validate_directories(&self) -> std::io::Result<()> {
        if self.output.exists() {
            // cleanup, in case we have remnants of old runs
            std::fs::remove_dir_all(&self.output)?;
            std::fs::create_dir(OUTPUT_DIR_NAME)?;
        } else {
            std::fs::create_dir(OUTPUT_DIR_NAME)?;
        }

        if !self.classes.exists() {
            std::fs::create_dir(CLASS_DIR_NAME)?;
        }

        if !self.configs.exists() {
            std::fs::create_dir(CONFIG_DIR_NAME)?;
        }

        if !self.input.exists() {
            std::fs::create_dir(INPUT_DIR_NAME)?;
            eprintln!(
                "input directory didn't exist, created one for you! Now go put some images there."
            );
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }

        Ok(())
    }
}

pub fn process_files(internal_args: &InternalArgs) -> io::Result<()> {
    let now: Instant = Instant::now();

    convert_to_tga_par(&internal_args)?;
    let converted_files = get_dir_files(&internal_args.directories.output)?;
    create_kf_files(&converted_files, &internal_args.directories)?;

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    Ok(())
}

pub fn create_kf_files(files: &Vec<PathBuf>, directories: &Directories) -> io::Result<()> {
    let uc_file: File = File::create(directories.classes.join(format!("{}.uc", US_FILE_NAME)))?;
    let mut uc_writer: BufWriter<File> = BufWriter::new(uc_file);

    let ini_file: File = File::create(directories.configs.join(format!("{}.ini", SP_CONFIG_NAME)))?;
    let mut ini_writer: BufWriter<File> = BufWriter::new(ini_file);

    writeln!(uc_writer, "class {} extends Actor;\n", US_FILE_NAME)?;
    writeln!(ini_writer, "[ServerPerksMut.ServerPerksMut]")?;

    let cwd = env::current_dir()?
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or("KFEmojiPack".into());

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
                    r#"SmileyTags=(iconTexture="{}.{}",IconTag="{}",bCaseInsensitive=False)"#,
                    cwd, file_stem_str, file_stem_str
                )?;
            };
        };
    }

    Ok(())
}

pub fn convert_to_tga_par(internal_args: &InternalArgs) -> io::Result<()> {
    let entries = get_dir_files(&internal_args.directories.input)?;

    entries.into_par_iter().for_each(|mut entry| {
        if let Ok(mut img) = image::open(entry.as_path()) {
            img = img.resize_exact(
                internal_args.cli_args.dimension,
                internal_args.cli_args.dimension,
                Lanczos3,
            );

            entry.set_extension(OUTPUT_EXTENSION);
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
pub fn get_dir_files(input: &Path) -> io::Result<Vec<PathBuf>> {
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
