#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x07f8],
    clkovr: Clkovr,
    _reserved2: [u8; 0x04],
    pdbgctl: Pdbgctl,
    _reserved3: [u8; 0x04],
    cpu_int: CpuInt,
    _reserved4: [u8; 0x04],
    gen_event0: GenEvent0,
    _reserved5: [u8; 0x04],
    gen_event1: GenEvent1,
    _reserved6: [u8; 0x34],
    evt_mode: EvtMode,
    _reserved7: [u8; 0x18],
    desc: Desc,
    _reserved8: [u8; 0x0100],
    dout3_0: Dout3_0,
    dout7_4: Dout7_4,
    dout11_8: Dout11_8,
    dout15_12: Dout15_12,
    dout19_16: Dout19_16,
    dout23_20: Dout23_20,
    dout27_24: Dout27_24,
    dout31_28: Dout31_28,
    _reserved16: [u8; 0x60],
    dout31_0: Dout31_0,
    _reserved17: [u8; 0x0c],
    doutset31_0: Doutset31_0,
    _reserved18: [u8; 0x0c],
    doutclr31_0: Doutclr31_0,
    _reserved19: [u8; 0x0c],
    douttgl31_0: Douttgl31_0,
    _reserved20: [u8; 0x0c],
    doe31_0: Doe31_0,
    _reserved21: [u8; 0x0c],
    doeset31_0: Doeset31_0,
    _reserved22: [u8; 0x0c],
    doeclr31_0: Doeclr31_0,
    _reserved23: [u8; 0x1c],
    din3_0: Din3_0,
    din7_4: Din7_4,
    din11_8: Din11_8,
    din15_12: Din15_12,
    din19_16: Din19_16,
    din23_20: Din23_20,
    din27_24: Din27_24,
    din31_28: Din31_28,
    _reserved31: [u8; 0x60],
    din31_0: Din31_0,
    _reserved32: [u8; 0x0c],
    polarity15_0: Polarity15_0,
    _reserved33: [u8; 0x0c],
    polarity31_16: Polarity31_16,
    _reserved34: [u8; 0x5c],
    ctl: Ctl,
    fastwake: Fastwake,
    _reserved36: [u8; 0xf8],
    sub0cfg: Sub0cfg,
    _reserved37: [u8; 0x04],
    filteren15_0: Filteren15_0,
    filteren31_16: Filteren31_16,
    dmamask: Dmamask,
    _reserved40: [u8; 0x0c],
    sub1cfg: Sub1cfg,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1010 - Clock Override"]
    #[inline(always)]
    pub const fn clkovr(&self) -> &Clkovr {
        &self.clkovr
    }
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
    #[doc = "0x1050..0x107c - GEN_EVENT0"]
    #[inline(always)]
    pub const fn gen_event0(&self) -> &GenEvent0 {
        &self.gen_event0
    }
    #[doc = "0x1080..0x10ac - GEN_EVENT1"]
    #[inline(always)]
    pub const fn gen_event1(&self) -> &GenEvent1 {
        &self.gen_event1
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
    #[doc = "0x1200 - Data output 3 to 0"]
    #[inline(always)]
    pub const fn dout3_0(&self) -> &Dout3_0 {
        &self.dout3_0
    }
    #[doc = "0x1204 - Data output 7 to 4"]
    #[inline(always)]
    pub const fn dout7_4(&self) -> &Dout7_4 {
        &self.dout7_4
    }
    #[doc = "0x1208 - Data output 11 to 8"]
    #[inline(always)]
    pub const fn dout11_8(&self) -> &Dout11_8 {
        &self.dout11_8
    }
    #[doc = "0x120c - Data output 15 to 12"]
    #[inline(always)]
    pub const fn dout15_12(&self) -> &Dout15_12 {
        &self.dout15_12
    }
    #[doc = "0x1210 - Data output 19 to 16"]
    #[inline(always)]
    pub const fn dout19_16(&self) -> &Dout19_16 {
        &self.dout19_16
    }
    #[doc = "0x1214 - Data output 23 to 20"]
    #[inline(always)]
    pub const fn dout23_20(&self) -> &Dout23_20 {
        &self.dout23_20
    }
    #[doc = "0x1218 - Data output 27 to 24"]
    #[inline(always)]
    pub const fn dout27_24(&self) -> &Dout27_24 {
        &self.dout27_24
    }
    #[doc = "0x121c - Data output 31 to 28"]
    #[inline(always)]
    pub const fn dout31_28(&self) -> &Dout31_28 {
        &self.dout31_28
    }
    #[doc = "0x1280 - Data output 31 to 0"]
    #[inline(always)]
    pub const fn dout31_0(&self) -> &Dout31_0 {
        &self.dout31_0
    }
    #[doc = "0x1290 - Data output set 31 to 0"]
    #[inline(always)]
    pub const fn doutset31_0(&self) -> &Doutset31_0 {
        &self.doutset31_0
    }
    #[doc = "0x12a0 - Data output clear 31 to 0"]
    #[inline(always)]
    pub const fn doutclr31_0(&self) -> &Doutclr31_0 {
        &self.doutclr31_0
    }
    #[doc = "0x12b0 - Data output toggle 31 to 0"]
    #[inline(always)]
    pub const fn douttgl31_0(&self) -> &Douttgl31_0 {
        &self.douttgl31_0
    }
    #[doc = "0x12c0 - Data output enable 31 to 0"]
    #[inline(always)]
    pub const fn doe31_0(&self) -> &Doe31_0 {
        &self.doe31_0
    }
    #[doc = "0x12d0 - Data output enable set 31 to 0"]
    #[inline(always)]
    pub const fn doeset31_0(&self) -> &Doeset31_0 {
        &self.doeset31_0
    }
    #[doc = "0x12e0 - Data output enable clear 31 to 0"]
    #[inline(always)]
    pub const fn doeclr31_0(&self) -> &Doeclr31_0 {
        &self.doeclr31_0
    }
    #[doc = "0x1300 - Data input 3 to 0"]
    #[inline(always)]
    pub const fn din3_0(&self) -> &Din3_0 {
        &self.din3_0
    }
    #[doc = "0x1304 - Data input 7 to 4"]
    #[inline(always)]
    pub const fn din7_4(&self) -> &Din7_4 {
        &self.din7_4
    }
    #[doc = "0x1308 - Data input 11 to 8"]
    #[inline(always)]
    pub const fn din11_8(&self) -> &Din11_8 {
        &self.din11_8
    }
    #[doc = "0x130c - Data input 15 to 12"]
    #[inline(always)]
    pub const fn din15_12(&self) -> &Din15_12 {
        &self.din15_12
    }
    #[doc = "0x1310 - Data input 19 to 16"]
    #[inline(always)]
    pub const fn din19_16(&self) -> &Din19_16 {
        &self.din19_16
    }
    #[doc = "0x1314 - Data input 23 to 20"]
    #[inline(always)]
    pub const fn din23_20(&self) -> &Din23_20 {
        &self.din23_20
    }
    #[doc = "0x1318 - Data input 27 to 24"]
    #[inline(always)]
    pub const fn din27_24(&self) -> &Din27_24 {
        &self.din27_24
    }
    #[doc = "0x131c - Data input 31 to 28"]
    #[inline(always)]
    pub const fn din31_28(&self) -> &Din31_28 {
        &self.din31_28
    }
    #[doc = "0x1380 - Data input 31 to 0"]
    #[inline(always)]
    pub const fn din31_0(&self) -> &Din31_0 {
        &self.din31_0
    }
    #[doc = "0x1390 - Polarity 15 to 0"]
    #[inline(always)]
    pub const fn polarity15_0(&self) -> &Polarity15_0 {
        &self.polarity15_0
    }
    #[doc = "0x13a0 - Polarity 31 to 16"]
    #[inline(always)]
    pub const fn polarity31_16(&self) -> &Polarity31_16 {
        &self.polarity31_16
    }
    #[doc = "0x1400 - FAST WAKE GLOBAL EN"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x1404 - FAST WAKE ENABLE"]
    #[inline(always)]
    pub const fn fastwake(&self) -> &Fastwake {
        &self.fastwake
    }
    #[doc = "0x1500 - Subscriber 0 configuration"]
    #[inline(always)]
    pub const fn sub0cfg(&self) -> &Sub0cfg {
        &self.sub0cfg
    }
    #[doc = "0x1508 - Filter Enable 15 to 0"]
    #[inline(always)]
    pub const fn filteren15_0(&self) -> &Filteren15_0 {
        &self.filteren15_0
    }
    #[doc = "0x150c - Filter Enable 31 to 16"]
    #[inline(always)]
    pub const fn filteren31_16(&self) -> &Filteren31_16 {
        &self.filteren31_16
    }
    #[doc = "0x1510 - DMA Write MASK"]
    #[inline(always)]
    pub const fn dmamask(&self) -> &Dmamask {
        &self.dmamask
    }
    #[doc = "0x1520 - Subscriber 1 configuration"]
    #[inline(always)]
    pub const fn sub1cfg(&self) -> &Sub1cfg {
        &self.sub1cfg
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "CLKOVR (rw) register accessor: Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`clkovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkovr`] module"]
#[doc(alias = "CLKOVR")]
pub type Clkovr = crate::Reg<clkovr::ClkovrSpec>;
#[doc = "Clock Override"]
pub mod clkovr;
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
#[doc = "GEN_EVENT0"]
pub use self::gen_event0::GenEvent0;
#[doc = r"Cluster"]
#[doc = "GEN_EVENT0"]
pub mod gen_event0;
#[doc = "GEN_EVENT1"]
pub use self::gen_event1::GenEvent1;
#[doc = r"Cluster"]
#[doc = "GEN_EVENT1"]
pub mod gen_event1;
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
#[doc = "DOUT3_0 (w) register accessor: Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout3_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout3_0`] module"]
#[doc(alias = "DOUT3_0")]
pub type Dout3_0 = crate::Reg<dout3_0::Dout3_0Spec>;
#[doc = "Data output 3 to 0"]
pub mod dout3_0;
#[doc = "DOUT7_4 (w) register accessor: Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout7_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout7_4`] module"]
#[doc(alias = "DOUT7_4")]
pub type Dout7_4 = crate::Reg<dout7_4::Dout7_4Spec>;
#[doc = "Data output 7 to 4"]
pub mod dout7_4;
#[doc = "DOUT11_8 (w) register accessor: Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout11_8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout11_8`] module"]
#[doc(alias = "DOUT11_8")]
pub type Dout11_8 = crate::Reg<dout11_8::Dout11_8Spec>;
#[doc = "Data output 11 to 8"]
pub mod dout11_8;
#[doc = "DOUT15_12 (w) register accessor: Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout15_12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout15_12`] module"]
#[doc(alias = "DOUT15_12")]
pub type Dout15_12 = crate::Reg<dout15_12::Dout15_12Spec>;
#[doc = "Data output 15 to 12"]
pub mod dout15_12;
#[doc = "DOUT19_16 (w) register accessor: Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout19_16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout19_16`] module"]
#[doc(alias = "DOUT19_16")]
pub type Dout19_16 = crate::Reg<dout19_16::Dout19_16Spec>;
#[doc = "Data output 19 to 16"]
pub mod dout19_16;
#[doc = "DOUT23_20 (w) register accessor: Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout23_20::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout23_20`] module"]
#[doc(alias = "DOUT23_20")]
pub type Dout23_20 = crate::Reg<dout23_20::Dout23_20Spec>;
#[doc = "Data output 23 to 20"]
pub mod dout23_20;
#[doc = "DOUT27_24 (w) register accessor: Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout27_24::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout27_24`] module"]
#[doc(alias = "DOUT27_24")]
pub type Dout27_24 = crate::Reg<dout27_24::Dout27_24Spec>;
#[doc = "Data output 27 to 24"]
pub mod dout27_24;
#[doc = "DOUT31_28 (w) register accessor: Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout31_28::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout31_28`] module"]
#[doc(alias = "DOUT31_28")]
pub type Dout31_28 = crate::Reg<dout31_28::Dout31_28Spec>;
#[doc = "Data output 31 to 28"]
pub mod dout31_28;
#[doc = "DOUT31_0 (rw) register accessor: Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dout31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout31_0`] module"]
#[doc(alias = "DOUT31_0")]
pub type Dout31_0 = crate::Reg<dout31_0::Dout31_0Spec>;
#[doc = "Data output 31 to 0"]
pub mod dout31_0;
#[doc = "DOUTSET31_0 (w) register accessor: Data output set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutset31_0`] module"]
#[doc(alias = "DOUTSET31_0")]
pub type Doutset31_0 = crate::Reg<doutset31_0::Doutset31_0Spec>;
#[doc = "Data output set 31 to 0"]
pub mod doutset31_0;
#[doc = "DOUTCLR31_0 (w) register accessor: Data output clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutclr31_0`] module"]
#[doc(alias = "DOUTCLR31_0")]
pub type Doutclr31_0 = crate::Reg<doutclr31_0::Doutclr31_0Spec>;
#[doc = "Data output clear 31 to 0"]
pub mod doutclr31_0;
#[doc = "DOUTTGL31_0 (w) register accessor: Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`douttgl31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl31_0`] module"]
#[doc(alias = "DOUTTGL31_0")]
pub type Douttgl31_0 = crate::Reg<douttgl31_0::Douttgl31_0Spec>;
#[doc = "Data output toggle 31 to 0"]
pub mod douttgl31_0;
#[doc = "DOE31_0 (rw) register accessor: Data output enable 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`doe31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doe31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe31_0`] module"]
#[doc(alias = "DOE31_0")]
pub type Doe31_0 = crate::Reg<doe31_0::Doe31_0Spec>;
#[doc = "Data output enable 31 to 0"]
pub mod doe31_0;
#[doc = "DOESET31_0 (w) register accessor: Data output enable set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeset31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeset31_0`] module"]
#[doc(alias = "DOESET31_0")]
pub type Doeset31_0 = crate::Reg<doeset31_0::Doeset31_0Spec>;
#[doc = "Data output enable set 31 to 0"]
pub mod doeset31_0;
#[doc = "DOECLR31_0 (w) register accessor: Data output enable clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeclr31_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeclr31_0`] module"]
#[doc(alias = "DOECLR31_0")]
pub type Doeclr31_0 = crate::Reg<doeclr31_0::Doeclr31_0Spec>;
#[doc = "Data output enable clear 31 to 0"]
pub mod doeclr31_0;
#[doc = "DIN3_0 (r) register accessor: Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din3_0`] module"]
#[doc(alias = "DIN3_0")]
pub type Din3_0 = crate::Reg<din3_0::Din3_0Spec>;
#[doc = "Data input 3 to 0"]
pub mod din3_0;
#[doc = "DIN7_4 (r) register accessor: Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`din7_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din7_4`] module"]
#[doc(alias = "DIN7_4")]
pub type Din7_4 = crate::Reg<din7_4::Din7_4Spec>;
#[doc = "Data input 7 to 4"]
pub mod din7_4;
#[doc = "DIN11_8 (r) register accessor: Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`din11_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din11_8`] module"]
#[doc(alias = "DIN11_8")]
pub type Din11_8 = crate::Reg<din11_8::Din11_8Spec>;
#[doc = "Data input 11 to 8"]
pub mod din11_8;
#[doc = "DIN15_12 (r) register accessor: Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`din15_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din15_12`] module"]
#[doc(alias = "DIN15_12")]
pub type Din15_12 = crate::Reg<din15_12::Din15_12Spec>;
#[doc = "Data input 15 to 12"]
pub mod din15_12;
#[doc = "DIN19_16 (r) register accessor: Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`din19_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din19_16`] module"]
#[doc(alias = "DIN19_16")]
pub type Din19_16 = crate::Reg<din19_16::Din19_16Spec>;
#[doc = "Data input 19 to 16"]
pub mod din19_16;
#[doc = "DIN23_20 (r) register accessor: Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`din23_20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din23_20`] module"]
#[doc(alias = "DIN23_20")]
pub type Din23_20 = crate::Reg<din23_20::Din23_20Spec>;
#[doc = "Data input 23 to 20"]
pub mod din23_20;
#[doc = "DIN27_24 (r) register accessor: Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`din27_24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din27_24`] module"]
#[doc(alias = "DIN27_24")]
pub type Din27_24 = crate::Reg<din27_24::Din27_24Spec>;
#[doc = "Data input 27 to 24"]
pub mod din27_24;
#[doc = "DIN31_28 (r) register accessor: Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din31_28`] module"]
#[doc(alias = "DIN31_28")]
pub type Din31_28 = crate::Reg<din31_28::Din31_28Spec>;
#[doc = "Data input 31 to 28"]
pub mod din31_28;
#[doc = "DIN31_0 (r) register accessor: Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din31_0`] module"]
#[doc(alias = "DIN31_0")]
pub type Din31_0 = crate::Reg<din31_0::Din31_0Spec>;
#[doc = "Data input 31 to 0"]
pub mod din31_0;
#[doc = "POLARITY15_0 (rw) register accessor: Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity15_0`] module"]
#[doc(alias = "POLARITY15_0")]
pub type Polarity15_0 = crate::Reg<polarity15_0::Polarity15_0Spec>;
#[doc = "Polarity 15 to 0"]
pub mod polarity15_0;
#[doc = "POLARITY31_16 (rw) register accessor: Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity31_16`] module"]
#[doc(alias = "POLARITY31_16")]
pub type Polarity31_16 = crate::Reg<polarity31_16::Polarity31_16Spec>;
#[doc = "Polarity 31 to 16"]
pub mod polarity31_16;
#[doc = "CTL (rw) register accessor: FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "FAST WAKE GLOBAL EN"]
pub mod ctl;
#[doc = "FASTWAKE (rw) register accessor: FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`fastwake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fastwake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fastwake`] module"]
#[doc(alias = "FASTWAKE")]
pub type Fastwake = crate::Reg<fastwake::FastwakeSpec>;
#[doc = "FAST WAKE ENABLE"]
pub mod fastwake;
#[doc = "SUB0CFG (rw) register accessor: Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sub0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub0cfg`] module"]
#[doc(alias = "SUB0CFG")]
pub type Sub0cfg = crate::Reg<sub0cfg::Sub0cfgSpec>;
#[doc = "Subscriber 0 configuration"]
pub mod sub0cfg;
#[doc = "FILTEREN15_0 (rw) register accessor: Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren15_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren15_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filteren15_0`] module"]
#[doc(alias = "FILTEREN15_0")]
pub type Filteren15_0 = crate::Reg<filteren15_0::Filteren15_0Spec>;
#[doc = "Filter Enable 15 to 0"]
pub mod filteren15_0;
#[doc = "FILTEREN31_16 (rw) register accessor: Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren31_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren31_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filteren31_16`] module"]
#[doc(alias = "FILTEREN31_16")]
pub type Filteren31_16 = crate::Reg<filteren31_16::Filteren31_16Spec>;
#[doc = "Filter Enable 31 to 16"]
pub mod filteren31_16;
#[doc = "DMAMASK (rw) register accessor: DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamask`] module"]
#[doc(alias = "DMAMASK")]
pub type Dmamask = crate::Reg<dmamask::DmamaskSpec>;
#[doc = "DMA Write MASK"]
pub mod dmamask;
#[doc = "SUB1CFG (rw) register accessor: Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sub1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub1cfg`] module"]
#[doc(alias = "SUB1CFG")]
pub type Sub1cfg = crate::Reg<sub1cfg::Sub1cfgSpec>;
#[doc = "Subscriber 1 configuration"]
pub mod sub1cfg;
