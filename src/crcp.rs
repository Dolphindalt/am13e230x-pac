#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x07ec],
    clksel: Clksel,
    _reserved2: [u8; 0xf4],
    desc: Desc,
    crcctrl: Crcctrl,
    crcseed: Crcseed,
    crcin: Crcin,
    crcout: Crcout,
    crcpoly: Crcpoly,
    _reserved8: [u8; 0x06ec],
    crcin_idx: [CrcinIdx; 512],
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1004 - Clock Select"]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - CRC Control Register"]
    #[inline(always)]
    pub const fn crcctrl(&self) -> &Crcctrl {
        &self.crcctrl
    }
    #[doc = "0x1104 - CRC Seed Register"]
    #[inline(always)]
    pub const fn crcseed(&self) -> &Crcseed {
        &self.crcseed
    }
    #[doc = "0x1108 - CRC Input Data Register"]
    #[inline(always)]
    pub const fn crcin(&self) -> &Crcin {
        &self.crcin
    }
    #[doc = "0x110c - CRC Output Result Register"]
    #[inline(always)]
    pub const fn crcout(&self) -> &Crcout {
        &self.crcout
    }
    #[doc = "0x1110 - CRC Polynomial configuration register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub const fn crcin_idx(&self, n: usize) -> &CrcinIdx {
        &self.crcin_idx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub fn crcin_idx_iter(&self) -> impl Iterator<Item = &CrcinIdx> {
        self.crcin_idx.iter()
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "CLKSEL (r) register accessor: Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`] module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Clock Select"]
pub mod clksel;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`] module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "CRCCTRL (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`] module"]
#[doc(alias = "CRCCTRL")]
pub type Crcctrl = crate::Reg<crcctrl::CrcctrlSpec>;
#[doc = "CRC Control Register"]
pub mod crcctrl;
#[doc = "CRCSEED (w) register accessor: CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcseed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcseed`] module"]
#[doc(alias = "CRCSEED")]
pub type Crcseed = crate::Reg<crcseed::CrcseedSpec>;
#[doc = "CRC Seed Register"]
pub mod crcseed;
#[doc = "CRCIN (w) register accessor: CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin`] module"]
#[doc(alias = "CRCIN")]
pub type Crcin = crate::Reg<crcin::CrcinSpec>;
#[doc = "CRC Input Data Register"]
pub mod crcin;
#[doc = "CRCOUT (r) register accessor: CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcout`] module"]
#[doc(alias = "CRCOUT")]
pub type Crcout = crate::Reg<crcout::CrcoutSpec>;
#[doc = "CRC Output Result Register"]
pub mod crcout;
#[doc = "CRCPOLY (rw) register accessor: CRC Polynomial configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`] module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC Polynomial configuration register"]
pub mod crcpoly;
#[doc = "CRCIN_IDX (w) register accessor: CRC Input Data Array Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin_idx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin_idx`] module"]
#[doc(alias = "CRCIN_IDX")]
pub type CrcinIdx = crate::Reg<crcin_idx::CrcinIdxSpec>;
#[doc = "CRC Input Data Array Register"]
pub mod crcin_idx;
