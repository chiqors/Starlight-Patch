# Starlight-Patch

An utility to patch certain features of *a certain anime game*, used to connect to emulators like Starlight, or Grasscutter.

## Prerequisites
[Cargo](https://rust-lang.org/tools/install/)
[Git](https://git-scm.com/) *(optional)*

## Building & using

1. Clone the repository (`git clone https://github.com/kitkat-multiverse/Starlight-Patch.git`), or download it on the GitHub website.
2. Open a terminal in the directory, and run `cargo build --release`
3. Get the DLL in `target/release` and rename it to `Astrolabe.dll` (if file extensions aren't shown, rename it to `Astrolabe`)
4. Move it into the game's root directory

## Compatibility & settings
This patch creates a default settings file at the game root folder called `config.toml`. There, you can change the redirection target, as well as the keys. The patch also requests a remote configuration from Starlight powered servers, for security and easy access.
Additionally, you can apply overrides with the `--ps-addr` and `--ps-port` CLI arguments. On the latest updates, a new in-game UI has been included. Toggle it using **Ctrl + L**.
