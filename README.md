# BQ2515x
Rust no-std device driver for the [BQ2515x](https://www.ti.com/battery-management/charger-ics/products.html#273=Linear&1341=I2C&1153max=0.5%3B0.5&) family of Texas Instruments linear battery chargers.

* [BQ25150](https://www.ti.com/product/BQ25150) - *Untested*
* [BQ25155](https://www.ti.com/product/BQ25155) - Tested
* [BQ25157](https://www.ti.com/product/BQ25157) - *Untested*

Should also work, but only a subset of registers and features are supported by the devices:
* [BQ21061](https://www.ti.com/product/BQ21061) - *Possibly?*
* [BQ21062](https://www.ti.com/product/BQ21062) - *Possibly?*

## Support
* All low level registers
* Easy management of the Low Power (/LP) pin

## Not (yet) supported
* High level basic operation
* High level ADC comparators

## Example
See the `examples/` folder in this repository on how to use the library crate.