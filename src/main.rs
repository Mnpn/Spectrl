// Only use GTK on Windows.
#![windows_subsystem = "windows"]
#[cfg(gtk)] extern crate gdk;
#[cfg(gtk)] extern crate gtk;
#[cfg(gtk)] use gdk::RGBA;
#[cfg(gtk)] use gtk::prelude::*;
#[cfg(gtk)] use gtk::{Box as GtkBox, Label, Orientation, StateFlags, Window, WindowType};

#[macro_use]
extern crate clap;
extern crate rand;
extern crate palette;
use clap::{App, Arg};
use std::error::Error;
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
}

fn inner_main() -> Result<(), Box<Error>> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("aoc")
            .help("Amount of colours to display.")
            .required(true)) // Make argument required.
        .get_matches();

    // Create a GTK window.
    #[cfg(gtk)]
    gtk::init()?;
    #[cfg(gtk)]
    let window = Window::new(WindowType::Toplevel);
    #[cfg(gtk)] {
        window.set_title(&format!("Spectrl {}", crate_version!()));
        window.set_default_size(300, 10);
    }
    #[cfg(gtk)]
    let container = GtkBox::new(Orientation::Vertical, 0);
    #[cfg(gtk)]
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
            .shift_hue(rand(-40, 40).into())
            .saturate(rand(0, 100) / 100.0)
            .lighten(rand(0, 100) / 100.0);
        let rgb = Rgb::from_hsv(new_colour); // Turn HSV into RGB.

        // Make f64s into i64s.
        let r = rgb.red * 100.0;
        let g = rgb.green * 100.0;
        let b = rgb.blue * 100.0;

        // If any value would drop below zero (which we can't display), continue.
        if r < 0.0 || g < 0.0 || b < 0.0 {
            continue;
        }

        let (r, g, b) = (r as u8, g as u8, b as u8);

        #[cfg(not(gtk))] { // Print unless GTK
            println!(
                "\x1b[38;2;{ri};{gi};{bi}m\x1b[48;2;{r};{g};{b}m   #{r:02X}{g:02X}{b:02X}   \x1b[0;0m",
                r = r,
                g = g,
                b = b,
                ri = 255 - r,
                gi = 255 - g,
                bi = 255 - b
            );
        }
        #[cfg(gtk)] { // Make GTK labels if compiled with GTK
            use std::fmt::Write;

            let mut string = String::with_capacity(1 + 2*3);
            write!(string, "#{:02X}{:02X}{:02X}", r, g, b).unwrap();

            let label = Label::new(&*string);
            label.override_color(StateFlags::NORMAL, &RGBA {
                red: 1.0 - rgb.red,
                green: 1.0 - rgb.green,
                blue: 1.0 - rgb.blue,
                alpha: 1.0
            });
            label.connect_draw(move |label, ctx| {
                let rect = label.get_allocation();
                println!("{:?}", rect);
                println!("{:?}", rgb);

                ctx.rectangle(0.0, 0.0, rect.width as f64, rect.height as f64);
                ctx.set_source_rgb(rgb.red, rgb.green, rgb.blue);
                ctx.fill();

                Inhibit(false)
            });
            container.add(&label);
        }
        aoc -= 1;
    }
    #[cfg(gtk)] {
        window.show_all();
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        gtk::main();
    }

    // We've made it to the end successfully! Well done, code.
    Ok(())
}

// Rand function takes i64 values and returns f64s.
fn rand(one: i64, two: i64) -> f64 {
    rand::thread_rng().gen_range(one, two) as f64
    // This mostly helps me, so that I can just use `rand(0, 1)`
    // instead of `rand::thread_rng().gen_range(0, 1)` every time.
}
