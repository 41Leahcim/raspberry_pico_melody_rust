#![no_std]
#![no_main]

use embedded_hal::PwmPin;
use panic_halt as _;
use rp_pico::{
    hal::{self, clocks::ClockSource},
    pac,
};

#[rp_pico::entry]
fn main() -> ! {
    // Take access of the peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Create a struct for controlling the watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Initialize the clocks
    let clock = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Create a struct for controlling the SIO
    let sio = hal::Sio::new(pac.SIO);

    // Create a struct for controlling the pins
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Create a struct for using delay
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clock.system_clock.get_freq().to_Hz());

    // Create a struct for controlling the pwm slices
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Take a reference to the pwm0 slice, make the pwm phase correct, dividing it by 4, and enable it
    let pwm = &mut pwm_slices.pwm0;
    pwm.set_ph_correct();
    pwm.set_div_int(4);
    pwm.enable();

    // Make channel_a of pwm0 output to gpio0
    pwm.channel_a.output_to(pins.gpio0);

    loop {
        // Test some different frequencies
        for i in 12..16 {
            // Set the frequency
            // frequency = pwm clock (system_clock, 12MHz) / divider (4) / top (1 << i)
            pwm.set_top(1 << i);

            // Set the duty cycle to half the top
            pwm.channel_a.set_duty(pwm.get_top() / 2);

            // Wait for half a second
            delay.delay_ms(500);
        }
    }
}
