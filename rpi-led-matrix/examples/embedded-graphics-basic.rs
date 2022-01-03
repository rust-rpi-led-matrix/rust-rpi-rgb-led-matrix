/// Example showing some basic usage of the C++ library.
use clap::{arg, crate_version, App};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};
use rpi_led_matrix::{args, LedMatrix};

const DELAY: std::time::Duration = std::time::Duration::from_secs(5);

fn main() {
    let app = args::add_matrix_args(
        App::new("C++ Library Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!())
            .arg(
                arg!(--loops <LOOPS> "number of cycles to spin the line")
                    .default_value("5")
                    .required(false),
            ),
    );
    let matches = app.get_matches();
    let (options, rt_options) = args::matrix_options_from_args(&matches);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.canvas();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::new(64, 0, 128), 1);
    let fill = PrimitiveStyle::with_fill(Rgb888::new(0, 128, 32));
    let text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::new(0xff, 0xff, 0xff));

    let yoffset = 10;

    // Draw a 3px wide outline around the matrix.
    // let display_size = canvas.size();
    let (width, height) = canvas.canvas_size();
    Rectangle::with_corners(
        Point::zero(),
        Point::new(width as i32 - 1, height as i32 - 1),
    )
    .into_styled(thin_stroke)
    .draw(&mut canvas)
    .unwrap();

    // Draw a triangle.
    Triangle::new(
        Point::new(4, 8 + yoffset),
        Point::new(4 + 8, 8 + yoffset),
        Point::new(4 + 4, yoffset),
    )
    .into_styled(fill)
    .draw(&mut canvas)
    .unwrap();

    // Draw a filled square
    Rectangle::with_corners(Point::new(52, yoffset), Point::new(16, 16))
        .into_styled(fill)
        .draw(&mut canvas)
        .unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new((width) / 2, yoffset - 2), 5)
        .into_styled(fill)
        .draw(&mut canvas)
        .unwrap();

    // Draw centered text.
    let eg_text = "EG+";
    Text::new(eg_text, Point::new(16, 16), text_style)
        .draw(&mut canvas)
        .unwrap();

    let rpi_text = "RPi";
    Text::new(rpi_text, Point::new(16, 22), text_style)
        .draw(&mut canvas)
        .unwrap();

    std::thread::sleep(DELAY);
}
