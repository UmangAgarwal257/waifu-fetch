# waifu-fetch

Set a random waifu image as your KDE Plasma wallpaper using images from [waifu.pics](https://waifu.pics/).

## Features

- SFW by default, NSFW opt-in (`--nsfw`)
- Choose from multiple categories
- KDE Plasma support (Wayland/X11)
- Instant wallpaper refresh (auto restarts Plasma shell)
- Simple CLI usage

## Usage

```bash
waifu-fetch
waifu-fetch --category neko
waifu-fetch --nsfw --category waifu
```

## Categories

**SFW:**

- waifu
- neko
- shinobu
- megumin
- bully
- cuddle
- cry
- hug
- awoo
- kiss
- lick
- pat
- smug
- bonk
- yeet
- blush
- smile
- wave
- highfive
- handhold
- nom
- bite
- glomp
- slap
- kill
- kick
- happy
- wink
- poke
- dance
- cringe

**NSFW:**

- waifu
- neko
- trap

See [waifu.pics docs](https://waifu.pics/docs) for more info.

## Installation

1. **Build the project:**

   ```bash
   cargo build --release
   ```

2. **Copy the binary to a global location (requires sudo):**
   ```bash
   sudo cp target/release/waifu-fetch /usr/local/bin/
   ```
   _(This works for most Linux distributions. Make sure `/usr/local/bin` is in your `$PATH`.)_

## Requirements

- **Operating System:** Linux (tested on Fedora KDE Plasma)
- **Desktop Environment:** KDE Plasma (Wayland or X11)
- **Dependencies:**
  - [Cargo & Rust](https://rustup.rs/) (for building)
  - `qdbus` (usually provided by `qt5-qttools`)
  - `kquitapp5` and `kstart5` (provided by KDE Plasma)
  - Internet connection (to fetch images)

**Note:**  
This CLI only works on KDE Plasma desktops with the required tools installed.  
Other desktop environments are not supported.

## How it works

- Downloads a random image from waifu.pics based on your chosen category and mode (SFW/NSFW).
- Saves the image as `waifu-wallpaper.jpg` in your home directory.
- Sets it as your KDE Plasma wallpaper using DBus.
- Restarts the Plasma shell for instant wallpaper refresh.

## Example Output

```text
✅ SFW mode enabled. Only safe images will be fetched.
📂 Category: neko
🎴 Fetched image URL: https://i.waifu.pics/xxxxxxx.jpg
📁 Temporary image file created.
📁 Wallpaper image saved successfully.
🖼️  Wallpaper set using KDE Plasma!
🔄 Plasma shell restarted for instant wallpaper refresh.
✨ Enjoy your new waifu wallpaper!
```

**Enjoy your new waifu wallpaper!**
