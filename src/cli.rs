// Reference: https://docs.rs/gumdrop/latest/gumdrop/
/// `kf_emoji_generator` supported arguments. For online help check: <https://github.com/InsultingPros/KFEmojiPack>
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, gumdrop::Options)]
pub struct Options {
    /// `-h` : print help information.
    #[options(help = "Prints the help message.")]
    pub help: bool,
    /// Emojii dimensions size (width x height).
    #[options(
        short = "s",
        meta = "<number>",
        default = "32",
        help = "Specifies emoji dimensions."
    )]
    pub size: u32,
}
