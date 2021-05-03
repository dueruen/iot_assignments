//#![deny(unsafe_code)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#[allow(unused_extern_crates)]

extern crate panic_itm; // panic handler
// #[allow(unused_imports)]
// use aux6::{entry, iprint, iprintln};
#[macro_use(block)]
extern crate nb;
use cortex_m_rt::entry;
use f3::hal::{prelude::*, serial::Serial, stm32f30x};
// use f3::hal::{
//     prelude::*,
//     stm32f30x::{self, USART1},
// };

#[entry]
fn main() -> ! {
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let serial = Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);
    let (mut tx, _) = serial.split();
    
    //block!(tx.write(34 as u8)).ok();
    block!(tx.write(b'X')).ok();

    loop {
        
    }
}