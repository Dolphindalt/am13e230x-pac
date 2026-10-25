#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    baud: Baud,
    baud2: Baud2,
    _reserved3: [u8; 0x04],
    _reserved_3_gpcfg: [u8; 0x04],
    _reserved_4_hb8cfg2: [u8; 0x04],
    _reserved5: [u8; 0x04],
    addrmap: Addrmap,
    rsize0: Rsize0,
    raddr0: Raddr0,
    rpstd0: Rpstd0,
    _reserved9: [u8; 0x04],
    rsize1: Rsize1,
    raddr1: Raddr1,
    rpstd1: Rpstd1,
    _reserved12: [u8; 0x24],
    stat: Stat,
    _reserved13: [u8; 0x08],
    rfifocnt: Rfifocnt,
    readfifo0: Readfifo0,
    readfifo1: Readfifo1,
    readfifo2: Readfifo2,
    readfifo3: Readfifo3,
    readfifo4: Readfifo4,
    readfifo5: Readfifo5,
    readfifo6: Readfifo6,
    readfifo7: Readfifo7,
    _reserved22: [u8; 0x0170],
    fifolvl: Fifolvl,
    wfifocnt: Wfifocnt,
    dmatxcnt: Dmatxcnt,
    _reserved25: [u8; 0x04],
    im: Im,
    ris: Ris,
    mis: Mis,
    eisc: Eisc,
    _reserved29: [u8; 0xe8],
    _reserved_29_hb8cfg3: [u8; 0x04],
    _reserved_30_hb8cfg4: [u8; 0x04],
    _reserved_31_hb8time: [u8; 0x04],
    _reserved_32_hb8time2: [u8; 0x04],
    _reserved_33_hb8time3: [u8; 0x04],
    _reserved_34_hb8time4: [u8; 0x04],
    _reserved35: [u8; 0x40],
    hbpsram: Hbpsram,
}
impl RegisterBlock {
    #[doc = "0x00 - EPI Configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - EPI Main Baud Rate"]
    #[inline(always)]
    pub const fn baud(&self) -> &Baud {
        &self.baud
    }
    #[doc = "0x08 - EPI Main Baud Rate"]
    #[inline(always)]
    pub const fn baud2(&self) -> &Baud2 {
        &self.baud2
    }
    #[doc = "0x10 - EPI Host-Bus 16 Configuration"]
    #[inline(always)]
    pub const fn hb16cfg(&self) -> &Hb16cfg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - EPI Host-Bus 8 Configuration"]
    #[inline(always)]
    pub const fn hb8cfg(&self) -> &Hb8cfg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - EPI SDRAM Configuration"]
    #[inline(always)]
    pub const fn sdramcfg(&self) -> &Sdramcfg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - EPI General-Purpose Configuration"]
    #[inline(always)]
    pub const fn gpcfg(&self) -> &Gpcfg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - EPI Host-Bus 16 Configuration 2"]
    #[inline(always)]
    pub const fn hb16cfg2(&self) -> &Hb16cfg2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - EPI Host-Bus 8 Configuration 2"]
    #[inline(always)]
    pub const fn hb8cfg2(&self) -> &Hb8cfg2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x1c - EPI Address Map"]
    #[inline(always)]
    pub const fn addrmap(&self) -> &Addrmap {
        &self.addrmap
    }
    #[doc = "0x20 - EPI Read Size 0"]
    #[inline(always)]
    pub const fn rsize0(&self) -> &Rsize0 {
        &self.rsize0
    }
    #[doc = "0x24 - EPI Read Address 0"]
    #[inline(always)]
    pub const fn raddr0(&self) -> &Raddr0 {
        &self.raddr0
    }
    #[doc = "0x28 - EPI Non-Blocking Read Data 0"]
    #[inline(always)]
    pub const fn rpstd0(&self) -> &Rpstd0 {
        &self.rpstd0
    }
    #[doc = "0x30 - EPI Read Size 1"]
    #[inline(always)]
    pub const fn rsize1(&self) -> &Rsize1 {
        &self.rsize1
    }
    #[doc = "0x34 - EPI Read Address 1"]
    #[inline(always)]
    pub const fn raddr1(&self) -> &Raddr1 {
        &self.raddr1
    }
    #[doc = "0x38 - EPI Non-Blocking Read Data 1"]
    #[inline(always)]
    pub const fn rpstd1(&self) -> &Rpstd1 {
        &self.rpstd1
    }
    #[doc = "0x60 - EPI Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x6c - EPI Read FIFO Count"]
    #[inline(always)]
    pub const fn rfifocnt(&self) -> &Rfifocnt {
        &self.rfifocnt
    }
    #[doc = "0x70 - EPI Read FIFO 0"]
    #[inline(always)]
    pub const fn readfifo0(&self) -> &Readfifo0 {
        &self.readfifo0
    }
    #[doc = "0x74 - EPI Read FIFO 1"]
    #[inline(always)]
    pub const fn readfifo1(&self) -> &Readfifo1 {
        &self.readfifo1
    }
    #[doc = "0x78 - EPI Read FIFO 2"]
    #[inline(always)]
    pub const fn readfifo2(&self) -> &Readfifo2 {
        &self.readfifo2
    }
    #[doc = "0x7c - EPI Read FIFO 3"]
    #[inline(always)]
    pub const fn readfifo3(&self) -> &Readfifo3 {
        &self.readfifo3
    }
    #[doc = "0x80 - EPI Read FIFO 4"]
    #[inline(always)]
    pub const fn readfifo4(&self) -> &Readfifo4 {
        &self.readfifo4
    }
    #[doc = "0x84 - EPI Read FIFO 5"]
    #[inline(always)]
    pub const fn readfifo5(&self) -> &Readfifo5 {
        &self.readfifo5
    }
    #[doc = "0x88 - EPI Read FIFO 6"]
    #[inline(always)]
    pub const fn readfifo6(&self) -> &Readfifo6 {
        &self.readfifo6
    }
    #[doc = "0x8c - EPI Read FIFO 7"]
    #[inline(always)]
    pub const fn readfifo7(&self) -> &Readfifo7 {
        &self.readfifo7
    }
    #[doc = "0x200 - EPI FIFO Level Selects"]
    #[inline(always)]
    pub const fn fifolvl(&self) -> &Fifolvl {
        &self.fifolvl
    }
    #[doc = "0x204 - EPI Write FIFO Count"]
    #[inline(always)]
    pub const fn wfifocnt(&self) -> &Wfifocnt {
        &self.wfifocnt
    }
    #[doc = "0x208 - EPI DMA Transmit Count"]
    #[inline(always)]
    pub const fn dmatxcnt(&self) -> &Dmatxcnt {
        &self.dmatxcnt
    }
    #[doc = "0x210 - EPI Interrupt Mask"]
    #[inline(always)]
    pub const fn im(&self) -> &Im {
        &self.im
    }
    #[doc = "0x214 - EPI Raw Interrupt Status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x218 - EPI Masked Interrupt Status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x21c - EPI Error and Interrupt Status and Clear"]
    #[inline(always)]
    pub const fn eisc(&self) -> &Eisc {
        &self.eisc
    }
    #[doc = "0x308 - EPI Host-Bus 16 Configuration 3"]
    #[inline(always)]
    pub const fn hb16cfg3(&self) -> &Hb16cfg3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(776).cast() }
    }
    #[doc = "0x308 - EPI Host-Bus 8 Configuration 3"]
    #[inline(always)]
    pub const fn hb8cfg3(&self) -> &Hb8cfg3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(776).cast() }
    }
    #[doc = "0x30c - EPI Host-Bus 16 Configuration 4"]
    #[inline(always)]
    pub const fn hb16cfg4(&self) -> &Hb16cfg4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(780).cast() }
    }
    #[doc = "0x30c - EPI Host-Bus 8 Configuration 4"]
    #[inline(always)]
    pub const fn hb8cfg4(&self) -> &Hb8cfg4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(780).cast() }
    }
    #[doc = "0x310 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub const fn hb16time(&self) -> &Hb16time {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(784).cast() }
    }
    #[doc = "0x310 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub const fn hb8time(&self) -> &Hb8time {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(784).cast() }
    }
    #[doc = "0x314 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub const fn hb16time2(&self) -> &Hb16time2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(788).cast() }
    }
    #[doc = "0x314 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub const fn hb8time2(&self) -> &Hb8time2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(788).cast() }
    }
    #[doc = "0x318 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub const fn hb16time3(&self) -> &Hb16time3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(792).cast() }
    }
    #[doc = "0x318 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub const fn hb8time3(&self) -> &Hb8time3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(792).cast() }
    }
    #[doc = "0x31c - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub const fn hb16time4(&self) -> &Hb16time4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(796).cast() }
    }
    #[doc = "0x31c - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub const fn hb8time4(&self) -> &Hb8time4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(796).cast() }
    }
    #[doc = "0x360 - EPI Host-Bus PSRAM"]
    #[inline(always)]
    pub const fn hbpsram(&self) -> &Hbpsram {
        &self.hbpsram
    }
}
#[doc = "CFG (rw) register accessor: EPI Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "EPI Configuration"]
pub mod cfg;
#[doc = "BAUD (rw) register accessor: EPI Main Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`] module"]
#[doc(alias = "BAUD")]
pub type Baud = crate::Reg<baud::BaudSpec>;
#[doc = "EPI Main Baud Rate"]
pub mod baud;
#[doc = "BAUD2 (rw) register accessor: EPI Main Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud2`] module"]
#[doc(alias = "BAUD2")]
pub type Baud2 = crate::Reg<baud2::Baud2Spec>;
#[doc = "EPI Main Baud Rate"]
pub mod baud2;
#[doc = "GPCFG (rw) register accessor: EPI General-Purpose Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpcfg`] module"]
#[doc(alias = "GPCFG")]
pub type Gpcfg = crate::Reg<gpcfg::GpcfgSpec>;
#[doc = "EPI General-Purpose Configuration"]
pub mod gpcfg;
#[doc = "SDRAMCFG (rw) register accessor: EPI SDRAM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sdramcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdramcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdramcfg`] module"]
#[doc(alias = "SDRAMCFG")]
pub type Sdramcfg = crate::Reg<sdramcfg::SdramcfgSpec>;
#[doc = "EPI SDRAM Configuration"]
pub mod sdramcfg;
#[doc = "HB8CFG (rw) register accessor: EPI Host-Bus 8 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8cfg`] module"]
#[doc(alias = "HB8CFG")]
pub type Hb8cfg = crate::Reg<hb8cfg::Hb8cfgSpec>;
#[doc = "EPI Host-Bus 8 Configuration"]
pub mod hb8cfg;
#[doc = "HB16CFG (rw) register accessor: EPI Host-Bus 16 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16cfg`] module"]
#[doc(alias = "HB16CFG")]
pub type Hb16cfg = crate::Reg<hb16cfg::Hb16cfgSpec>;
#[doc = "EPI Host-Bus 16 Configuration"]
pub mod hb16cfg;
#[doc = "HB8CFG2 (rw) register accessor: EPI Host-Bus 8 Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8cfg2`] module"]
#[doc(alias = "HB8CFG2")]
pub type Hb8cfg2 = crate::Reg<hb8cfg2::Hb8cfg2Spec>;
#[doc = "EPI Host-Bus 8 Configuration 2"]
pub mod hb8cfg2;
#[doc = "HB16CFG2 (rw) register accessor: EPI Host-Bus 16 Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16cfg2`] module"]
#[doc(alias = "HB16CFG2")]
pub type Hb16cfg2 = crate::Reg<hb16cfg2::Hb16cfg2Spec>;
#[doc = "EPI Host-Bus 16 Configuration 2"]
pub mod hb16cfg2;
#[doc = "ADDRMAP (rw) register accessor: EPI Address Map\n\nYou can [`read`](crate::Reg::read) this register and get [`addrmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrmap`] module"]
#[doc(alias = "ADDRMAP")]
pub type Addrmap = crate::Reg<addrmap::AddrmapSpec>;
#[doc = "EPI Address Map"]
pub mod addrmap;
#[doc = "RSIZE0 (rw) register accessor: EPI Read Size 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rsize0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsize0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsize0`] module"]
#[doc(alias = "RSIZE0")]
pub type Rsize0 = crate::Reg<rsize0::Rsize0Spec>;
#[doc = "EPI Read Size 0"]
pub mod rsize0;
#[doc = "RADDR0 (rw) register accessor: EPI Read Address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raddr0`] module"]
#[doc(alias = "RADDR0")]
pub type Raddr0 = crate::Reg<raddr0::Raddr0Spec>;
#[doc = "EPI Read Address 0"]
pub mod raddr0;
#[doc = "RPSTD0 (rw) register accessor: EPI Non-Blocking Read Data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rpstd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpstd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpstd0`] module"]
#[doc(alias = "RPSTD0")]
pub type Rpstd0 = crate::Reg<rpstd0::Rpstd0Spec>;
#[doc = "EPI Non-Blocking Read Data 0"]
pub mod rpstd0;
#[doc = "RSIZE1 (rw) register accessor: EPI Read Size 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rsize1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsize1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsize1`] module"]
#[doc(alias = "RSIZE1")]
pub type Rsize1 = crate::Reg<rsize1::Rsize1Spec>;
#[doc = "EPI Read Size 1"]
pub mod rsize1;
#[doc = "RADDR1 (rw) register accessor: EPI Read Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`raddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raddr1`] module"]
#[doc(alias = "RADDR1")]
pub type Raddr1 = crate::Reg<raddr1::Raddr1Spec>;
#[doc = "EPI Read Address 1"]
pub mod raddr1;
#[doc = "RPSTD1 (rw) register accessor: EPI Non-Blocking Read Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rpstd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpstd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpstd1`] module"]
#[doc(alias = "RPSTD1")]
pub type Rpstd1 = crate::Reg<rpstd1::Rpstd1Spec>;
#[doc = "EPI Non-Blocking Read Data 1"]
pub mod rpstd1;
#[doc = "STAT (rw) register accessor: EPI Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "EPI Status"]
pub mod stat;
#[doc = "RFIFOCNT (rw) register accessor: EPI Read FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifocnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifocnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifocnt`] module"]
#[doc(alias = "RFIFOCNT")]
pub type Rfifocnt = crate::Reg<rfifocnt::RfifocntSpec>;
#[doc = "EPI Read FIFO Count"]
pub mod rfifocnt;
#[doc = "READFIFO0 (rw) register accessor: EPI Read FIFO 0\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo0`] module"]
#[doc(alias = "READFIFO0")]
pub type Readfifo0 = crate::Reg<readfifo0::Readfifo0Spec>;
#[doc = "EPI Read FIFO 0"]
pub mod readfifo0;
#[doc = "READFIFO1 (rw) register accessor: EPI Read FIFO 1\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo1`] module"]
#[doc(alias = "READFIFO1")]
pub type Readfifo1 = crate::Reg<readfifo1::Readfifo1Spec>;
#[doc = "EPI Read FIFO 1"]
pub mod readfifo1;
#[doc = "READFIFO2 (rw) register accessor: EPI Read FIFO 2\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo2`] module"]
#[doc(alias = "READFIFO2")]
pub type Readfifo2 = crate::Reg<readfifo2::Readfifo2Spec>;
#[doc = "EPI Read FIFO 2"]
pub mod readfifo2;
#[doc = "READFIFO3 (rw) register accessor: EPI Read FIFO 3\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo3`] module"]
#[doc(alias = "READFIFO3")]
pub type Readfifo3 = crate::Reg<readfifo3::Readfifo3Spec>;
#[doc = "EPI Read FIFO 3"]
pub mod readfifo3;
#[doc = "READFIFO4 (rw) register accessor: EPI Read FIFO 4\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo4`] module"]
#[doc(alias = "READFIFO4")]
pub type Readfifo4 = crate::Reg<readfifo4::Readfifo4Spec>;
#[doc = "EPI Read FIFO 4"]
pub mod readfifo4;
#[doc = "READFIFO5 (rw) register accessor: EPI Read FIFO 5\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo5`] module"]
#[doc(alias = "READFIFO5")]
pub type Readfifo5 = crate::Reg<readfifo5::Readfifo5Spec>;
#[doc = "EPI Read FIFO 5"]
pub mod readfifo5;
#[doc = "READFIFO6 (rw) register accessor: EPI Read FIFO 6\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo6`] module"]
#[doc(alias = "READFIFO6")]
pub type Readfifo6 = crate::Reg<readfifo6::Readfifo6Spec>;
#[doc = "EPI Read FIFO 6"]
pub mod readfifo6;
#[doc = "READFIFO7 (rw) register accessor: EPI Read FIFO 7\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readfifo7`] module"]
#[doc(alias = "READFIFO7")]
pub type Readfifo7 = crate::Reg<readfifo7::Readfifo7Spec>;
#[doc = "EPI Read FIFO 7"]
pub mod readfifo7;
#[doc = "FIFOLVL (rw) register accessor: EPI FIFO Level Selects\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolvl`] module"]
#[doc(alias = "FIFOLVL")]
pub type Fifolvl = crate::Reg<fifolvl::FifolvlSpec>;
#[doc = "EPI FIFO Level Selects"]
pub mod fifolvl;
#[doc = "WFIFOCNT (rw) register accessor: EPI Write FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifocnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifocnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfifocnt`] module"]
#[doc(alias = "WFIFOCNT")]
pub type Wfifocnt = crate::Reg<wfifocnt::WfifocntSpec>;
#[doc = "EPI Write FIFO Count"]
pub mod wfifocnt;
#[doc = "DMATXCNT (rw) register accessor: EPI DMA Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxcnt`] module"]
#[doc(alias = "DMATXCNT")]
pub type Dmatxcnt = crate::Reg<dmatxcnt::DmatxcntSpec>;
#[doc = "EPI DMA Transmit Count"]
pub mod dmatxcnt;
#[doc = "IM (rw) register accessor: EPI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`] module"]
#[doc(alias = "IM")]
pub type Im = crate::Reg<im::ImSpec>;
#[doc = "EPI Interrupt Mask"]
pub mod im;
#[doc = "RIS (rw) register accessor: EPI Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "EPI Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: EPI Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "EPI Masked Interrupt Status"]
pub mod mis;
#[doc = "EISC (rw) register accessor: EPI Error and Interrupt Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`eisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eisc`] module"]
#[doc(alias = "EISC")]
pub type Eisc = crate::Reg<eisc::EiscSpec>;
#[doc = "EPI Error and Interrupt Status and Clear"]
pub mod eisc;
#[doc = "HB8CFG3 (rw) register accessor: EPI Host-Bus 8 Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8cfg3`] module"]
#[doc(alias = "HB8CFG3")]
pub type Hb8cfg3 = crate::Reg<hb8cfg3::Hb8cfg3Spec>;
#[doc = "EPI Host-Bus 8 Configuration 3"]
pub mod hb8cfg3;
#[doc = "HB16CFG3 (rw) register accessor: EPI Host-Bus 16 Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16cfg3`] module"]
#[doc(alias = "HB16CFG3")]
pub type Hb16cfg3 = crate::Reg<hb16cfg3::Hb16cfg3Spec>;
#[doc = "EPI Host-Bus 16 Configuration 3"]
pub mod hb16cfg3;
#[doc = "HB8CFG4 (rw) register accessor: EPI Host-Bus 8 Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8cfg4`] module"]
#[doc(alias = "HB8CFG4")]
pub type Hb8cfg4 = crate::Reg<hb8cfg4::Hb8cfg4Spec>;
#[doc = "EPI Host-Bus 8 Configuration 4"]
pub mod hb8cfg4;
#[doc = "HB16CFG4 (rw) register accessor: EPI Host-Bus 16 Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16cfg4`] module"]
#[doc(alias = "HB16CFG4")]
pub type Hb16cfg4 = crate::Reg<hb16cfg4::Hb16cfg4Spec>;
#[doc = "EPI Host-Bus 16 Configuration 4"]
pub mod hb16cfg4;
#[doc = "HB8TIME (rw) register accessor: EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8time`] module"]
#[doc(alias = "HB8TIME")]
pub type Hb8time = crate::Reg<hb8time::Hb8timeSpec>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time;
#[doc = "HB16TIME (rw) register accessor: EPI Host-Bus 16 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16time`] module"]
#[doc(alias = "HB16TIME")]
pub type Hb16time = crate::Reg<hb16time::Hb16timeSpec>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time;
#[doc = "HB8TIME2 (rw) register accessor: EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8time2`] module"]
#[doc(alias = "HB8TIME2")]
pub type Hb8time2 = crate::Reg<hb8time2::Hb8time2Spec>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time2;
#[doc = "HB16TIME2 (rw) register accessor: EPI Host-Bus 16 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16time2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16time2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16time2`] module"]
#[doc(alias = "HB16TIME2")]
pub type Hb16time2 = crate::Reg<hb16time2::Hb16time2Spec>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time2;
#[doc = "HB8TIME3 (rw) register accessor: EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8time3`] module"]
#[doc(alias = "HB8TIME3")]
pub type Hb8time3 = crate::Reg<hb8time3::Hb8time3Spec>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time3;
#[doc = "HB16TIME3 (rw) register accessor: EPI Host-Bus 16 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16time3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16time3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16time3`] module"]
#[doc(alias = "HB16TIME3")]
pub type Hb16time3 = crate::Reg<hb16time3::Hb16time3Spec>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time3;
#[doc = "HB8TIME4 (rw) register accessor: EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb8time4`] module"]
#[doc(alias = "HB8TIME4")]
pub type Hb8time4 = crate::Reg<hb8time4::Hb8time4Spec>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time4;
#[doc = "HB16TIME4 (rw) register accessor: EPI Host-Bus 16 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16time4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16time4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hb16time4`] module"]
#[doc(alias = "HB16TIME4")]
pub type Hb16time4 = crate::Reg<hb16time4::Hb16time4Spec>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time4;
#[doc = "HBPSRAM (rw) register accessor: EPI Host-Bus PSRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`hbpsram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbpsram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbpsram`] module"]
#[doc(alias = "HBPSRAM")]
pub type Hbpsram = crate::Reg<hbpsram::HbpsramSpec>;
#[doc = "EPI Host-Bus PSRAM"]
pub mod hbpsram;
