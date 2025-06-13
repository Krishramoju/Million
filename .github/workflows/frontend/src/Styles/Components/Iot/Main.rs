#![no_std]
#![no_main]

use esp_idf_hal::adc::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_sys as _;
use core::fmt::Write;

#[derive(Clone, Copy)]
enum Mode {
    Car,
    Civil,
    Doctor,
    Educator,
    Farmer,
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut uart = UartDriver::new(
        peripherals.uart1,
        pins.gpio4, // TX
        pins.gpio5, // RX
        &config::Config::default().baudrate(115200),
    )
    .unwrap();

    let mut adc = AdcDriver::new(peripherals.adc1, &AdcConfig::new()).unwrap();
    let mut analog = adc.adc1_channels().adc1_0; // GPIO36
    let digital = PinDriver::input(pins.gpio13).unwrap(); // GPIO13

    let current_mode = Mode::Farmer; // change this to switch applications

    writeln!(uart, "Neuro OS Microcontroller: Mode {:?}", mode_name(current_mode)).ok();

    loop {
        match current_mode {
            Mode::Car => run_car_mode(&mut uart, &mut analog, &digital, &adc),
            Mode::Civil => run_civil_mode(&mut uart, &mut analog, &digital, &adc),
            Mode::Doctor => run_doctor_mode(&mut uart, &mut analog, &digital, &adc),
            Mode::Educator => run_educator_mode(&mut uart, &mut analog, &digital, &adc),
            Mode::Farmer => run_farmer_mode(&mut uart, &mut analog, &digital, &adc),
        }
        FreeRtos::delay_ms(1000);
    }
}

fn run_car_mode(uart: &mut UartDriver, analog: &mut AdcChannelDriver<'_, ADC1>, digital: &PinDriver<Input>, adc: &AdcDriver) {
    let speed = adc.read(analog).unwrap(); // simulate speed sensor
    let obstacle = digital.is_high().unwrap(); // simulate proximity alert
    writeln!(uart, "[CAR] Speed:{} | Obstacle:{}", speed, obstacle as u8).ok();
}

fn run_civil_mode(uart: &mut UartDriver, analog: &mut AdcChannelDriver<'_, ADC1>, digital: &PinDriver<Input>, adc: &AdcDriver) {
    let vibration = adc.read(analog).unwrap(); // structural vibration
    let tilt = digital.is_high().unwrap(); // simulate tilt sensor
    writeln!(uart, "[CIVIL] Vibration:{} | Tilt:{}", vibration, tilt as u8).ok();
}

fn run_doctor_mode(uart: &mut UartDriver, analog: &mut AdcChannelDriver<'_, ADC1>, digital: &PinDriver<Input>, adc: &AdcDriver) {
    let heart_rate = adc.read(analog).unwrap(); // simulate pulse
    let stress = digital.is_high().unwrap(); // stress sensor
    writeln!(uart, "[DOCTOR] Pulse:{} | Stress:{}", heart_rate, stress as u8).ok();
}

fn run_educator_mode(uart: &mut UartDriver, analog: &mut AdcChannelDriver<'_, ADC1>, digital: &PinDriver<Input>, adc: &AdcDriver) {
    let light = adc.read(analog).unwrap(); // classroom light
    let presence = digital.is_high().unwrap(); // student presence
    writeln!(uart, "[EDUCATOR] Light:{} | Presence:{}", light, presence as u8).ok();
}

fn run_farmer_mode(uart: &mut UartDriver, analog: &mut AdcChannelDriver<'_, ADC1>, digital: &PinDriver<Input>, adc: &AdcDriver) {
    let soil = adc.read(analog).unwrap(); // soil moisture
    let rain = digital.is_high().unwrap(); // rain detection
    writeln!(uart, "[FARMER] Soil:{} | Rain:{}", soil, rain as u8).ok();
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::Car => "Car",
        Mode::Civil => "Civil Engineer",
        Mode::Doctor => "Doctor",
        Mode::Educator => "Educator",
        Mode::Farmer => "Farmer",
    }
}
