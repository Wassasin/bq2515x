Tested with the devkits or the NRF52840 and BQ25155. Using dupont cables to connect:

* P1.01: SDA
* P1.02: SCL
* P1.04: /LP
* VDD: 3p3
* GND

The configuration for the BQ25155 devkit jumpers for this example is as follows:

* VIO: 3p3
* LED: disconnected
* TPS: disconnected
* VINLS: PMID
* LP: disconnected
* CE: disconnected
* Vpullup: enabled

VBAT was powered using an Otii Arc to simulate a battery. VIN can be powered by USB or a variable power supply.

## How to run
```bash
cargo run --release --bin adc
```