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
    ackctl: Ackctl,
    sr: Sr,
    ifls: Ifls,
    _reserved11: [u8; 0x08],
    gfctl: Gfctl,
    _reserved12: [u8; 0x04],
    txdata: Txdata,
    rxdata: Rxdata,
    pecsr: Pecsr,
    _reserved15: [u8; 0x1c],
    oar2: Oar2,
    oar: Oar,
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
    #[doc = "0x100 - I2C Target Control Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x104 - I2C Target ACK Control"]
    #[inline(always)]
    pub const fn ackctl(&self) -> &Ackctl {
        &self.ackctl
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
    #[doc = "0x118 - I2C Glitch Filter Control"]
    #[inline(always)]
    pub const fn gfctl(&self) -> &Gfctl {
        &self.gfctl
    }
    #[doc = "0x120 - I2C TXData"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x124 - I2C RXData"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x128 - PEC status register"]
    #[inline(always)]
    pub const fn pecsr(&self) -> &Pecsr {
        &self.pecsr
    }
    #[doc = "0x148 - Own Address 2"]
    #[inline(always)]
    pub const fn oar2(&self) -> &Oar2 {
        &self.oar2
    }
    #[doc = "0x14c - I2C Own Address"]
    #[inline(always)]
    pub const fn oar(&self) -> &Oar {
        &self.oar
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
#[doc = "CTR (rw) register accessor: I2C Target Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`] module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "I2C Target Control Register"]
pub mod ctr;
#[doc = "ACKCTL (rw) register accessor: I2C Target ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ackctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ackctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ackctl`] module"]
#[doc(alias = "ACKCTL")]
pub type Ackctl = crate::Reg<ackctl::AckctlSpec>;
#[doc = "I2C Target ACK Control"]
pub mod ackctl;
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
#[doc = "GFCTL (rw) register accessor: I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`] module"]
#[doc(alias = "GFCTL")]
pub type Gfctl = crate::Reg<gfctl::GfctlSpec>;
#[doc = "I2C Glitch Filter Control"]
pub mod gfctl;
#[doc = "TXDATA (w) register accessor: I2C TXData\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "I2C TXData"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: I2C RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "I2C RXData"]
pub mod rxdata;
#[doc = "PECSR (r) register accessor: PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecsr`] module"]
#[doc(alias = "PECSR")]
pub type Pecsr = crate::Reg<pecsr::PecsrSpec>;
#[doc = "PEC status register"]
pub mod pecsr;
#[doc = "OAR2 (rw) register accessor: Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar2`] module"]
#[doc(alias = "OAR2")]
pub type Oar2 = crate::Reg<oar2::Oar2Spec>;
#[doc = "Own Address 2"]
pub mod oar2;
#[doc = "OAR (rw) register accessor: I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`oar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar`] module"]
#[doc(alias = "OAR")]
pub type Oar = crate::Reg<oar::OarSpec>;
#[doc = "I2C Own Address"]
pub mod oar;
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
