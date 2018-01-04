#[macro_use]
extern crate clap;
extern crate rand;
extern crate palette;
use clap::{App, Arg};
use std::io::Error;
use rand::Rng;
use palette::{Rgb, Hsl, Hue, Saturate};
use palette::pixel::Srgb;
use palette::FromColor;
use palette::Shade;

fn main() {
    // If any error would occur in inner_main(), print the error.
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }
}

fn inner_main() -> Result<(), Error> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("aoc")
            .help("Amount of colours to display.")
            .required(true)) // Make argument required.
        .get_matches();

    // Define variables.
    let aoc = matches.value_of("aoc").unwrap();
    // Programming isn't about WHY, it's about WHY NOT!
    // WHY is so much of our code panicking upon an error? Why not MARRY Result<T, E> if you love it so much?
    // In fact, why not invent a special safety door that won't kick your butt on the way out, because YOU ARE FIRED.
    let red = rand(0, 255);
    let green = rand(0, 255);
    let blue = rand(0, 255);
    // HSL
    let generated_colour = Hsl::from_rgb(Rgb::from(Srgb::new(red, green, blue)).into());
    let new_colour = Hsl::from_hsl(generated_colour.shift_hue(rand(-80, 80).into()).saturate(rand(0, 255)).lighten(rand(0, 255)));

    let h = new_colour.hue;
    let s = new_colour.saturation as i64;
    let l = 0;

    println!("{:?}", generated_colour);
    println!("{:?}", new_colour.saturation);
    println!("\x1b[48;2;{:?};{};{}m  ", h, s, l);
    println!("{}, {}, {}", red, green, blue);

    // We've made it to the end successfully! Well done, code.
    Ok(())
}

fn rand(one: i64, two: i64) -> f64 {
    rand::thread_rng().gen_range(one, two) as f64
}
