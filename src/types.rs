pub const ERR_SUCCESS: u8 = 0;
pub const ERR_CANNOT_MAKE: u8 = 82;

pub const INPUT_DIR_NAME: &str = "input";
pub const OUTPUT_DIR_NAME: &str = "output";
pub const CLASS_DIR_NAME: &str = "classes";
pub const CONFIG_DIR_NAME: &str = "configs";

pub const US_FILE_NAME: &str = "GenerateTexture";
pub const SP_CONFIG_NAME: &str = "ServerPerks_Template";

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
    pub dimension: u16,
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
