use std::{
    fs::File,
    io::{prelude::*, BufWriter},
    path::PathBuf,
    str::FromStr,
};
use structopt::StructOpt;
use structopt::clap::{AppSettings::DeriveDisplayOrder};
#[derive(StructOpt, Debug)]
#[structopt(name = "pgm_converter", setting = DeriveDisplayOrder )]
struct CliOption {
    /// Output PGM Type (b: Binary, a: Ascii)
    #[structopt(short, long)]
    mode: FileType,
    /// Source file path
    #[structopt(short, long, parse(from_os_str))]
    src: PathBuf,
    /// Output file path
    #[structopt(short, long, parse(from_os_str))]
    dst: PathBuf,

    /// left corrdinate
    #[structopt(short, long, default_value = "0")]
    left: u32,
    /// top cordinate
    #[structopt(short, long, default_value = "0")]
    top: u32,

    /// width (Optional)
    #[structopt(short, long)]
    width: Option<u32>,
    /// height (Optional)
    #[structopt(short, long)]
    height: Option<u32>,
}

#[derive(Debug)]
enum FileType {
    Binary,
    Ascii,
}
impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "binary" | "Binary" => Ok(FileType::Binary),
            "a" | "ascii" | "Ascii" => Ok(FileType::Ascii),
            _ => Err(
                r#"Unknown type. Use "b" or "Binary" to P5 File, "a" or "Ascii" to P2 File."#
                    .to_string(),
            ),
        }
    }
}
fn main() {
    let opt: CliOption = CliOption::from_args();
    match doit(opt) {
        Ok(v) => println!("{}", v),
        Err(v) => println!("Error: {}", v),
    }
}
fn doit(opt: CliOption) -> Result<String, String> {
    // read image
    let img = match {
        match image::open(opt.src.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(format!(
                    "source img not found: {}",
                    opt.src.to_str().unwrap()
                ))
            }
        }
        .as_luma8()
    } {
        Some(v) => v,
        None => {
            return Err("sorce file is not 8bit 
        Monochrome"
                .to_string())
        }
    }
    .to_owned();

    let (img_width, img_height) = img.dimensions();

    let (x_range, y_range) = {
        let left = opt.left;
        let top = opt.top;
        let right = if let Some(width) = opt.width {
            let row_right = width + left;
            if row_right > img_width {
                img_width
            } else {
                row_right
            }
        } else {
            img_width
        };
        let bottom = if let Some(height) = opt.height {
            let row_bottom = height + top;
            if row_bottom > img_height {
                img_height
            } else {
                row_bottom
            }
        } else {
            img_height
        };
        (left..right, top..bottom)
    };

    let mut writer = BufWriter::new(File::create(opt.dst).unwrap());

    let max = x_range
        .clone()
        .zip(y_range.clone())
        .map(|(x, y)| img.get_pixel(x, y).0[0])
        .max()
        .unwrap();

    let magic_number = match opt.mode {
        FileType::Binary => "P5",
        FileType::Ascii => "P2",
    };

    let header = format!(
        "{}\n{} {}\n{}\n",
        magic_number,
        x_range.len(),
        y_range.len(),
        max
    );
    writer.write_all(header.as_bytes()).unwrap();

    // write body
    match opt.mode {
        FileType::Ascii => {
            for y in y_range {
                let line = (x_range.clone())
                    .map(|x| format!("{}", img.get_pixel(x, y).0[0]))
                    .collect::<Vec<_>>()
                    .join(" ");
                let line_lf = line + "\n";
                writer.write_all(line_lf.as_bytes()).unwrap();
            }
        }
        FileType::Binary => {
            for y in y_range {
                for x in x_range.clone() {
                    writer.write_all(&img.get_pixel(x, y).0).unwrap();
                }
            }
        }
    }

    Ok("finish".to_string())
}
