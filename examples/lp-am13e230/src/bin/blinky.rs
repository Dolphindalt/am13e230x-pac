//! Register sequence from TI's `gpio_toggle_output` example and the `DL_GPIO_*`
//! driverlib functions it calls.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use am13e230x_pac::{gpio0, iomux, CorePeripherals, Peripherals};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;

/// MCLK after reset: SYSOSC at 32 MHz.
const MCLK_HZ: u32 = 32_000_000;
/// IOMUX slots of the red LED (PA5) and the green LED (PB0) (hw_pinmap.h: `IOMUX_PINCM_*`).
const PA5_PINCM: usize = 5;
const PB0_PINCM: usize = 32;
/// Pin function 1 is GPIO on every pin (dl_gpio.h).
const PF_GPIO: u8 = 1;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    // GPIO0 is port A and GPIO1 is port B.
    power_up(&p.gpio0);
    power_up(&p.gpio1);
    connect(&p.iomux, PA5_PINCM);
    connect(&p.iomux, PB0_PINCM);
    // The red LED is active low.
    p.gpio0.doutset31_0().write(|w| w.dio5().set_bit());
    p.gpio0.doeset31_0().write(|w| w.dio5().set_bit());
    p.gpio1.doeset31_0().write(|w| w.dio0().set_bit());

    let mut delay = Delay::new(cp.SYST, MCLK_HZ);
    loop {
        p.gpio1.douttgl31_0().write(|w| w.dio0().set_bit());
        delay.delay_ms(500);
    }
}

fn power_up(gpio: &gpio0::RegisterBlock) {
    gpio.gprcm().rstctl().write(|w| {
        w.key().unlock();
        w.resetstkyclr().set_bit();
        w.resetassert().set_bit()
    });
    gpio.gprcm()
        .pwren()
        .write(|w| w.key().unlock().enable().set_bit());
    // Power-up time: TI waits 16 cycles (POWER_STARTUP_DELAY).
    cortex_m::asm::delay(16);
}

fn connect(iomux: &iomux::RegisterBlock, pincm: usize) {
    iomux
        .pincm(pincm)
        .write(|w| w.pf().set(PF_GPIO).pc().connected());
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    // SAFETY: the handler never returns, so no other code uses the peripherals again.
    let p = unsafe { Peripherals::steal() };
    p.gpio0
        .gprcm()
        .pwren()
        .write(|w| w.key().unlock().enable().set_bit());
    cortex_m::asm::delay(16);
    connect(&p.iomux, PA5_PINCM);
    p.gpio0.doutclr31_0().write(|w| w.dio5().set_bit());
    p.gpio0.doeset31_0().write(|w| w.dio5().set_bit());
    loop {
        cortex_m::asm::nop();
    }
}
