/// Example showing some basic usage of the C++ library.
use clap::{crate_version, App, Arg, value_t};
use rpi_led_matrix::{args, LedMatrix};
use embedded_graphics::{
    fonts::{Font6x6, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
};

const DELAY: std::time::Duration = std::time::Duration::from_secs(5);

fn main() {
    let app = args::add_matrix_args(
        App::new("C++ Library Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!())
            .arg(Arg::from_usage("--loops=[LOOPS] 'number of cycles to spin the line'").default_value("5"))
    );
    let matches = app.get_matches();
    let (options, rt_options) = args::matrix_options_from_args(&matches);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.canvas();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = TextStyle::new(Font6x6, BinaryColor::On);

    let yoffset = 10;

    // Draw a 3px wide outline around the matrix.
    // let display_size = canvas.size();
    let s = canvas.size();
    let (width, height) = (s.width as i32, s.height as i32);
    Rectangle::new(
        Point::zero(),
        Point::new(
            width as i32 - 1,
            height as i32 - 1,
        ))
        .into_styled(thin_stroke)
        .draw(&mut canvas).unwrap();

    // Draw a triangle.
    Triangle::new(
        Point::new(4, 8 + yoffset),
        Point::new(4 + 8, 8 + yoffset),
        Point::new(4 + 4, yoffset),
    )
    .into_styled(thick_stroke)
    .draw(&mut canvas).unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Point::new(16, 16))
        .into_styled(fill)
        .draw(&mut canvas).unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new((width) / 2, yoffset - 2), 5)
        .into_styled(thick_stroke)
        .draw(&mut canvas).unwrap();

    // Draw centered text.
    let eg_text = "EG+";
    Text::new(eg_text, Point::new(16, 16))
        .into_styled(text_style)
        .draw(&mut canvas).unwrap();

    let rpi_text = "RPi";
    Text::new(rpi_text, Point::new(16, 22))
        .into_styled(text_style)
        .draw(&mut canvas).unwrap();

    std::thread::sleep(DELAY);
}
