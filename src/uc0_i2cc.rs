#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkdiv: Clkdiv,
    _reserved1: [u8; 0x04],
    clksel: Clksel,
    _reserved2: [u8; 0x0c],
    pdbgctl: Pdbgctl,
    _reserved3: [u8; 0x04],
    cpu_int: CpuInt,
    _reserved4: [u8; 0x04],
    dma_trig1: DmaTrig1,
    _reserved5: [u8; 0x0c],
    dma_trig0: DmaTrig0,
    _reserved6: [u8; 0x40],
    intctl: Intctl,
    _reserved7: [u8; 0x18],
    ctr: Ctr,
    cr: Cr,
    sr: Sr,
    ifls: Ifls,
    tpr: Tpr,
    _reserved12: [u8; 0x04],
    gfctl: Gfctl,
    bmon: Bmon,
    txdata: Txdata,
    rxdata: Rxdata,
    pecsr: Pecsr,
    _reserved17: [u8; 0x20],
    ta: Ta,
    timeout_cnt: TimeoutCnt,
    timeout_ctl: TimeoutCtl,
    pecctl: Pecctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x08 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
    }
    #[doc = "0x18 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
    }
    #[doc = "0x20..0x4c - CPU_INT"]
    #[inline(always)]
    pub const fn cpu_int(&self) -> &CpuInt {
        &self.cpu_int
    }
    #[doc = "0x50..0x74 - DMA_TRIG1"]
    #[inline(always)]
    pub const fn dma_trig1(&self) -> &DmaTrig1 {
        &self.dma_trig1
    }
    #[doc = "0x80..0xa4 - DMA_TRIG0"]
    #[inline(always)]
    pub const fn dma_trig0(&self) -> &DmaTrig0 {
        &self.dma_trig0
    }
    #[doc = "0xe4 - Interrupt control register"]
    #[inline(always)]
    pub const fn intctl(&self) -> &Intctl {
        &self.intctl
    }
    #[doc = "0x100 - Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x104 - Configuration"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x108 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10c - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x110 - Timer Period"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0x118 - I2C Glitch Filter Control"]
    #[inline(always)]
    pub const fn gfctl(&self) -> &Gfctl {
        &self.gfctl
    }
    #[doc = "0x11c - Bus Monitor"]
    #[inline(always)]
    pub const fn bmon(&self) -> &Bmon {
        &self.bmon
    }
    #[doc = "0x120 - TXData"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x124 - RXData"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x128 - PEC status register"]
    #[inline(always)]
    pub const fn pecsr(&self) -> &Pecsr {
        &self.pecsr
    }
    #[doc = "0x14c - Target Address Register"]
    #[inline(always)]
    pub const fn ta(&self) -> &Ta {
        &self.ta
    }
    #[doc = "0x150 - I2C Timeout Count Register"]
    #[inline(always)]
    pub const fn timeout_cnt(&self) -> &TimeoutCnt {
        &self.timeout_cnt
    }
    #[doc = "0x154 - I2C Timeout Count Control Register"]
    #[inline(always)]
    pub const fn timeout_ctl(&self) -> &TimeoutCtl {
        &self.timeout_ctl
    }
    #[doc = "0x158 - I2C PEC control register"]
    #[inline(always)]
    pub const fn pecctl(&self) -> &Pecctl {
        &self.pecctl
    }
}
#[doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod clkdiv;
#[doc = "CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`] module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod clksel;
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
#[doc = "DMA_TRIG1"]
pub use self::dma_trig1::DmaTrig1;
#[doc = r"Cluster"]
#[doc = "DMA_TRIG1"]
pub mod dma_trig1;
#[doc = "DMA_TRIG0"]
pub use self::dma_trig0::DmaTrig0;
#[doc = r"Cluster"]
#[doc = "DMA_TRIG0"]
pub mod dma_trig0;
#[doc = "INTCTL (w) register accessor: Interrupt control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctl`] module"]
#[doc(alias = "INTCTL")]
pub type Intctl = crate::Reg<intctl::IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod intctl;
#[doc = "CTR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`] module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Control Register"]
pub mod ctr;
#[doc = "CR (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Configuration"]
pub mod cr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`] module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "TPR (rw) register accessor: Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`] module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Timer Period"]
pub mod tpr;
#[doc = "GFCTL (rw) register accessor: I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`] module"]
#[doc(alias = "GFCTL")]
pub type Gfctl = crate::Reg<gfctl::GfctlSpec>;
#[doc = "I2C Glitch Filter Control"]
pub mod gfctl;
#[doc = "BMON (r) register accessor: Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`bmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmon`] module"]
#[doc(alias = "BMON")]
pub type Bmon = crate::Reg<bmon::BmonSpec>;
#[doc = "Bus Monitor"]
pub mod bmon;
#[doc = "TXDATA (w) register accessor: TXData\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "TXData"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "RXData"]
pub mod rxdata;
#[doc = "PECSR (r) register accessor: PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecsr`] module"]
#[doc(alias = "PECSR")]
pub type Pecsr = crate::Reg<pecsr::PecsrSpec>;
#[doc = "PEC status register"]
pub mod pecsr;
#[doc = "TA (rw) register accessor: Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta`] module"]
#[doc(alias = "TA")]
pub type Ta = crate::Reg<ta::TaSpec>;
#[doc = "Target Address Register"]
pub mod ta;
#[doc = "TIMEOUT_CNT (r) register accessor: I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_cnt`] module"]
#[doc(alias = "TIMEOUT_CNT")]
pub type TimeoutCnt = crate::Reg<timeout_cnt::TimeoutCntSpec>;
#[doc = "I2C Timeout Count Register"]
pub mod timeout_cnt;
#[doc = "TIMEOUT_CTL (rw) register accessor: I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctl`] module"]
#[doc(alias = "TIMEOUT_CTL")]
pub type TimeoutCtl = crate::Reg<timeout_ctl::TimeoutCtlSpec>;
#[doc = "I2C Timeout Count Control Register"]
pub mod timeout_ctl;
#[doc = "PECCTL (rw) register accessor: I2C PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecctl`] module"]
#[doc(alias = "PECCTL")]
pub type Pecctl = crate::Reg<pecctl::PecctlSpec>;
#[doc = "I2C PEC control register"]
pub mod pecctl;
