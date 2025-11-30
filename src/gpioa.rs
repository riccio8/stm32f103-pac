#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crl: Crl,
    crh: Crh,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    brr: Brr,
    lckr: Lckr,
}
impl RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crl(&self) -> &Crl {
        &self.crl
    }
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crh(&self) -> &Crh {
        &self.crh
    }
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x18 - Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
}
#[doc = "CRL (rw) register accessor: Port configuration register low (GPIOn_CRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`] module"]
#[doc(alias = "CRL")]
pub type Crl = crate::Reg<crl::CrlSpec>;
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "CRH (rw) register accessor: Port configuration register high (GPIOn_CRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`] module"]
#[doc(alias = "CRH")]
pub type Crh = crate::Reg<crh::CrhSpec>;
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "IDR (r) register accessor: Port input data register (GPIOn_IDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "ODR (rw) register accessor: Port output data register (GPIOn_ODR)\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "BSRR (w) register accessor: Port bit set/reset register (GPIOn_BSRR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: Port bit reset register (GPIOn_BRR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "Port configuration lock register"]
pub mod lckr;

use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, OutputPin};

/// Represents a general-purpose GPIO pin configured as push-pull output at 50 MHz.
///
/// # Example
///
/// ```no_run
/// use stm32f103_pac::{Peripherals, Pin, GPIOA};
///
/// let dp = Peripherals::take().unwrap();
///
/// // Create PA5 as an output pin
/// let mut pa5 = Pin::new(dp.GPIOA, 5);
///
/// // Set the pin high
/// pa5.set_high().unwrap();
///
/// // Set the pin low
/// pa5.set_low().unwrap();
/// ```
///
/// # Notes
///
/// - Internally uses the `BSRR` register for atomic set/reset operations,
///   avoiding read-modify-write.  
/// - The error type is `Infallible` since there are no real error conditions.
/// - `pin_number` must be between 0 and 15; otherwise, `new` will panic.
/// - Compatible with **embedded-hal 1.0**, so it can be used in generic code
///   requiring `embedded_hal::digital::OutputPin`.
pub struct Pin<PORT> {
    port: PORT,
    pin_number: u8,
}

impl<PORT> ErrorType for Pin<PORT> {
    type Error = Infallible;
}

impl<PORT> Pin<PORT>
where
    PORT: core::ops::Deref<Target = crate::gpioa::RegisterBlock>,
{
    /// Create new instance of a general-purpose GPIO
    ///
    /// Configures as:
    /// - Push-pull output (CNF = 00)
    /// - 50 MHz output (MODE = 11)
    pub fn new(port: PORT, pin_number: u8) -> Self {
        assert!(pin_number < 16, "Pin number must be 0..15");

        let shift = if pin_number < 8 {
            (pin_number as u32) * 4
        } else {
            ((pin_number - 8) as u32) * 4
        };

        if pin_number < 8 {
            port.crl.modify(|r, w| unsafe {
                let mut bits = r.bits();
                bits &= !(0b1111 << shift);
                bits |= 0b0011 << shift;
                w.bits(bits)
            });
        } else {
            port.crh.modify(|r, w| unsafe {
                let mut bits = r.bits();
                bits &= !(0b1111 << shift);
                bits |= 0b0011 << shift;
                w.bits(bits)
            });
        }

        Self { port, pin_number }
    }
    

    /// Sets the GPIO pin high (logic level 1)
    ///
    /// # Example
    ///
    /// ```
    /// # let mut pin: Pin<GPIOA> = todo!();
    /// pin.set_high();
    /// ```
    pub fn set_high_direct(&self) {
        self.port
            .bsrr
            .write(|w| unsafe { w.bits(1 << self.pin_number) });
    }

    /// Sets the GPIO pin low (logic level 0)
    ///
    /// # Example
    ///
    /// ```
    /// # let mut pin: Pin<GPIOA> = todo!();
    /// pin.set_low();
    /// ```
    pub fn set_low_direct(&self) {
        self.port
            .bsrr
            .write(|w| unsafe { w.bits(1 << (self.pin_number + 16)) });
    }
}

impl<PORT> OutputPin for Pin<PORT>
where
    PORT: core::ops::Deref<Target = crate::gpioa::RegisterBlock>,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high_direct();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low_direct();
        Ok(())
    }
}
