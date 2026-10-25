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
    dma_trig_rx: DmaTrigRx,
    _reserved5: [u8; 0x0c],
    dma_trig_tx: DmaTrigTx,
    _reserved6: [u8; 0x40],
    intctl: Intctl,
    _reserved7: [u8; 0x18],
    ctl0: Ctl0,
    _reserved8: [u8; 0x04],
    stat: Stat,
    ifls: Ifls,
    clkctl: Clkctl,
    _reserved11: [u8; 0x0c],
    txdata: Txdata,
    rxdata: Rxdata,
    _reserved13: [u8; 0x24],
    ctl1: Ctl1,
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
    #[doc = "0x50..0x74 - DMA_TRIG_RX"]
    #[inline(always)]
    pub const fn dma_trig_rx(&self) -> &DmaTrigRx {
        &self.dma_trig_rx
    }
    #[doc = "0x80..0xa4 - DMA_TRIG_TX"]
    #[inline(always)]
    pub const fn dma_trig_tx(&self) -> &DmaTrigTx {
        &self.dma_trig_tx
    }
    #[doc = "0xe4 - Interrupt control register"]
    #[inline(always)]
    pub const fn intctl(&self) -> &Intctl {
        &self.intctl
    }
    #[doc = "0x100 - SPI control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x108 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10c - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x110 - Clock prescaler and divider register."]
    #[inline(always)]
    pub const fn clkctl(&self) -> &Clkctl {
        &self.clkctl
    }
    #[doc = "0x120 - TXDATA Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x124 - RXDATA Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x14c - SPI control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
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
#[doc = "DMA_TRIG_RX"]
pub use self::dma_trig_rx::DmaTrigRx;
#[doc = r"Cluster"]
#[doc = "DMA_TRIG_RX"]
pub mod dma_trig_rx;
#[doc = "DMA_TRIG_TX"]
pub use self::dma_trig_tx::DmaTrigTx;
#[doc = r"Cluster"]
#[doc = "DMA_TRIG_TX"]
pub mod dma_trig_tx;
#[doc = "INTCTL (w) register accessor: Interrupt control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctl`] module"]
#[doc(alias = "INTCTL")]
pub type Intctl = crate::Reg<intctl::IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod intctl;
#[doc = "CTL0 (rw) register accessor: SPI control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "SPI control register 0"]
pub mod ctl0;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`] module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "CLKCTL (rw) register accessor: Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctl`] module"]
#[doc(alias = "CLKCTL")]
pub type Clkctl = crate::Reg<clkctl::ClkctlSpec>;
#[doc = "Clock prescaler and divider register."]
pub mod clkctl;
#[doc = "TXDATA (w) register accessor: TXDATA Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "TXDATA Register"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: RXDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "RXDATA Register"]
pub mod rxdata;
#[doc = "CTL1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "SPI control register 1"]
pub mod ctl1;
