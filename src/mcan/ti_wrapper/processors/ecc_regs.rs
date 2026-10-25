#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "ECC_REGS"]
#[doc(alias = "ECC_REGS")]
pub struct EccRegs {
    rev: Rev,
    _reserved1: [u8; 0x04],
    vector: Vector,
    ti_wrapper_processors_ecc_regs_stat: TiWrapperProcessorsEccRegsStat,
    wrap_rev: WrapRev,
    ti_wrapper_ecc_regs_ctrl: TiWrapperEccRegsCtrl,
    err_ctrl1: ErrCtrl1,
    err_ctrl2: ErrCtrl2,
    err_stat1: ErrStat1,
    err_stat2: ErrStat2,
    err_stat3: ErrStat3,
    _reserved10: [u8; 0x10],
    sec_eoi: SecEoi,
    sec_status: SecStatus,
    _reserved12: [u8; 0x3c],
    sec_enable_set: SecEnableSet,
    _reserved13: [u8; 0x3c],
    sec_enable_clr: SecEnableClr,
    _reserved14: [u8; 0x78],
    ded_eoi: DedEoi,
    ded_status: DedStatus,
    _reserved16: [u8; 0x3c],
    ded_enable_set: DedEnableSet,
    _reserved17: [u8; 0x3c],
    ded_enable_clr: DedEnableClr,
    _reserved18: [u8; 0x3c],
    aggr_enable_set: AggrEnableSet,
    aggr_enable_clr: AggrEnableClr,
    aggr_status_set: AggrStatusSet,
    aggr_status_clr: AggrStatusClr,
}
impl EccRegs {
    #[doc = "0x00 - MCAN Error Aggregator Revision Register"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x08 - MCAN ECC Vector Register"]
    #[inline(always)]
    pub const fn vector(&self) -> &Vector {
        &self.vector
    }
    #[doc = "0x0c - MCAN Error Misc Status"]
    #[inline(always)]
    pub const fn ti_wrapper_processors_ecc_regs_stat(&self) -> &TiWrapperProcessorsEccRegsStat {
        &self.ti_wrapper_processors_ecc_regs_stat
    }
    #[doc = "0x10 - MCAN ECC Wrapper Revision Register"]
    #[inline(always)]
    pub const fn wrap_rev(&self) -> &WrapRev {
        &self.wrap_rev
    }
    #[doc = "0x14 - MCAN ECC Control"]
    #[inline(always)]
    pub const fn ti_wrapper_ecc_regs_ctrl(&self) -> &TiWrapperEccRegsCtrl {
        &self.ti_wrapper_ecc_regs_ctrl
    }
    #[doc = "0x18 - MCAN ECC Error Control 1 Register"]
    #[inline(always)]
    pub const fn err_ctrl1(&self) -> &ErrCtrl1 {
        &self.err_ctrl1
    }
    #[doc = "0x1c - MCAN ECC Error Control 2 Register"]
    #[inline(always)]
    pub const fn err_ctrl2(&self) -> &ErrCtrl2 {
        &self.err_ctrl2
    }
    #[doc = "0x20 - MCAN ECC Error Status 1 Register"]
    #[inline(always)]
    pub const fn err_stat1(&self) -> &ErrStat1 {
        &self.err_stat1
    }
    #[doc = "0x24 - MCAN ECC Error Status 2 Register"]
    #[inline(always)]
    pub const fn err_stat2(&self) -> &ErrStat2 {
        &self.err_stat2
    }
    #[doc = "0x28 - MCAN ECC Error Status 3 Register"]
    #[inline(always)]
    pub const fn err_stat3(&self) -> &ErrStat3 {
        &self.err_stat3
    }
    #[doc = "0x3c - MCAN Single Error Corrected End of Interrupt Register"]
    #[inline(always)]
    pub const fn sec_eoi(&self) -> &SecEoi {
        &self.sec_eoi
    }
    #[doc = "0x40 - MCAN Single Error Corrected Interrupt Status Register"]
    #[inline(always)]
    pub const fn sec_status(&self) -> &SecStatus {
        &self.sec_status
    }
    #[doc = "0x80 - MCAN Single Error Corrected Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn sec_enable_set(&self) -> &SecEnableSet {
        &self.sec_enable_set
    }
    #[doc = "0xc0 - MCAN Single Error Corrected Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn sec_enable_clr(&self) -> &SecEnableClr {
        &self.sec_enable_clr
    }
    #[doc = "0x13c - MCAN Double Error Detected End of Interrupt Register"]
    #[inline(always)]
    pub const fn ded_eoi(&self) -> &DedEoi {
        &self.ded_eoi
    }
    #[doc = "0x140 - MCAN Double Error Detected Interrupt Status Register"]
    #[inline(always)]
    pub const fn ded_status(&self) -> &DedStatus {
        &self.ded_status
    }
    #[doc = "0x180 - MCAN Double Error Detected Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn ded_enable_set(&self) -> &DedEnableSet {
        &self.ded_enable_set
    }
    #[doc = "0x1c0 - MCAN Double Error Detected Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn ded_enable_clr(&self) -> &DedEnableClr {
        &self.ded_enable_clr
    }
    #[doc = "0x200 - MCAN Error Aggregator Enable Set Register"]
    #[inline(always)]
    pub const fn aggr_enable_set(&self) -> &AggrEnableSet {
        &self.aggr_enable_set
    }
    #[doc = "0x204 - MCAN Error Aggregator Enable Clear Register"]
    #[inline(always)]
    pub const fn aggr_enable_clr(&self) -> &AggrEnableClr {
        &self.aggr_enable_clr
    }
    #[doc = "0x208 - MCAN Error Aggregator Status Set Register"]
    #[inline(always)]
    pub const fn aggr_status_set(&self) -> &AggrStatusSet {
        &self.aggr_status_set
    }
    #[doc = "0x20c - MCAN Error Aggregator Status Clear Register"]
    #[inline(always)]
    pub const fn aggr_status_clr(&self) -> &AggrStatusClr {
        &self.aggr_status_clr
    }
}
#[doc = "REV (r) register accessor: MCAN Error Aggregator Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`] module"]
#[doc(alias = "REV")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "MCAN Error Aggregator Revision Register"]
pub mod rev;
#[doc = "VECTOR (rw) register accessor: MCAN ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vector`] module"]
#[doc(alias = "VECTOR")]
pub type Vector = crate::Reg<vector::VectorSpec>;
#[doc = "MCAN ECC Vector Register"]
pub mod vector;
#[doc = "TI_WRAPPER_PROCESSORS_ECC_REGS_STAT (r) register accessor: MCAN Error Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_ecc_regs_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti_wrapper_processors_ecc_regs_stat`] module"]
#[doc(alias = "TI_WRAPPER_PROCESSORS_ECC_REGS_STAT")]
pub type TiWrapperProcessorsEccRegsStat =
    crate::Reg<ti_wrapper_processors_ecc_regs_stat::TiWrapperProcessorsEccRegsStatSpec>;
#[doc = "MCAN Error Misc Status"]
pub mod ti_wrapper_processors_ecc_regs_stat;
#[doc = "WRAP_REV (r) register accessor: MCAN ECC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrap_rev`] module"]
#[doc(alias = "WRAP_REV")]
pub type WrapRev = crate::Reg<wrap_rev::WrapRevSpec>;
#[doc = "MCAN ECC Wrapper Revision Register"]
pub mod wrap_rev;
#[doc = "TI_WRAPPER_ECC_REGS_CTRL (rw) register accessor: MCAN ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_ecc_regs_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_ecc_regs_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti_wrapper_ecc_regs_ctrl`] module"]
#[doc(alias = "TI_WRAPPER_ECC_REGS_CTRL")]
pub type TiWrapperEccRegsCtrl = crate::Reg<ti_wrapper_ecc_regs_ctrl::TiWrapperEccRegsCtrlSpec>;
#[doc = "MCAN ECC Control"]
pub mod ti_wrapper_ecc_regs_ctrl;
#[doc = "ERR_CTRL1 (rw) register accessor: MCAN ECC Error Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl1`] module"]
#[doc(alias = "ERR_CTRL1")]
pub type ErrCtrl1 = crate::Reg<err_ctrl1::ErrCtrl1Spec>;
#[doc = "MCAN ECC Error Control 1 Register"]
pub mod err_ctrl1;
#[doc = "ERR_CTRL2 (rw) register accessor: MCAN ECC Error Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl2`] module"]
#[doc(alias = "ERR_CTRL2")]
pub type ErrCtrl2 = crate::Reg<err_ctrl2::ErrCtrl2Spec>;
#[doc = "MCAN ECC Error Control 2 Register"]
pub mod err_ctrl2;
#[doc = "ERR_STAT1 (rw) register accessor: MCAN ECC Error Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat1`] module"]
#[doc(alias = "ERR_STAT1")]
pub type ErrStat1 = crate::Reg<err_stat1::ErrStat1Spec>;
#[doc = "MCAN ECC Error Status 1 Register"]
pub mod err_stat1;
#[doc = "ERR_STAT2 (r) register accessor: MCAN ECC Error Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat2`] module"]
#[doc(alias = "ERR_STAT2")]
pub type ErrStat2 = crate::Reg<err_stat2::ErrStat2Spec>;
#[doc = "MCAN ECC Error Status 2 Register"]
pub mod err_stat2;
#[doc = "ERR_STAT3 (rw) register accessor: MCAN ECC Error Status 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat3`] module"]
#[doc(alias = "ERR_STAT3")]
pub type ErrStat3 = crate::Reg<err_stat3::ErrStat3Spec>;
#[doc = "MCAN ECC Error Status 3 Register"]
pub mod err_stat3;
#[doc = "SEC_EOI (rw) register accessor: MCAN Single Error Corrected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_eoi`] module"]
#[doc(alias = "SEC_EOI")]
pub type SecEoi = crate::Reg<sec_eoi::SecEoiSpec>;
#[doc = "MCAN Single Error Corrected End of Interrupt Register"]
pub mod sec_eoi;
#[doc = "SEC_STATUS (rw) register accessor: MCAN Single Error Corrected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_status`] module"]
#[doc(alias = "SEC_STATUS")]
pub type SecStatus = crate::Reg<sec_status::SecStatusSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Status Register"]
pub mod sec_status;
#[doc = "SEC_ENABLE_SET (rw) register accessor: MCAN Single Error Corrected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_set`] module"]
#[doc(alias = "SEC_ENABLE_SET")]
pub type SecEnableSet = crate::Reg<sec_enable_set::SecEnableSetSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Enable Set Register"]
pub mod sec_enable_set;
#[doc = "SEC_ENABLE_CLR (rw) register accessor: MCAN Single Error Corrected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_clr`] module"]
#[doc(alias = "SEC_ENABLE_CLR")]
pub type SecEnableClr = crate::Reg<sec_enable_clr::SecEnableClrSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Enable Clear Register"]
pub mod sec_enable_clr;
#[doc = "DED_EOI (rw) register accessor: MCAN Double Error Detected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_eoi`] module"]
#[doc(alias = "DED_EOI")]
pub type DedEoi = crate::Reg<ded_eoi::DedEoiSpec>;
#[doc = "MCAN Double Error Detected End of Interrupt Register"]
pub mod ded_eoi;
#[doc = "DED_STATUS (rw) register accessor: MCAN Double Error Detected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_status`] module"]
#[doc(alias = "DED_STATUS")]
pub type DedStatus = crate::Reg<ded_status::DedStatusSpec>;
#[doc = "MCAN Double Error Detected Interrupt Status Register"]
pub mod ded_status;
#[doc = "DED_ENABLE_SET (rw) register accessor: MCAN Double Error Detected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_set`] module"]
#[doc(alias = "DED_ENABLE_SET")]
pub type DedEnableSet = crate::Reg<ded_enable_set::DedEnableSetSpec>;
#[doc = "MCAN Double Error Detected Interrupt Enable Set Register"]
pub mod ded_enable_set;
#[doc = "DED_ENABLE_CLR (rw) register accessor: MCAN Double Error Detected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_clr`] module"]
#[doc(alias = "DED_ENABLE_CLR")]
pub type DedEnableClr = crate::Reg<ded_enable_clr::DedEnableClrSpec>;
#[doc = "MCAN Double Error Detected Interrupt Enable Clear Register"]
pub mod ded_enable_clr;
#[doc = "AGGR_ENABLE_SET (rw) register accessor: MCAN Error Aggregator Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_set`] module"]
#[doc(alias = "AGGR_ENABLE_SET")]
pub type AggrEnableSet = crate::Reg<aggr_enable_set::AggrEnableSetSpec>;
#[doc = "MCAN Error Aggregator Enable Set Register"]
pub mod aggr_enable_set;
#[doc = "AGGR_ENABLE_CLR (rw) register accessor: MCAN Error Aggregator Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_clr`] module"]
#[doc(alias = "AGGR_ENABLE_CLR")]
pub type AggrEnableClr = crate::Reg<aggr_enable_clr::AggrEnableClrSpec>;
#[doc = "MCAN Error Aggregator Enable Clear Register"]
pub mod aggr_enable_clr;
#[doc = "AGGR_STATUS_SET (rw) register accessor: MCAN Error Aggregator Status Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_set`] module"]
#[doc(alias = "AGGR_STATUS_SET")]
pub type AggrStatusSet = crate::Reg<aggr_status_set::AggrStatusSetSpec>;
#[doc = "MCAN Error Aggregator Status Set Register"]
pub mod aggr_status_set;
#[doc = "AGGR_STATUS_CLR (rw) register accessor: MCAN Error Aggregator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_clr`] module"]
#[doc(alias = "AGGR_STATUS_CLR")]
pub type AggrStatusClr = crate::Reg<aggr_status_clr::AggrStatusClrSpec>;
#[doc = "MCAN Error Aggregator Status Clear Register"]
pub mod aggr_status_clr;
