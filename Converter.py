# Neat script that converts images and generates a ready to compile template
# Author    : Shtoyan
# Home repo : https://github.com/InsultingPros/KFEmojiPack
# License   : https://www.gnu.org/licenses/gpl-3.0.en.html

from pathlib import Path
from shutil import rmtree
from sys import exit as s_exit
from typing import Any, Final

try:
    from PIL import Image
except Exception as e:
    print(str(e))
    input(
        (
            "Install Pillow to continue -> pip install Pillow\n"
            "Press any key to continue..."
        )
    )
    s_exit()

# what format we want for output files?
FILE_FORMAT: Final[str] = "TGA"
# must be in degrees of 2
RESIZE_DIMENSIONS: Final[tuple[int, int]] = (32, 32)
EXEC_DIRECTIVE: Final[
    str
] = r"#exec TEXTURE IMPORT FILE=Output\{tmp}.TGA NAME={tmp} MIPS=0 MASKED=1 DXT=3"
SERVERPERKS_TEMPLATE: str = (
    """SmileyTags=(iconTexture="{texure_ref}",IconTag="{tag}",bCaseInsensitive=False)"""
)

EXTENSIONS: Final[list[str]] = [
    ".BMP",
    ".DIB",
    ".EPS",
    ".ICNS",
    ".ICO",
    ".IM",
    ".JPG",
    ".JPEG",
    ".J2K",
    ".J2P",
    ".JPX",
    ".MSP",
    ".PCX",
    ".PNG",
    ".PPM",
    ".SGI",
    ".SPIDER",
    ".TGA",
    ".TIFF",
    ".WebP",
    ".XBM",
]


def parse_input_files(input_path: Path, output_path: Path) -> list[str]:
    """Check `Input` folder and convert files to `Output`.
    Returns list of files for further use."""
    file_list: list[str] = []

    for path_object in input_path.rglob("*"):
        if path_object.is_file():
            if path_object.suffix.upper() not in EXTENSIONS:
                continue

            base_name: str = f"{path_object.stem}.{FILE_FORMAT}"
            opath: Path = output_path.joinpath(base_name)
            try:
                with Image.open(path_object) as image:
                    resized_image = image.resize(
                        RESIZE_DIMENSIONS, resample=Image.LANCZOS
                    )
                    resized_image.save(opath)
                    file_list.append(path_object.stem)
            except OSError as err:
                print("cannot convert", path_object, str(err))

    return file_list


def add_files_to_generator(input_path: Path, file_list: list[str]) -> None:
    """Add converted and processed files to `GenerateTexture.uc`"""
    generator_class: Path = input_path.joinpath("GenerateTexture.uc")
    if not generator_class.exists():
        generator_class.touch(exist_ok=True)

    try:
        with open(generator_class, "w") as f:
            f.writelines("class GenerateTexture extends Actor;" + "\n")
            f.writelines("\n")

            for file in file_list:
                f.writelines(EXEC_DIRECTIVE.format(tmp=file) + "\n")

    except Exception as err:
        print(str(err))


def create_config_template(input_path: Path, file_list: list[str]) -> None:
    template_config: Path = input_path.joinpath("ServerPerks_Template.ini")
    if not template_config.exists():
        template_config.touch(exist_ok=True)

    package_name: str = input_path.parent.name
    try:
        with open(template_config, "w") as f:
            f.writelines("[ServerPerksMut.ServerPerksMut]" + "\n")

            for file in file_list:
                f.writelines(
                    SERVERPERKS_TEMPLATE.format(
                        texure_ref=package_name + "." + file, tag=file
                    )
                    + "\n"
                )

    except Exception as err:
        print(str(err))


def remove_dir(path_output: Path) -> None:
    # https://docs.python.org/3/library/shutil.html#rmtree-example
    def remove_readonly(func: Any, path: Any, _: Any) -> None:
        """Clear the readonly bit and reattempt the removal"""
        Path(path).chmod(0o0200)
        func(path)

    if path_output.exists():
        rmtree(path_output, onerror=remove_readonly)


def main() -> None:
    my_path: Path = Path(__file__).parent
    path_input: Path = my_path.joinpath("Input")
    path_output: Path = my_path.joinpath("Output")
    path_classes: Path = my_path.joinpath("Classes")
    path_configs: Path = my_path.joinpath("Configs")

    if not path_input.exists():
        path_input.mkdir(parents=True, exist_ok=True)
        print(
            (
                "You did not have `Input` folder. "
                "We fixed it for you, no go and put some files to there!"
            )
        )
        s_exit()

    if not path_classes.exists():
        path_classes.mkdir(parents=True, exist_ok=True)

    # cleanup `Output` from your old experiments
    remove_dir(path_output)
    path_output.mkdir(parents=True, exist_ok=True)

    if not path_configs.exists():
        path_configs.mkdir(parents=True, exist_ok=True)

    file_list: list[str] = parse_input_files(path_input, path_output)
    if not file_list:
        print("There were no files in Input folder! Exiting!")
        s_exit()

    add_files_to_generator(path_classes, file_list)
    create_config_template(path_configs, file_list)

    input(
        (
            "Script did the magic! Now compile your emoji pack!\n"
            "Press any key to continue..."
        )
    )


if __name__ == "__main__":
    main()
