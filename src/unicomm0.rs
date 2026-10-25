#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x08e8],
    ipmode: Ipmode,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1100 - Mode Selection Register"]
    #[inline(always)]
    pub const fn ipmode(&self) -> &Ipmode {
        &self.ipmode
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "IPMODE (rw) register accessor: Mode Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipmode`] module"]
#[doc(alias = "IPMODE")]
pub type Ipmode = crate::Reg<ipmode::IpmodeSpec>;
#[doc = "Mode Selection Register"]
pub mod ipmode;
