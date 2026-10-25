#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "MSP"]
#[doc(alias = "MSP")]
pub struct Msp {
    _reserved0: [u8; 0x20],
    cpu_int: CpuInt,
    _reserved1: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved2: [u8; 0x18],
    desc: Desc,
    clken: Clken,
    clkdiv: Clkdiv,
    clkctl: Clkctl,
    clksts: Clksts,
}
impl Msp {
    #[doc = "0x20..0x4c - CPU_INT"]
    #[inline(always)]
    pub const fn cpu_int(&self) -> &CpuInt {
        &self.cpu_int
    }
    #[doc = "0xe0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0xfc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x100 - MCAN module clock enable"]
    #[inline(always)]
    pub const fn clken(&self) -> &Clken {
        &self.clken
    }
    #[doc = "0x104 - Clock divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x108 - MCAN-SS clock stop control register"]
    #[inline(always)]
    pub const fn clkctl(&self) -> &Clkctl {
        &self.clkctl
    }
    #[doc = "0x10c - MCANSS clock stop status register"]
    #[inline(always)]
    pub const fn clksts(&self) -> &Clksts {
        &self.clksts
    }
}
#[doc = "CPU_INT"]
pub use self::cpu_int::CpuInt;
#[doc = r"Cluster"]
#[doc = "CPU_INT"]
pub mod cpu_int;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`] module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`] module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "CLKEN (rw) register accessor: MCAN module clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken`] module"]
#[doc(alias = "CLKEN")]
pub type Clken = crate::Reg<clken::ClkenSpec>;
#[doc = "MCAN module clock enable"]
pub mod clken;
#[doc = "CLKDIV (rw) register accessor: Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock divider"]
pub mod clkdiv;
#[doc = "CLKCTL (rw) register accessor: MCAN-SS clock stop control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctl`] module"]
#[doc(alias = "CLKCTL")]
pub type Clkctl = crate::Reg<clkctl::ClkctlSpec>;
#[doc = "MCAN-SS clock stop control register"]
pub mod clkctl;
#[doc = "CLKSTS (r) register accessor: MCANSS clock stop status register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksts`] module"]
#[doc(alias = "CLKSTS")]
pub type Clksts = crate::Reg<clksts::ClkstsSpec>;
#[doc = "MCANSS clock stop status register"]
pub mod clksts;
