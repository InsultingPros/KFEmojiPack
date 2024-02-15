use std::path::PathBuf;

pub mod util;

pub const INPUT_DIR_NAME: &str = "input";
pub const OUTPUT_DIR_NAME: &str = "output";
pub const CLASS_DIR_NAME: &str = "classes";
pub const CONFIG_DIR_NAME: &str = "configs";

pub const US_FILE_NAME: &str = "GenerateTexture";
pub const SP_CONFIG_NAME: &str = "ServerPerks_Template";
pub const OUTPUT_EXTENSION: &str = "TGA";

#[derive(thiserror::Error, Debug)]
pub enum MyErrors {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ImageResult(#[from] image::ImageError),
}

// Reference: https://docs.rs/gumdrop/latest/gumdrop/
/// `kf_emoji_generator` supported arguments. For online help check: <https://github.com/InsultingPros/KFEmojiPack>
#[derive(Debug, gumdrop::Options)]
pub struct MyOptions {
    #[options(help = "Prints the help message.")]
    pub help: bool,
    #[options(
        short = "d",
        meta = "<number>",
        default = "32",
        help = "Specifies emoji dimensions. Higher the number, bigger will be the emojii in game chat."
    )]
    pub dimensions: u32,
    #[options(
        short = "p",
        meta = "<string>",
        help = "Package name, for config output. Defaults to executible's working directory if not specified."
    )]
    pub package: Option<String>,
    #[options(
        no_short,
        meta = "<number>",
        help = "Set resulting texture's MIPS level. Don't touch if you don't know what is this.",
        default = "0"
    )]
    pub mips: u32,
    #[options(
        no_short,
        meta = "<number>",
        help = "Set resulting texture's MASKED level. Don't touch if you don't know what is this.",
        default = "1"
    )]
    pub masked: u32,
    #[options(
        no_short,
        meta = "<number>",
        help = "Set resulting texture's DXT level. Don't touch if you don't know what is this.",
        default = "3"
    )]
    pub dxt: u32,
}

#[derive(Debug)]
pub struct InternalArgs {
    pub directories: Directories,
    pub cli_args: MyOptions,
}

#[derive(Debug, Default)]
pub struct Directories {
    pub input: PathBuf,
    pub output: PathBuf,
    pub classes: PathBuf,
    pub configs: PathBuf,
}

impl Directories {
    #[must_use]
    pub fn new() -> Self {
        Self {
            input: PathBuf::from(INPUT_DIR_NAME),
            output: PathBuf::from(OUTPUT_DIR_NAME),
            classes: PathBuf::from(CLASS_DIR_NAME),
            configs: PathBuf::from(CONFIG_DIR_NAME),
        }
    }

    /// # Errors
    /// _
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
