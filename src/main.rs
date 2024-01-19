use embedded_graphics::{
    mono_font::{ascii::FONT_10X20 as A_FONT, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate90)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&A_FONT)
        .text_color(BinaryColor::On)
        .build();

    // The following comments contributed by Emma:
    // FSa3tyu
    // LION
    // DOG
    // KANGAROO
    // ELEPHANT
    // BOBCAT

    let line_height = 16_i32;
    let lines = [
        "H ello", "E mma!", "  Have", "  a", "  nice", "  day!", "  <3",
    ];

    for (i, line) in lines.iter().enumerate() {
        Text::with_baseline(
            line,
            if i == 0 {
                Point::zero()
            } else {
                Point::new(0, i as i32 * line_height)
            },
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
    }

    display.flush().unwrap();

    log::info!("☠︎");
}
