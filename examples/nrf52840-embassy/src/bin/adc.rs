#![no_std]
#![no_main]

use core::u16;

use bq2515x::prelude::*;
use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals,
    twim::{self, Frequency},
};
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
    let _nint = p.P1_03;
    let nlp = Output::new(p.P1_04, Level::Low, OutputDrive::Disconnect0Standard1);

    let mut config = twim::Config::default();
    config.frequency = Frequency::K400;
    // config.sda_pullup = true;
    // config.scl_pullup = true;
    let twim = twim::Twim::new(p.TWISPI0, Irqs, sda, scl, config);

    let mut bq = bq2515x::hl::Bq2515xLowPower::new(twim, nlp, Delay);

    {
        let mut bq = bq.activate().await;
        let ll = bq.ll();

        let id = ll.device_id().read_async().await.unwrap();
        info!("{:?}", defmt::Debug2Format(&id));

        let stat = ll.stat().read_async().await.unwrap();
        info!("{:?}", defmt::Debug2Format(&stat));

        Timer::after_millis(1).await;

        let mask = ll.mask().read_async().await.unwrap();
        info!("{:?}", defmt::Debug2Format(&mask));

        Timer::after_millis(1).await;

        let vbat_ctrl = ll.vbat_ctrl().read_async().await.unwrap();
        info!("{:?}", defmt::Debug2Format(&vbat_ctrl));

        ll.icctrl()
            .modify_async(|w| {
                w.pmid_mode(PmidMode::BatOrVin)
                    .pmid_reg_ctrl(PmidRegCtrl::PassThrough)
            })
            .await
            .unwrap();

        ll.buvlo()
            .modify_async(|w| w.buvlo_threshold(BuvloThreshold::Uvlo2V8))
            .await
            .unwrap();

        ll.vbat_ctrl()
            .write_async(|w| w.vbat_reg(Millivolts(4200).into()))
            .await
            .unwrap();

        ll.ilimctrl()
            .write_async(|w| w.ilim(CurrentLimit::_500mA))
            .await
            .unwrap();

        let range = IchargeRange::Step2MilliA5;

        ll.pchrgctrl()
            .modify_async(|w| w.icharge_range(range))
            .await
            .unwrap();

        ll.ichg_ctrl()
            .write_async(|w| w.ichg(FastChargeCurrent::from_milliampere(Milliampere(200), range)))
            .await
            .unwrap();

        ll.adc_read_en()
            .write_async(|w| w.value(0).vin(true).vbat(true))
            .await
            .unwrap();

        bq.ldo(bq2515x::hl::LdoConfig::Ldo(Millivolts(900).into()))
            .await
            .unwrap();
    }

    loop {
        {
            let mut bq = bq.activate().await;

            // Purge flags
            let _ = bq.ll().flag().read_async().await.unwrap();

            bq.adc_start_one_shot().await.unwrap();

            loop {
                let stat = bq.ll().stat().read_async().await.unwrap();
                let flag = bq.ll().flag().read_async().await.unwrap();
                if stat.vin_vgood() || flag.adc_ready() {
                    break;
                }
                Timer::after_millis(1).await;
            }

            let adc_data = bq.adc_fetch_latest().await.unwrap();
            info!("{:?}", adc_data);

            let vin: Millivolts = adc_data.vin.unwrap().into();
            let vbat: Millivolts = adc_data.vbat.unwrap().into();

            info!("vin: {} vbat: {}", vin, vbat);

            let stat = bq.ll().stat().read_async().await.unwrap();
            info!("{:?}", defmt::Debug2Format(&stat));
        }

        Timer::after_millis(950).await;
    }
}
