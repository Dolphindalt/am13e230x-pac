#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    outputxbarflaginvert: Outputxbarflaginvert,
    _reserved1: [u8; 0x0c],
    outputxbaroutlatch: Outputxbaroutlatch,
    _reserved2: [u8; 0x08],
    outputxbaroutinvert: Outputxbaroutinvert,
    _reserved3: [u8; 0x4c],
    outputxbarlock: Outputxbarlock,
    _reserved4: [u8; 0x7c],
    outputxbar_regs_registerfile0: [OutputxbarRegsRegisterfile0; 12],
}
impl RegisterBlock {
    #[doc = "0x14 - Output Signal inversion register"]
    #[inline(always)]
    pub const fn outputxbarflaginvert(&self) -> &Outputxbarflaginvert {
        &self.outputxbarflaginvert
    }
    #[doc = "0x24 - Output Signal Select Latch"]
    #[inline(always)]
    pub const fn outputxbaroutlatch(&self) -> &Outputxbaroutlatch {
        &self.outputxbaroutlatch
    }
    #[doc = "0x30 - Output Signal Invert Select"]
    #[inline(always)]
    pub const fn outputxbaroutinvert(&self) -> &Outputxbaroutinvert {
        &self.outputxbaroutinvert
    }
    #[doc = "0x80 - Configuration Lock register"]
    #[inline(always)]
    pub const fn outputxbarlock(&self) -> &Outputxbarlock {
        &self.outputxbarlock
    }
    #[doc = "0x100..0x160 - OUTPUTXBAR_REGS_REGISTERFILE0\\[%s\\]"]
    #[inline(always)]
    pub const fn outputxbar_regs_registerfile0(&self, n: usize) -> &OutputxbarRegsRegisterfile0 {
        &self.outputxbar_regs_registerfile0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x160 - OUTPUTXBAR_REGS_REGISTERFILE0\\[%s\\]"]
    #[inline(always)]
    pub fn outputxbar_regs_registerfile0_iter(
        &self,
    ) -> impl Iterator<Item = &OutputxbarRegsRegisterfile0> {
        self.outputxbar_regs_registerfile0.iter()
    }
}
#[doc = "OUTPUTXBARFLAGINVERT (rw) register accessor: Output Signal inversion register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflaginvert::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflaginvert::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarflaginvert`] module"]
#[doc(alias = "OUTPUTXBARFLAGINVERT")]
pub type Outputxbarflaginvert = crate::Reg<outputxbarflaginvert::OutputxbarflaginvertSpec>;
#[doc = "Output Signal inversion register"]
pub mod outputxbarflaginvert;
#[doc = "OUTPUTXBAROUTLATCH (rw) register accessor: Output Signal Select Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbaroutlatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbaroutlatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbaroutlatch`] module"]
#[doc(alias = "OUTPUTXBAROUTLATCH")]
pub type Outputxbaroutlatch = crate::Reg<outputxbaroutlatch::OutputxbaroutlatchSpec>;
#[doc = "Output Signal Select Latch"]
pub mod outputxbaroutlatch;
#[doc = "OUTPUTXBAROUTINVERT (rw) register accessor: Output Signal Invert Select\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbaroutinvert::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbaroutinvert::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbaroutinvert`] module"]
#[doc(alias = "OUTPUTXBAROUTINVERT")]
pub type Outputxbaroutinvert = crate::Reg<outputxbaroutinvert::OutputxbaroutinvertSpec>;
#[doc = "Output Signal Invert Select"]
pub mod outputxbaroutinvert;
#[doc = "OUTPUTXBARLOCK (rw) register accessor: Configuration Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarlock`] module"]
#[doc(alias = "OUTPUTXBARLOCK")]
pub type Outputxbarlock = crate::Reg<outputxbarlock::OutputxbarlockSpec>;
#[doc = "Configuration Lock register"]
pub mod outputxbarlock;
#[doc = "OUTPUTXBAR_REGS_REGISTERFILE0\\[%s\\]"]
pub use self::outputxbar_regs_registerfile0::OutputxbarRegsRegisterfile0;
#[doc = r"Cluster"]
#[doc = "OUTPUTXBAR_REGS_REGISTERFILE0\\[%s\\]"]
pub mod outputxbar_regs_registerfile0;
