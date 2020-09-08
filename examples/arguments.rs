/// Extremely simple use of arguments to create matrix options
use clap::{crate_version, App};
use rpi_led_matrix::args;

fn main() {
    let app = args::add_matrix_args(
        App::new("Argument Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!()),
    );
    let matches = app.get_matches();
    let (options, rt_options) = args::matrix_options_from_args(&matches);
    println!("Options: {:?}", options);
    println!("Runtime Options: {:?}", rt_options);
}
