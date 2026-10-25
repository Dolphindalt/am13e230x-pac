#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcresult0: Adcresult0,
    adcresult1: Adcresult1,
    adcresult2: Adcresult2,
    adcresult3: Adcresult3,
    adcresult4: Adcresult4,
    adcresult5: Adcresult5,
    adcresult6: Adcresult6,
    adcresult7: Adcresult7,
    adcresult8: Adcresult8,
    adcresult9: Adcresult9,
    adcresult10: Adcresult10,
    adcresult11: Adcresult11,
    adcresult12: Adcresult12,
    adcresult13: Adcresult13,
    adcresult14: Adcresult14,
    adcresult15: Adcresult15,
    _reserved16: [u8; 0x20],
    adcppb1result: Adcppb1result,
    adcppb2result: Adcppb2result,
    adcppb3result: Adcppb3result,
    adcppb4result: Adcppb4result,
    adcppb1sum: Adcppb1sum,
    adcppb1count: Adcppb1count,
    _reserved22: [u8; 0x02],
    adcppb2sum: Adcppb2sum,
    adcppb2count: Adcppb2count,
    _reserved24: [u8; 0x02],
    adcppb3sum: Adcppb3sum,
    adcppb3count: Adcppb3count,
    _reserved26: [u8; 0x02],
    adcppb4sum: Adcppb4sum,
    adcppb4count: Adcppb4count,
    _reserved28: [u8; 0x52],
    adcseq1fiforesult: Adcseq1fiforesult,
    adcseq2fiforesult: Adcseq2fiforesult,
    adcseq3fiforesult: Adcseq3fiforesult,
    adcseq4fiforesult: Adcseq4fiforesult,
    adcseq1fifostatus: Adcseq1fifostatus,
    adcseq2fifostatus: Adcseq2fifostatus,
    adcseq3fifostatus: Adcseq3fifostatus,
    adcseq4fifostatus: Adcseq4fifostatus,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Result 0 Register"]
    #[inline(always)]
    pub const fn adcresult0(&self) -> &Adcresult0 {
        &self.adcresult0
    }
    #[doc = "0x02 - ADC Result 1 Register"]
    #[inline(always)]
    pub const fn adcresult1(&self) -> &Adcresult1 {
        &self.adcresult1
    }
    #[doc = "0x04 - ADC Result 2 Register"]
    #[inline(always)]
    pub const fn adcresult2(&self) -> &Adcresult2 {
        &self.adcresult2
    }
    #[doc = "0x06 - ADC Result 3 Register"]
    #[inline(always)]
    pub const fn adcresult3(&self) -> &Adcresult3 {
        &self.adcresult3
    }
    #[doc = "0x08 - ADC Result 4 Register"]
    #[inline(always)]
    pub const fn adcresult4(&self) -> &Adcresult4 {
        &self.adcresult4
    }
    #[doc = "0x0a - ADC Result 5 Register"]
    #[inline(always)]
    pub const fn adcresult5(&self) -> &Adcresult5 {
        &self.adcresult5
    }
    #[doc = "0x0c - ADC Result 6 Register"]
    #[inline(always)]
    pub const fn adcresult6(&self) -> &Adcresult6 {
        &self.adcresult6
    }
    #[doc = "0x0e - ADC Result 7 Register"]
    #[inline(always)]
    pub const fn adcresult7(&self) -> &Adcresult7 {
        &self.adcresult7
    }
    #[doc = "0x10 - ADC Result 8 Register"]
    #[inline(always)]
    pub const fn adcresult8(&self) -> &Adcresult8 {
        &self.adcresult8
    }
    #[doc = "0x12 - ADC Result 9 Register"]
    #[inline(always)]
    pub const fn adcresult9(&self) -> &Adcresult9 {
        &self.adcresult9
    }
    #[doc = "0x14 - ADC Result 10 Register"]
    #[inline(always)]
    pub const fn adcresult10(&self) -> &Adcresult10 {
        &self.adcresult10
    }
    #[doc = "0x16 - ADC Result 11 Register"]
    #[inline(always)]
    pub const fn adcresult11(&self) -> &Adcresult11 {
        &self.adcresult11
    }
    #[doc = "0x18 - ADC Result 12 Register"]
    #[inline(always)]
    pub const fn adcresult12(&self) -> &Adcresult12 {
        &self.adcresult12
    }
    #[doc = "0x1a - ADC Result 13 Register"]
    #[inline(always)]
    pub const fn adcresult13(&self) -> &Adcresult13 {
        &self.adcresult13
    }
    #[doc = "0x1c - ADC Result 14 Register"]
    #[inline(always)]
    pub const fn adcresult14(&self) -> &Adcresult14 {
        &self.adcresult14
    }
    #[doc = "0x1e - ADC Result 15 Register"]
    #[inline(always)]
    pub const fn adcresult15(&self) -> &Adcresult15 {
        &self.adcresult15
    }
    #[doc = "0x40 - ADC Post Processing Block 1 Result Register"]
    #[inline(always)]
    pub const fn adcppb1result(&self) -> &Adcppb1result {
        &self.adcppb1result
    }
    #[doc = "0x44 - ADC Post Processing Block 2 Result Register"]
    #[inline(always)]
    pub const fn adcppb2result(&self) -> &Adcppb2result {
        &self.adcppb2result
    }
    #[doc = "0x48 - ADC Post Processing Block 3 Result Register"]
    #[inline(always)]
    pub const fn adcppb3result(&self) -> &Adcppb3result {
        &self.adcppb3result
    }
    #[doc = "0x4c - ADC Post Processing Block 4 Result Register"]
    #[inline(always)]
    pub const fn adcppb4result(&self) -> &Adcppb4result {
        &self.adcppb4result
    }
    #[doc = "0x50 - ADC PPB 1 Final Sum Result Register"]
    #[inline(always)]
    pub const fn adcppb1sum(&self) -> &Adcppb1sum {
        &self.adcppb1sum
    }
    #[doc = "0x54 - ADC PPB1 Final Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb1count(&self) -> &Adcppb1count {
        &self.adcppb1count
    }
    #[doc = "0x58 - ADC PPB 2 Final Sum Result Register"]
    #[inline(always)]
    pub const fn adcppb2sum(&self) -> &Adcppb2sum {
        &self.adcppb2sum
    }
    #[doc = "0x5c - ADC PPB2 Final Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb2count(&self) -> &Adcppb2count {
        &self.adcppb2count
    }
    #[doc = "0x60 - ADC PPB 3 Final Sum Result Register"]
    #[inline(always)]
    pub const fn adcppb3sum(&self) -> &Adcppb3sum {
        &self.adcppb3sum
    }
    #[doc = "0x64 - ADC PPB3 Final Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb3count(&self) -> &Adcppb3count {
        &self.adcppb3count
    }
    #[doc = "0x68 - ADC PPB 4 Final Sum Result Register"]
    #[inline(always)]
    pub const fn adcppb4sum(&self) -> &Adcppb4sum {
        &self.adcppb4sum
    }
    #[doc = "0x6c - ADC PPB4 Final Conversion Count Register"]
    #[inline(always)]
    pub const fn adcppb4count(&self) -> &Adcppb4count {
        &self.adcppb4count
    }
    #[doc = "0xc0 - ADC Sequence 1 FIFO Result Register"]
    #[inline(always)]
    pub const fn adcseq1fiforesult(&self) -> &Adcseq1fiforesult {
        &self.adcseq1fiforesult
    }
    #[doc = "0xc4 - ADC Sequence 2 FIFO Result Register"]
    #[inline(always)]
    pub const fn adcseq2fiforesult(&self) -> &Adcseq2fiforesult {
        &self.adcseq2fiforesult
    }
    #[doc = "0xc8 - ADC Sequence 3 FIFO Result Register"]
    #[inline(always)]
    pub const fn adcseq3fiforesult(&self) -> &Adcseq3fiforesult {
        &self.adcseq3fiforesult
    }
    #[doc = "0xcc - ADC Sequence 4 FIFO Result Register"]
    #[inline(always)]
    pub const fn adcseq4fiforesult(&self) -> &Adcseq4fiforesult {
        &self.adcseq4fiforesult
    }
    #[doc = "0xd0 - ADC Sequence 1 FIFO Status"]
    #[inline(always)]
    pub const fn adcseq1fifostatus(&self) -> &Adcseq1fifostatus {
        &self.adcseq1fifostatus
    }
    #[doc = "0xd4 - ADC Sequence 2 FIFO Status"]
    #[inline(always)]
    pub const fn adcseq2fifostatus(&self) -> &Adcseq2fifostatus {
        &self.adcseq2fifostatus
    }
    #[doc = "0xd8 - ADC Sequence 3 FIFO Status"]
    #[inline(always)]
    pub const fn adcseq3fifostatus(&self) -> &Adcseq3fifostatus {
        &self.adcseq3fifostatus
    }
    #[doc = "0xdc - ADC Sequence 4 FIFO Status"]
    #[inline(always)]
    pub const fn adcseq4fifostatus(&self) -> &Adcseq4fifostatus {
        &self.adcseq4fifostatus
    }
}
#[doc = "ADCRESULT0 (rw) register accessor: ADC Result 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult0`] module"]
#[doc(alias = "ADCRESULT0")]
pub type Adcresult0 = crate::Reg<adcresult0::Adcresult0Spec>;
#[doc = "ADC Result 0 Register"]
pub mod adcresult0;
#[doc = "ADCRESULT1 (rw) register accessor: ADC Result 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult1`] module"]
#[doc(alias = "ADCRESULT1")]
pub type Adcresult1 = crate::Reg<adcresult1::Adcresult1Spec>;
#[doc = "ADC Result 1 Register"]
pub mod adcresult1;
#[doc = "ADCRESULT2 (rw) register accessor: ADC Result 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult2`] module"]
#[doc(alias = "ADCRESULT2")]
pub type Adcresult2 = crate::Reg<adcresult2::Adcresult2Spec>;
#[doc = "ADC Result 2 Register"]
pub mod adcresult2;
#[doc = "ADCRESULT3 (rw) register accessor: ADC Result 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult3`] module"]
#[doc(alias = "ADCRESULT3")]
pub type Adcresult3 = crate::Reg<adcresult3::Adcresult3Spec>;
#[doc = "ADC Result 3 Register"]
pub mod adcresult3;
#[doc = "ADCRESULT4 (rw) register accessor: ADC Result 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult4`] module"]
#[doc(alias = "ADCRESULT4")]
pub type Adcresult4 = crate::Reg<adcresult4::Adcresult4Spec>;
#[doc = "ADC Result 4 Register"]
pub mod adcresult4;
#[doc = "ADCRESULT5 (rw) register accessor: ADC Result 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult5`] module"]
#[doc(alias = "ADCRESULT5")]
pub type Adcresult5 = crate::Reg<adcresult5::Adcresult5Spec>;
#[doc = "ADC Result 5 Register"]
pub mod adcresult5;
#[doc = "ADCRESULT6 (rw) register accessor: ADC Result 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult6`] module"]
#[doc(alias = "ADCRESULT6")]
pub type Adcresult6 = crate::Reg<adcresult6::Adcresult6Spec>;
#[doc = "ADC Result 6 Register"]
pub mod adcresult6;
#[doc = "ADCRESULT7 (rw) register accessor: ADC Result 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult7`] module"]
#[doc(alias = "ADCRESULT7")]
pub type Adcresult7 = crate::Reg<adcresult7::Adcresult7Spec>;
#[doc = "ADC Result 7 Register"]
pub mod adcresult7;
#[doc = "ADCRESULT8 (rw) register accessor: ADC Result 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult8`] module"]
#[doc(alias = "ADCRESULT8")]
pub type Adcresult8 = crate::Reg<adcresult8::Adcresult8Spec>;
#[doc = "ADC Result 8 Register"]
pub mod adcresult8;
#[doc = "ADCRESULT9 (rw) register accessor: ADC Result 9 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult9`] module"]
#[doc(alias = "ADCRESULT9")]
pub type Adcresult9 = crate::Reg<adcresult9::Adcresult9Spec>;
#[doc = "ADC Result 9 Register"]
pub mod adcresult9;
#[doc = "ADCRESULT10 (rw) register accessor: ADC Result 10 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult10`] module"]
#[doc(alias = "ADCRESULT10")]
pub type Adcresult10 = crate::Reg<adcresult10::Adcresult10Spec>;
#[doc = "ADC Result 10 Register"]
pub mod adcresult10;
#[doc = "ADCRESULT11 (rw) register accessor: ADC Result 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult11`] module"]
#[doc(alias = "ADCRESULT11")]
pub type Adcresult11 = crate::Reg<adcresult11::Adcresult11Spec>;
#[doc = "ADC Result 11 Register"]
pub mod adcresult11;
#[doc = "ADCRESULT12 (rw) register accessor: ADC Result 12 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult12`] module"]
#[doc(alias = "ADCRESULT12")]
pub type Adcresult12 = crate::Reg<adcresult12::Adcresult12Spec>;
#[doc = "ADC Result 12 Register"]
pub mod adcresult12;
#[doc = "ADCRESULT13 (rw) register accessor: ADC Result 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult13`] module"]
#[doc(alias = "ADCRESULT13")]
pub type Adcresult13 = crate::Reg<adcresult13::Adcresult13Spec>;
#[doc = "ADC Result 13 Register"]
pub mod adcresult13;
#[doc = "ADCRESULT14 (rw) register accessor: ADC Result 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult14`] module"]
#[doc(alias = "ADCRESULT14")]
pub type Adcresult14 = crate::Reg<adcresult14::Adcresult14Spec>;
#[doc = "ADC Result 14 Register"]
pub mod adcresult14;
#[doc = "ADCRESULT15 (rw) register accessor: ADC Result 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcresult15`] module"]
#[doc(alias = "ADCRESULT15")]
pub type Adcresult15 = crate::Reg<adcresult15::Adcresult15Spec>;
#[doc = "ADC Result 15 Register"]
pub mod adcresult15;
#[doc = "ADCPPB1RESULT (rw) register accessor: ADC Post Processing Block 1 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1result`] module"]
#[doc(alias = "ADCPPB1RESULT")]
pub type Adcppb1result = crate::Reg<adcppb1result::Adcppb1resultSpec>;
#[doc = "ADC Post Processing Block 1 Result Register"]
pub mod adcppb1result;
#[doc = "ADCPPB2RESULT (rw) register accessor: ADC Post Processing Block 2 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2result`] module"]
#[doc(alias = "ADCPPB2RESULT")]
pub type Adcppb2result = crate::Reg<adcppb2result::Adcppb2resultSpec>;
#[doc = "ADC Post Processing Block 2 Result Register"]
pub mod adcppb2result;
#[doc = "ADCPPB3RESULT (rw) register accessor: ADC Post Processing Block 3 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3result`] module"]
#[doc(alias = "ADCPPB3RESULT")]
pub type Adcppb3result = crate::Reg<adcppb3result::Adcppb3resultSpec>;
#[doc = "ADC Post Processing Block 3 Result Register"]
pub mod adcppb3result;
#[doc = "ADCPPB4RESULT (rw) register accessor: ADC Post Processing Block 4 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4result`] module"]
#[doc(alias = "ADCPPB4RESULT")]
pub type Adcppb4result = crate::Reg<adcppb4result::Adcppb4resultSpec>;
#[doc = "ADC Post Processing Block 4 Result Register"]
pub mod adcppb4result;
#[doc = "ADCPPB1SUM (rw) register accessor: ADC PPB 1 Final Sum Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1sum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1sum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1sum`] module"]
#[doc(alias = "ADCPPB1SUM")]
pub type Adcppb1sum = crate::Reg<adcppb1sum::Adcppb1sumSpec>;
#[doc = "ADC PPB 1 Final Sum Result Register"]
pub mod adcppb1sum;
#[doc = "ADCPPB1COUNT (rw) register accessor: ADC PPB1 Final Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb1count`] module"]
#[doc(alias = "ADCPPB1COUNT")]
pub type Adcppb1count = crate::Reg<adcppb1count::Adcppb1countSpec>;
#[doc = "ADC PPB1 Final Conversion Count Register"]
pub mod adcppb1count;
#[doc = "ADCPPB2SUM (rw) register accessor: ADC PPB 2 Final Sum Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2sum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2sum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2sum`] module"]
#[doc(alias = "ADCPPB2SUM")]
pub type Adcppb2sum = crate::Reg<adcppb2sum::Adcppb2sumSpec>;
#[doc = "ADC PPB 2 Final Sum Result Register"]
pub mod adcppb2sum;
#[doc = "ADCPPB2COUNT (rw) register accessor: ADC PPB2 Final Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb2count`] module"]
#[doc(alias = "ADCPPB2COUNT")]
pub type Adcppb2count = crate::Reg<adcppb2count::Adcppb2countSpec>;
#[doc = "ADC PPB2 Final Conversion Count Register"]
pub mod adcppb2count;
#[doc = "ADCPPB3SUM (rw) register accessor: ADC PPB 3 Final Sum Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3sum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3sum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3sum`] module"]
#[doc(alias = "ADCPPB3SUM")]
pub type Adcppb3sum = crate::Reg<adcppb3sum::Adcppb3sumSpec>;
#[doc = "ADC PPB 3 Final Sum Result Register"]
pub mod adcppb3sum;
#[doc = "ADCPPB3COUNT (rw) register accessor: ADC PPB3 Final Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb3count`] module"]
#[doc(alias = "ADCPPB3COUNT")]
pub type Adcppb3count = crate::Reg<adcppb3count::Adcppb3countSpec>;
#[doc = "ADC PPB3 Final Conversion Count Register"]
pub mod adcppb3count;
#[doc = "ADCPPB4SUM (rw) register accessor: ADC PPB 4 Final Sum Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4sum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4sum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4sum`] module"]
#[doc(alias = "ADCPPB4SUM")]
pub type Adcppb4sum = crate::Reg<adcppb4sum::Adcppb4sumSpec>;
#[doc = "ADC PPB 4 Final Sum Result Register"]
pub mod adcppb4sum;
#[doc = "ADCPPB4COUNT (rw) register accessor: ADC PPB4 Final Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcppb4count`] module"]
#[doc(alias = "ADCPPB4COUNT")]
pub type Adcppb4count = crate::Reg<adcppb4count::Adcppb4countSpec>;
#[doc = "ADC PPB4 Final Conversion Count Register"]
pub mod adcppb4count;
#[doc = "ADCSEQ1FIFORESULT (rw) register accessor: ADC Sequence 1 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq1fiforesult::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq1fiforesult::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq1fiforesult`] module"]
#[doc(alias = "ADCSEQ1FIFORESULT")]
pub type Adcseq1fiforesult = crate::Reg<adcseq1fiforesult::Adcseq1fiforesultSpec>;
#[doc = "ADC Sequence 1 FIFO Result Register"]
pub mod adcseq1fiforesult;
#[doc = "ADCSEQ2FIFORESULT (rw) register accessor: ADC Sequence 2 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq2fiforesult::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq2fiforesult::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq2fiforesult`] module"]
#[doc(alias = "ADCSEQ2FIFORESULT")]
pub type Adcseq2fiforesult = crate::Reg<adcseq2fiforesult::Adcseq2fiforesultSpec>;
#[doc = "ADC Sequence 2 FIFO Result Register"]
pub mod adcseq2fiforesult;
#[doc = "ADCSEQ3FIFORESULT (rw) register accessor: ADC Sequence 3 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq3fiforesult::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq3fiforesult::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq3fiforesult`] module"]
#[doc(alias = "ADCSEQ3FIFORESULT")]
pub type Adcseq3fiforesult = crate::Reg<adcseq3fiforesult::Adcseq3fiforesultSpec>;
#[doc = "ADC Sequence 3 FIFO Result Register"]
pub mod adcseq3fiforesult;
#[doc = "ADCSEQ4FIFORESULT (rw) register accessor: ADC Sequence 4 FIFO Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq4fiforesult::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq4fiforesult::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq4fiforesult`] module"]
#[doc(alias = "ADCSEQ4FIFORESULT")]
pub type Adcseq4fiforesult = crate::Reg<adcseq4fiforesult::Adcseq4fiforesultSpec>;
#[doc = "ADC Sequence 4 FIFO Result Register"]
pub mod adcseq4fiforesult;
#[doc = "ADCSEQ1FIFOSTATUS (rw) register accessor: ADC Sequence 1 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq1fifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq1fifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq1fifostatus`] module"]
#[doc(alias = "ADCSEQ1FIFOSTATUS")]
pub type Adcseq1fifostatus = crate::Reg<adcseq1fifostatus::Adcseq1fifostatusSpec>;
#[doc = "ADC Sequence 1 FIFO Status"]
pub mod adcseq1fifostatus;
#[doc = "ADCSEQ2FIFOSTATUS (rw) register accessor: ADC Sequence 2 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq2fifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq2fifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq2fifostatus`] module"]
#[doc(alias = "ADCSEQ2FIFOSTATUS")]
pub type Adcseq2fifostatus = crate::Reg<adcseq2fifostatus::Adcseq2fifostatusSpec>;
#[doc = "ADC Sequence 2 FIFO Status"]
pub mod adcseq2fifostatus;
#[doc = "ADCSEQ3FIFOSTATUS (rw) register accessor: ADC Sequence 3 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq3fifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq3fifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq3fifostatus`] module"]
#[doc(alias = "ADCSEQ3FIFOSTATUS")]
pub type Adcseq3fifostatus = crate::Reg<adcseq3fifostatus::Adcseq3fifostatusSpec>;
#[doc = "ADC Sequence 3 FIFO Status"]
pub mod adcseq3fifostatus;
#[doc = "ADCSEQ4FIFOSTATUS (rw) register accessor: ADC Sequence 4 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseq4fifostatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseq4fifostatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseq4fifostatus`] module"]
#[doc(alias = "ADCSEQ4FIFOSTATUS")]
pub type Adcseq4fifostatus = crate::Reg<adcseq4fifostatus::Adcseq4fifostatusSpec>;
#[doc = "ADC Sequence 4 FIFO Status"]
pub mod adcseq4fifostatus;
