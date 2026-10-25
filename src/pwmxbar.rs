#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    pwmxbaroutinvert: Pwmxbaroutinvert,
    _reserved1: [u8; 0x4c],
    pwmxbarlock: Pwmxbarlock,
    _reserved2: [u8; 0x7c],
    epwm_xbar_regs_registerfile0: [EpwmXbarRegsRegisterfile0; 8],
}
impl RegisterBlock {
    #[doc = "0x30 - Output Signal Invert Select"]
    #[inline(always)]
    pub const fn pwmxbaroutinvert(&self) -> &Pwmxbaroutinvert {
        &self.pwmxbaroutinvert
    }
    #[doc = "0x80 - Configuration Lock register"]
    #[inline(always)]
    pub const fn pwmxbarlock(&self) -> &Pwmxbarlock {
        &self.pwmxbarlock
    }
    #[doc = "0x100..0x140 - EPWM_XBAR_REGS_REGISTERFILE0\\[%s\\]"]
    #[inline(always)]
    pub const fn epwm_xbar_regs_registerfile0(&self, n: usize) -> &EpwmXbarRegsRegisterfile0 {
        &self.epwm_xbar_regs_registerfile0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - EPWM_XBAR_REGS_REGISTERFILE0\\[%s\\]"]
    #[inline(always)]
    pub fn epwm_xbar_regs_registerfile0_iter(
        &self,
    ) -> impl Iterator<Item = &EpwmXbarRegsRegisterfile0> {
        self.epwm_xbar_regs_registerfile0.iter()
    }
}
#[doc = "PWMXBAROUTINVERT (rw) register accessor: Output Signal Invert Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmxbaroutinvert::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmxbaroutinvert::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmxbaroutinvert`] module"]
#[doc(alias = "PWMXBAROUTINVERT")]
pub type Pwmxbaroutinvert = crate::Reg<pwmxbaroutinvert::PwmxbaroutinvertSpec>;
#[doc = "Output Signal Invert Select"]
pub mod pwmxbaroutinvert;
#[doc = "PWMXBARLOCK (rw) register accessor: Configuration Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmxbarlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmxbarlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmxbarlock`] module"]
#[doc(alias = "PWMXBARLOCK")]
pub type Pwmxbarlock = crate::Reg<pwmxbarlock::PwmxbarlockSpec>;
#[doc = "Configuration Lock register"]
pub mod pwmxbarlock;
#[doc = "EPWM_XBAR_REGS_REGISTERFILE0\\[%s\\]"]
pub use self::epwm_xbar_regs_registerfile0::EpwmXbarRegsRegisterfile0;
#[doc = r"Cluster"]
#[doc = "EPWM_XBAR_REGS_REGISTERFILE0\\[%s\\]"]
pub mod epwm_xbar_regs_registerfile0;
