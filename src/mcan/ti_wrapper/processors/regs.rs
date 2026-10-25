#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "REGS"]
#[doc(alias = "REGS")]
pub struct Regs {
    pid: Pid,
    ti_wrapper_regs_ctrl: TiWrapperRegsCtrl,
    ti_wrapper_processors_regs_stat: TiWrapperProcessorsRegsStat,
    ics: Ics,
    irs: Irs,
    iecs: Iecs,
    ti_wrapper_processors_regs_ie: TiWrapperProcessorsRegsIe,
    ies: Ies,
    eoi: Eoi,
    ext_ts_prescaler: ExtTsPrescaler,
    ext_ts_unserviced_intr_cntr: ExtTsUnservicedIntrCntr,
}
impl Regs {
    #[doc = "0x00 - MCAN Subsystem Revision Register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - MCAN Subsystem Control Register"]
    #[inline(always)]
    pub const fn ti_wrapper_regs_ctrl(&self) -> &TiWrapperRegsCtrl {
        &self.ti_wrapper_regs_ctrl
    }
    #[doc = "0x08 - MCAN Subsystem Status Register"]
    #[inline(always)]
    pub const fn ti_wrapper_processors_regs_stat(&self) -> &TiWrapperProcessorsRegsStat {
        &self.ti_wrapper_processors_regs_stat
    }
    #[doc = "0x0c - MCAN Subsystem Interrupt Clear Shadow Register"]
    #[inline(always)]
    pub const fn ics(&self) -> &Ics {
        &self.ics
    }
    #[doc = "0x10 - MCAN Subsystem Interrupt Raw Satus Register"]
    #[inline(always)]
    pub const fn irs(&self) -> &Irs {
        &self.irs
    }
    #[doc = "0x14 - MCAN Subsystem Interrupt Enable Clear Shadow Register"]
    #[inline(always)]
    pub const fn iecs(&self) -> &Iecs {
        &self.iecs
    }
    #[doc = "0x18 - MCAN Subsystem Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ti_wrapper_processors_regs_ie(&self) -> &TiWrapperProcessorsRegsIe {
        &self.ti_wrapper_processors_regs_ie
    }
    #[doc = "0x1c - MCAN Subsystem Interrupt Enable Status"]
    #[inline(always)]
    pub const fn ies(&self) -> &Ies {
        &self.ies
    }
    #[doc = "0x20 - MCAN Subsystem End of Interrupt"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x24 - MCAN Subsystem External Timestamp Prescaler 0"]
    #[inline(always)]
    pub const fn ext_ts_prescaler(&self) -> &ExtTsPrescaler {
        &self.ext_ts_prescaler
    }
    #[doc = "0x28 - MCAN Subsystem External Timestamp Unserviced Interrupts Counter"]
    #[inline(always)]
    pub const fn ext_ts_unserviced_intr_cntr(&self) -> &ExtTsUnservicedIntrCntr {
        &self.ext_ts_unserviced_intr_cntr
    }
}
#[doc = "PID (r) register accessor: MCAN Subsystem Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`] module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "MCAN Subsystem Revision Register"]
pub mod pid;
#[doc = "TI_WRAPPER_REGS_CTRL (rw) register accessor: MCAN Subsystem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_regs_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_regs_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti_wrapper_regs_ctrl`] module"]
#[doc(alias = "TI_WRAPPER_REGS_CTRL")]
pub type TiWrapperRegsCtrl = crate::Reg<ti_wrapper_regs_ctrl::TiWrapperRegsCtrlSpec>;
#[doc = "MCAN Subsystem Control Register"]
pub mod ti_wrapper_regs_ctrl;
#[doc = "TI_WRAPPER_PROCESSORS_REGS_STAT (r) register accessor: MCAN Subsystem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_regs_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti_wrapper_processors_regs_stat`] module"]
#[doc(alias = "TI_WRAPPER_PROCESSORS_REGS_STAT")]
pub type TiWrapperProcessorsRegsStat =
    crate::Reg<ti_wrapper_processors_regs_stat::TiWrapperProcessorsRegsStatSpec>;
#[doc = "MCAN Subsystem Status Register"]
pub mod ti_wrapper_processors_regs_stat;
#[doc = "ICS (rw) register accessor: MCAN Subsystem Interrupt Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ics::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ics::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ics`] module"]
#[doc(alias = "ICS")]
pub type Ics = crate::Reg<ics::IcsSpec>;
#[doc = "MCAN Subsystem Interrupt Clear Shadow Register"]
pub mod ics;
#[doc = "IRS (rw) register accessor: MCAN Subsystem Interrupt Raw Satus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irs`] module"]
#[doc(alias = "IRS")]
pub type Irs = crate::Reg<irs::IrsSpec>;
#[doc = "MCAN Subsystem Interrupt Raw Satus Register"]
pub mod irs;
#[doc = "IECS (rw) register accessor: MCAN Subsystem Interrupt Enable Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iecs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iecs`] module"]
#[doc(alias = "IECS")]
pub type Iecs = crate::Reg<iecs::IecsSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Clear Shadow Register"]
pub mod iecs;
#[doc = "TI_WRAPPER_PROCESSORS_REGS_IE (rw) register accessor: MCAN Subsystem Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_regs_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_processors_regs_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti_wrapper_processors_regs_ie`] module"]
#[doc(alias = "TI_WRAPPER_PROCESSORS_REGS_IE")]
pub type TiWrapperProcessorsRegsIe =
    crate::Reg<ti_wrapper_processors_regs_ie::TiWrapperProcessorsRegsIeSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Register"]
pub mod ti_wrapper_processors_regs_ie;
#[doc = "IES (r) register accessor: MCAN Subsystem Interrupt Enable Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ies::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`] module"]
#[doc(alias = "IES")]
pub type Ies = crate::Reg<ies::IesSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Status"]
pub mod ies;
#[doc = "EOI (rw) register accessor: MCAN Subsystem End of Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`] module"]
#[doc(alias = "EOI")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "MCAN Subsystem End of Interrupt"]
pub mod eoi;
#[doc = "EXT_TS_PRESCALER (rw) register accessor: MCAN Subsystem External Timestamp Prescaler 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ts_prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ts_prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ts_prescaler`] module"]
#[doc(alias = "EXT_TS_PRESCALER")]
pub type ExtTsPrescaler = crate::Reg<ext_ts_prescaler::ExtTsPrescalerSpec>;
#[doc = "MCAN Subsystem External Timestamp Prescaler 0"]
pub mod ext_ts_prescaler;
#[doc = "EXT_TS_UNSERVICED_INTR_CNTR (r) register accessor: MCAN Subsystem External Timestamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ts_unserviced_intr_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ts_unserviced_intr_cntr`] module"]
#[doc(alias = "EXT_TS_UNSERVICED_INTR_CNTR")]
pub type ExtTsUnservicedIntrCntr =
    crate::Reg<ext_ts_unserviced_intr_cntr::ExtTsUnservicedIntrCntrSpec>;
#[doc = "MCAN Subsystem External Timestamp Unserviced Interrupts Counter"]
pub mod ext_ts_unserviced_intr_cntr;
