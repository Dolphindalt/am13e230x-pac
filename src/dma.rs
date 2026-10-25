#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1018],
    pdbgctl: Pdbgctl,
    _reserved1: [u8; 0x04],
    cpu_int: CpuInt,
    _reserved2: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved3: [u8; 0x18],
    desc: Desc,
    dmaprio: Dmaprio,
    _reserved5: [u8; 0x0c],
    dmatctl: [Dmatctl; 12],
    _reserved6: [u8; 0xc0],
    dmachan: [Dmachan; 12],
}
impl RegisterBlock {
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
    }
    #[doc = "0x1020..0x104c - CPU_INT"]
    #[inline(always)]
    pub const fn cpu_int(&self) -> &CpuInt {
        &self.cpu_int
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - DMA Channel Priority Control"]
    #[inline(always)]
    pub const fn dmaprio(&self) -> &Dmaprio {
        &self.dmaprio
    }
    #[doc = "0x1110..0x1140 - DMA Trigger Select"]
    #[inline(always)]
    pub const fn dmatctl(&self, n: usize) -> &Dmatctl {
        &self.dmatctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1110..0x1140 - DMA Trigger Select"]
    #[inline(always)]
    pub fn dmatctl_iter(&self) -> impl Iterator<Item = &Dmatctl> {
        self.dmatctl.iter()
    }
    #[doc = "0x1200..0x12c0 - DMACHAN\\[%s\\]"]
    #[inline(always)]
    pub const fn dmachan(&self, n: usize) -> &Dmachan {
        &self.dmachan[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1200..0x12c0 - DMACHAN\\[%s\\]"]
    #[inline(always)]
    pub fn dmachan_iter(&self) -> impl Iterator<Item = &Dmachan> {
        self.dmachan.iter()
    }
}
#[doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`] module"]
#[doc(alias = "PDBGCTL")]
pub type Pdbgctl = crate::Reg<pdbgctl::PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod pdbgctl;
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
#[doc = "DMAPRIO (rw) register accessor: DMA Channel Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaprio`] module"]
#[doc(alias = "DMAPRIO")]
pub type Dmaprio = crate::Reg<dmaprio::DmaprioSpec>;
#[doc = "DMA Channel Priority Control"]
pub mod dmaprio;
#[doc = "DMATCTL (rw) register accessor: DMA Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatctl`] module"]
#[doc(alias = "DMATCTL")]
pub type Dmatctl = crate::Reg<dmatctl::DmatctlSpec>;
#[doc = "DMA Trigger Select"]
pub mod dmatctl;
#[doc = "DMACHAN\\[%s\\]"]
pub use self::dmachan::Dmachan;
#[doc = r"Cluster"]
#[doc = "DMACHAN\\[%s\\]"]
pub mod dmachan;
