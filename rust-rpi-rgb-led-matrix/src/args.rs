//! Provides functions to add arguments to control various parameters of your
//! RGB LED matrix.
use crate::options::{LedMatrixOptions, LedRuntimeOptions};
use clap::{value_t, App, Arg};

/// Given a clap App, adds arguments specific to the matrix initialization.
pub fn add_matrix_args(app: App<'static, 'static>) -> App<'static, 'static> {
    app
    .arg(
        Arg::from_usage(
            "--gpio-mapping=[name] 'Name of GPIO mapping used'")
            .default_value("Regular"))
    .arg(
        Arg::from_usage(
            "--rows=[rows] 'Panel rows. Typically 8, 16, 32 or 64'")
            .default_value("32"))
    .arg(
        Arg::from_usage(
            "--cols=[cols] 'Panel columns. Typically 32 or 64'")
            .default_value("32"))
    .arg(
        Arg::from_usage(
            "--chain=[chained] 'Number of daisy-chained panels'")
            .default_value("1"))
    .arg(
        Arg::from_usage(
            "--parallel=[parallel] 'Parallel chains. range=1..3'")
            .default_value("1"))
    .arg(
        Arg::from_usage(
            "--multiplexing=[0..16] 'Mux type: 0=direct, 1=Stripe, 2=Checkered, 3=Spiral, 4=ZStripe, 5=ZnMirrorZStripe, 6=coreman, 7=Kaler2Scan, 8=ZStripeUneven, 9=P10-128x4-Z, 10=QiangLiQ8, 11=InversedZStripe, 12=P10Outdoor1R1G1-1, 13=P10Outdoor1R1G1-2, 14=P10Outdoor1R1G1-3, 15=P10CoremanMapper, 16=P8Outdoor1R1G1'")
            .default_value("0"))
    .arg(
        Arg::from_usage(
            "--pixel-mapper 'Semicolon-separated list of pixel-mappers to arrange pixels. Optional params after a colon e.g. \"U-mapper;Rotate:90\"\"Available: \"Mirror\", \"Rotate\", \"U-mapper\", \"V-mapper\"'")
            .default_value(""))
    .arg(
        Arg::from_usage(
            "--pwm-bits=[1..11] 'PWM bits'")
            .default_value("11"))
    .arg(
        Arg::from_usage(
            "--brightness=[percent] 'Brightness in percent'")
            .default_value("100"))
    .arg(
        Arg::from_usage(
            "--scan-mode=[0..1] '0 = progressive; 1 = interlaced'")
            .default_value("0"))
    .arg(
        Arg::from_usage(
            "--row-addr-type=[0..4] '0 = default; 1 = AB-addressed panels; 2 = direct row select; 3 = ABC-addressed panels; 4 = ABC Shift + DE direct'")
            .default_value("0"))
    .arg(
        Arg::from_usage(
            "--limit-refresh=[Hz] 'Limit refresh rate to this frequency in Hz. Useful to keep a constant refresh rate on loaded system. 0=no limit'")
            .default_value("0"))
    .arg(
        Arg::from_usage(
            "--rgb-sequence 'Switch if your matrix has led colors swapped'")
            .default_value("RGB"))
    .arg(
        Arg::from_usage(
            "--pwm-lsb-nanoseconds=[ns] 'PWM Nanoseconds for LSB'")
            .default_value("130"))
    .arg(
        Arg::from_usage(
            "--pwm-dither-bits=[0..2] 'Time dithering of lower bits'")
            .default_value("0"))
    .arg(
        Arg::from_usage(
            "--panel-type=[name] 'Needed to initialize special panels. Supported: 'FM6126A', 'FM6127''")
            .default_value(""))
    .arg(
        Arg::from_usage(
            "--slowdown-gpio=[0..4] 'Slowdown GPIO. Needed for faster Pis/slower panels'")
            .default_value("1"))

    // Flags
    .arg(
        Arg::from_usage(
            "--show-refresh 'Show refresh rate'"))
    .arg(
        Arg::from_usage(
            "--inverse 'Switch if your matrix has inverse colors on'"))
    .arg(
        Arg::from_usage(
            "--no-hardware-pulse 'Don't use hardware pin-pulse generation'"))
    .arg(
        Arg::from_usage(
            "--daemon 'Make the process run in the background as daemon'"))
    .arg(
        Arg::from_usage(
            "--no-drop-privs 'Don't drop privileges from 'root' after initializing the hardware'"))
}

/// Given the parsed matches, returns LedMatrixOptions, LedRuntimeOptions`
pub fn matrix_options_from_args<'a>(
    parsed_args: &clap::ArgMatches<'a>,
) -> (LedMatrixOptions, LedRuntimeOptions) {
    let mut options = LedMatrixOptions::new();
    let mut rt_options = LedRuntimeOptions::new();

    let gpio_mapping = parsed_args.value_of("gpio-mapping").unwrap();
    let rows = value_t!(parsed_args, "rows", u32).unwrap();
    let cols = value_t!(parsed_args, "cols", u32).unwrap();
    let chain = value_t!(parsed_args, "chain", u32).unwrap();
    let parallel = value_t!(parsed_args, "parallel", u32).unwrap();
    let multiplexing = value_t!(parsed_args, "multiplexing", u32).unwrap();
    let pixel_mapper = parsed_args.value_of("pixel-mapper").unwrap();
    let pwm_bits = value_t!(parsed_args, "pwm-bits", u8).unwrap();
    let brightness = value_t!(parsed_args, "brightness", u8).unwrap();
    let scan_mode = value_t!(parsed_args, "scan-mode", u32).unwrap();
    let row_addr_type = value_t!(parsed_args, "row-addr-type", u32).unwrap();
    let limit_refresh = value_t!(parsed_args, "limit-refresh", u32).unwrap();
    let rgb_sequence = parsed_args.value_of("rgb-sequence").unwrap();
    let pwm_lsb_nanoseconds = value_t!(parsed_args, "pwm-lsb-nanoseconds", u32).unwrap();
    let pwm_dither_bits = value_t!(parsed_args, "pwm-dither-bits", u32).unwrap();
    let panel_type = parsed_args.value_of("panel-type").unwrap();
    let slowdown_gpio = value_t!(parsed_args, "slowdown-gpio", u32).unwrap();

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
    fn matrix_args_add() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app"]);
        let _slowdown = value_t!(matches, "slowdown-gpio", u32).unwrap();
    }

    #[test]
    fn matrix_args_clap_basic() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--limit-refresh", "42"]);
        let slowdown = value_t!(matches, "limit-refresh", u32).unwrap();
        assert_eq!(slowdown, 42);
    }

    #[test]
    fn matrix_args_to_options() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--pwm-dither-bits", "42"]);
        let (options, _rt_options) = matrix_options_from_args(&matches);
        assert_eq!(options.0.pwm_dither_bits, 42);
    }

    #[test]
    fn matrix_args_to_rt_options() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--slowdown-gpio", "4"]);
        let (_options, rt_options) = matrix_options_from_args(&matches);
        assert_eq!(rt_options.0.gpio_slowdown, 4);
    }

    #[test]
    fn matrix_args_to_rt_options_flag() {
        let app = add_matrix_args(App::new("test"));
        let matches = app.get_matches_from(vec!["app", "--daemon"]);
        let (_options, rt_options) = matrix_options_from_args(&matches);
        assert_eq!(rt_options.0.daemon, 1);
    }
}
