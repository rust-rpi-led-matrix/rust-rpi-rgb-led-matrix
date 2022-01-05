//! Provides functions to add arguments to control various parameters of your
//! RGB LED matrix.
use crate::options::{LedMatrixOptions, LedRuntimeOptions};
use clap::{arg, App};

/// Given a clap App, adds arguments specific to the matrix initialization and returns
/// a new [`App`](clap::App).
#[must_use]
#[allow(clippy::cognitive_complexity)]
pub fn add_matrix_args(app: App<'static>) -> App<'static> {
    app
    .arg(
        arg!(
            --"gpio-mapping" <name> "'Name of GPIO mapping used'")
            .default_value("Regular").required(false))
    .arg(
        arg!(
            --rows <rows> "Panel rows. Typically 8, 16, 32 or 64")
            .default_value("32").required(false))
    .arg(
        arg!(
            --cols <cols> "Panel columns. Typically 32 or 64")
            .default_value("32").required(false))
    .arg(
        arg!(
            --chain <chained> "Number of daisy-chained panels")
            .default_value("1").required(false))
    .arg(
        arg!(
            --parallel <parallel> "Parallel chains. range=1..3")
            .default_value("1").required(false))
    .arg(
        arg!(
            --multiplexing <VAL> "[0,16] Mux type: 0=direct, 1=Stripe, 2=Checkered, 3=Spiral, 4=ZStripe, 5=ZnMirrorZStripe, 6=coreman, 7=Kaler2Scan, 8=ZStripeUneven, 9=P10-128x4-Z, 10=QiangLiQ8, 11=InversedZStripe, 12=P10Outdoor1R1G1-1, 13=P10Outdoor1R1G1-2, 14=P10Outdoor1R1G1-3, 15=P10CoremanMapper, 16=P8Outdoor1R1G1")
            .default_value("0").required(false))
    .arg(
        arg!(
            --"pixel-mapper" <VAL> "Semicolon-separated list of pixel-mappers to arrange pixels. Optional params after a colon e.g. \"U-mapper;Rotate:90\"\"Available: \"Mirror\", \"Rotate\", \"U-mapper\", \"V-mapper\"")
            .default_value("").required(false))
    .arg(
        arg!(
            --"pwm-bits" <VAL> "[1,11] PWM bits")
            .default_value("11").required(false))
    .arg(
        arg!(
            --brightness <percent> "Brightness in percent")
            .default_value("100").required(false))
    .arg(
        arg!(
            --"scan-mode" <VAL> "0 = progressive; 1 = interlaced")
            .default_value("0").required(false))
    .arg(
        arg!(
            --"row-addr-type" <VAL> "0 = default; 1 = AB-addressed panels; 2 = direct row select; 3 = ABC-addressed panels; 4 = ABC Shift + DE direct")
            .default_value("0").required(false))
    .arg(
        arg!(
            --"limit-refresh" <Hz> "Limit refresh rate to this frequency in Hz. Useful to keep a constant refresh rate on loaded system. 0=no limit")
            .default_value("0").required(false))
    .arg(
        arg!(
            --"rgb-sequence" <SEQ> "Switch if your matrix has led colors swapped")
            .default_value("RGB").required(false))
    .arg(
        arg!(
            --"pwm-lsb-nanoseconds" <ns> "PWM Nanoseconds for LSB")
            .default_value("130").required(false))
    .arg(
        arg!(
            --"pwm-dither-bits" <VAL> "[0,2] Time dithering of lower bits")
            .default_value("0").required(false))
    .arg(
        arg!(
            --"panel-type" <name> "Needed to initialize special panels. Supported: 'FM6126A', 'FM6127'")
            .default_value("").required(false))
    .arg(
        arg!(
            --"slowdown-gpio" <VAL> "[0,4] Slowdown GPIO. Needed for faster Pis/slower panels")
            .default_value("1").required(false))

    // Flags
    .arg(
        arg!(
            --"show-refresh" "Show refresh rate"))
    .arg(
        arg!(
            --inverse "Switch if your matrix has inverse colors on"))
    .arg(
        arg!(
            --"no-hardware-pulse" "Don't use hardware pin-pulse generation"))
    .arg(
        arg!(
            --daemon "Make the process run in the background as daemon"))
    .arg(
        arg!(
            --"no-drop-privs" "Don't drop privileges from 'root' after initializing the hardware"))
}

/// Given the parsed matches, returns `(LedMatrixOptions, LedRuntimeOptions)`
///
/// # Panics
/// If the values we try to parse out are invalid from any of the arguments.
#[must_use]
#[rustfmt::skip]
pub fn matrix_options_from_args(
    parsed_args: &clap::ArgMatches,
) -> (LedMatrixOptions, LedRuntimeOptions) {
    let mut options = LedMatrixOptions::new();
    let mut rt_options = LedRuntimeOptions::new();

    let gpio_mapping = parsed_args.value_of("gpio-mapping").expect("Invalid value given for gpio_mapping");
    let rows: u32 = parsed_args.value_of_t("rows").expect("Invalid value given for rows");
    let cols: u32 = parsed_args.value_of_t("cols").expect("Invalid value given for cols");
    let chain: u32 = parsed_args.value_of_t("chain").expect("Invalid value given for chain");
    let parallel: u32 = parsed_args.value_of_t("parallel").expect("Invalid value given for parallel");
    let multiplexing: u32 = parsed_args.value_of_t("multiplexing").expect("Invalid value given for multiplexing");
    let pixel_mapper = parsed_args.value_of("pixel-mapper").expect("Invalid value given for pixel_mapper");
    let pwm_bits: u8 = parsed_args.value_of_t("pwm-bits").expect("Invalid value given for pwm_bits");
    let brightness: u8 = parsed_args.value_of_t("brightness").expect("Invalid value given for brightness");
    let scan_mode: u32 = parsed_args.value_of_t("scan-mode").expect("Invalid value given for scan_mode");
    let row_addr_type: u32 = parsed_args.value_of_t("row-addr-type").expect("Invalid value given for row_addr_type");
    let limit_refresh: u32 = parsed_args.value_of_t("limit-refresh").expect("Invalid value given for limit_refresh");
    let rgb_sequence = parsed_args.value_of("rgb-sequence").expect("Invalid value given for rgb_sequence");
    let pwm_lsb_nanoseconds: u32 = parsed_args.value_of_t("pwm-lsb-nanoseconds").expect("Invalid value given for pwm_lsb_nanoseconds");
    let pwm_dither_bits: u32 = parsed_args.value_of_t("pwm-dither-bits").expect("Invalid value given for pwm_dither_bits");
    let panel_type = parsed_args.value_of("panel-type").expect("Invalid value given for panel_type");
    let slowdown_gpio: u32 = parsed_args.value_of_t("slowdown-gpio").expect("Invalid value given for slowdown_gpio");

    // flags
    let show_refresh: bool = parsed_args.is_present("show-refresh");
    let inverse: bool = parsed_args.is_present("inverse");
    let no_hardware_pulse: bool = parsed_args.is_present("no-hardware-pulse");
    let daemon: bool = parsed_args.is_present("daemon");
    let no_drop_privs: bool = parsed_args.is_present("no-drop-privs");

    options.set_hardware_mapping(gpio_mapping);
    options.set_rows(rows);
    options.set_cols(cols);
    options.set_chain_length(chain);
    options.set_parallel(parallel);
    options.set_multiplexing(multiplexing);
    options.set_pixel_mapper_config(pixel_mapper);
    options.set_pwm_bits(pwm_bits).unwrap();
    options.set_brightness(brightness).unwrap();
    options.set_scan_mode(scan_mode);
    options.set_row_addr_type(row_addr_type);
    options.set_limit_refresh(limit_refresh);
    options.set_led_rgb_sequence(rgb_sequence);
    options.set_pwm_lsb_nanoseconds(pwm_lsb_nanoseconds);
    options.set_pwm_dither_bits(pwm_dither_bits);
    options.set_panel_type(panel_type);

    options.set_hardware_pulsing(!no_hardware_pulse);
    options.set_refresh_rate(show_refresh);
    options.set_inverse_colors(inverse);

    // Part of RuntimeOptions - not accessable in C-based API
    rt_options.set_gpio_slowdown(slowdown_gpio);
    rt_options.set_daemon(daemon);
    rt_options.set_drop_privileges(!no_drop_privs);

    (options, rt_options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::App;

    #[test]
    #[serial_test::serial]
    fn matrix_args_add() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app"]);
        let _slowdown: u32 = matches.value_of_t("slowdown-gpio").unwrap();
    }

    #[test]
    #[serial_test::serial]
    fn matrix_args_clap_basic() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--limit-refresh", "42"]);
        let slowdown: u32 = matches.value_of_t("limit-refresh").unwrap();
        assert_eq!(slowdown, 42);
    }

    #[test]
    #[serial_test::serial]
    fn matrix_args_to_options() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--pwm-dither-bits", "42"]);
        let (options, _rt_options) = matrix_options_from_args(&matches);
        assert_eq!(options.0.pwm_dither_bits, 42);
    }

    #[test]
    #[serial_test::serial]
    fn matrix_args_to_rt_options() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--slowdown-gpio", "4"]);
        let (_options, rt_options) = matrix_options_from_args(&matches);
        assert_eq!(rt_options.0.gpio_slowdown, 4);
    }

    #[test]
    #[serial_test::serial]
    fn matrix_args_to_rt_options_flag() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--daemon"]);
        let (_options, rt_options) = matrix_options_from_args(&matches);
        assert_eq!(rt_options.0.daemon, 1);
    }
}
