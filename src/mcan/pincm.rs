#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "PINCM"]
#[doc(alias = "PINCM")]
pub struct Pincm {
    canrx: Canrx,
    cantx: Cantx,
}
impl Pincm {
    #[doc = "0x00 - CAN RX IO"]
    #[inline(always)]
    pub const fn canrx(&self) -> &Canrx {
        &self.canrx
    }
    #[doc = "0x04 - CAN TX IO"]
    #[inline(always)]
    pub const fn cantx(&self) -> &Cantx {
        &self.cantx
    }
}
#[doc = "CANRX (rw) register accessor: CAN RX IO\n\nYou can [`read`](crate::Reg::read) this register and get [`canrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canrx`] module"]
#[doc(alias = "CANRX")]
pub type Canrx = crate::Reg<canrx::CanrxSpec>;
#[doc = "CAN RX IO"]
pub mod canrx;
#[doc = "CANTX (rw) register accessor: CAN TX IO\n\nYou can [`read`](crate::Reg::read) this register and get [`cantx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cantx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cantx`] module"]
#[doc(alias = "CANTX")]
pub type Cantx = crate::Reg<cantx::CantxSpec>;
#[doc = "CAN TX IO"]
pub mod cantx;
