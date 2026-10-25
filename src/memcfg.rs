#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    test: Test,
    _reserved1: [u8; 0x0c],
    rom_ws_config: RomWsConfig,
    tmurom_test: TmuromTest,
}
impl RegisterBlock {
    #[doc = "0x1000 - RAM TEST Register"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x1010 - ROM wait state configuration"]
    #[inline(always)]
    pub const fn rom_ws_config(&self) -> &RomWsConfig {
        &self.rom_ws_config
    }
    #[doc = "0x1014 - ROM wait state configuration"]
    #[inline(always)]
    pub const fn tmurom_test(&self) -> &TmuromTest {
        &self.tmurom_test
    }
}
#[doc = "TEST (rw) register accessor: RAM TEST Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "RAM TEST Register"]
pub mod test;
#[doc = "ROM_WS_CONFIG (rw) register accessor: ROM wait state configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ws_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ws_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_ws_config`] module"]
#[doc(alias = "ROM_WS_CONFIG")]
pub type RomWsConfig = crate::Reg<rom_ws_config::RomWsConfigSpec>;
#[doc = "ROM wait state configuration"]
pub mod rom_ws_config;
#[doc = "TMUROM_TEST (rw) register accessor: ROM wait state configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmurom_test`] module"]
#[doc(alias = "TMUROM_TEST")]
pub type TmuromTest = crate::Reg<tmurom_test::TmuromTestSpec>;
#[doc = "ROM wait state configuration"]
pub mod tmurom_test;
