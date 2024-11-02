use std::thread;
use std::time::{Duration};

use esp_idf_svc::{
    hal::{
        prelude,
        peripherals::Peripherals,
        peripheral::Peripheral,
        delay::Ets,
        gpio::{PinDriver, RTCPin},
        spi::{config, SpiDeviceDriver, SpiDriverConfig},
        units::FromValueType,
        reset::WakeupReason,
        ulp::UlpDriver,
        task::CriticalSection,
        into_ref,
    }
};
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*
};
use embedded_hal::spi::MODE_3;
use esp_idf_sys::{self as _, esp, time}; // If using the binstart feature of esp-idf-sys, always keep this module loaded
use log::info;
use mipidsi::{
    models::ST7789, 
    Builder,
    options::{ColorInversion, ColorOrder},
};

use tiny_bean_boi_lib::{Game, InputState};

mod ulp_code_vars {
    include!(env!("ULP_FSM_RS"));
}

const ULP_CODE: &[u8] = include_bytes!(env!("ULP_FSM_BIN"));

fn main() -> anyhow::Result<()>  {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    
    let peripherals = Peripherals::take()?;
    let mut ulp_driver = UlpDriver::new(peripherals.ulp)?;

    let wakeup_reason = WakeupReason::get();
    info!("Wakeup Reason: {:?}", wakeup_reason);
    if wakeup_reason == WakeupReason::ULP {
        // Pull information from ULP / RTC memory
        let (time, wakeup) = {
          let _cs = CriticalSection::new();
            unsafe {
                let time = ulp_driver.read_word(ulp_code_vars::time)?.value();
                let wakeup = ulp_driver.read_word(ulp_code_vars::wakeup)?.value();
                (time, wakeup)
            }
        };
        info!("woke up at Time: {:?} Wakeup time: {:?}", time, wakeup);
    }


    let spi = peripherals.spi2;

    let rst = PinDriver::output(peripherals.pins.gpio23)?;
    let dc = PinDriver::output(peripherals.pins.gpio16)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio4)?;

    let sclk = peripherals.pins.gpio18;
    let mosi = peripherals.pins.gpio19;
    let cs = peripherals.pins.gpio5;
    let miso = peripherals.pins.gpio20;

    let right_button_pin = peripherals.pins.gpio35.into_ref();
    let right_button_pin_rtc = right_button_pin.rtc_pin();
    let left_button_pin = peripherals.pins.gpio0.into_ref();
    let left_button_pin_rtc = left_button_pin.rtc_pin();

    let left_button = PinDriver::input(left_button_pin)?;
    let right_button = PinDriver::input(right_button_pin)?;

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
    )?;

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

    // We will eventually want to turn off the backlight and screen to go into low power mode
    // That will be an extended scope option tho
    backlight.set_high()?;
    
    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();

    let mut game = Game::default();

    // ULP stuff
    if wakeup_reason == WakeupReason::Unknown {
        unsafe {
            ulp_driver.load(ULP_CODE)?;
            // Write what we need to save to RTC memory.
            // ulp_driver.write_word(ulp_code_vars::edge_count, 0)?;
            // ulp_driver.write_word(ulp_code_vars::edge_count_to_wake_up, 10)?;
            // ulp_driver.write_word(ulp_code_vars::debounce_counter, 3)?;
            // ulp_driver.write_word(ulp_code_vars::debounce_max_count, 3)?;
            // ulp_driver.write_word(ulp_code_vars::next_edge, 0)?;
            // ulp_driver.write_word(ulp_code_vars::io_number, left_button_pin_rtc as _)?;
            ulp_driver.write_word(ulp_code_vars::wakeup, 300)?;

            // Start the program
            ulp_driver.start(ulp_code_vars::entry)?;
        }
    }
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
        game.update(&mut display, current_input_status);


        // If the game is "going to sleep" then trigger the sleep mode.
        if game.sleep {
            println!("Going to sleep!");
            unsafe {
                ulp_driver.write_word(ulp_code_vars::wakeup, 300)?;
                esp!(esp_idf_sys::esp_sleep_enable_ulp_wakeup())?;
                esp_idf_sys::esp_deep_sleep_start();
            }
            break;
        }
    }
    Ok(())
}

