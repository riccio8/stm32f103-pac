# stm32f103-pac

A low-level peripheral access crate for STM32F103 microcontrollers, generated using `svd2rust`. This crate gives you direct access to all registers of the MCU, allowing fine-grained control over peripherals without relying on a higher-level HAL.

## Features

* Access to all STM32F103 peripherals via safe Rust abstractions over raw registers.
* Supports direct manipulation of GPIO, timers, ADC, USART, I2C, SPI, and more.
* Compatible with `embedded-hal` traits when wrapped in your own abstractions.
* No dependencies on external HAL crates — minimal and lightweight.

## Example: Blinking an LED on PA5

```rust
use stm32f103_pac::Peripherals;
use embedded_hal::digital::v2::OutputPin;

struct PinPA5 {
    gpioa: stm32f103_pac::GPIOA,
}

impl PinPA5 {
    pub fn new(gpioa: stm32f103_pac::GPIOA) -> Self {
        gpioa.crl.modify(|_, w| unsafe {
            w.mode5().bits(0b11) // output 50 MHz
             .cnf5().bits(0b00)  // push-pull
        });
        Self { gpioa }
    }
}

impl OutputPin for PinPA5 {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpioa.bsrr.write(|w| w.bs5().set_bit());
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpioa.bsrr.write(|w| w.br5().set_bit());
        Ok(())
    }
}

fn main() {
    let dp = Peripherals::take().unwrap();
    let mut led = PinPA5::new(dp.GPIOA);

    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(8_000_000);
        led.set_low().unwrap();
        cortex_m::asm::delay(8_000_000);
    }
}
```

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
stm32f103-pac = { path = "../stm32f103-pac" }
embedded-hal = "0.2"
```

This crate is designed for **bare-metal development** on STM32F103 MCUs

## Documentation

Run:

```bash
cargo doc --open
```

for the full API documentation generated from the PAC.

## License

See [Licence](License.md)


Se vuoi, posso scrivere **una versione ancora più colloquiale e “friendly”**, che spieghi la filosofia della crate e come usarla senza sembrare un documento tecnico pesante. Vuoi che faccia quella versione?
