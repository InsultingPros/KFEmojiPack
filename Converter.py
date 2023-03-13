import os
from pathlib import Path

try:
    from PIL import Image
except Exception as e:
    print(str(e))
    input("Install Pillow to continue `pip install Pillow`...")

# what format we want for output files?
FILE_FORMAT: str = "TGA"
# must be in degrees of 2
RESIZE_DIMENSIONS: tuple[int, int] = [32, 32]
EXEC_DIRECTIVE: str = (
    r"#exec TEXTURE IMPORT FILE=Output\{tmp}.TGA NAME={tmp} MIPS=0 MASKED=1 DXT=3"
)
SERVERPERKS_TEMPLATE: str = (
    """SmileyTags=(iconTexture="{texure_ref}",IconTag="{tag}",bCaseInsensitive=False)"""
)

formats_list: list[str] = [
    ".BMP",
    ".DIB",
    ".EPS",
    ".GIF",
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
    lancoz: Image.Resampling = Image.Resampling(1)

    for path_object in input_path.rglob("*"):
        if path_object.is_file():
            if path_object.suffix.upper() not in formats_list:
                continue

            base_name: str = "{}.{}".format(path_object.stem, FILE_FORMAT)
            opath: Path = output_path.joinpath(base_name)
            try:
                with Image.open(path_object) as image:
                    resized_image: Image = image.resize(
                        RESIZE_DIMENSIONS, resample=lancoz
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


def main() -> None:
    absolute_path: Path = Path(os.path.dirname(__file__))

    path_input: Path = absolute_path.joinpath("Input")
    path_output: Path = absolute_path.joinpath("Output")
    path_classes: Path = absolute_path.joinpath("Classes")
    path_configs: Path = absolute_path.joinpath("Configs")

    if not path_input.exists():
        path_input.mkdir(parents=True, exist_ok=True)
        print(
            "You did not have `Input` folder. We fixed it for you, no go and put some files to there!"
        )
        exit()

    if not path_classes.exists():
        path_classes.mkdir(parents=True, exist_ok=True)

    if not path_output.exists():
        path_output.mkdir(parents=True, exist_ok=True)

    if not path_configs.exists():
        path_configs.mkdir(parents=True, exist_ok=True)

    file_list: list[str] = parse_input_files(path_input, path_output)
    if len(file_list) == 0:
        print("There were no files in Input folder! Exiting!")
        exit()

    add_files_to_generator(path_classes, file_list)
    create_config_template(path_configs, file_list)

    input("Script did the magic! Now compile your emoji pack!")


if __name__ == "__main__":
    main()
