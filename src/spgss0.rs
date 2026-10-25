#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0120],
    lpbk0: Lpbk0,
    _reserved1: [u8; 0x9c],
    pair0: Pair0,
    _reserved2: [u8; 0x0a3c],
    test0: Test0,
}
impl RegisterBlock {
    #[doc = "0x120 - Loopback between SPG instances User has to ensure that the SPGs looped back have the same underlying IP instantiated; i.e. both UART, both SPI, both I2C, etc.,"]
    #[inline(always)]
    pub const fn lpbk0(&self) -> &Lpbk0 {
        &self.lpbk0
    }
    #[doc = "0x1c0 - Pairing of I2C for MULTI-MASTER/SMBUS applications"]
    #[inline(always)]
    pub const fn pair0(&self) -> &Pair0 {
        &self.pair0
    }
    #[doc = "0xc00 - Test 0 register"]
    #[inline(always)]
    pub const fn test0(&self) -> &Test0 {
        &self.test0
    }
}
#[doc = "LPBK0 (rw) register accessor: Loopback between SPG instances User has to ensure that the SPGs looped back have the same underlying IP instantiated; i.e. both UART, both SPI, both I2C, etc.,\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbk0`] module"]
#[doc(alias = "LPBK0")]
pub type Lpbk0 = crate::Reg<lpbk0::Lpbk0Spec>;
#[doc = "Loopback between SPG instances User has to ensure that the SPGs looped back have the same underlying IP instantiated; i.e. both UART, both SPI, both I2C, etc.,"]
pub mod lpbk0;
#[doc = "PAIR0 (rw) register accessor: Pairing of I2C for MULTI-MASTER/SMBUS applications\n\nYou can [`read`](crate::Reg::read) this register and get [`pair0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pair0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pair0`] module"]
#[doc(alias = "PAIR0")]
pub type Pair0 = crate::Reg<pair0::Pair0Spec>;
#[doc = "Pairing of I2C for MULTI-MASTER/SMBUS applications"]
pub mod pair0;
#[doc = "TEST0 (rw) register accessor: Test 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`test0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test0`] module"]
#[doc(alias = "TEST0")]
pub type Test0 = crate::Reg<test0::Test0Spec>;
#[doc = "Test 0 register"]
pub mod test0;
