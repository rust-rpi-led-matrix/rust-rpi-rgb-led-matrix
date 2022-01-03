/// Example showing some basic usage of the C++ library.
use clap::{arg, crate_version, App};
use rpi_led_matrix::{args, LedColor, LedMatrix};

const INTER_LINE_DELAY: std::time::Duration = std::time::Duration::from_millis(16 * 2);

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
    let mut canvas = matrix.offscreen_canvas();
    let (width, height) = canvas.canvas_size();
    let color = LedColor {
        red: 255,
        green: 255,
        blue: 255,
    };
    let num_loops: u32 = matches.value_of_t("loops").unwrap();

    for _ in 0..num_loops {
        let y: i32 = 0;
        for x in 0..width {
            canvas.clear();
            canvas.draw_line(x, y, width - x, height - y, &color);
            canvas = matrix.swap(canvas);
            std::thread::sleep(INTER_LINE_DELAY);
        }

        let x: i32 = width;
        for y in 0..height {
            canvas.clear();
            canvas.draw_line(x, y, width - x, height - y, &color);
            canvas = matrix.swap(canvas);
            std::thread::sleep(INTER_LINE_DELAY);
        }

        let y: i32 = height;
        for x in width..0 {
            canvas.clear();
            canvas.draw_line(x, y, width - x, height - y, &color);
            canvas = matrix.swap(canvas);
            std::thread::sleep(INTER_LINE_DELAY);
        }

        let x: i32 = width;
        for y in height..0 {
            canvas.clear();
            canvas.draw_line(x, y, width - x, height - y, &color);
            canvas = matrix.swap(canvas);
            std::thread::sleep(INTER_LINE_DELAY);
        }
    }
}
