// SPDX-FileCopyrightText: 2025 Funkeleinhorn <git@funkeleinhorn.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::{
    future::join,
    sync::atomic::{AtomicBool, Ordering},
};

use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

pub static GPIO_PAUSE_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_REC_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_STOP_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_RWD_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_REV_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_FWD_STATE: AtomicBool = AtomicBool::new(false);
pub static GPIO_FF_STATE: AtomicBool = AtomicBool::new(false);

async fn trigger_gpio(state: &AtomicBool, gpio: &mut Output<'static>) {
    if state.load(Ordering::Relaxed) {
        gpio.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        gpio.set_low();
        state.store(false, Ordering::Relaxed);
    } else {
        gpio.set_low();
    }
}

#[embassy_executor::task]
pub async fn gpio_task(
    mut pause_gpio: Output<'static>,
    mut rec_gpio: Output<'static>,
    mut stop_gpio: Output<'static>,
    mut rwd_gpio: Output<'static>,
    mut rev_gpio: Output<'static>,
    mut fwd_gpio: Output<'static>,
    mut ff_gpio: Output<'static>,
) {
    loop {
        let pause_fut = trigger_gpio(&GPIO_PAUSE_STATE, &mut pause_gpio);
        let rec_fut = trigger_gpio(&GPIO_REC_STATE, &mut rec_gpio);
        let stop_fut = trigger_gpio(&GPIO_STOP_STATE, &mut stop_gpio);
        let rwd_fut = trigger_gpio(&GPIO_RWD_STATE, &mut rwd_gpio);
        let rev_fut = trigger_gpio(&GPIO_REV_STATE, &mut rev_gpio);
        let fwd_fut = trigger_gpio(&GPIO_FWD_STATE, &mut fwd_gpio);
        let ff_fut = trigger_gpio(&GPIO_FF_STATE, &mut ff_gpio);
        join!(pause_fut, rec_fut, stop_fut, rwd_fut, rev_fut, fwd_fut, ff_fut).await;
        Timer::after(Duration::from_millis(50)).await;
    }
}
