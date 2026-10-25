#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "OUTPUTXBAR_REGS_REGISTERFILE0\\[%s\\]"]
#[doc(alias = "OUTPUTXBAR_REGS_REGISTERFILE0")]
pub struct OutputxbarRegsRegisterfile0 {
    outputxbarg0sel: Outputxbarg0sel,
    outputxbarg1sel: Outputxbarg1sel,
}
impl OutputxbarRegsRegisterfile0 {
    #[doc = "0x00 - OUTPUTXBAR G0 Input Select"]
    #[inline(always)]
    pub const fn outputxbarg0sel(&self) -> &Outputxbarg0sel {
        &self.outputxbarg0sel
    }
    #[doc = "0x04 - OUTPUTXBAR G1 Input Select"]
    #[inline(always)]
    pub const fn outputxbarg1sel(&self) -> &Outputxbarg1sel {
        &self.outputxbarg1sel
    }
}
#[doc = "OUTPUTXBARG0SEL (rw) register accessor: OUTPUTXBAR G0 Input Select\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarg0sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarg0sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarg0sel`] module"]
#[doc(alias = "OUTPUTXBARG0SEL")]
pub type Outputxbarg0sel = crate::Reg<outputxbarg0sel::Outputxbarg0selSpec>;
#[doc = "OUTPUTXBAR G0 Input Select"]
pub mod outputxbarg0sel;
#[doc = "OUTPUTXBARG1SEL (rw) register accessor: OUTPUTXBAR G1 Input Select\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarg1sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarg1sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarg1sel`] module"]
#[doc(alias = "OUTPUTXBARG1SEL")]
pub type Outputxbarg1sel = crate::Reg<outputxbarg1sel::Outputxbarg1selSpec>;
#[doc = "OUTPUTXBAR G1 Input Select"]
pub mod outputxbarg1sel;
