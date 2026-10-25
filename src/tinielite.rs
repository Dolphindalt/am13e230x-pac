#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    cpu_int: CpuInt,
    _reserved1: [u8; 0xd4],
    ctl5: Ctl5,
    ctl4: Ctl4,
    ctl3: Ctl3,
    ctl2: Ctl2,
    ctl1: Ctl1,
    ctl0: Ctl0,
    lc: [Lc; 6],
    pc: Pc,
    areg: [Areg; 20],
    _reserved10: [u8; 0x056c],
    datain: [Datain; 2],
    _reserved11: [u8; 0x08],
    dataout: Dataout,
    _reserved12: [u8; 0x0c],
    ss0low: Ss0low,
    ss0high: Ss0high,
    ss1low: Ss1low,
    ss1high: Ss1high,
    _reserved16: [u8; 0x10],
    wght0low: Wght0low,
    wght0high: Wght0high,
    wght1low: Wght1low,
    wght1high: Wght1high,
    _reserved20: [u8; 0x04a0],
    accval: [Accval; 32],
    _reserved21: [u8; 0x0380],
    instmem: [Instmem; 256],
    _reserved22: [u8; 0x0400],
    rfdata: [Rfdata; 512],
}
impl RegisterBlock {
    #[doc = "0x1020..0x104c - Interrupt registers"]
    #[inline(always)]
    pub const fn cpu_int(&self) -> &CpuInt {
        &self.cpu_int
    }
    #[doc = "0x1120 - CTL5 register"]
    #[inline(always)]
    pub const fn ctl5(&self) -> &Ctl5 {
        &self.ctl5
    }
    #[doc = "0x1124 - CTL4 register"]
    #[inline(always)]
    pub const fn ctl4(&self) -> &Ctl4 {
        &self.ctl4
    }
    #[doc = "0x1128 - CTL3 register"]
    #[inline(always)]
    pub const fn ctl3(&self) -> &Ctl3 {
        &self.ctl3
    }
    #[doc = "0x112c - CTL2 register"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x1130 - CTL1 register"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x1134 - CTL0 register"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x1138..0x1150 - Loop count registers."]
    #[inline(always)]
    pub const fn lc(&self, n: usize) -> &Lc {
        &self.lc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1138..0x1150 - Loop count registers."]
    #[inline(always)]
    pub fn lc_iter(&self) -> impl Iterator<Item = &Lc> {
        self.lc.iter()
    }
    #[doc = "0x1150 - Program count register"]
    #[inline(always)]
    pub const fn pc(&self) -> &Pc {
        &self.pc
    }
    #[doc = "0x1154..0x11a4 - Address registers."]
    #[inline(always)]
    pub const fn areg(&self, n: usize) -> &Areg {
        &self.areg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1154..0x11a4 - Address registers."]
    #[inline(always)]
    pub fn areg_iter(&self) -> impl Iterator<Item = &Areg> {
        self.areg.iter()
    }
    #[doc = "0x1710..0x1718 - Input Data registers, used to load data from memory."]
    #[inline(always)]
    pub const fn datain(&self, n: usize) -> &Datain {
        &self.datain[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1710..0x1718 - Input Data registers, used to load data from memory."]
    #[inline(always)]
    pub fn datain_iter(&self) -> impl Iterator<Item = &Datain> {
        self.datain.iter()
    }
    #[doc = "0x1720 - Output Data registers, used to store data to memory."]
    #[inline(always)]
    pub const fn dataout(&self) -> &Dataout {
        &self.dataout
    }
    #[doc = "0x1730 - Scale and shift register 0, lower 32 bits."]
    #[inline(always)]
    pub const fn ss0low(&self) -> &Ss0low {
        &self.ss0low
    }
    #[doc = "0x1734 - Scale and shift register 0, upper 32 bits."]
    #[inline(always)]
    pub const fn ss0high(&self) -> &Ss0high {
        &self.ss0high
    }
    #[doc = "0x1738 - Scale and shift register 1, lower 32 bits."]
    #[inline(always)]
    pub const fn ss1low(&self) -> &Ss1low {
        &self.ss1low
    }
    #[doc = "0x173c - Scale and shift register 1, upper 32 bits."]
    #[inline(always)]
    pub const fn ss1high(&self) -> &Ss1high {
        &self.ss1high
    }
    #[doc = "0x1750 - Weight register 0, lower 32 bits."]
    #[inline(always)]
    pub const fn wght0low(&self) -> &Wght0low {
        &self.wght0low
    }
    #[doc = "0x1754 - Weight register 0, upper 32 bits."]
    #[inline(always)]
    pub const fn wght0high(&self) -> &Wght0high {
        &self.wght0high
    }
    #[doc = "0x1758 - Weight register 1, lower 32 bits."]
    #[inline(always)]
    pub const fn wght1low(&self) -> &Wght1low {
        &self.wght1low
    }
    #[doc = "0x175c - Weight register 1, upper 32 bits."]
    #[inline(always)]
    pub const fn wght1high(&self) -> &Wght1high {
        &self.wght1high
    }
    #[doc = "0x1c00..0x1c80 - Register file data."]
    #[inline(always)]
    pub const fn accval(&self, n: usize) -> &Accval {
        &self.accval[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c00..0x1c80 - Register file data."]
    #[inline(always)]
    pub fn accval_iter(&self) -> impl Iterator<Item = &Accval> {
        self.accval.iter()
    }
    #[doc = "0x2000..0x2400 - Instruction memory word."]
    #[inline(always)]
    pub const fn instmem(&self, n: usize) -> &Instmem {
        &self.instmem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2400 - Instruction memory word."]
    #[inline(always)]
    pub fn instmem_iter(&self) -> impl Iterator<Item = &Instmem> {
        self.instmem.iter()
    }
    #[doc = "0x2800..0x3000 - Register file data."]
    #[inline(always)]
    pub const fn rfdata(&self, n: usize) -> &Rfdata {
        &self.rfdata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2800..0x3000 - Register file data."]
    #[inline(always)]
    pub fn rfdata_iter(&self) -> impl Iterator<Item = &Rfdata> {
        self.rfdata.iter()
    }
}
#[doc = "Interrupt registers"]
pub use self::cpu_int::CpuInt;
#[doc = r"Cluster"]
#[doc = "Interrupt registers"]
pub mod cpu_int;
#[doc = "CTL5 (rw) register accessor: CTL5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl5`] module"]
#[doc(alias = "CTL5")]
pub type Ctl5 = crate::Reg<ctl5::Ctl5Spec>;
#[doc = "CTL5 register"]
pub mod ctl5;
#[doc = "CTL4 (rw) register accessor: CTL4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl4`] module"]
#[doc(alias = "CTL4")]
pub type Ctl4 = crate::Reg<ctl4::Ctl4Spec>;
#[doc = "CTL4 register"]
pub mod ctl4;
#[doc = "CTL3 (rw) register accessor: CTL3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl3`] module"]
#[doc(alias = "CTL3")]
pub type Ctl3 = crate::Reg<ctl3::Ctl3Spec>;
#[doc = "CTL3 register"]
pub mod ctl3;
#[doc = "CTL2 (rw) register accessor: CTL2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`] module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "CTL2 register"]
pub mod ctl2;
#[doc = "CTL1 (rw) register accessor: CTL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "CTL1 register"]
pub mod ctl1;
#[doc = "CTL0 (rw) register accessor: CTL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "CTL0 register"]
pub mod ctl0;
#[doc = "LC (rw) register accessor: Loop count registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc`] module"]
#[doc(alias = "LC")]
pub type Lc = crate::Reg<lc::LcSpec>;
#[doc = "Loop count registers."]
pub mod lc;
#[doc = "PC (rw) register accessor: Program count register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`] module"]
#[doc(alias = "PC")]
pub type Pc = crate::Reg<pc::PcSpec>;
#[doc = "Program count register"]
pub mod pc;
#[doc = "AREG (rw) register accessor: Address registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`areg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`areg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@areg`] module"]
#[doc(alias = "AREG")]
pub type Areg = crate::Reg<areg::AregSpec>;
#[doc = "Address registers."]
pub mod areg;
#[doc = "DATAIN (rw) register accessor: Input Data registers, used to load data from memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain`] module"]
#[doc(alias = "DATAIN")]
pub type Datain = crate::Reg<datain::DatainSpec>;
#[doc = "Input Data registers, used to load data from memory."]
pub mod datain;
#[doc = "DATAOUT (rw) register accessor: Output Data registers, used to store data to memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout`] module"]
#[doc(alias = "DATAOUT")]
pub type Dataout = crate::Reg<dataout::DataoutSpec>;
#[doc = "Output Data registers, used to store data to memory."]
pub mod dataout;
#[doc = "SS0LOW (rw) register accessor: Scale and shift register 0, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss0low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss0low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss0low`] module"]
#[doc(alias = "SS0LOW")]
pub type Ss0low = crate::Reg<ss0low::Ss0lowSpec>;
#[doc = "Scale and shift register 0, lower 32 bits."]
pub mod ss0low;
#[doc = "SS0HIGH (rw) register accessor: Scale and shift register 0, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss0high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss0high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss0high`] module"]
#[doc(alias = "SS0HIGH")]
pub type Ss0high = crate::Reg<ss0high::Ss0highSpec>;
#[doc = "Scale and shift register 0, upper 32 bits."]
pub mod ss0high;
#[doc = "SS1LOW (rw) register accessor: Scale and shift register 1, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss1low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss1low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss1low`] module"]
#[doc(alias = "SS1LOW")]
pub type Ss1low = crate::Reg<ss1low::Ss1lowSpec>;
#[doc = "Scale and shift register 1, lower 32 bits."]
pub mod ss1low;
#[doc = "SS1HIGH (rw) register accessor: Scale and shift register 1, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss1high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss1high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss1high`] module"]
#[doc(alias = "SS1HIGH")]
pub type Ss1high = crate::Reg<ss1high::Ss1highSpec>;
#[doc = "Scale and shift register 1, upper 32 bits."]
pub mod ss1high;
#[doc = "WGHT0LOW (rw) register accessor: Weight register 0, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght0low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght0low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wght0low`] module"]
#[doc(alias = "WGHT0LOW")]
pub type Wght0low = crate::Reg<wght0low::Wght0lowSpec>;
#[doc = "Weight register 0, lower 32 bits."]
pub mod wght0low;
#[doc = "WGHT0HIGH (rw) register accessor: Weight register 0, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght0high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght0high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wght0high`] module"]
#[doc(alias = "WGHT0HIGH")]
pub type Wght0high = crate::Reg<wght0high::Wght0highSpec>;
#[doc = "Weight register 0, upper 32 bits."]
pub mod wght0high;
#[doc = "WGHT1LOW (rw) register accessor: Weight register 1, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght1low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght1low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wght1low`] module"]
#[doc(alias = "WGHT1LOW")]
pub type Wght1low = crate::Reg<wght1low::Wght1lowSpec>;
#[doc = "Weight register 1, lower 32 bits."]
pub mod wght1low;
#[doc = "WGHT1HIGH (rw) register accessor: Weight register 1, upper 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`wght1high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wght1high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wght1high`] module"]
#[doc(alias = "WGHT1HIGH")]
pub type Wght1high = crate::Reg<wght1high::Wght1highSpec>;
#[doc = "Weight register 1, upper 32 bits."]
pub mod wght1high;
#[doc = "ACCVAL (rw) register accessor: Register file data.\n\nYou can [`read`](crate::Reg::read) this register and get [`accval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`accval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accval`] module"]
#[doc(alias = "ACCVAL")]
pub type Accval = crate::Reg<accval::AccvalSpec>;
#[doc = "Register file data."]
pub mod accval;
#[doc = "INSTMEM (rw) register accessor: Instruction memory word.\n\nYou can [`read`](crate::Reg::read) this register and get [`instmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instmem`] module"]
#[doc(alias = "INSTMEM")]
pub type Instmem = crate::Reg<instmem::InstmemSpec>;
#[doc = "Instruction memory word."]
pub mod instmem;
#[doc = "RFDATA (rw) register accessor: Register file data.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdata`] module"]
#[doc(alias = "RFDATA")]
pub type Rfdata = crate::Reg<rfdata::RfdataSpec>;
#[doc = "Register file data."]
pub mod rfdata;
