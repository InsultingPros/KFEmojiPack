use crate::{InternalArgs, MyErrors, OUTPUT_EXTENSION, SP_CONFIG_NAME, US_FILE_NAME};
use image::imageops::FilterType::Lanczos3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    env,
    fs::{self, File},
    io::{self, BufWriter, Error, Write},
    path::{Path, PathBuf},
    time::Instant,
};

/// # Errors
/// _
pub fn process_files(internal_args: &InternalArgs) -> Result<(), MyErrors> {
    let now: Instant = Instant::now();

    convert_to_tga_par(internal_args)?;
    let converted_files = get_dir_files(&internal_args.directories.output)?;
    create_kf_files(&converted_files, internal_args)?;

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    Ok(())
}

/// # Errors
/// _
pub fn convert_to_tga_par(internal_args: &InternalArgs) -> Result<(), MyErrors> {
    let entries = get_dir_files(&internal_args.directories.input)?;

    entries.into_par_iter().for_each(|mut entry| {
        if let Ok(mut img) = image::open(entry.as_path()) {
            img = img.resize_exact(
                internal_args.cli_args.dimensions,
                internal_args.cli_args.dimensions,
                Lanczos3,
            );

            entry.set_extension(OUTPUT_EXTENSION);
            if let Some(new_file_name) = entry.file_name() {
                let dest = internal_args.directories.output.join(new_file_name);
                if let Err(e) = img.save(dest) {
                    eprintln!("error while saving image: {e}");
                };
            };
        }
    });

    Ok(())
}

/// # Errors
/// _
pub fn create_kf_files(files: &Vec<PathBuf>, internal_args: &InternalArgs) -> io::Result<()> {
    let uc_file: File = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(
            internal_args
                .directories
                .classes
                .join(format!("{US_FILE_NAME}.uc")),
        )?;
    let mut uc_writer: BufWriter<File> = BufWriter::new(uc_file);

    let ini_file: File = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(
            internal_args
                .directories
                .configs
                .join(format!("{SP_CONFIG_NAME}.ini")),
        )?;

    let mut ini_writer: BufWriter<File> = BufWriter::new(ini_file);

    writeln!(uc_writer, "class {US_FILE_NAME} extends Actor;\n")?;
    writeln!(ini_writer, "[ServerPerksMut.ServerPerksMut]")?;

    let cwd = match &internal_args.cli_args.package {
        Some(res) => res,
        None => &env::current_dir()?
            .file_name()
            .map_or("KFEmojiPack".into(), |name| {
                name.to_string_lossy().into_owned()
            }),
    };

    for file in files {
        if let Some(file_stem) = file.file_stem() {
            if let Some(file_stem_str) = file_stem.to_str() {
                writeln!(
                    uc_writer,
                    r"#exec TEXTURE IMPORT FILE={} NAME={} MIPS={} MASKED={} DXT={}",
                    file.display(),
                    file_stem_str,
                    internal_args.cli_args.mips,
                    internal_args.cli_args.masked,
                    internal_args.cli_args.dxt
                )?;

                writeln!(
                    ini_writer,
                    r#"SmileyTags=(iconTexture="{cwd}.{file_stem_str}",IconTag="{file_stem_str}",bCaseInsensitive=False)"#
                )?;
            };
        };
    }

    Ok(())
}

// get input file
// https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
/// # Errors
/// _
pub fn get_dir_files(input: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = fs::read_dir(input)?
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
