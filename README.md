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

This project aims to make [Server Perks] emoji pack creation much easier. There are several community made packs, but they are well hidden and there is almost no up-to-date documentation on how to make them. So we are filling that gap.

![img](Docs/example.png)

```text
say some smiles: face-with-tears-of-joy pile-of-poo nauseated-face kissing-face face-savouring-delicious-food thinking-face hell yea
```

With **KF Emoji Pack Generator** you can simply drag-n-drop images, run a single exectible and get a ready to compile files. No SDK or long googling required.

This repo provides a compiled example with [android 7 emojis] and a handy script which allows to create your custom pack in few clicks.

## Basic Usage

1. Download `kf_emoji_generator` from [releases], and extract the archive anywhere you want.
2. Run `kf_emoji_generator` once, it will create `input` directory nearby.
3. Put your fancy images there.
4. Run `kf_emoji_generator` again. It will generate several direcories:
    - `output`  - your images from `input` will be resized, converted into `TGA` and moved here.
    - `classes` - generates `GenerateTexture.uc` with corresponding exec directives. This is the file you want to compile.
    - `configs` - generates `ServerPerks_Template.ini` with smiles tag information.
5. Compile the resulting as a usual KF1 mod.
6. Manually change compiled file extension from `u` to `utx`.
7. Copy-paste `SmileyTags` lines from generated `ServerPerks_Template.ini` to your server's `ServerPerks.ini`.
8. Done! Start your server and enjoy.

## Advanced Usage

Here are all available arguments, if you want to tweak your output:

- `-h`, `--help` - Prints the help message.
- `-d`, `--dimensions <number>` - Specifies emoji dimensions. Higher the number, bigger will be the emojii in game chat. (default: 32)
- `-p`, `--package <string>` - Package name, for config output. Defaults to executable's working directory if not specified.
- `--mips <number>` - Set resulting texture's MIPS level. Don't touch if you don't know what is this. (default: 0)
- `--masked <number>` - Set resulting texture's MASKED level. Don't touch if you don't know what is this. (default: 1)
- `--dxt <number>` - Set resulting texture's DXT level. Don't touch if you don't know what is this. (default: 3)

> [!NOTE]
> About emojis dimensions - 128x128 are shown in the example screenshot, 32x32 will be same size as default ingame text. And if you want to make emoji pack's file size a bit smaller, change the [DXT compression] as well.

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

- [Marco] - I took the idea from his [Server Perks].
- [dkanus] - he hinted me that I can simply rename `u` package to `utx`.
- Google - your [android 7 emojis] are the best.
