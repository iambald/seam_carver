extern crate docopt;
extern crate rustc_serialize;
extern crate image;

use docopt::Docopt;

static USAGE: &'static str = "
Usage:  seam_carver <source> <dest> -w <width> -h <height>

Options:
    <source>                            Source image location.
    <dest>                              Destination image location.
    -w <width>, --width <width>         Output width in pixels.
    -h <height>, --height <height>      Output height in pixels.
";

#[derive(RustcDecodable)]
struct Args {
    arg_source  : String,
    arg_dest    : String,
    flag_width   : u64,
    flag_height  : u64
}

fn main() {
    let args : Args = Docopt::new(USAGE)
                      .and_then(|d| d.decode())
                      .unwrap_or_else(|e| e.exit());

    println!("({}, {})", args.flag_width, args.flag_height);
    let img = image::open(args.arg_source).unwrap();
    let input = img.as_rgb8().unwrap();

    input.save(args.arg_dest);
}
