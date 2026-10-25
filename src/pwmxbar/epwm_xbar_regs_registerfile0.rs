#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "EPWM_XBAR_REGS_REGISTERFILE0\\[%s\\]"]
#[doc(alias = "EPWM_XBAR_REGS_REGISTERFILE0")]
pub struct EpwmXbarRegsRegisterfile0 {
    pwmxbarg0sel: Pwmxbarg0sel,
    pwmxbarg1sel: Pwmxbarg1sel,
}
impl EpwmXbarRegsRegisterfile0 {
    #[doc = "0x00 - PWMXBAR G0 Input Select"]
    #[inline(always)]
    pub const fn pwmxbarg0sel(&self) -> &Pwmxbarg0sel {
        &self.pwmxbarg0sel
    }
    #[doc = "0x04 - PWMXBAR G1 Input Select"]
    #[inline(always)]
    pub const fn pwmxbarg1sel(&self) -> &Pwmxbarg1sel {
        &self.pwmxbarg1sel
    }
}
#[doc = "PWMXBARG0SEL (rw) register accessor: PWMXBAR G0 Input Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmxbarg0sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmxbarg0sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmxbarg0sel`] module"]
#[doc(alias = "PWMXBARG0SEL")]
pub type Pwmxbarg0sel = crate::Reg<pwmxbarg0sel::Pwmxbarg0selSpec>;
#[doc = "PWMXBAR G0 Input Select"]
pub mod pwmxbarg0sel;
#[doc = "PWMXBARG1SEL (rw) register accessor: PWMXBAR G1 Input Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmxbarg1sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmxbarg1sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmxbarg1sel`] module"]
#[doc(alias = "PWMXBARG1SEL")]
pub type Pwmxbarg1sel = crate::Reg<pwmxbarg1sel::Pwmxbarg1selSpec>;
#[doc = "PWMXBAR G1 Input Select"]
pub mod pwmxbarg1sel;
