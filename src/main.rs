use std::ffi::OsStr;

use anyhow::Context;
use image::DynamicImage;

const USAGE: &str = "Usage: rust-imageconv src dst";

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let src = args.next().context(USAGE)?;
    let dst = args.next().context(USAGE)?;
    conv(&src, &dst)?;
    Ok(())
}

fn conv(src: &OsStr, dst: &OsStr) -> anyhow::Result<()> {
    let dst_type = determine_dst_type(dst)?;
    let img = image::open(src)?;
    let bytes = conv_to_type(&img, dst_type);
    std::fs::write(dst, &bytes)?;
    Ok(())
}

fn conv_to_type(img: &DynamicImage, type_: DstType) -> Vec<u8> {
    match type_ {
        DstType::Argb32 => conv_argb32(img),
    }
}

fn conv_argb32(img: &DynamicImage) -> Vec<u8> {
    let mut rgba8 = img.to_rgba8();
    for chunk in rgba8.chunks_mut(4) {
        chunk.rotate_right(1)
    }
    rgba8.into_raw()
}

fn determine_dst_type(path: &OsStr) -> anyhow::Result<DstType> {
    let ext = os_str_ext(path)?;
    DstType::from_ext(ext).context("Couldn't determine extension for destination")
}

fn os_str_ext(path: &OsStr) -> anyhow::Result<&str> {
    let str = path.to_str().context("Could not convert path to str")?;
    let period = str.rfind('.').context("Path doesn't contain period")?;
    Ok(&str[period + 1..])
}

enum DstType {
    Argb32,
}

impl DstType {
    fn from_ext(ext: &str) -> Option<Self> {
        match ext {
            "argb32" => Some(Self::Argb32),
            _ => None,
        }
    }
}
