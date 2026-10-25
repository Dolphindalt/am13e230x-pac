#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    revision: Revision,
    _reserved1: [u8; 0x0c],
    security_err_flag: SecurityErrFlag,
    security_err_clr: SecurityErrClr,
    security_err_mstid: SecurityErrMstid,
    security_err_addr: SecurityErrAddr,
    _reserved5: [u8; 0x10],
    fri_sec_flag: FriSecFlag,
    fri_sec_clr: FriSecClr,
    fri_sec_addr: FriSecAddr,
    fri_sec_mstid: FriSecMstid,
    _reserved9: [u8; 0x10],
    fri_ded_flag: FriDedFlag,
    fri_ded_clr: FriDedClr,
    fri_ded_addr: FriDedAddr,
    fri_ded_mstid: FriDedMstid,
    _reserved13: [u8; 0x50],
    sysmem_par_flag: SysmemParFlag,
    sysmem_par_clr: SysmemParClr,
    sysmem_par_addr: SysmemParAddr,
    sysmem_par_mstid: SysmemParMstid,
    _reserved17: [u8; 0x10],
    tmurom_par_flag: TmuromParFlag,
    tmurom_par_clr: TmuromParClr,
    tmurom_par_addr: TmuromParAddr,
    tmurom_par_type: TmuromParType,
    _reserved21: [u8; 0x10],
    sysmem_acc_flag: SysmemAccFlag,
    sysmem_acc_clr: SysmemAccClr,
    sysmem_acc_addr: SysmemAccAddr,
    sysmem_acc_type: SysmemAccType,
}
impl RegisterBlock {
    #[doc = "0x1000 - IP revision id register"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x1010 - Security error flag"]
    #[inline(always)]
    pub const fn security_err_flag(&self) -> &SecurityErrFlag {
        &self.security_err_flag
    }
    #[doc = "0x1014 - Security error clear"]
    #[inline(always)]
    pub const fn security_err_clr(&self) -> &SecurityErrClr {
        &self.security_err_clr
    }
    #[doc = "0x1018 - Security error master id"]
    #[inline(always)]
    pub const fn security_err_mstid(&self) -> &SecurityErrMstid {
        &self.security_err_mstid
    }
    #[doc = "0x101c - Security error address"]
    #[inline(always)]
    pub const fn security_err_addr(&self) -> &SecurityErrAddr {
        &self.security_err_addr
    }
    #[doc = "0x1030 - FRI SEC flag"]
    #[inline(always)]
    pub const fn fri_sec_flag(&self) -> &FriSecFlag {
        &self.fri_sec_flag
    }
    #[doc = "0x1034 - FRI SEC clear"]
    #[inline(always)]
    pub const fn fri_sec_clr(&self) -> &FriSecClr {
        &self.fri_sec_clr
    }
    #[doc = "0x1038 - FRI SEC address"]
    #[inline(always)]
    pub const fn fri_sec_addr(&self) -> &FriSecAddr {
        &self.fri_sec_addr
    }
    #[doc = "0x103c - FRI SEC master id"]
    #[inline(always)]
    pub const fn fri_sec_mstid(&self) -> &FriSecMstid {
        &self.fri_sec_mstid
    }
    #[doc = "0x1050 - FRI DED flag"]
    #[inline(always)]
    pub const fn fri_ded_flag(&self) -> &FriDedFlag {
        &self.fri_ded_flag
    }
    #[doc = "0x1054 - FRI DED clear"]
    #[inline(always)]
    pub const fn fri_ded_clr(&self) -> &FriDedClr {
        &self.fri_ded_clr
    }
    #[doc = "0x1058 - FRI DED address"]
    #[inline(always)]
    pub const fn fri_ded_addr(&self) -> &FriDedAddr {
        &self.fri_ded_addr
    }
    #[doc = "0x105c - FRI DED master id"]
    #[inline(always)]
    pub const fn fri_ded_mstid(&self) -> &FriDedMstid {
        &self.fri_ded_mstid
    }
    #[doc = "0x10b0 - SYSMEM DED flag"]
    #[inline(always)]
    pub const fn sysmem_par_flag(&self) -> &SysmemParFlag {
        &self.sysmem_par_flag
    }
    #[doc = "0x10b4 - SYSMEM DED clear"]
    #[inline(always)]
    pub const fn sysmem_par_clr(&self) -> &SysmemParClr {
        &self.sysmem_par_clr
    }
    #[doc = "0x10b8 - SYSMEM DED address"]
    #[inline(always)]
    pub const fn sysmem_par_addr(&self) -> &SysmemParAddr {
        &self.sysmem_par_addr
    }
    #[doc = "0x10bc - SYSMEM DED master id"]
    #[inline(always)]
    pub const fn sysmem_par_mstid(&self) -> &SysmemParMstid {
        &self.sysmem_par_mstid
    }
    #[doc = "0x10d0 - SYSMEM DED flag"]
    #[inline(always)]
    pub const fn tmurom_par_flag(&self) -> &TmuromParFlag {
        &self.tmurom_par_flag
    }
    #[doc = "0x10d4 - SYSMEM DED clear"]
    #[inline(always)]
    pub const fn tmurom_par_clr(&self) -> &TmuromParClr {
        &self.tmurom_par_clr
    }
    #[doc = "0x10d8 - SYSMEM DED address"]
    #[inline(always)]
    pub const fn tmurom_par_addr(&self) -> &TmuromParAddr {
        &self.tmurom_par_addr
    }
    #[doc = "0x10dc - SYSMEM DED master id"]
    #[inline(always)]
    pub const fn tmurom_par_type(&self) -> &TmuromParType {
        &self.tmurom_par_type
    }
    #[doc = "0x10f0 - SYSMEM access error flag"]
    #[inline(always)]
    pub const fn sysmem_acc_flag(&self) -> &SysmemAccFlag {
        &self.sysmem_acc_flag
    }
    #[doc = "0x10f4 - SYSMEM access error clear"]
    #[inline(always)]
    pub const fn sysmem_acc_clr(&self) -> &SysmemAccClr {
        &self.sysmem_acc_clr
    }
    #[doc = "0x10f8 - SYSMEM access error address"]
    #[inline(always)]
    pub const fn sysmem_acc_addr(&self) -> &SysmemAccAddr {
        &self.sysmem_acc_addr
    }
    #[doc = "0x10fc - SYSMEM access error type"]
    #[inline(always)]
    pub const fn sysmem_acc_type(&self) -> &SysmemAccType {
        &self.sysmem_acc_type
    }
}
#[doc = "REVISION (rw) register accessor: IP revision id register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`] module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "IP revision id register"]
pub mod revision;
#[doc = "SECURITY_ERR_FLAG (rw) register accessor: Security error flag\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@security_err_flag`] module"]
#[doc(alias = "SECURITY_ERR_FLAG")]
pub type SecurityErrFlag = crate::Reg<security_err_flag::SecurityErrFlagSpec>;
#[doc = "Security error flag"]
pub mod security_err_flag;
#[doc = "SECURITY_ERR_CLR (rw) register accessor: Security error clear\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@security_err_clr`] module"]
#[doc(alias = "SECURITY_ERR_CLR")]
pub type SecurityErrClr = crate::Reg<security_err_clr::SecurityErrClrSpec>;
#[doc = "Security error clear"]
pub mod security_err_clr;
#[doc = "SECURITY_ERR_MSTID (rw) register accessor: Security error master id\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@security_err_mstid`] module"]
#[doc(alias = "SECURITY_ERR_MSTID")]
pub type SecurityErrMstid = crate::Reg<security_err_mstid::SecurityErrMstidSpec>;
#[doc = "Security error master id"]
pub mod security_err_mstid;
#[doc = "SECURITY_ERR_ADDR (rw) register accessor: Security error address\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@security_err_addr`] module"]
#[doc(alias = "SECURITY_ERR_ADDR")]
pub type SecurityErrAddr = crate::Reg<security_err_addr::SecurityErrAddrSpec>;
#[doc = "Security error address"]
pub mod security_err_addr;
#[doc = "FRI_SEC_FLAG (rw) register accessor: FRI SEC flag\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_sec_flag`] module"]
#[doc(alias = "FRI_SEC_FLAG")]
pub type FriSecFlag = crate::Reg<fri_sec_flag::FriSecFlagSpec>;
#[doc = "FRI SEC flag"]
pub mod fri_sec_flag;
#[doc = "FRI_SEC_CLR (rw) register accessor: FRI SEC clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_sec_clr`] module"]
#[doc(alias = "FRI_SEC_CLR")]
pub type FriSecClr = crate::Reg<fri_sec_clr::FriSecClrSpec>;
#[doc = "FRI SEC clear"]
pub mod fri_sec_clr;
#[doc = "FRI_SEC_ADDR (rw) register accessor: FRI SEC address\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_sec_addr`] module"]
#[doc(alias = "FRI_SEC_ADDR")]
pub type FriSecAddr = crate::Reg<fri_sec_addr::FriSecAddrSpec>;
#[doc = "FRI SEC address"]
pub mod fri_sec_addr;
#[doc = "FRI_SEC_MSTID (rw) register accessor: FRI SEC master id\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_sec_mstid`] module"]
#[doc(alias = "FRI_SEC_MSTID")]
pub type FriSecMstid = crate::Reg<fri_sec_mstid::FriSecMstidSpec>;
#[doc = "FRI SEC master id"]
pub mod fri_sec_mstid;
#[doc = "FRI_DED_FLAG (rw) register accessor: FRI DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_ded_flag`] module"]
#[doc(alias = "FRI_DED_FLAG")]
pub type FriDedFlag = crate::Reg<fri_ded_flag::FriDedFlagSpec>;
#[doc = "FRI DED flag"]
pub mod fri_ded_flag;
#[doc = "FRI_DED_CLR (rw) register accessor: FRI DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_ded_clr`] module"]
#[doc(alias = "FRI_DED_CLR")]
pub type FriDedClr = crate::Reg<fri_ded_clr::FriDedClrSpec>;
#[doc = "FRI DED clear"]
pub mod fri_ded_clr;
#[doc = "FRI_DED_ADDR (rw) register accessor: FRI DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_ded_addr`] module"]
#[doc(alias = "FRI_DED_ADDR")]
pub type FriDedAddr = crate::Reg<fri_ded_addr::FriDedAddrSpec>;
#[doc = "FRI DED address"]
pub mod fri_ded_addr;
#[doc = "FRI_DED_MSTID (rw) register accessor: FRI DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fri_ded_mstid`] module"]
#[doc(alias = "FRI_DED_MSTID")]
pub type FriDedMstid = crate::Reg<fri_ded_mstid::FriDedMstidSpec>;
#[doc = "FRI DED master id"]
pub mod fri_ded_mstid;
#[doc = "SYSMEM_PAR_FLAG (rw) register accessor: SYSMEM DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_par_flag`] module"]
#[doc(alias = "SYSMEM_PAR_FLAG")]
pub type SysmemParFlag = crate::Reg<sysmem_par_flag::SysmemParFlagSpec>;
#[doc = "SYSMEM DED flag"]
pub mod sysmem_par_flag;
#[doc = "SYSMEM_PAR_CLR (rw) register accessor: SYSMEM DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_par_clr`] module"]
#[doc(alias = "SYSMEM_PAR_CLR")]
pub type SysmemParClr = crate::Reg<sysmem_par_clr::SysmemParClrSpec>;
#[doc = "SYSMEM DED clear"]
pub mod sysmem_par_clr;
#[doc = "SYSMEM_PAR_ADDR (rw) register accessor: SYSMEM DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_par_addr`] module"]
#[doc(alias = "SYSMEM_PAR_ADDR")]
pub type SysmemParAddr = crate::Reg<sysmem_par_addr::SysmemParAddrSpec>;
#[doc = "SYSMEM DED address"]
pub mod sysmem_par_addr;
#[doc = "SYSMEM_PAR_MSTID (rw) register accessor: SYSMEM DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_par_mstid`] module"]
#[doc(alias = "SYSMEM_PAR_MSTID")]
pub type SysmemParMstid = crate::Reg<sysmem_par_mstid::SysmemParMstidSpec>;
#[doc = "SYSMEM DED master id"]
pub mod sysmem_par_mstid;
#[doc = "TMUROM_PAR_FLAG (rw) register accessor: SYSMEM DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmurom_par_flag`] module"]
#[doc(alias = "TMUROM_PAR_FLAG")]
pub type TmuromParFlag = crate::Reg<tmurom_par_flag::TmuromParFlagSpec>;
#[doc = "SYSMEM DED flag"]
pub mod tmurom_par_flag;
#[doc = "TMUROM_PAR_CLR (rw) register accessor: SYSMEM DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmurom_par_clr`] module"]
#[doc(alias = "TMUROM_PAR_CLR")]
pub type TmuromParClr = crate::Reg<tmurom_par_clr::TmuromParClrSpec>;
#[doc = "SYSMEM DED clear"]
pub mod tmurom_par_clr;
#[doc = "TMUROM_PAR_ADDR (rw) register accessor: SYSMEM DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmurom_par_addr`] module"]
#[doc(alias = "TMUROM_PAR_ADDR")]
pub type TmuromParAddr = crate::Reg<tmurom_par_addr::TmuromParAddrSpec>;
#[doc = "SYSMEM DED address"]
pub mod tmurom_par_addr;
#[doc = "TMUROM_PAR_TYPE (rw) register accessor: SYSMEM DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmurom_par_type`] module"]
#[doc(alias = "TMUROM_PAR_TYPE")]
pub type TmuromParType = crate::Reg<tmurom_par_type::TmuromParTypeSpec>;
#[doc = "SYSMEM DED master id"]
pub mod tmurom_par_type;
#[doc = "SYSMEM_ACC_FLAG (rw) register accessor: SYSMEM access error flag\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_acc_flag`] module"]
#[doc(alias = "SYSMEM_ACC_FLAG")]
pub type SysmemAccFlag = crate::Reg<sysmem_acc_flag::SysmemAccFlagSpec>;
#[doc = "SYSMEM access error flag"]
pub mod sysmem_acc_flag;
#[doc = "SYSMEM_ACC_CLR (rw) register accessor: SYSMEM access error clear\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_acc_clr`] module"]
#[doc(alias = "SYSMEM_ACC_CLR")]
pub type SysmemAccClr = crate::Reg<sysmem_acc_clr::SysmemAccClrSpec>;
#[doc = "SYSMEM access error clear"]
pub mod sysmem_acc_clr;
#[doc = "SYSMEM_ACC_ADDR (rw) register accessor: SYSMEM access error address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_acc_addr`] module"]
#[doc(alias = "SYSMEM_ACC_ADDR")]
pub type SysmemAccAddr = crate::Reg<sysmem_acc_addr::SysmemAccAddrSpec>;
#[doc = "SYSMEM access error address"]
pub mod sysmem_acc_addr;
#[doc = "SYSMEM_ACC_TYPE (rw) register accessor: SYSMEM access error type\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysmem_acc_type`] module"]
#[doc(alias = "SYSMEM_ACC_TYPE")]
pub type SysmemAccType = crate::Reg<sysmem_acc_type::SysmemAccTypeSpec>;
#[doc = "SYSMEM access error type"]
pub mod sysmem_acc_type;
