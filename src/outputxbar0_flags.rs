#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    outputxbarstatus: Outputxbarstatus,
    _reserved1: [u8; 0x04],
    outputxbarflag: Outputxbarflag,
    outputxbarflagclear: Outputxbarflagclear,
    outputxbarflagforce: Outputxbarflagforce,
}
impl RegisterBlock {
    #[doc = "0x10 - Output Signal Status register"]
    #[inline(always)]
    pub const fn outputxbarstatus(&self) -> &Outputxbarstatus {
        &self.outputxbarstatus
    }
    #[doc = "0x18 - Output latched flag register"]
    #[inline(always)]
    pub const fn outputxbarflag(&self) -> &Outputxbarflag {
        &self.outputxbarflag
    }
    #[doc = "0x1c - Output latched flag clear register"]
    #[inline(always)]
    pub const fn outputxbarflagclear(&self) -> &Outputxbarflagclear {
        &self.outputxbarflagclear
    }
    #[doc = "0x20 - Output latched flag Force register"]
    #[inline(always)]
    pub const fn outputxbarflagforce(&self) -> &Outputxbarflagforce {
        &self.outputxbarflagforce
    }
}
#[doc = "OUTPUTXBARSTATUS (rw) register accessor: Output Signal Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarstatus`] module"]
#[doc(alias = "OUTPUTXBARSTATUS")]
pub type Outputxbarstatus = crate::Reg<outputxbarstatus::OutputxbarstatusSpec>;
#[doc = "Output Signal Status register"]
pub mod outputxbarstatus;
#[doc = "OUTPUTXBARFLAG (rw) register accessor: Output latched flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarflag`] module"]
#[doc(alias = "OUTPUTXBARFLAG")]
pub type Outputxbarflag = crate::Reg<outputxbarflag::OutputxbarflagSpec>;
#[doc = "Output latched flag register"]
pub mod outputxbarflag;
#[doc = "OUTPUTXBARFLAGCLEAR (rw) register accessor: Output latched flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflagclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflagclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarflagclear`] module"]
#[doc(alias = "OUTPUTXBARFLAGCLEAR")]
pub type Outputxbarflagclear = crate::Reg<outputxbarflagclear::OutputxbarflagclearSpec>;
#[doc = "Output latched flag clear register"]
pub mod outputxbarflagclear;
#[doc = "OUTPUTXBARFLAGFORCE (rw) register accessor: Output latched flag Force register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflagforce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflagforce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputxbarflagforce`] module"]
#[doc(alias = "OUTPUTXBARFLAGFORCE")]
pub type Outputxbarflagforce = crate::Reg<outputxbarflagforce::OutputxbarflagforceSpec>;
#[doc = "Output latched flag Force register"]
pub mod outputxbarflagforce;
