#![no_std]
#![no_main]

use core::u16;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::{bind_interrupts, peripherals, twim};
use embassy_time::{Delay, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("running!");

    let sda = p.P1_01;
    let scl = p.P1_02;
    let nint = p.P1_03;
    let nlp = Output::new(p.P1_04, Level::Low, OutputDrive::Disconnect0Standard1);

    let mut config = twim::Config::default();
    // config.sda_pullup = true;
    // config.scl_pullup = true;
    let twim = twim::Twim::new(p.TWISPI0, Irqs, sda, scl, config);

    let mut bq = bq2515x::hl::Bq2515xLowPower::new(twim, nlp, Delay);

    {
        let mut bq = bq.activate().await;
        let bq = bq.ll();

        let id = bq.device_id().read_async().await;
        info!("{:?}", defmt::Debug2Format(&id));

        let stat = bq.stat().read_async().await;
        info!("{:?}", defmt::Debug2Format(&stat));

        Timer::after_millis(1).await;

        let mask = bq.mask().read_async().await;
        info!("{:?}", defmt::Debug2Format(&mask));

        Timer::after_millis(1).await;

        let vbat_ctrl = bq.vbat_ctrl().read_async().await;
        info!("{:?}", defmt::Debug2Format(&vbat_ctrl));

        bq.adc_read_en()
            .write_async(|w| w.value(0).vin(true).vbat(true))
            .await
            .unwrap();
    }

    loop {
        {
            let mut bq = bq.activate().await;
            let bq = bq.ll();

            // purge
            let flag = bq.flag().read_async().await;
            info!("{:?}", defmt::Debug2Format(&flag));

            bq.adcctrl()
                .modify_async(|w| w.adc_conv_start(true))
                .await
                .unwrap();

            loop {
                let flag = bq.flag().read_async().await.unwrap();
                if flag.adc_ready() {
                    break;
                }
                Timer::after_millis(1).await;
            }

            let adc_data = bq.adc_data().read_async().await.unwrap();
            info!("{:?}", defmt::Debug2Format(&adc_data));
            info!(
                "vin: {} vbat: {}",
                to_mv(adc_data.vin(), 6),
                to_mv(adc_data.vbat(), 6)
            );
        }

        Timer::after_millis(950).await;
    }
}

fn to_mv(x: u16, max: u16) -> u16 {
    (x as u32 * max as u32 * 1000 / u16::MAX as u32) as u16
}
