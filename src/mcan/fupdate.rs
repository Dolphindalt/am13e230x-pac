#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "FUPDATE"]
#[doc(alias = "FUPDATE")]
pub struct Fupdate {
    canrx: Canrx,
    cantx: Cantx,
}
impl Fupdate {
    #[doc = "0x00 - FUPDATE version of CANRX"]
    #[inline(always)]
    pub const fn canrx(&self) -> &Canrx {
        &self.canrx
    }
    #[doc = "0x04 - FUPDATE version of CANTX"]
    #[inline(always)]
    pub const fn cantx(&self) -> &Cantx {
        &self.cantx
    }
}
#[doc = "CANRX (rw) register accessor: FUPDATE version of CANRX\n\nYou can [`read`](crate::Reg::read) this register and get [`canrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canrx`] module"]
#[doc(alias = "CANRX")]
pub type Canrx = crate::Reg<canrx::CanrxSpec>;
#[doc = "FUPDATE version of CANRX"]
pub mod canrx;
#[doc = "CANTX (rw) register accessor: FUPDATE version of CANTX\n\nYou can [`read`](crate::Reg::read) this register and get [`cantx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cantx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cantx`] module"]
#[doc(alias = "CANTX")]
pub type Cantx = crate::Reg<cantx::CantxSpec>;
#[doc = "FUPDATE version of CANTX"]
pub mod cantx;
