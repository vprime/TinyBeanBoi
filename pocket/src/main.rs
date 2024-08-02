use std::thread;
use std::time::Duration;
use esp_idf_svc::{
    hal::{
        peripherals::Peripherals
    }
};
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Primitive, PrimitiveStyle, Triangle},
};

// Provides the parallel port and display interface builders
use embedded_hal::spi::MODE_3;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::spi::{config, SpiDeviceDriver, SpiDriverConfig};
use esp_idf_svc::hal::units::FromValueType;
// Provides the Display builder
use mipidsi::{models::ST7789, Builder};
use mipidsi::options::{ColorInversion, ColorOrder, Orientation};
use tiny_bean_boi_lib::{Game, InputState};

fn main() -> anyhow::Result<()>  {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let rst = PinDriver::output(peripherals.pins.gpio23).unwrap();
    let dc = PinDriver::output(peripherals.pins.gpio16).unwrap();
    let mut backlight = PinDriver::output(peripherals.pins.gpio4).unwrap();

    let sclk = peripherals.pins.gpio18;
    let mosi = peripherals.pins.gpio19;
    let cs = peripherals.pins.gpio5;
    let miso = peripherals.pins.gpio20;

    let mut left_button = PinDriver::input(peripherals.pins.gpio0).unwrap();
    let mut right_button = PinDriver::input(peripherals.pins.gpio35).unwrap();

    let mut delay = Ets;

    let config = config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Some(miso),
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    ).unwrap();

    //let spi_device = ExclusiveDevice::new_no_delay(device, cs.unwrap()).unwrap();
    //let di = SPIInterface::new(spi_device, dc);
    let di = SPIInterface::new(device, dc);

    // Define the display from the display interface and initialize it
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .color_order(ColorOrder::Rgb)
        .invert_colors(ColorInversion::Inverted)
        .display_offset(52, 40)
        .display_size(135, 240)
        .init(&mut delay)
        .unwrap();

    backlight.set_high().unwrap();
    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();

    // Draw a smiley face with white eyes and a red mouth
    draw_smiley(&mut display).unwrap();

    let mut game = Game::default();


    loop {
        // Use thread::sleep to prevent Watchdog from triggering
        thread::sleep(Duration::from_millis(10));

        // Obtain input from device
        // The face buttons go low when pressed.
        let current_input_status = InputState{
            left: left_button.is_low(),
            right: right_button.is_low(),
        };

        // Update game
        let output_state = game.update(current_input_status);

    }
}

fn draw_smiley<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Result<(), T::Error> {
    // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
    Circle::new(Point::new(25, 40), 15)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw the right eye as a circle located at (50, 200), with a diameter of 40, filled with white
    Circle::new(Point::new(25, 60), 15)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw an upside down red triangle to represent a smiling mouth
    Triangle::new(
        Point::new(50, 45),
        Point::new(50, 55),
        Point::new(60, 50),
    )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(display)?;

    // // Cover the top part of the mouth with a black triangle so it looks closed instead of open
    // Triangle::new(
    //     Point::new(130, 150),
    //     Point::new(130, 190),
    //     Point::new(150, 170),
    // )
    //     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
    //     .draw(display)?;

    Ok(())
}