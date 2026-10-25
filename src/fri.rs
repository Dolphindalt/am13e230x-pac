#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    frdcntl: Frdcntl,
    _reserved1: [u8; 0x08],
    frd_intf_ctrl: FrdIntfCtrl,
    dtb_muxsel: DtbMuxsel,
    _reserved3: [u8; 0xec],
    ecc_enable: EccEnable,
    fecc_ctrl: FeccCtrl,
}
impl RegisterBlock {
    #[doc = "0x1000 - Flash Read Control Register"]
    #[inline(always)]
    pub const fn frdcntl(&self) -> &Frdcntl {
        &self.frdcntl
    }
    #[doc = "0x100c - Flash Read Interface Control Register"]
    #[inline(always)]
    pub const fn frd_intf_ctrl(&self) -> &FrdIntfCtrl {
        &self.frd_intf_ctrl
    }
    #[doc = "0x1010 - Flash Read Interface DTB Mux select"]
    #[inline(always)]
    pub const fn dtb_muxsel(&self) -> &DtbMuxsel {
        &self.dtb_muxsel
    }
    #[doc = "0x1100 - ECC Enable"]
    #[inline(always)]
    pub const fn ecc_enable(&self) -> &EccEnable {
        &self.ecc_enable
    }
    #[doc = "0x1104 - ECC Control"]
    #[inline(always)]
    pub const fn fecc_ctrl(&self) -> &FeccCtrl {
        &self.fecc_ctrl
    }
}
#[doc = "FRDCNTL (rw) register accessor: Flash Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frdcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frdcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frdcntl`] module"]
#[doc(alias = "FRDCNTL")]
pub type Frdcntl = crate::Reg<frdcntl::FrdcntlSpec>;
#[doc = "Flash Read Control Register"]
pub mod frdcntl;
#[doc = "FRD_INTF_CTRL (rw) register accessor: Flash Read Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frd_intf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frd_intf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frd_intf_ctrl`] module"]
#[doc(alias = "FRD_INTF_CTRL")]
pub type FrdIntfCtrl = crate::Reg<frd_intf_ctrl::FrdIntfCtrlSpec>;
#[doc = "Flash Read Interface Control Register"]
pub mod frd_intf_ctrl;
#[doc = "DTB_MUXSEL (rw) register accessor: Flash Read Interface DTB Mux select\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_muxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_muxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb_muxsel`] module"]
#[doc(alias = "DTB_MUXSEL")]
pub type DtbMuxsel = crate::Reg<dtb_muxsel::DtbMuxselSpec>;
#[doc = "Flash Read Interface DTB Mux select"]
pub mod dtb_muxsel;
#[doc = "ECC_ENABLE (rw) register accessor: ECC Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_enable`] module"]
#[doc(alias = "ECC_ENABLE")]
pub type EccEnable = crate::Reg<ecc_enable::EccEnableSpec>;
#[doc = "ECC Enable"]
pub mod ecc_enable;
#[doc = "FECC_CTRL (rw) register accessor: ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fecc_ctrl`] module"]
#[doc(alias = "FECC_CTRL")]
pub type FeccCtrl = crate::Reg<fecc_ctrl::FeccCtrlSpec>;
#[doc = "ECC Control"]
pub mod fecc_ctrl;
