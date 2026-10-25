#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0480],
    cpu_connect_0: CpuConnect0,
    _reserved1: [u8; 0x0b9c],
    iidx: Iidx,
    _reserved2: [u8; 0x04],
    imask: Imask,
    _reserved3: [u8; 0x04],
    ris: Ris,
    _reserved4: [u8; 0x04],
    mis: Mis,
    _reserved5: [u8; 0x04],
    iset: Iset,
    _reserved6: [u8; 0x04],
    iclr: Iclr,
    _reserved7: [u8; 0xb4],
    cmdexec: Cmdexec,
    cmdtype: Cmdtype,
    cmdctl: Cmdctl,
    _reserved10: [u8; 0x14],
    cmdaddr: Cmdaddr,
    cmdbyten: Cmdbyten,
    _reserved12: [u8; 0x04],
    cmddataindex: Cmddataindex,
    cmddata0: Cmddata0,
    cmddata1: Cmddata1,
    cmddata2: Cmddata2,
    cmddata3: Cmddata3,
    cmddata4: Cmddata4,
    cmddata5: Cmddata5,
    cmddata6: Cmddata6,
    cmddata7: Cmddata7,
    cmddata8: Cmddata8,
    cmddata9: Cmddata9,
    cmddata10: Cmddata10,
    cmddata11: Cmddata11,
    cmddata12: Cmddata12,
    cmddata13: Cmddata13,
    cmddata14: Cmddata14,
    cmddata15: Cmddata15,
    _reserved29: [u8; 0x40],
    cmddataecc0: Cmddataecc0,
    cmddataecc1: Cmddataecc1,
    cmddataecc2: Cmddataecc2,
    cmddataecc3: Cmddataecc3,
    _reserved33: [u8; 0x10],
    cmdweprota: Cmdweprota,
    cmdweprotb: Cmdweprotb,
    _reserved35: [u8; 0x38],
    cmdweprotnm: Cmdweprotnm,
    _reserved36: [u8; 0x01bc],
    statcmd: Statcmd,
    _reserved37: [u8; 0x04],
    statpcnt: Statpcnt,
}
impl RegisterBlock {
    #[doc = "0x480 - CPU Connect"]
    #[inline(always)]
    pub const fn cpu_connect_0(&self) -> &CpuConnect0 {
        &self.cpu_connect_0
    }
    #[doc = "0x1020 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
    #[doc = "0x1028 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x1030 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x1038 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1040 - Interrupt Set Register"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x1048 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x1100 - Command Execute Register"]
    #[inline(always)]
    pub const fn cmdexec(&self) -> &Cmdexec {
        &self.cmdexec
    }
    #[doc = "0x1104 - Command Type Register"]
    #[inline(always)]
    pub const fn cmdtype(&self) -> &Cmdtype {
        &self.cmdtype
    }
    #[doc = "0x1108 - Command Control Register"]
    #[inline(always)]
    pub const fn cmdctl(&self) -> &Cmdctl {
        &self.cmdctl
    }
    #[doc = "0x1120 - Command Address Register"]
    #[inline(always)]
    pub const fn cmdaddr(&self) -> &Cmdaddr {
        &self.cmdaddr
    }
    #[doc = "0x1124 - Command Program Byte Enable Register"]
    #[inline(always)]
    pub const fn cmdbyten(&self) -> &Cmdbyten {
        &self.cmdbyten
    }
    #[doc = "0x112c - Command Data Index Register"]
    #[inline(always)]
    pub const fn cmddataindex(&self) -> &Cmddataindex {
        &self.cmddataindex
    }
    #[doc = "0x1130 - Command Data Register 0"]
    #[inline(always)]
    pub const fn cmddata0(&self) -> &Cmddata0 {
        &self.cmddata0
    }
    #[doc = "0x1134 - Command Data Register 1"]
    #[inline(always)]
    pub const fn cmddata1(&self) -> &Cmddata1 {
        &self.cmddata1
    }
    #[doc = "0x1138 - Command Data Register 2"]
    #[inline(always)]
    pub const fn cmddata2(&self) -> &Cmddata2 {
        &self.cmddata2
    }
    #[doc = "0x113c - Command Data Register Bits 127:96"]
    #[inline(always)]
    pub const fn cmddata3(&self) -> &Cmddata3 {
        &self.cmddata3
    }
    #[doc = "0x1140 - Command Data Register 4"]
    #[inline(always)]
    pub const fn cmddata4(&self) -> &Cmddata4 {
        &self.cmddata4
    }
    #[doc = "0x1144 - Command Data Register 5"]
    #[inline(always)]
    pub const fn cmddata5(&self) -> &Cmddata5 {
        &self.cmddata5
    }
    #[doc = "0x1148 - Command Data Register 6"]
    #[inline(always)]
    pub const fn cmddata6(&self) -> &Cmddata6 {
        &self.cmddata6
    }
    #[doc = "0x114c - Command Data Register 7"]
    #[inline(always)]
    pub const fn cmddata7(&self) -> &Cmddata7 {
        &self.cmddata7
    }
    #[doc = "0x1150 - Command Data Register 8"]
    #[inline(always)]
    pub const fn cmddata8(&self) -> &Cmddata8 {
        &self.cmddata8
    }
    #[doc = "0x1154 - Command Data Register 9"]
    #[inline(always)]
    pub const fn cmddata9(&self) -> &Cmddata9 {
        &self.cmddata9
    }
    #[doc = "0x1158 - Command Data Register 10"]
    #[inline(always)]
    pub const fn cmddata10(&self) -> &Cmddata10 {
        &self.cmddata10
    }
    #[doc = "0x115c - Command Data Register 11"]
    #[inline(always)]
    pub const fn cmddata11(&self) -> &Cmddata11 {
        &self.cmddata11
    }
    #[doc = "0x1160 - Command Data Register 12"]
    #[inline(always)]
    pub const fn cmddata12(&self) -> &Cmddata12 {
        &self.cmddata12
    }
    #[doc = "0x1164 - Command Data Register 13"]
    #[inline(always)]
    pub const fn cmddata13(&self) -> &Cmddata13 {
        &self.cmddata13
    }
    #[doc = "0x1168 - Command Data Register 14"]
    #[inline(always)]
    pub const fn cmddata14(&self) -> &Cmddata14 {
        &self.cmddata14
    }
    #[doc = "0x116c - Command Data Register 15"]
    #[inline(always)]
    pub const fn cmddata15(&self) -> &Cmddata15 {
        &self.cmddata15
    }
    #[doc = "0x11b0 - Command Data Register ECC 0"]
    #[inline(always)]
    pub const fn cmddataecc0(&self) -> &Cmddataecc0 {
        &self.cmddataecc0
    }
    #[doc = "0x11b4 - Command Data Register ECC 1"]
    #[inline(always)]
    pub const fn cmddataecc1(&self) -> &Cmddataecc1 {
        &self.cmddataecc1
    }
    #[doc = "0x11b8 - Command Data Register ECC 2"]
    #[inline(always)]
    pub const fn cmddataecc2(&self) -> &Cmddataecc2 {
        &self.cmddataecc2
    }
    #[doc = "0x11bc - Command Data Register ECC 3"]
    #[inline(always)]
    pub const fn cmddataecc3(&self) -> &Cmddataecc3 {
        &self.cmddataecc3
    }
    #[doc = "0x11d0 - Command Write Erase Protect A Register"]
    #[inline(always)]
    pub const fn cmdweprota(&self) -> &Cmdweprota {
        &self.cmdweprota
    }
    #[doc = "0x11d4 - Command Write Erase Protect B Register"]
    #[inline(always)]
    pub const fn cmdweprotb(&self) -> &Cmdweprotb {
        &self.cmdweprotb
    }
    #[doc = "0x1210 - Command Write Erase Protect Non-Main Register"]
    #[inline(always)]
    pub const fn cmdweprotnm(&self) -> &Cmdweprotnm {
        &self.cmdweprotnm
    }
    #[doc = "0x13d0 - Command Status Register"]
    #[inline(always)]
    pub const fn statcmd(&self) -> &Statcmd {
        &self.statcmd
    }
    #[doc = "0x13d8 - Pulse Count Status Register"]
    #[inline(always)]
    pub const fn statpcnt(&self) -> &Statpcnt {
        &self.statpcnt
    }
}
#[doc = "CPU_CONNECT_0 (rw) register accessor: CPU Connect\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_connect_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_connect_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_connect_0`] module"]
#[doc(alias = "CPU_CONNECT_0")]
pub type CpuConnect0 = crate::Reg<cpu_connect_0::CpuConnect0Spec>;
#[doc = "CPU Connect"]
pub mod cpu_connect_0;
#[doc = "IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`] module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`] module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imask;
#[doc = "RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ISET (w) register accessor: Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`] module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt Set Register"]
pub mod iset;
#[doc = "ICLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`] module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod iclr;
#[doc = "CMDEXEC (rw) register accessor: Command Execute Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdexec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdexec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdexec`] module"]
#[doc(alias = "CMDEXEC")]
pub type Cmdexec = crate::Reg<cmdexec::CmdexecSpec>;
#[doc = "Command Execute Register"]
pub mod cmdexec;
#[doc = "CMDTYPE (rw) register accessor: Command Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdtype`] module"]
#[doc(alias = "CMDTYPE")]
pub type Cmdtype = crate::Reg<cmdtype::CmdtypeSpec>;
#[doc = "Command Type Register"]
pub mod cmdtype;
#[doc = "CMDCTL (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdctl`] module"]
#[doc(alias = "CMDCTL")]
pub type Cmdctl = crate::Reg<cmdctl::CmdctlSpec>;
#[doc = "Command Control Register"]
pub mod cmdctl;
#[doc = "CMDADDR (rw) register accessor: Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdaddr`] module"]
#[doc(alias = "CMDADDR")]
pub type Cmdaddr = crate::Reg<cmdaddr::CmdaddrSpec>;
#[doc = "Command Address Register"]
pub mod cmdaddr;
#[doc = "CMDBYTEN (rw) register accessor: Command Program Byte Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdbyten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdbyten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdbyten`] module"]
#[doc(alias = "CMDBYTEN")]
pub type Cmdbyten = crate::Reg<cmdbyten::CmdbytenSpec>;
#[doc = "Command Program Byte Enable Register"]
pub mod cmdbyten;
#[doc = "CMDDATAINDEX (rw) register accessor: Command Data Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataindex`] module"]
#[doc(alias = "CMDDATAINDEX")]
pub type Cmddataindex = crate::Reg<cmddataindex::CmddataindexSpec>;
#[doc = "Command Data Index Register"]
pub mod cmddataindex;
#[doc = "CMDDATA0 (rw) register accessor: Command Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata0`] module"]
#[doc(alias = "CMDDATA0")]
pub type Cmddata0 = crate::Reg<cmddata0::Cmddata0Spec>;
#[doc = "Command Data Register 0"]
pub mod cmddata0;
#[doc = "CMDDATA1 (rw) register accessor: Command Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata1`] module"]
#[doc(alias = "CMDDATA1")]
pub type Cmddata1 = crate::Reg<cmddata1::Cmddata1Spec>;
#[doc = "Command Data Register 1"]
pub mod cmddata1;
#[doc = "CMDDATA2 (rw) register accessor: Command Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata2`] module"]
#[doc(alias = "CMDDATA2")]
pub type Cmddata2 = crate::Reg<cmddata2::Cmddata2Spec>;
#[doc = "Command Data Register 2"]
pub mod cmddata2;
#[doc = "CMDDATA3 (rw) register accessor: Command Data Register Bits 127:96\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata3`] module"]
#[doc(alias = "CMDDATA3")]
pub type Cmddata3 = crate::Reg<cmddata3::Cmddata3Spec>;
#[doc = "Command Data Register Bits 127:96"]
pub mod cmddata3;
#[doc = "CMDDATA4 (rw) register accessor: Command Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata4`] module"]
#[doc(alias = "CMDDATA4")]
pub type Cmddata4 = crate::Reg<cmddata4::Cmddata4Spec>;
#[doc = "Command Data Register 4"]
pub mod cmddata4;
#[doc = "CMDDATA5 (rw) register accessor: Command Data Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata5`] module"]
#[doc(alias = "CMDDATA5")]
pub type Cmddata5 = crate::Reg<cmddata5::Cmddata5Spec>;
#[doc = "Command Data Register 5"]
pub mod cmddata5;
#[doc = "CMDDATA6 (rw) register accessor: Command Data Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata6`] module"]
#[doc(alias = "CMDDATA6")]
pub type Cmddata6 = crate::Reg<cmddata6::Cmddata6Spec>;
#[doc = "Command Data Register 6"]
pub mod cmddata6;
#[doc = "CMDDATA7 (rw) register accessor: Command Data Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata7`] module"]
#[doc(alias = "CMDDATA7")]
pub type Cmddata7 = crate::Reg<cmddata7::Cmddata7Spec>;
#[doc = "Command Data Register 7"]
pub mod cmddata7;
#[doc = "CMDDATA8 (rw) register accessor: Command Data Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata8`] module"]
#[doc(alias = "CMDDATA8")]
pub type Cmddata8 = crate::Reg<cmddata8::Cmddata8Spec>;
#[doc = "Command Data Register 8"]
pub mod cmddata8;
#[doc = "CMDDATA9 (rw) register accessor: Command Data Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata9`] module"]
#[doc(alias = "CMDDATA9")]
pub type Cmddata9 = crate::Reg<cmddata9::Cmddata9Spec>;
#[doc = "Command Data Register 9"]
pub mod cmddata9;
#[doc = "CMDDATA10 (rw) register accessor: Command Data Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata10`] module"]
#[doc(alias = "CMDDATA10")]
pub type Cmddata10 = crate::Reg<cmddata10::Cmddata10Spec>;
#[doc = "Command Data Register 10"]
pub mod cmddata10;
#[doc = "CMDDATA11 (rw) register accessor: Command Data Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata11`] module"]
#[doc(alias = "CMDDATA11")]
pub type Cmddata11 = crate::Reg<cmddata11::Cmddata11Spec>;
#[doc = "Command Data Register 11"]
pub mod cmddata11;
#[doc = "CMDDATA12 (rw) register accessor: Command Data Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata12`] module"]
#[doc(alias = "CMDDATA12")]
pub type Cmddata12 = crate::Reg<cmddata12::Cmddata12Spec>;
#[doc = "Command Data Register 12"]
pub mod cmddata12;
#[doc = "CMDDATA13 (rw) register accessor: Command Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata13`] module"]
#[doc(alias = "CMDDATA13")]
pub type Cmddata13 = crate::Reg<cmddata13::Cmddata13Spec>;
#[doc = "Command Data Register 13"]
pub mod cmddata13;
#[doc = "CMDDATA14 (rw) register accessor: Command Data Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata14`] module"]
#[doc(alias = "CMDDATA14")]
pub type Cmddata14 = crate::Reg<cmddata14::Cmddata14Spec>;
#[doc = "Command Data Register 14"]
pub mod cmddata14;
#[doc = "CMDDATA15 (rw) register accessor: Command Data Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata15`] module"]
#[doc(alias = "CMDDATA15")]
pub type Cmddata15 = crate::Reg<cmddata15::Cmddata15Spec>;
#[doc = "Command Data Register 15"]
pub mod cmddata15;
#[doc = "CMDDATAECC0 (rw) register accessor: Command Data Register ECC 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataecc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataecc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataecc0`] module"]
#[doc(alias = "CMDDATAECC0")]
pub type Cmddataecc0 = crate::Reg<cmddataecc0::Cmddataecc0Spec>;
#[doc = "Command Data Register ECC 0"]
pub mod cmddataecc0;
#[doc = "CMDDATAECC1 (rw) register accessor: Command Data Register ECC 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataecc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataecc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataecc1`] module"]
#[doc(alias = "CMDDATAECC1")]
pub type Cmddataecc1 = crate::Reg<cmddataecc1::Cmddataecc1Spec>;
#[doc = "Command Data Register ECC 1"]
pub mod cmddataecc1;
#[doc = "CMDDATAECC2 (rw) register accessor: Command Data Register ECC 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataecc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataecc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataecc2`] module"]
#[doc(alias = "CMDDATAECC2")]
pub type Cmddataecc2 = crate::Reg<cmddataecc2::Cmddataecc2Spec>;
#[doc = "Command Data Register ECC 2"]
pub mod cmddataecc2;
#[doc = "CMDDATAECC3 (rw) register accessor: Command Data Register ECC 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataecc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataecc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataecc3`] module"]
#[doc(alias = "CMDDATAECC3")]
pub type Cmddataecc3 = crate::Reg<cmddataecc3::Cmddataecc3Spec>;
#[doc = "Command Data Register ECC 3"]
pub mod cmddataecc3;
#[doc = "CMDWEPROTA (rw) register accessor: Command Write Erase Protect A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprota::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprota::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprota`] module"]
#[doc(alias = "CMDWEPROTA")]
pub type Cmdweprota = crate::Reg<cmdweprota::CmdweprotaSpec>;
#[doc = "Command Write Erase Protect A Register"]
pub mod cmdweprota;
#[doc = "CMDWEPROTB (rw) register accessor: Command Write Erase Protect B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprotb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprotb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotb`] module"]
#[doc(alias = "CMDWEPROTB")]
pub type Cmdweprotb = crate::Reg<cmdweprotb::CmdweprotbSpec>;
#[doc = "Command Write Erase Protect B Register"]
pub mod cmdweprotb;
#[doc = "CMDWEPROTNM (rw) register accessor: Command Write Erase Protect Non-Main Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprotnm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprotnm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotnm`] module"]
#[doc(alias = "CMDWEPROTNM")]
pub type Cmdweprotnm = crate::Reg<cmdweprotnm::CmdweprotnmSpec>;
#[doc = "Command Write Erase Protect Non-Main Register"]
pub mod cmdweprotnm;
#[doc = "STATCMD (r) register accessor: Command Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`statcmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statcmd`] module"]
#[doc(alias = "STATCMD")]
pub type Statcmd = crate::Reg<statcmd::StatcmdSpec>;
#[doc = "Command Status Register"]
pub mod statcmd;
#[doc = "STATPCNT (r) register accessor: Pulse Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`statpcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statpcnt`] module"]
#[doc(alias = "STATPCNT")]
pub type Statpcnt = crate::Reg<statpcnt::StatpcntSpec>;
#[doc = "Pulse Count Status Register"]
pub mod statpcnt;
