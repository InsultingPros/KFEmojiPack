# KF Emoji Pack Generator

## General Information

[DXT compression]: https://beyondunrealwiki.github.io/pages/dxt.html
[dkanus]: https://github.com/dkanus
[Marco]: https://steamcommunity.com/profiles/76561197975509070
[Server Perks]: https://forums.tripwireinteractive.com/index.php?threads/mut-per-server-stats.36898/
[android 7 emojis]: https://emojipedia.org/google/android-7.0/
[releases]: https://github.com/InsultingPros/KFEmojiPack/releases
[build_badge]: https://img.shields.io/github/actions/workflow/status/InsultingPros/KFEmojiPack/test.yml?style=for-the-badge
[release_badge]: https://img.shields.io/github/downloads/InsultingPros/KFEmojiPack/total?style=for-the-badge

[![build_badge]](https://github.com/InsultingPros/KFEmojiPack/actions/workflows/test.yml) [![release_badge]](https://github.com/InsultingPros/KFEmojiPack/releases)

This project simplifies the creation of emoji packs for [Server Perks]. While community-made packs exist, they are difficult to find, and up-to-date documentation is scarce. This tool fills that gap.

![img](Docs/example.png)

```text
say some smiles: face-with-tears-of-joy pile-of-poo nauseated-face kissing-face face-savouring-delicious-food thinking-face hell yea
```

With **KF Emoji Pack Generator**, creating an emoji pack is as easy as dragging and dropping images, running a single executable, and getting ready-to-compile files—no SDKs or extensive research needed.

This repo also provides a compiled example with [android 7 emojis], as a reference.

## Basic Usage

1. Download `kf_emoji_generator` from [releases], and extract the archive anywhere you want.
2. Run `kf_emoji_generator` once, it will create `input` directory nearby.
3. Put your emoji images there.
4. Run `kf_emoji_generator` again. It will generate several directories:
    - `output`  - your images from `input` will be resized, converted into `TGA` and moved here.
    - `classes` - generates `GenerateTexture.uc` with corresponding exec directives. This is the file you want to compile.
    - `configs` - generates `ServerPerks_Template.ini` with smiles tag information.
5. Compile the generated files as you would for any KF1 mod.
6. After compilation, rename the output file from `u` to `utx`.
7. Copy the SmileyTags lines from ServerPerks_Template.ini into your server’s ServerPerks.ini file.
8. That's it! Start your server and test your new emoji pack.

## Advanced Usage

Here are all available arguments, if you want to tweak your output:

- `-h`, `--help` - Prints the help message.
- `-d`, `--dimensions <number>` - Specifies emoji dimensions. Higher the number, bigger will be the emoji in game chat. (default: 32)
- `-p`, `--package <string>` - Package name, for config output. Defaults to executable's working directory if not specified.
- `--mips <number>` - Set resulting texture's MIPS level. Don't change this unless you know what it does. (default: 0)
- `--masked <number>` - Set resulting texture's MASKED level. Don't change this unless you know what it does. (default: 1)
- `--dxt <number>` - Set resulting texture's DXT level. Don't change this unless you know what it does. (default: 3)

> [!NOTE]
> About emojis dimensions - 128x128 are shown in the example screenshot, 32x32 will be the same size as the default in-game text. And if you want to make emoji pack's file size a bit smaller, change the [DXT compression] as well.

## Supported input file formats

- AVIF
- BMP
- DDS
- Farbfeld
- GIF
- HDR
- ICO
- JPEG
- EXR
- PNG
- PNM
- QOI
- TGA
- TIFF
- WebP

## Credits

- [Marco] - I got the idea from his [Server Perks].
- [dkanus] - he hinted to me that I can simply rename `u` package to `utx`.
- Google - Used [android 7 emojis] as an example set.
