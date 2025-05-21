use reqwest;
use std::{env, fs, path::Path};
const FONT_FILE_NAME: &str = "NotoSerifCJKjp-VF.otf";
const FONT_FILE_URL_BASE: &str = "https://github.com/notofonts/noto-cjk/raw/refs/tags/Serif2.003/Serif/Variable/OTF/";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let font_url = format!("{}{}", FONT_FILE_URL_BASE, FONT_FILE_NAME);
    let out_dir = env::var("OUT_DIR").unwrap();
    let font_path = Path::new(&out_dir).join(FONT_FILE_NAME);
    let font_data = reqwest::blocking::get(font_url)
        .unwrap()
        .bytes()
        .unwrap();
    fs::write(&font_path, &font_data).unwrap();
    println!("cargo:rustc-env=FONT_PATH={}", font_path.display());
}
