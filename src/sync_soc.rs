#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    syncselect: Syncselect,
    adcsocoutselect: Adcsocoutselect,
    syncsoclock: Syncsoclock,
}
impl RegisterBlock {
    #[doc = "0x00 - Sync Input and Output Select Register"]
    #[inline(always)]
    pub const fn syncselect(&self) -> &Syncselect {
        &self.syncselect
    }
    #[doc = "0x04 - External ADCSOC Select Register"]
    #[inline(always)]
    pub const fn adcsocoutselect(&self) -> &Adcsocoutselect {
        &self.adcsocoutselect
    }
    #[doc = "0x08 - SYNCSEL and EXTADCSOC Select Lock register"]
    #[inline(always)]
    pub const fn syncsoclock(&self) -> &Syncsoclock {
        &self.syncsoclock
    }
}
#[doc = "SYNCSELECT (rw) register accessor: Sync Input and Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncselect`] module"]
#[doc(alias = "SYNCSELECT")]
pub type Syncselect = crate::Reg<syncselect::SyncselectSpec>;
#[doc = "Sync Input and Output Select Register"]
pub mod syncselect;
#[doc = "ADCSOCOUTSELECT (rw) register accessor: External ADCSOC Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsocoutselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocoutselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocoutselect`] module"]
#[doc(alias = "ADCSOCOUTSELECT")]
pub type Adcsocoutselect = crate::Reg<adcsocoutselect::AdcsocoutselectSpec>;
#[doc = "External ADCSOC Select Register"]
pub mod adcsocoutselect;
#[doc = "SYNCSOCLOCK (rw) register accessor: SYNCSEL and EXTADCSOC Select Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncsoclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncsoclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncsoclock`] module"]
#[doc(alias = "SYNCSOCLOCK")]
pub type Syncsoclock = crate::Reg<syncsoclock::SyncsoclockSpec>;
#[doc = "SYNCSEL and EXTADCSOC Select Lock register"]
pub mod syncsoclock;
