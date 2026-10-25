#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1100],
    cfg: Cfg,
    keywr: Keywr,
    keyrd: Keyrd,
    status: Status,
    keyin: Keyin,
    keylock: Keylock,
}
impl RegisterBlock {
    #[doc = "0x1100 - Keystore configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x1104 - Key write configuration"]
    #[inline(always)]
    pub const fn keywr(&self) -> &Keywr {
        &self.keywr
    }
    #[doc = "0x1108 - Key read configuration"]
    #[inline(always)]
    pub const fn keyrd(&self) -> &Keyrd {
        &self.keyrd
    }
    #[doc = "0x110c - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1110 - Input key"]
    #[inline(always)]
    pub const fn keyin(&self) -> &Keyin {
        &self.keyin
    }
    #[doc = "0x1114 - Keylock"]
    #[inline(always)]
    pub const fn keylock(&self) -> &Keylock {
        &self.keylock
    }
}
#[doc = "CFG (rw) register accessor: Keystore configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Keystore configuration"]
pub mod cfg;
#[doc = "KEYWR (rw) register accessor: Key write configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keywr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keywr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywr`] module"]
#[doc(alias = "KEYWR")]
pub type Keywr = crate::Reg<keywr::KeywrSpec>;
#[doc = "Key write configuration"]
pub mod keywr;
#[doc = "KEYRD (rw) register accessor: Key read configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keyrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyrd`] module"]
#[doc(alias = "KEYRD")]
pub type Keyrd = crate::Reg<keyrd::KeyrdSpec>;
#[doc = "Key read configuration"]
pub mod keyrd;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "KEYIN (w) register accessor: Input key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyin`] module"]
#[doc(alias = "KEYIN")]
pub type Keyin = crate::Reg<keyin::KeyinSpec>;
#[doc = "Input key"]
pub mod keyin;
#[doc = "KEYLOCK (rw) register accessor: Keylock\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keylock`] module"]
#[doc(alias = "KEYLOCK")]
pub type Keylock = crate::Reg<keylock::KeylockSpec>;
#[doc = "Keylock"]
pub mod keylock;
