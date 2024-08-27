Tested with the devkits of the NRF52840 and BQ25155. Using dupont cables to connect:

* P1.01: SDA
* P1.02: SCL
* P1.04: /LP
* **nRF** VDD (P20 on NRF DK): 3p3
* GND

The configuration for the BQ25155 devkit jumpers for these examples is as follows:

* VIO: 3p3
* LED: disconnected
* TPS: disconnected
* VINLS: PMID
* LP: disconnected
* CE: disconnected
* Vpullup: enabled

VBAT was powered using an Otii Arc to simulate a battery. VIN can be powered by USB or a variable power supply.

The NRF DK can be configured to select 3V3, 1V8, or something else to power VIO. By default the DK shorts VDD (3V) to nRF VDD. To allow REG0 to select a voltage switch over to USB Power (SW9) and remove the VDD jumper (P22). Depending on your DK revision you may have to cut a trace.

## How to run
```bash
cargo run --release --bin adc
```