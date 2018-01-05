// Only use GTK on Windows.
#[cfg(windows)]
extern crate gtk;
#[cfg(windows)]
use gtk::prelude::*;

#[macro_use]
extern crate clap;
extern crate rand;
extern crate palette;
use clap::{App, Arg};
use std::io::Error;
use rand::Rng;
use palette::{Rgb, Hsv, Hue, Saturate};
use palette::pixel::Srgb;
use palette::FromColor;
use palette::Shade;

fn main() {
    // If any error would occur in inner_main(), print the error.
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }

    // We're using GTK to display colours, but only on Windows.
    #[cfg(windows)] {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
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
    // Create a GTK window, but only on Windows.
    #[cfg(windows)]
    let window = Window::new(WindowType::Toplevel);
    #[cfg(windows)] {
        window.set_title("Spectrl");
        window.set_default_size(800, 600);
    }
    #[cfg(windows)]
    let container = GtkBox::new(Orientation::Vertical, 0);
    #[cfg(windows)]
    window.add(&container);

    let mut aoc = value_t!(matches, "aoc", i32).unwrap_or_else(|e| e.exit());
    // AOC is a string and we want it to be an i32.
    // If it fails (Number isn't a number), exit and error.

    // Programming isn't about WHY, it's about WHY NOT!
    // WHY is so much of our code panicking upon an error?
    // Why not MARRY Result<T, E> if you love it so much?
    // In fact, why not invent a special safety door that
    // won't kick your butt on the way out, because YOU ARE FIRED.

    // Randomise three initial colours.
    let red = rand(0, 100);
    let green = rand(0, 100);
    let blue = rand(0, 100);

    // Create an HSV colour from RGB inputs.
    let generated_colour = Hsv::from_rgb(
        Rgb::from(Srgb::new(red / 100.0, green / 100.0, blue / 100.0)).into(),
    );
    while aoc > 0 {
        // Randomly change HSV values of generated_colour.
        let new_colour = generated_colour
            .shift_hue(rand(-80, 80).into())
            .saturate(rand(0, 100) / 100.0)
            .lighten(rand(0, 100) / 100.0);
        let rgb = Rgb::from_hsv(new_colour); // Turn HSV into RGB.

        // Make f64s into i64s.
        let r = (rgb.red * 100.0) as i64;
        let g = (rgb.green * 100.0) as i64;
        let b = (rgb.blue * 100.0) as i64;

        // If any value would drop below zero (which we can't display), continue.
        if r < 0 || g < 0 || b < 0 {
            continue;
        }

        #[cfg(not(windows))] { // Print on all systems, except for Windows.
            println!(
                "\x1b[48;2;{r};{g};{b}m   #{r:02X}{g:02X}{b:02X}   \x1b[0;0m",
                r = r,
                g = g,
                b = b
            );
        }
        #[cfg(windows)] { // Make GTK Labels on Windows.
            let label = Label::new("hello");
            container.add(&label);
        }
        aoc -= 1;
    }
    #[cfg(windows)]
    window.show_all();
    #[cfg(windows)]
    gtk::main();

    // We've made it to the end successfully! Well done, code.
    Ok(())
}

// Rand function takes i64 values and returns f64s.
fn rand(one: i64, two: i64) -> f64 {
    rand::thread_rng().gen_range(one, two) as f64
    // This mostly helps me, so that I can just use `rand(0, 1)`
    // instead of `rand::thread_rng().gen_range(0, 1)` every time.
}
