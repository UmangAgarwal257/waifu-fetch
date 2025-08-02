use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

/// Set a random waifu image as your KDE Plasma wallpaper.
#[derive(Parser)]
#[command(name = "waifu-fetch")]
#[command(author = "Your Name")]
#[command(version = "0.1")]
#[command(about = "Set a random waifu image as wallpaper")]
struct Args {
    /// Enable NSFW mode (explicit images)
    #[arg(long)]
    nsfw: bool,
    /// Image category (default: neko)
    #[arg(long, default_value = "neko")]
    category: String,
}

#[derive(Deserialize)]
struct WaifuResponse {
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mode = if args.nsfw { "nsfw" } else { "sfw" };
    let category = &args.category;

    // Mode info
    if args.nsfw {
        println!("⚠️  NSFW mode enabled! Images may be explicit.");
    } else {
        println!("✅ SFW mode enabled. Only safe images will be fetched.");
    }
    println!("📂 Category: {}", category);

    // Fetch image URL
    let url = format!("https://api.waifu.pics/{}/{}", mode, category);
    let client = Client::new();
    let res: WaifuResponse = client.get(&url).send()?.json()?;
    println!("🎴 Fetched image URL.");

    // Download image bytes
    let img_bytes = client.get(&res.url).send()?.bytes()?;

    // Save image to a permanent location in the user's home directory
    let home_dir = env::var("HOME")?;
    let img_path = format!("{}/waifu-wallpaper.jpg", home_dir);
    {
        let mut file = File::create(&img_path)?;
        file.write_all(&img_bytes)?;
    }
    println!("📁 Wallpaper image saved successfully.");

    // Set wallpaper using KDE DBus
    let script = format!(
        "var allDesktops = desktops();for (i=0;i<allDesktops.length;i++) {{d = allDesktops[i];d.wallpaperPlugin = 'org.kde.image';d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');d.writeConfig('Image', 'file://{}')}}",
        img_path
    );
    let status = Command::new("qdbus")
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &script,
        ])
        .status()?;

    if status.success() {
        println!("🖼️  Wallpaper set using KDE Plasma!");
        let _ = Command::new("kquitapp5").arg("plasmashell").status();
        let _ = Command::new("kstart5").arg("plasmashell").status();
        println!("🔄 Plasma shell restarted for instant wallpaper refresh.");
        println!("✨ Enjoy your new waifu wallpaper!");
    } else {
        eprintln!("❌ Failed to set wallpaper using KDE Plasma.");
        std::process::exit(1);
    }

    Ok(())
}
