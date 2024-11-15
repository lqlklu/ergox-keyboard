#![no_main]
#![no_std]

#[macro_use]
mod macros;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{AnyPin, Input, Output},
    peripherals::{UART0, USB},
    uart::{self, BufferedUart},
    usb::InterruptHandler,
};
use panic_probe as _;
use rmk::split::{peripheral::run_rmk_split_peripheral, SPLIT_MESSAGE_MAX_SIZE};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    UART0_IRQ => uart::BufferedInterruptHandler<UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Pin config
    let (input_pins, output_pins) =
        config_matrix_pins_rp!(peripherals: p,
            input: [PIN_2, PIN_3, PIN_6, PIN_15, PIN_14, PIN_11],
            output: [PIN_16 ,PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_22]);

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance = BufferedUart::new(
        p.UART0,
        Irqs,
        p.PIN_0,
        p.PIN_1,
        tx_buf,
        rx_buf,
        uart::Config::default(),
    );

    // Start serving
    run_rmk_split_peripheral::<Input<'_>, Output<'_>, _, 6, 7>(
        input_pins,
        output_pins,
        uart_instance,
    )
        .await;
}
