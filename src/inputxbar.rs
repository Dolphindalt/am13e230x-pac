#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    inputselect: [Inputselect; 28],
    _reserved1: [u8; 0x0390],
    inputselectlock1: Inputselectlock1,
    _reserved2: [u8; 0x0c],
    raw_inputselect_pa: RawInputselectPa,
    raw_inputselect_pb: RawInputselectPb,
    raw_inputselect_pc: RawInputselectPc,
    raw_inputselect_pd: RawInputselectPd,
}
impl RegisterBlock {
    #[doc = "0x00..0x70 - Input Select Register (GPIO0 to x)"]
    #[inline(always)]
    pub const fn inputselect(&self, n: usize) -> &Inputselect {
        &self.inputselect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x70 - Input Select Register (GPIO0 to x)"]
    #[inline(always)]
    pub fn inputselect_iter(&self) -> impl Iterator<Item = &Inputselect> {
        self.inputselect.iter()
    }
    #[doc = "0x400 - Input Select Lock Register 1"]
    #[inline(always)]
    pub const fn inputselectlock1(&self) -> &Inputselectlock1 {
        &self.inputselectlock1
    }
    #[doc = "0x410 - Raw Input Select Register for Port-A GPIOs"]
    #[inline(always)]
    pub const fn raw_inputselect_pa(&self) -> &RawInputselectPa {
        &self.raw_inputselect_pa
    }
    #[doc = "0x414 - Raw Input Select Register for Port-B GPIOs"]
    #[inline(always)]
    pub const fn raw_inputselect_pb(&self) -> &RawInputselectPb {
        &self.raw_inputselect_pb
    }
    #[doc = "0x418 - Raw Input Select Register for Port-C GPIOs"]
    #[inline(always)]
    pub const fn raw_inputselect_pc(&self) -> &RawInputselectPc {
        &self.raw_inputselect_pc
    }
    #[doc = "0x41c - Raw Input Select Register for Port-D GPIOs"]
    #[inline(always)]
    pub const fn raw_inputselect_pd(&self) -> &RawInputselectPd {
        &self.raw_inputselect_pd
    }
}
#[doc = "INPUTSELECT (rw) register accessor: Input Select Register (GPIO0 to x)\n\nYou can [`read`](crate::Reg::read) this register and get [`inputselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputselect`] module"]
#[doc(alias = "INPUTSELECT")]
pub type Inputselect = crate::Reg<inputselect::InputselectSpec>;
#[doc = "Input Select Register (GPIO0 to x)"]
pub mod inputselect;
#[doc = "INPUTSELECTLOCK1 (rw) register accessor: Input Select Lock Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`inputselectlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputselectlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputselectlock1`] module"]
#[doc(alias = "INPUTSELECTLOCK1")]
pub type Inputselectlock1 = crate::Reg<inputselectlock1::Inputselectlock1Spec>;
#[doc = "Input Select Lock Register 1"]
pub mod inputselectlock1;
#[doc = "RAW_INPUTSELECT_PA (rw) register accessor: Raw Input Select Register for Port-A GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_inputselect_pa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_inputselect_pa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_inputselect_pa`] module"]
#[doc(alias = "RAW_INPUTSELECT_PA")]
pub type RawInputselectPa = crate::Reg<raw_inputselect_pa::RawInputselectPaSpec>;
#[doc = "Raw Input Select Register for Port-A GPIOs"]
pub mod raw_inputselect_pa;
#[doc = "RAW_INPUTSELECT_PB (rw) register accessor: Raw Input Select Register for Port-B GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_inputselect_pb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_inputselect_pb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_inputselect_pb`] module"]
#[doc(alias = "RAW_INPUTSELECT_PB")]
pub type RawInputselectPb = crate::Reg<raw_inputselect_pb::RawInputselectPbSpec>;
#[doc = "Raw Input Select Register for Port-B GPIOs"]
pub mod raw_inputselect_pb;
#[doc = "RAW_INPUTSELECT_PC (rw) register accessor: Raw Input Select Register for Port-C GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_inputselect_pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_inputselect_pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_inputselect_pc`] module"]
#[doc(alias = "RAW_INPUTSELECT_PC")]
pub type RawInputselectPc = crate::Reg<raw_inputselect_pc::RawInputselectPcSpec>;
#[doc = "Raw Input Select Register for Port-C GPIOs"]
pub mod raw_inputselect_pc;
#[doc = "RAW_INPUTSELECT_PD (rw) register accessor: Raw Input Select Register for Port-D GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_inputselect_pd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_inputselect_pd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_inputselect_pd`] module"]
#[doc(alias = "RAW_INPUTSELECT_PD")]
pub type RawInputselectPd = crate::Reg<raw_inputselect_pd::RawInputselectPdSpec>;
#[doc = "Raw Input Select Register for Port-D GPIOs"]
pub mod raw_inputselect_pd;
