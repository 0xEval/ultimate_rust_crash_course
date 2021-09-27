// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

extern crate clap;
use clap::{Arg, App};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    let matches = App::new("mirage")
        .about("A simple image processing tool")
        .arg(Arg::with_name("input")
            .help("Target image file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("Target output file")
            .required(true)
            .takes_value(true))
        .subcommand(
            App::new("filter")
                .about("Apply a range of filters to input file")
                .author("0xEval")
                .arg(
                    Arg::with_name("blur")
                    .help("Apply blur filter to input")
                    .long("blur")
                    .takes_value(true)
                    .value_name("amount")
                )
                .arg(
                    Arg::with_name("brighten")
                    .help("Apply brighten filter to input")
                    .long("brighten")
                    .takes_value(true)
                    .value_name("amount")
                )
                .arg(
                    Arg::with_name("grayscale")
                    .help("Apply grayscale filter to input")
                    .long("grayscale")
                )
                .arg(
                    Arg::with_name("crop")
                    .help("Crop input image to given dimensions (x, y, w, h)")
                    .long("crop")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(4)
                    .value_names(&["x", "y", "width", "height"])
                )
                .arg(
                    Arg::with_name("rotate")
                    .help("Rotate image")
                    .long("rotate")
                    .possible_values(&["90","180","270"])
                    .takes_value(true)
                    .value_name("angle")
                )
                .arg(
                    Arg::with_name("invert")
                    .help("Invert the colors of input")
                    .long("invert")
                    .takes_value(false)
                )
                .arg(
                    Arg::with_name("generate")
                    .help("Generate an image with given colors")
                    .long("generate")
                    .takes_value(true)
                    .min_values(3)
                    .value_names(&["red", "green", "blue"])
                )
                .arg(
                    Arg::with_name("fractal")
                    .help("Generate a fractal and save to output file")
                    .long("fractal")
                )
        ) 
        .get_matches();

    let infile = String::from(matches.value_of("input").unwrap());
    let outfile = String::from(matches.value_of("output").unwrap());

    if let Some(filter_matches) = matches.subcommand_matches("filter") {
        if filter_matches.is_present("blur") {
            let amt = filter_matches.value_of("blur").unwrap().parse::<f32>().unwrap();
            blur(infile.to_owned(), outfile.to_owned(), amt);
        }
        if filter_matches.is_present("brighten") {
            let amt = filter_matches.value_of("brighten").unwrap().parse::<i32>().unwrap();
            brighten(infile.to_owned(), outfile.to_owned(), amt);
        }
        if filter_matches.is_present("grayscale") {
            grayscale(infile.to_owned(), outfile.to_owned());
        }
        if filter_matches.is_present("crop") {
            let vals: Vec<&str> = filter_matches.values_of("crop").unwrap().collect();
            let x = vals[0].parse::<u32>().unwrap();
            let y = vals[0].parse::<u32>().unwrap();
            let width = vals[0].parse::<u32>().unwrap();
            let height = vals[0].parse::<u32>().unwrap();
            crop(infile.to_owned(), outfile.to_owned(), x, y, width, height);
        }
        if filter_matches.is_present("rotate") {
            let angle = filter_matches.value_of("rotate").unwrap().parse::<u32>().unwrap();
            rotate(infile.to_owned(), outfile.to_owned(), angle);
        }
        if filter_matches.is_present("invert") {
            invert(infile.to_owned(), outfile.to_owned());
        }
        if filter_matches.is_present("generate") {
            let vals: Vec<&str> = filter_matches.values_of("generate").unwrap().collect();
            let r = vals[0].parse::<u8>().unwrap();
            let g = vals[1].parse::<u8>().unwrap();
            let b = vals[2].parse::<u8>().unwrap();
            generate(outfile.to_owned(), r, g, b);
        }
        if filter_matches.is_present("fractal") {
            fractal(outfile.to_owned());
        }
    }
}

fn blur(infile: String, outfile: String, amt: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");

    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(amt);

    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amt: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(amt);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    let img2 = img.crop(x, y, width, height);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, angle: u32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    let img2: image::DynamicImage;
    match angle {
        90 => img2 = img.rotate90(),
        180 => img2 = img.rotate180(),
        270 => img2 = img.rotate270(),
        _ => return
    }

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();

    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    let width = 800;
    let height = 800;

    // Create an ImageBuffer -- see fractal() for an example
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    // Set the image to some solid color. -- see fractal() for an example
    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([r, g, b]);
    }

    // Challenge 2: Generate something more interesting!
    imgbuf.save(outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
