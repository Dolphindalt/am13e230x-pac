#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x07e8],
    adcctl1: Adcctl1,
    adcctl2: Adcctl2,
    _reserved3: [u8; 0x08],
    adcintsel: Adcintsel,
    adcdmaintsel: Adcdmaintsel,
    adcrawintflg: Adcrawintflg,
    adcintflg: Adcintflg,
    adcintflgfrc: Adcintflgfrc,
    adcintflgclr: Adcintflgclr,
    adcintovf: Adcintovf,
    adcintovfclr: Adcintovfclr,
    _reserved11: [u8; 0x0c],
    adcsocflg1: Adcsocflg1,
    _reserved12: [u8; 0x04],
    adcsocovf1: Adcsocovf1,
    adcsocovfclr1: Adcsocovfclr1,
    adcsoc0ctl: Adcsoc0ctl,
    adcsoc1ctl: Adcsoc1ctl,
    adcsoc2ctl: Adcsoc2ctl,
    adcsoc3ctl: Adcsoc3ctl,
    adcsoc4ctl: Adcsoc4ctl,
    adcsoc5ctl: Adcsoc5ctl,
    adcsoc6ctl: Adcsoc6ctl,
    adcsoc7ctl: Adcsoc7ctl,
    adcsoc8ctl: Adcsoc8ctl,
    adcsoc9ctl: Adcsoc9ctl,
    adcsoc10ctl: Adcsoc10ctl,
    adcsoc11ctl: Adcsoc11ctl,
    adcsoc12ctl: Adcsoc12ctl,
    adcsoc13ctl: Adcsoc13ctl,
    adcsoc14ctl: Adcsoc14ctl,
    adcsoc15ctl: Adcsoc15ctl,
    _reserved30: [u8; 0x40],
    adcevtstat: Adcevtstat,
    adcevtclr: Adcevtclr,
    adcevtsel: Adcevtsel,
    adcevtintsel: Adcevtintsel,
    _reserved34: [u8; 0x08],
    adcrev: Adcrev,
    adcofftrim: Adcofftrim,
    _reserved36: [u8; 0x14],
    adcppb1config: Adcppb1config,
    _reserved37: [u8; 0x0c],
    adcppb1triphi: Adcppb1triphi,
    adcppb1triplo: Adcppb1triplo,
    _reserved39: [u8; 0x08],
    adcppb2config: Adcppb2config,
    _reserved40: [u8; 0x0c],
    adcppb2triphi: Adcppb2triphi,
    adcppb2triplo: Adcppb2triplo,
    _reserved42: [u8; 0x08],
    adcppb3config: Adcppb3config,
    _reserved43: [u8; 0x0c],
    adcppb3triphi: Adcppb3triphi,
    adcppb3triplo: Adcppb3triplo,
    _reserved45: [u8; 0x08],
    adcppb4config: Adcppb4config,
    _reserved46: [u8; 0x0c],
    adcppb4triphi: Adcppb4triphi,
    adcppb4triplo: Adcppb4triplo,
    _reserved48: [u8; 0x08],
    adcintcycle: Adcintcycle,
    _reserved49: [u8; 0x18],
    adcrev2: Adcrev2,
    _reserved50: [u8; 0x60],
    adcppb1limit: Adcppb1limit,
    adcppb1pcount: Adcppb1pcount,
    adcppb1config2: Adcppb1config2,
    adcppb1psum: Adcppb1psum,
    _reserved54: [u8; 0x30],
    adcppb2limit: Adcppb2limit,
    adcppb2pcount: Adcppb2pcount,
    adcppb2config2: Adcppb2config2,
    adcppb2psum: Adcppb2psum,
    _reserved58: [u8; 0x30],
    adcppb3limit: Adcppb3limit,
    adcppb3pcount: Adcppb3pcount,
    adcppb3config2: Adcppb3config2,
    adcppb3psum: Adcppb3psum,
    _reserved62: [u8; 0x30],
    adcppb4limit: Adcppb4limit,
    adcppb4pcount: Adcppb4pcount,
    adcppb4config2: Adcppb4config2,
    adcppb4psum: Adcppb4psum,
    _reserved66: [u8; 0x50],
    adcseqctl: Adcseqctl,
    adcseq1config: Adcseq1config,
    adcseq2config: Adcseq2config,
    adcseq3config: Adcseq3config,
    adcseq4config: Adcseq4config,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1000 - ADC Control 1 Register"]
    #[inline(always)]
    pub const fn adcctl1(&self) -> &Adcctl1 {
        &self.adcctl1
    }
    #[doc = "0x1004 - ADC Control 2 Register"]
    #[inline(always)]
    pub const fn adcctl2(&self) -> &Adcctl2 {
        &self.adcctl2
    }
    #[doc = "0x1010 - ADC Interrupt 1, 2, 3 and 4 Selection Register"]
    #[inline(always)]
    pub const fn adcintsel(&self) -> &Adcintsel {
        &self.adcintsel
    }
    #[doc = "0x1014 - ADC DMA Interrupt 1, 2, 3 and 4 Selection Register"]
    #[inline(always)]
    pub const fn adcdmaintsel(&self) -> &Adcdmaintsel {
        &self.adcdmaintsel
    }
    #[doc = "0x1018 - ADC Raw Interrupt Flag Register"]
    #[inline(always)]
    pub const fn adcrawintflg(&self) -> &Adcrawintflg {
        &self.adcrawintflg
    }
    #[doc = "0x101c - ADC Interrupt Flag Register"]
    #[inline(always)]
    pub const fn adcintflg(&self) -> &Adcintflg {
        &self.adcintflg
    }
    #[doc = "0x1020 - ADC Interrupt Flag Force Register"]
    #[inline(always)]
    pub const fn adcintflgfrc(&self) -> &Adcintflgfrc {
        &self.adcintflgfrc
    }
    #[doc = "0x1024 - ADC Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn adcintflgclr(&self) -> &Adcintflgclr {
        &self.adcintflgclr
    }
    #[doc = "0x1028 - ADC Interrupt Overflow Register"]
    #[inline(always)]
    pub const fn adcintovf(&self) -> &Adcintovf {
        &self.adcintovf
    }
    #[doc = "0x102c - ADC Interrupt Overflow Clear Register"]
    #[inline(always)]
    pub const fn adcintovfclr(&self) -> &Adcintovfclr {
        &self.adcintovfclr
    }
    #[doc = "0x103c - ADC SOC Flag 1 Register"]
    #[inline(always)]
    pub const fn adcsocflg1(&self) -> &Adcsocflg1 {
        &self.adcsocflg1
    }
    #[doc = "0x1044 - ADC SOC Overflow 1 Register"]
    #[inline(always)]
    pub const fn adcsocovf1(&self) -> &Adcsocovf1 {
        &self.adcsocovf1
    }
    #[doc = "0x1048 - ADC SOC Overflow Clear 1 Register"]
    #[inline(always)]
    pub const fn adcsocovfclr1(&self) -> &Adcsocovfclr1 {
        &self.adcsocovfclr1
    }
    #[doc = "0x104c - ADC SOC0 Control Register"]
    #[inline(always)]
    pub const fn adcsoc0ctl(&self) -> &Adcsoc0ctl {
        &self.adcsoc0ctl
    }
    #[doc = "0x1050 - ADC SOC1 Control Register"]
    #[inline(always)]
    pub const fn adcsoc1ctl(&self) -> &Adcsoc1ctl {
        &self.adcsoc1ctl
    }
    #[doc = "0x1054 - ADC SOC2 Control Register"]
    #[inline(always)]
    pub const fn adcsoc2ctl(&self) -> &Adcsoc2ctl {
        &self.adcsoc2ctl
    }
    #[doc = "0x1058 - ADC SOC3 Control Register"]
    #[inline(always)]
    pub const fn adcsoc3ctl(&self) -> &Adcsoc3ctl {
        &self.adcsoc3ctl
    }
    #[doc = "0x105c - ADC SOC4 Control Register"]
    #[inline(always)]
    pub const fn adcsoc4ctl(&self) -> &Adcsoc4ctl {
        &self.adcsoc4ctl
    }
    #[doc = "0x1060 - ADC SOC5 Control Register"]
    #[inline(always)]
    pub const fn adcsoc5ctl(&self) -> &Adcsoc5ctl {
        &self.adcsoc5ctl
    }
    #[doc = "0x1064 - ADC SOC6 Control Register"]
    #[inline(always)]
    pub const fn adcsoc6ctl(&self) -> &Adcsoc6ctl {
        &self.adcsoc6ctl
    }
    #[doc = "0x1068 - ADC SOC7 Control Register"]
    #[inline(always)]
    pub const fn adcsoc7ctl(&self) -> &Adcsoc7ctl {
        &self.adcsoc7ctl
    }
    #[doc = "0x106c - ADC SOC8 Control Register"]
    #[inline(always)]
    pub const fn adcsoc8ctl(&self) -> &Adcsoc8ctl {
        &self.adcsoc8ctl
    }
    #[doc = "0x1070 - ADC SOC9 Control Register"]
    #[inline(always)]
    pub const fn adcsoc9ctl(&self) -> &Adcsoc9ctl {
        &self.adcsoc9ctl
    }
    #[doc = "0x1074 - ADC SOC10 Control Register"]
    #[inline(always)]
    pub const fn adcsoc10ctl(&self) -> &Adcsoc10ctl {
        &self.adcsoc10ctl
    }
    #[doc = "0x1078 - ADC SOC11 Control Register"]
    #[inline(always)]
    pub const fn adcsoc11ctl(&self) -> &Adcsoc11ctl {
        &self.adcsoc11ctl
    }
    #[doc = "0x107c - ADC SOC12 Control Register"]
    #[inline(always)]
    pub const fn adcsoc12ctl(&self) -> &Adcsoc12ctl {
        &self.adcsoc12ctl
    }
    #[doc = "0x1080 - ADC SOC13 Control Register"]
    #[inline(always)]
    pub const fn adcsoc13ctl(&self) -> &Adcsoc13ctl {
        &self.adcsoc13ctl
    }
    #[doc = "0x1084 - ADC SOC14 Control Register"]
    #[inline(always)]
    pub const fn adcsoc14ctl(&self) -> &Adcsoc14ctl {
        &self.adcsoc14ctl
    }
    #[doc = "0x1088 - ADC SOC15 Control Register"]
    #[inline(always)]
    pub const fn adcsoc15ctl(&self) -> &Adcsoc15ctl {
        &self.adcsoc15ctl
    }
    #[doc = "0x10cc - ADC Event Status Register"]
    #[inline(always)]
    pub const fn adcevtstat(&self) -> &Adcevtstat {
        &self.adcevtstat
    }
    #[doc = "0x10d0 - ADC Event Clear Register"]
    #[inline(always)]
    pub const fn adcevtclr(&self) -> &Adcevtclr {
        &self.adcevtclr
    }
    #[doc = "0x10d4 - ADC Event Selection Register"]
    #[inline(always)]
    pub const fn adcevtsel(&self) -> &Adcevtsel {
        &self.adcevtsel
    }
    #[doc = "0x10d8 - ADC Event Interrupt Selection Register"]
    #[inline(always)]
    pub const fn adcevtintsel(&self) -> &Adcevtintsel {
        &self.adcevtintsel
    }
    #[doc = "0x10e4 - ADC Revision Register"]
    #[inline(always)]
    pub const fn adcrev(&self) -> &Adcrev {
        &self.adcrev
    }
    #[doc = "0x10e8 - ADC Offset Trim Register 1"]
    #[inline(always)]
    pub const fn adcofftrim(&self) -> &Adcofftrim {
        &self.adcofftrim
    }
    #[doc = "0x1100 - ADC PPB1 Config Register"]
    #[inline(always)]
    pub const fn adcppb1config(&self) -> &Adcppb1config {
        &self.adcppb1config
    }
    #[doc = "0x1110 - ADC PPB1 Trip High Register"]
    #[inline(always)]
    pub const fn adcppb1triphi(&self) -> &Adcppb1triphi {
        &self.adcppb1triphi
    }
    #[doc = "0x1114 - ADC PPB1 Trip Low/Trigger Time Stamp Register"]
    #[inline(always)]
    pub const fn adcppb1triplo(&self) -> &Adcppb1triplo {
        &self.adcppb1triplo
    }
    #[doc = "0x1120 - ADC PPB2 Config Register"]
    #[inline(always)]
    pub const fn adcppb2config(&self) -> &Adcppb2config {
        &self.adcppb2config
    }
    #[doc = "0x1130 - ADC PPB2 Trip High Register"]
    #[inline(always)]
    pub const fn adcppb2triphi(&self) -> &Adcppb2triphi {
        &self.adcppb2triphi
    }
    #[doc = "0x1134 - ADC PPB2 Trip Low/Trigger Time Stamp Register"]
    #[inline(always)]
    pub const fn adcppb2triplo(&self) -> &Adcppb2triplo {
        &self.adcppb2triplo
    }
    #[doc = "0x1140 - ADC PPB3 Config Register"]
    #[inline(always)]
    pub const fn adcppb3config(&self) -> &Adcppb3config {
        &self.adcppb3config
    }
    #[doc = "0x1150 - ADC PPB3 Trip High Register"]
    #[inline(always)]
    pub const fn adcppb3triphi(&self) -> &Adcppb3triphi {
        &self.adcppb3triphi
    }
    #[doc = "0x1154 - ADC PPB3 Trip Low/Trigger Time Stamp Register"]
    #[inline(always)]
    pub const fn adcppb3triplo(&self) -> &Adcppb3triplo {
        &self.adcppb3triplo
    }
    #[doc = "0x1160 - ADC PPB4 Config Register"]
    #[inline(always)]
    pub const fn adcppb4config(&self) -> &Adcppb4config {
        &self.adcppb4config
    }
    #[doc = "0x1170 - ADC PPB4 Trip High Register"]
    #[inline(always)]
    pub const fn adcppb4triphi(&self) -> &Adcppb4triphi {
        &self.adcppb4triphi
    }
    #[doc = "0x1174 - ADC PPB4 Trip Low/Trigger Time Stamp Register"]
    #[inline(always)]
    pub const fn adcppb4triplo(&self) -> &Adcppb4triplo {
        &self.adcppb4triplo
    }
    #[doc = "0x1180 - ADC Early Interrupt Generation Cycle"]
    #[inline(always)]
    pub const fn adcintcycle(&self) -> &Adcintcycle {
        &self.adcintcycle
    }
    #[doc = "0x119c - ADC Wrapper Revision Register"]
    #[inline(always)]
    pub const fn adcrev2(&self) -> &Adcrev2 {
        &self.adcrev2
    }
    #[doc = "0x1200 - ADC PPB1Conversion Count Limit Register"]
    #[inline(always)]
    pub const fn adcppb1limit(&self) -> &Adcppb1limit {
        &self.adcppb1limit
    }
    #[doc = "0x1204 - ADC PPB1 Partial Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb1pcount(&self) -> &Adcppb1pcount {
        &self.adcppb1pcount
    }
    #[doc = "0x1208 - ADC PPB1 Sum Shift Register"]
    #[inline(always)]
    pub const fn adcppb1config2(&self) -> &Adcppb1config2 {
        &self.adcppb1config2
    }
    #[doc = "0x120c - ADC PPB1 Partial Sum Register"]
    #[inline(always)]
    pub const fn adcppb1psum(&self) -> &Adcppb1psum {
        &self.adcppb1psum
    }
    #[doc = "0x1240 - ADC PPB2Conversion Count Limit Register"]
    #[inline(always)]
    pub const fn adcppb2limit(&self) -> &Adcppb2limit {
        &self.adcppb2limit
    }
    #[doc = "0x1244 - ADC PPB2 Partial Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb2pcount(&self) -> &Adcppb2pcount {
        &self.adcppb2pcount
    }
    #[doc = "0x1248 - ADC PPB2 Sum Shift Register"]
    #[inline(always)]
    pub const fn adcppb2config2(&self) -> &Adcppb2config2 {
        &self.adcppb2config2
    }
    #[doc = "0x124c - ADC PPB2 Partial Sum Register"]
    #[inline(always)]
    pub const fn adcppb2psum(&self) -> &Adcppb2psum {
        &self.adcppb2psum
    }
    #[doc = "0x1280 - ADC PPB3Conversion Count Limit Register"]
    #[inline(always)]
    pub const fn adcppb3limit(&self) -> &Adcppb3limit {
        &self.adcppb3limit
    }
    #[doc = "0x1284 - ADC PPB3 Partial Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb3pcount(&self) -> &Adcppb3pcount {
        &self.adcppb3pcount
    }
    #[doc = "0x1288 - ADC PPB3 Sum Shift Register"]
    #[inline(always)]
    pub const fn adcppb3config2(&self) -> &Adcppb3config2 {
        &self.adcppb3config2
    }
    #[doc = "0x128c - ADC PPB3 Partial Sum Register"]
    #[inline(always)]
    pub const fn adcppb3psum(&self) -> &Adcppb3psum {
        &self.adcppb3psum
    }
    #[doc = "0x12c0 - ADC PPB4Conversion Count Limit Register"]
    #[inline(always)]
    pub const fn adcppb4limit(&self) -> &Adcppb4limit {
        &self.adcppb4limit
    }
    #[doc = "0x12c4 - ADC PPB4 Partial Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb4pcount(&self) -> &Adcppb4pcount {
        &self.adcppb4pcount
    }
    #[doc = "0x12c8 - ADC PPB4 Sum Shift Register"]
    #[inline(always)]
    pub const fn adcppb4config2(&self) -> &Adcppb4config2 {
        &self.adcppb4config2
    }
    #[doc = "0x12cc - ADC PPB4 Partial Sum Register"]
    #[inline(always)]
    pub const fn adcppb4psum(&self) -> &Adcppb4psum {
        &self.adcppb4psum
    }
    #[doc = "0x1320 - ADC Sequencer common control Register"]
    #[inline(always)]
    pub const fn adcseqctl(&self) -> &Adcseqctl {
        &self.adcseqctl
    }
    #[doc = "0x1324 - ADC Sequencer 1 Config register"]
    #[inline(always)]
    pub const fn adcseq1config(&self) -> &Adcseq1config {
        &self.adcseq1config
    }
    #[doc = "0x1328 - ADC Sequencer 2 Config register"]
    #[inline(always)]
    pub const fn adcseq2config(&self) -> &Adcseq2config {
        &self.adcseq2config
    }
    #[doc = "0x132c - ADC Sequencer 3 Config register"]
    #[inline(always)]
    pub const fn adcseq3config(&self) -> &Adcseq3config {
        &self.adcseq3config
    }
    #[doc = "0x1330 - ADC Sequencer 4 Config register"]
    #[inline(always)]
    pub const fn adcseq4config(&self) -> &Adcseq4config {
        &self.adcseq4config
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "ADCCTL1 (rw) register accessor: ADC Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl1`] module"]
#[doc(alias = "ADCCTL1")]
pub type Adcctl1 = crate::Reg<adcctl1::Adcctl1Spec>;
#[doc = "ADC Control 1 Register"]
pub mod adcctl1;
#[doc = "ADCCTL2 (rw) register accessor: ADC Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl2`] module"]
#[doc(alias = "ADCCTL2")]
pub type Adcctl2 = crate::Reg<adcctl2::Adcctl2Spec>;
#[doc = "ADC Control 2 Register"]
pub mod adcctl2;
#[doc = "ADCINTSEL (rw) register accessor: ADC Interrupt 1, 2, 3 and 4 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintsel`] module"]
#[doc(alias = "ADCINTSEL")]
pub type Adcintsel = crate::Reg<adcintsel::AdcintselSpec>;
#[doc = "ADC Interrupt 1, 2, 3 and 4 Selection Register"]
pub mod adcintsel;
#[doc = "ADCDMAINTSEL (rw) register accessor: ADC DMA Interrupt 1, 2, 3 and 4 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcdmaintsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcdmaintsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcdmaintsel`] module"]
#[doc(alias = "ADCDMAINTSEL")]
pub type Adcdmaintsel = crate::Reg<adcdmaintsel::AdcdmaintselSpec>;
#[doc = "ADC DMA Interrupt 1, 2, 3 and 4 Selection Register"]
pub mod adcdmaintsel;
#[doc = "ADCRAWINTFLG (rw) register accessor: ADC Raw Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrawintflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrawintflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcrawintflg`] module"]
#[doc(alias = "ADCRAWINTFLG")]
pub type Adcrawintflg = crate::Reg<adcrawintflg::AdcrawintflgSpec>;
#[doc = "ADC Raw Interrupt Flag Register"]
pub mod adcrawintflg;
#[doc = "ADCINTFLG (rw) register accessor: ADC Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintflg`] module"]
#[doc(alias = "ADCINTFLG")]
pub type Adcintflg = crate::Reg<adcintflg::AdcintflgSpec>;
#[doc = "ADC Interrupt Flag Register"]
pub mod adcintflg;
#[doc = "ADCINTFLGFRC (rw) register accessor: ADC Interrupt Flag Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintflgfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintflgfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintflgfrc`] module"]
#[doc(alias = "ADCINTFLGFRC")]
pub type Adcintflgfrc = crate::Reg<adcintflgfrc::AdcintflgfrcSpec>;
#[doc = "ADC Interrupt Flag Force Register"]
pub mod adcintflgfrc;
#[doc = "ADCINTFLGCLR (rw) register accessor: ADC Interrupt Flag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintflgclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintflgclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintflgclr`] module"]
#[doc(alias = "ADCINTFLGCLR")]
pub type Adcintflgclr = crate::Reg<adcintflgclr::AdcintflgclrSpec>;
#[doc = "ADC Interrupt Flag Clear Register"]
pub mod adcintflgclr;
#[doc = "ADCINTOVF (rw) register accessor: ADC Interrupt Overflow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintovf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintovf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintovf`] module"]
#[doc(alias = "ADCINTOVF")]
pub type Adcintovf = crate::Reg<adcintovf::AdcintovfSpec>;
#[doc = "ADC Interrupt Overflow Register"]
pub mod adcintovf;
#[doc = "ADCINTOVFCLR (rw) register accessor: ADC Interrupt Overflow Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintovfclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintovfclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintovfclr`] module"]
#[doc(alias = "ADCINTOVFCLR")]
pub type Adcintovfclr = crate::Reg<adcintovfclr::AdcintovfclrSpec>;
#[doc = "ADC Interrupt Overflow Clear Register"]
pub mod adcintovfclr;
#[doc = "ADCSOCFLG1 (rw) register accessor: ADC SOC Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocflg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocflg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocflg1`] module"]
#[doc(alias = "ADCSOCFLG1")]
pub type Adcsocflg1 = crate::Reg<adcsocflg1::Adcsocflg1Spec>;
#[doc = "ADC SOC Flag 1 Register"]
pub mod adcsocflg1;
#[doc = "ADCSOCOVF1 (rw) register accessor: ADC SOC Overflow 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocovf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocovf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocovf1`] module"]
#[doc(alias = "ADCSOCOVF1")]
pub type Adcsocovf1 = crate::Reg<adcsocovf1::Adcsocovf1Spec>;
#[doc = "ADC SOC Overflow 1 Register"]
pub mod adcsocovf1;
#[doc = "ADCSOCOVFCLR1 (rw) register accessor: ADC SOC Overflow Clear 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocovfclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocovfclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocovfclr1`] module"]
#[doc(alias = "ADCSOCOVFCLR1")]
pub type Adcsocovfclr1 = crate::Reg<adcsocovfclr1::Adcsocovfclr1Spec>;
#[doc = "ADC SOC Overflow Clear 1 Register"]
pub mod adcsocovfclr1;
#[doc = "ADCSOC0CTL (rw) register accessor: ADC SOC0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc0ctl`] module"]
#[doc(alias = "ADCSOC0CTL")]
pub type Adcsoc0ctl = crate::Reg<adcsoc0ctl::Adcsoc0ctlSpec>;
#[doc = "ADC SOC0 Control Register"]
pub mod adcsoc0ctl;
#[doc = "ADCSOC1CTL (rw) register accessor: ADC SOC1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc1ctl`] module"]
#[doc(alias = "ADCSOC1CTL")]
pub type Adcsoc1ctl = crate::Reg<adcsoc1ctl::Adcsoc1ctlSpec>;
#[doc = "ADC SOC1 Control Register"]
pub mod adcsoc1ctl;
#[doc = "ADCSOC2CTL (rw) register accessor: ADC SOC2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc2ctl`] module"]
#[doc(alias = "ADCSOC2CTL")]
pub type Adcsoc2ctl = crate::Reg<adcsoc2ctl::Adcsoc2ctlSpec>;
#[doc = "ADC SOC2 Control Register"]
pub mod adcsoc2ctl;
#[doc = "ADCSOC3CTL (rw) register accessor: ADC SOC3 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc3ctl`] module"]
#[doc(alias = "ADCSOC3CTL")]
pub type Adcsoc3ctl = crate::Reg<adcsoc3ctl::Adcsoc3ctlSpec>;
#[doc = "ADC SOC3 Control Register"]
pub mod adcsoc3ctl;
#[doc = "ADCSOC4CTL (rw) register accessor: ADC SOC4 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc4ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc4ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc4ctl`] module"]
#[doc(alias = "ADCSOC4CTL")]
pub type Adcsoc4ctl = crate::Reg<adcsoc4ctl::Adcsoc4ctlSpec>;
#[doc = "ADC SOC4 Control Register"]
pub mod adcsoc4ctl;
#[doc = "ADCSOC5CTL (rw) register accessor: ADC SOC5 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc5ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc5ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc5ctl`] module"]
#[doc(alias = "ADCSOC5CTL")]
pub type Adcsoc5ctl = crate::Reg<adcsoc5ctl::Adcsoc5ctlSpec>;
#[doc = "ADC SOC5 Control Register"]
pub mod adcsoc5ctl;
#[doc = "ADCSOC6CTL (rw) register accessor: ADC SOC6 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc6ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc6ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc6ctl`] module"]
#[doc(alias = "ADCSOC6CTL")]
pub type Adcsoc6ctl = crate::Reg<adcsoc6ctl::Adcsoc6ctlSpec>;
#[doc = "ADC SOC6 Control Register"]
pub mod adcsoc6ctl;
#[doc = "ADCSOC7CTL (rw) register accessor: ADC SOC7 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc7ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc7ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc7ctl`] module"]
#[doc(alias = "ADCSOC7CTL")]
pub type Adcsoc7ctl = crate::Reg<adcsoc7ctl::Adcsoc7ctlSpec>;
#[doc = "ADC SOC7 Control Register"]
pub mod adcsoc7ctl;
#[doc = "ADCSOC8CTL (rw) register accessor: ADC SOC8 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc8ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc8ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc8ctl`] module"]
#[doc(alias = "ADCSOC8CTL")]
pub type Adcsoc8ctl = crate::Reg<adcsoc8ctl::Adcsoc8ctlSpec>;
#[doc = "ADC SOC8 Control Register"]
pub mod adcsoc8ctl;
#[doc = "ADCSOC9CTL (rw) register accessor: ADC SOC9 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc9ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc9ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc9ctl`] module"]
#[doc(alias = "ADCSOC9CTL")]
pub type Adcsoc9ctl = crate::Reg<adcsoc9ctl::Adcsoc9ctlSpec>;
#[doc = "ADC SOC9 Control Register"]
pub mod adcsoc9ctl;
#[doc = "ADCSOC10CTL (rw) register accessor: ADC SOC10 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc10ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc10ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc10ctl`] module"]
#[doc(alias = "ADCSOC10CTL")]
pub type Adcsoc10ctl = crate::Reg<adcsoc10ctl::Adcsoc10ctlSpec>;
#[doc = "ADC SOC10 Control Register"]
pub mod adcsoc10ctl;
#[doc = "ADCSOC11CTL (rw) register accessor: ADC SOC11 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc11ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc11ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc11ctl`] module"]
#[doc(alias = "ADCSOC11CTL")]
pub type Adcsoc11ctl = crate::Reg<adcsoc11ctl::Adcsoc11ctlSpec>;
#[doc = "ADC SOC11 Control Register"]
pub mod adcsoc11ctl;
#[doc = "ADCSOC12CTL (rw) register accessor: ADC SOC12 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc12ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc12ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc12ctl`] module"]
#[doc(alias = "ADCSOC12CTL")]
pub type Adcsoc12ctl = crate::Reg<adcsoc12ctl::Adcsoc12ctlSpec>;
#[doc = "ADC SOC12 Control Register"]
pub mod adcsoc12ctl;
#[doc = "ADCSOC13CTL (rw) register accessor: ADC SOC13 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc13ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc13ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc13ctl`] module"]
#[doc(alias = "ADCSOC13CTL")]
pub type Adcsoc13ctl = crate::Reg<adcsoc13ctl::Adcsoc13ctlSpec>;
#[doc = "ADC SOC13 Control Register"]
pub mod adcsoc13ctl;
#[doc = "ADCSOC14CTL (rw) register accessor: ADC SOC14 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc14ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc14ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc14ctl`] module"]
#[doc(alias = "ADCSOC14CTL")]
pub type Adcsoc14ctl = crate::Reg<adcsoc14ctl::Adcsoc14ctlSpec>;
#[doc = "ADC SOC14 Control Register"]
pub mod adcsoc14ctl;
#[doc = "ADCSOC15CTL (rw) register accessor: ADC SOC15 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsoc15ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsoc15ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsoc15ctl`] module"]
#[doc(alias = "ADCSOC15CTL")]
pub type Adcsoc15ctl = crate::Reg<adcsoc15ctl::Adcsoc15ctlSpec>;
#[doc = "ADC SOC15 Control Register"]
pub mod adcsoc15ctl;
#[doc = "ADCEVTSTAT (rw) register accessor: ADC Event Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcevtstat`] module"]
#[doc(alias = "ADCEVTSTAT")]
pub type Adcevtstat = crate::Reg<adcevtstat::AdcevtstatSpec>;
#[doc = "ADC Event Status Register"]
pub mod adcevtstat;
#[doc = "ADCEVTCLR (rw) register accessor: ADC Event Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcevtclr`] module"]
#[doc(alias = "ADCEVTCLR")]
pub type Adcevtclr = crate::Reg<adcevtclr::AdcevtclrSpec>;
#[doc = "ADC Event Clear Register"]
pub mod adcevtclr;
#[doc = "ADCEVTSEL (rw) register accessor: ADC Event Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcevtsel`] module"]
#[doc(alias = "ADCEVTSEL")]
pub type Adcevtsel = crate::Reg<adcevtsel::AdcevtselSpec>;
#[doc = "ADC Event Selection Register"]
pub mod adcevtsel;
#[doc = "ADCEVTINTSEL (rw) register accessor: ADC Event Interrupt Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtintsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtintsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcevtintsel`] module"]
#[doc(alias = "ADCEVTINTSEL")]
pub type Adcevtintsel = crate::Reg<adcevtintsel::AdcevtintselSpec>;
#[doc = "ADC Event Interrupt Selection Register"]
pub mod adcevtintsel;
#[doc = "ADCREV (rw) register accessor: ADC Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcrev`] module"]
#[doc(alias = "ADCREV")]
pub type Adcrev = crate::Reg<adcrev::AdcrevSpec>;
#[doc = "ADC Revision Register"]
pub mod adcrev;
#[doc = "ADCOFFTRIM (rw) register accessor: ADC Offset Trim Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcofftrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcofftrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcofftrim`] module"]
#[doc(alias = "ADCOFFTRIM")]
pub type Adcofftrim = crate::Reg<adcofftrim::AdcofftrimSpec>;
#[doc = "ADC Offset Trim Register 1"]
pub mod adcofftrim;
#[doc = "ADCPPB1CONFIG (rw) register accessor: ADC PPB1 Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1config`] module"]
#[doc(alias = "ADCPPB1CONFIG")]
pub type Adcppb1config = crate::Reg<adcppb1config::Adcppb1configSpec>;
#[doc = "ADC PPB1 Config Register"]
pub mod adcppb1config;
#[doc = "ADCPPB1TRIPHI (rw) register accessor: ADC PPB1 Trip High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1triphi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1triphi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1triphi`] module"]
#[doc(alias = "ADCPPB1TRIPHI")]
pub type Adcppb1triphi = crate::Reg<adcppb1triphi::Adcppb1triphiSpec>;
#[doc = "ADC PPB1 Trip High Register"]
pub mod adcppb1triphi;
#[doc = "ADCPPB1TRIPLO (rw) register accessor: ADC PPB1 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1triplo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1triplo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1triplo`] module"]
#[doc(alias = "ADCPPB1TRIPLO")]
pub type Adcppb1triplo = crate::Reg<adcppb1triplo::Adcppb1triploSpec>;
#[doc = "ADC PPB1 Trip Low/Trigger Time Stamp Register"]
pub mod adcppb1triplo;
#[doc = "ADCPPB2CONFIG (rw) register accessor: ADC PPB2 Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2config`] module"]
#[doc(alias = "ADCPPB2CONFIG")]
pub type Adcppb2config = crate::Reg<adcppb2config::Adcppb2configSpec>;
#[doc = "ADC PPB2 Config Register"]
pub mod adcppb2config;
#[doc = "ADCPPB2TRIPHI (rw) register accessor: ADC PPB2 Trip High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2triphi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2triphi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2triphi`] module"]
#[doc(alias = "ADCPPB2TRIPHI")]
pub type Adcppb2triphi = crate::Reg<adcppb2triphi::Adcppb2triphiSpec>;
#[doc = "ADC PPB2 Trip High Register"]
pub mod adcppb2triphi;
#[doc = "ADCPPB2TRIPLO (rw) register accessor: ADC PPB2 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2triplo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2triplo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2triplo`] module"]
#[doc(alias = "ADCPPB2TRIPLO")]
pub type Adcppb2triplo = crate::Reg<adcppb2triplo::Adcppb2triploSpec>;
#[doc = "ADC PPB2 Trip Low/Trigger Time Stamp Register"]
pub mod adcppb2triplo;
#[doc = "ADCPPB3CONFIG (rw) register accessor: ADC PPB3 Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3config`] module"]
#[doc(alias = "ADCPPB3CONFIG")]
pub type Adcppb3config = crate::Reg<adcppb3config::Adcppb3configSpec>;
#[doc = "ADC PPB3 Config Register"]
pub mod adcppb3config;
#[doc = "ADCPPB3TRIPHI (rw) register accessor: ADC PPB3 Trip High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3triphi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3triphi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3triphi`] module"]
#[doc(alias = "ADCPPB3TRIPHI")]
pub type Adcppb3triphi = crate::Reg<adcppb3triphi::Adcppb3triphiSpec>;
#[doc = "ADC PPB3 Trip High Register"]
pub mod adcppb3triphi;
#[doc = "ADCPPB3TRIPLO (rw) register accessor: ADC PPB3 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3triplo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3triplo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3triplo`] module"]
#[doc(alias = "ADCPPB3TRIPLO")]
pub type Adcppb3triplo = crate::Reg<adcppb3triplo::Adcppb3triploSpec>;
#[doc = "ADC PPB3 Trip Low/Trigger Time Stamp Register"]
pub mod adcppb3triplo;
#[doc = "ADCPPB4CONFIG (rw) register accessor: ADC PPB4 Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4config`] module"]
#[doc(alias = "ADCPPB4CONFIG")]
pub type Adcppb4config = crate::Reg<adcppb4config::Adcppb4configSpec>;
#[doc = "ADC PPB4 Config Register"]
pub mod adcppb4config;
#[doc = "ADCPPB4TRIPHI (rw) register accessor: ADC PPB4 Trip High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4triphi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4triphi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4triphi`] module"]
#[doc(alias = "ADCPPB4TRIPHI")]
pub type Adcppb4triphi = crate::Reg<adcppb4triphi::Adcppb4triphiSpec>;
#[doc = "ADC PPB4 Trip High Register"]
pub mod adcppb4triphi;
#[doc = "ADCPPB4TRIPLO (rw) register accessor: ADC PPB4 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4triplo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4triplo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4triplo`] module"]
#[doc(alias = "ADCPPB4TRIPLO")]
pub type Adcppb4triplo = crate::Reg<adcppb4triplo::Adcppb4triploSpec>;
#[doc = "ADC PPB4 Trip Low/Trigger Time Stamp Register"]
pub mod adcppb4triplo;
#[doc = "ADCINTCYCLE (rw) register accessor: ADC Early Interrupt Generation Cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintcycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintcycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcintcycle`] module"]
#[doc(alias = "ADCINTCYCLE")]
pub type Adcintcycle = crate::Reg<adcintcycle::AdcintcycleSpec>;
#[doc = "ADC Early Interrupt Generation Cycle"]
pub mod adcintcycle;
#[doc = "ADCREV2 (rw) register accessor: ADC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrev2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrev2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcrev2`] module"]
#[doc(alias = "ADCREV2")]
pub type Adcrev2 = crate::Reg<adcrev2::Adcrev2Spec>;
#[doc = "ADC Wrapper Revision Register"]
pub mod adcrev2;
#[doc = "ADCPPB1LIMIT (rw) register accessor: ADC PPB1Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1limit`] module"]
#[doc(alias = "ADCPPB1LIMIT")]
pub type Adcppb1limit = crate::Reg<adcppb1limit::Adcppb1limitSpec>;
#[doc = "ADC PPB1Conversion Count Limit Register"]
pub mod adcppb1limit;
#[doc = "ADCPPB1PCOUNT (rw) register accessor: ADC PPB1 Partial Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1pcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1pcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1pcount`] module"]
#[doc(alias = "ADCPPB1PCOUNT")]
pub type Adcppb1pcount = crate::Reg<adcppb1pcount::Adcppb1pcountSpec>;
#[doc = "ADC PPB1 Partial Conversion Count Register"]
pub mod adcppb1pcount;
#[doc = "ADCPPB1CONFIG2 (rw) register accessor: ADC PPB1 Sum Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1config2`] module"]
#[doc(alias = "ADCPPB1CONFIG2")]
pub type Adcppb1config2 = crate::Reg<adcppb1config2::Adcppb1config2Spec>;
#[doc = "ADC PPB1 Sum Shift Register"]
pub mod adcppb1config2;
#[doc = "ADCPPB1PSUM (rw) register accessor: ADC PPB1 Partial Sum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1psum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1psum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1psum`] module"]
#[doc(alias = "ADCPPB1PSUM")]
pub type Adcppb1psum = crate::Reg<adcppb1psum::Adcppb1psumSpec>;
#[doc = "ADC PPB1 Partial Sum Register"]
pub mod adcppb1psum;
#[doc = "ADCPPB2LIMIT (rw) register accessor: ADC PPB2Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2limit`] module"]
#[doc(alias = "ADCPPB2LIMIT")]
pub type Adcppb2limit = crate::Reg<adcppb2limit::Adcppb2limitSpec>;
#[doc = "ADC PPB2Conversion Count Limit Register"]
pub mod adcppb2limit;
#[doc = "ADCPPB2PCOUNT (rw) register accessor: ADC PPB2 Partial Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2pcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2pcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2pcount`] module"]
#[doc(alias = "ADCPPB2PCOUNT")]
pub type Adcppb2pcount = crate::Reg<adcppb2pcount::Adcppb2pcountSpec>;
#[doc = "ADC PPB2 Partial Conversion Count Register"]
pub mod adcppb2pcount;
#[doc = "ADCPPB2CONFIG2 (rw) register accessor: ADC PPB2 Sum Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2config2`] module"]
#[doc(alias = "ADCPPB2CONFIG2")]
pub type Adcppb2config2 = crate::Reg<adcppb2config2::Adcppb2config2Spec>;
#[doc = "ADC PPB2 Sum Shift Register"]
pub mod adcppb2config2;
#[doc = "ADCPPB2PSUM (rw) register accessor: ADC PPB2 Partial Sum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2psum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2psum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2psum`] module"]
#[doc(alias = "ADCPPB2PSUM")]
pub type Adcppb2psum = crate::Reg<adcppb2psum::Adcppb2psumSpec>;
#[doc = "ADC PPB2 Partial Sum Register"]
pub mod adcppb2psum;
#[doc = "ADCPPB3LIMIT (rw) register accessor: ADC PPB3Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3limit`] module"]
#[doc(alias = "ADCPPB3LIMIT")]
pub type Adcppb3limit = crate::Reg<adcppb3limit::Adcppb3limitSpec>;
#[doc = "ADC PPB3Conversion Count Limit Register"]
pub mod adcppb3limit;
#[doc = "ADCPPB3PCOUNT (rw) register accessor: ADC PPB3 Partial Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3pcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3pcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3pcount`] module"]
#[doc(alias = "ADCPPB3PCOUNT")]
pub type Adcppb3pcount = crate::Reg<adcppb3pcount::Adcppb3pcountSpec>;
#[doc = "ADC PPB3 Partial Conversion Count Register"]
pub mod adcppb3pcount;
#[doc = "ADCPPB3CONFIG2 (rw) register accessor: ADC PPB3 Sum Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3config2`] module"]
#[doc(alias = "ADCPPB3CONFIG2")]
pub type Adcppb3config2 = crate::Reg<adcppb3config2::Adcppb3config2Spec>;
#[doc = "ADC PPB3 Sum Shift Register"]
pub mod adcppb3config2;
#[doc = "ADCPPB3PSUM (rw) register accessor: ADC PPB3 Partial Sum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3psum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3psum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3psum`] module"]
#[doc(alias = "ADCPPB3PSUM")]
pub type Adcppb3psum = crate::Reg<adcppb3psum::Adcppb3psumSpec>;
#[doc = "ADC PPB3 Partial Sum Register"]
pub mod adcppb3psum;
#[doc = "ADCPPB4LIMIT (rw) register accessor: ADC PPB4Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4limit`] module"]
#[doc(alias = "ADCPPB4LIMIT")]
pub type Adcppb4limit = crate::Reg<adcppb4limit::Adcppb4limitSpec>;
#[doc = "ADC PPB4Conversion Count Limit Register"]
pub mod adcppb4limit;
#[doc = "ADCPPB4PCOUNT (rw) register accessor: ADC PPB4 Partial Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4pcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4pcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4pcount`] module"]
#[doc(alias = "ADCPPB4PCOUNT")]
pub type Adcppb4pcount = crate::Reg<adcppb4pcount::Adcppb4pcountSpec>;
#[doc = "ADC PPB4 Partial Conversion Count Register"]
pub mod adcppb4pcount;
#[doc = "ADCPPB4CONFIG2 (rw) register accessor: ADC PPB4 Sum Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4config2`] module"]
#[doc(alias = "ADCPPB4CONFIG2")]
pub type Adcppb4config2 = crate::Reg<adcppb4config2::Adcppb4config2Spec>;
#[doc = "ADC PPB4 Sum Shift Register"]
pub mod adcppb4config2;
#[doc = "ADCPPB4PSUM (rw) register accessor: ADC PPB4 Partial Sum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4psum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4psum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4psum`] module"]
#[doc(alias = "ADCPPB4PSUM")]
pub type Adcppb4psum = crate::Reg<adcppb4psum::Adcppb4psumSpec>;
#[doc = "ADC PPB4 Partial Sum Register"]
pub mod adcppb4psum;
#[doc = "ADCSEQCTL (rw) register accessor: ADC Sequencer common control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseqctl`] module"]
#[doc(alias = "ADCSEQCTL")]
pub type Adcseqctl = crate::Reg<adcseqctl::AdcseqctlSpec>;
#[doc = "ADC Sequencer common control Register"]
pub mod adcseqctl;
#[doc = "ADCSEQ1CONFIG (rw) register accessor: ADC Sequencer 1 Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq1config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq1config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq1config`] module"]
#[doc(alias = "ADCSEQ1CONFIG")]
pub type Adcseq1config = crate::Reg<adcseq1config::Adcseq1configSpec>;
#[doc = "ADC Sequencer 1 Config register"]
pub mod adcseq1config;
#[doc = "ADCSEQ2CONFIG (rw) register accessor: ADC Sequencer 2 Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq2config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq2config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq2config`] module"]
#[doc(alias = "ADCSEQ2CONFIG")]
pub type Adcseq2config = crate::Reg<adcseq2config::Adcseq2configSpec>;
#[doc = "ADC Sequencer 2 Config register"]
pub mod adcseq2config;
#[doc = "ADCSEQ3CONFIG (rw) register accessor: ADC Sequencer 3 Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq3config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq3config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq3config`] module"]
#[doc(alias = "ADCSEQ3CONFIG")]
pub type Adcseq3config = crate::Reg<adcseq3config::Adcseq3configSpec>;
#[doc = "ADC Sequencer 3 Config register"]
pub mod adcseq3config;
#[doc = "ADCSEQ4CONFIG (rw) register accessor: ADC Sequencer 4 Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq4config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq4config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq4config`] module"]
#[doc(alias = "ADCSEQ4CONFIG")]
pub type Adcseq4config = crate::Reg<adcseq4config::Adcseq4configSpec>;
#[doc = "ADC Sequencer 4 Config register"]
pub mod adcseq4config;
