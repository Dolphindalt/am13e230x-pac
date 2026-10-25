#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pincm: [Pincm; 108],
}
impl RegisterBlock {
    #[doc = "0x00..0x1b0 - Pin Control Management Register in SECCFG region"]
    #[inline(always)]
    pub const fn pincm(&self, n: usize) -> &Pincm {
        &self.pincm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1b0 - Pin Control Management Register in SECCFG region"]
    #[inline(always)]
    pub fn pincm_iter(&self) -> impl Iterator<Item = &Pincm> {
        self.pincm.iter()
    }
}
#[doc = "PINCM (rw) register accessor: Pin Control Management Register in SECCFG region\n\nYou can [`read`](crate::Reg::read) this register and get [`pincm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincm`] module"]
#[doc(alias = "PINCM")]
pub type Pincm = crate::Reg<pincm::PincmSpec>;
#[doc = "Pin Control Management Register in SECCFG region"]
pub mod pincm;
