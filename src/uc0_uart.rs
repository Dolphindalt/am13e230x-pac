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
    lcrh: Lcrh,
    stat: Stat,
    ifls: Ifls,
    ibrd: Ibrd,
    fbrd: Fbrd,
    gfctl: Gfctl,
    _reserved14: [u8; 0x04],
    txdata: Txdata,
    rxdata: Rxdata,
    _reserved16: [u8; 0x08],
    lincnt: Lincnt,
    linctl: Linctl,
    linc0: Linc0,
    linc1: Linc1,
    _reserved20: [u8; 0x08],
    amask: Amask,
    addr: Addr,
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
    #[doc = "0x100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn lcrh(&self) -> &Lcrh {
        &self.lcrh
    }
    #[doc = "0x108 - UART Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn gfctl(&self) -> &Gfctl {
        &self.gfctl
    }
    #[doc = "0x120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x130 - UART LIN Mode Counter Register"]
    #[inline(always)]
    pub const fn lincnt(&self) -> &Lincnt {
        &self.lincnt
    }
    #[doc = "0x134 - UART LIN Mode Control Register"]
    #[inline(always)]
    pub const fn linctl(&self) -> &Linctl {
        &self.linctl
    }
    #[doc = "0x138 - UART LIN Mode Capture 0 Register"]
    #[inline(always)]
    pub const fn linc0(&self) -> &Linc0 {
        &self.linc0
    }
    #[doc = "0x13c - UART LIN Mode Capture 1 Register"]
    #[inline(always)]
    pub const fn linc1(&self) -> &Linc1 {
        &self.linc1
    }
    #[doc = "0x148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn amask(&self) -> &Amask {
        &self.amask
    }
    #[doc = "0x14c - Self Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
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
#[doc = "CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod ctl0;
#[doc = "LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`] module"]
#[doc(alias = "LCRH")]
pub type Lcrh = crate::Reg<lcrh::LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod lcrh;
#[doc = "STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "UART Status Register"]
pub mod stat;
#[doc = "IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`] module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`] module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`] module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod fbrd;
#[doc = "GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`] module"]
#[doc(alias = "GFCTL")]
pub type Gfctl = crate::Reg<gfctl::GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod gfctl;
#[doc = "TXDATA (w) register accessor: UART Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod rxdata;
#[doc = "LINCNT (rw) register accessor: UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lincnt`] module"]
#[doc(alias = "LINCNT")]
pub type Lincnt = crate::Reg<lincnt::LincntSpec>;
#[doc = "UART LIN Mode Counter Register"]
pub mod lincnt;
#[doc = "LINCTL (rw) register accessor: UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linctl`] module"]
#[doc(alias = "LINCTL")]
pub type Linctl = crate::Reg<linctl::LinctlSpec>;
#[doc = "UART LIN Mode Control Register"]
pub mod linctl;
#[doc = "LINC0 (rw) register accessor: UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linc0`] module"]
#[doc(alias = "LINC0")]
pub type Linc0 = crate::Reg<linc0::Linc0Spec>;
#[doc = "UART LIN Mode Capture 0 Register"]
pub mod linc0;
#[doc = "LINC1 (rw) register accessor: UART LIN Mode Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linc1`] module"]
#[doc(alias = "LINC1")]
pub type Linc1 = crate::Reg<linc1::Linc1Spec>;
#[doc = "UART LIN Mode Capture 1 Register"]
pub mod linc1;
#[doc = "AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amask`] module"]
#[doc(alias = "AMASK")]
pub type Amask = crate::Reg<amask::AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod amask;
#[doc = "ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Self Address Register"]
pub mod addr;
