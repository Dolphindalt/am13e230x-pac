#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    xbarflg1: Xbarflg1,
    xbarflg2: Xbarflg2,
    _reserved2: [u8; 0x08],
    xbarclr1: Xbarclr1,
    xbarclr2: Xbarclr2,
}
impl RegisterBlock {
    #[doc = "0x00 - X-Bar Input Flag Register 1"]
    #[inline(always)]
    pub const fn xbarflg1(&self) -> &Xbarflg1 {
        &self.xbarflg1
    }
    #[doc = "0x04 - X-Bar Input Flag Register 2"]
    #[inline(always)]
    pub const fn xbarflg2(&self) -> &Xbarflg2 {
        &self.xbarflg2
    }
    #[doc = "0x10 - X-Bar Input Flag Clear Register 1"]
    #[inline(always)]
    pub const fn xbarclr1(&self) -> &Xbarclr1 {
        &self.xbarclr1
    }
    #[doc = "0x14 - X-Bar Input Flag Clear Register 2"]
    #[inline(always)]
    pub const fn xbarclr2(&self) -> &Xbarclr2 {
        &self.xbarclr2
    }
}
#[doc = "XBARFLG1 (rw) register accessor: X-Bar Input Flag Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarflg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarflg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xbarflg1`] module"]
#[doc(alias = "XBARFLG1")]
pub type Xbarflg1 = crate::Reg<xbarflg1::Xbarflg1Spec>;
#[doc = "X-Bar Input Flag Register 1"]
pub mod xbarflg1;
#[doc = "XBARFLG2 (rw) register accessor: X-Bar Input Flag Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarflg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarflg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xbarflg2`] module"]
#[doc(alias = "XBARFLG2")]
pub type Xbarflg2 = crate::Reg<xbarflg2::Xbarflg2Spec>;
#[doc = "X-Bar Input Flag Register 2"]
pub mod xbarflg2;
#[doc = "XBARCLR1 (rw) register accessor: X-Bar Input Flag Clear Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xbarclr1`] module"]
#[doc(alias = "XBARCLR1")]
pub type Xbarclr1 = crate::Reg<xbarclr1::Xbarclr1Spec>;
#[doc = "X-Bar Input Flag Clear Register 1"]
pub mod xbarclr1;
#[doc = "XBARCLR2 (rw) register accessor: X-Bar Input Flag Clear Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xbarclr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xbarclr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xbarclr2`] module"]
#[doc(alias = "XBARCLR2")]
pub type Xbarclr2 = crate::Reg<xbarclr2::Xbarclr2Spec>;
#[doc = "X-Bar Input Flag Clear Register 2"]
pub mod xbarclr2;
