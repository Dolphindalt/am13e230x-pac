#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1004],
    spc_attribviolp_config: SpcAttribviolpConfig,
    _reserved1: [u8; 0x0c],
    ppc_attribviolp_config: PpcAttribviolpConfig,
    _reserved2: [u8; 0x0c],
    fpc_attribviolp_config: FpcAttribviolpConfig,
    _reserved3: [u8; 0x18],
    fpc_hdpprot0_config: FpcHdpprot0Config,
    fpc_hdpprot1_config: FpcHdpprot1Config,
    _reserved5: [u8; 0x04],
    fpc_hdpprot_control: FpcHdpprotControl,
    fpc_hdpen_control: FpcHdpenControl,
    fpc_nonmain0_hdpprot_control: FpcNonmain0HdpprotControl,
    fpc_nonmain1_hdpprot_control: FpcNonmain1HdpprotControl,
    fpc_trim0_hdpprot_control: FpcTrim0HdpprotControl,
    fpc_trim1_hdpprot_control: FpcTrim1HdpprotControl,
    fpc_engr0_hdpprot_control: FpcEngr0HdpprotControl,
    fpc_engr1_hdpprot_control: FpcEngr1HdpprotControl,
    _reserved13: [u8; 0x98],
    ppc_privattrib_adc: PpcPrivattribAdc,
    _reserved14: [u8; 0x04],
    ppc_privattrib_timer: PpcPrivattribTimer,
    _reserved15: [u8; 0x04],
    ppc_privattrib_unicomm: PpcPrivattribUnicomm,
    _reserved16: [u8; 0x04],
    ppc_privattrib_gpio: PpcPrivattribGpio,
    _reserved17: [u8; 0x04],
    ppc_privattrib_mcan: PpcPrivattribMcan,
    _reserved18: [u8; 0x0c],
    ppc_privattrib_cmpss: PpcPrivattribCmpss,
    _reserved19: [u8; 0x04],
    ppc_privattrib_wdt: PpcPrivattribWdt,
    _reserved20: [u8; 0x0c],
    ppc_privattrib_crc: PpcPrivattribCrc,
    _reserved21: [u8; 0x04],
    ppc_privattrib_aes: PpcPrivattribAes,
    _reserved22: [u8; 0x14],
    ppc_privattrib_iomux: PpcPrivattribIomux,
    _reserved23: [u8; 0x14],
    ppc_privattrib_debugss: PpcPrivattribDebugss,
    _reserved24: [u8; 0x0c],
    ppc_privattrib_keystorectl: PpcPrivattribKeystorectl,
    _reserved25: [u8; 0x04],
    ppc_privattrib_eam: PpcPrivattribEam,
    _reserved26: [u8; 0x04],
    ppc_privattrib_wuc: PpcPrivattribWuc,
    _reserved27: [u8; 0x04],
    ppc_privattrib_friregs: PpcPrivattribFriregs,
    _reserved28: [u8; 0x04],
    ppc_privattrib_memcfg: PpcPrivattribMemcfg,
    _reserved29: [u8; 0x54],
    ppc_privattrib_sysctl: PpcPrivattribSysctl,
    _reserved30: [u8; 0x04],
    ppc_privattrib_dma: PpcPrivattribDma,
    _reserved31: [u8; 0x04],
    ppc_privattrib_gsc: PpcPrivattribGsc,
    _reserved32: [u8; 0x04],
    ppc_privattrib_coproc: PpcPrivattribCoproc,
    _reserved33: [u8; 0x04],
    ppc_privattrib_epi: PpcPrivattribEpi,
    _reserved34: [u8; 0x04],
    ppc_privattrib_pga: PpcPrivattribPga,
    _reserved35: [u8; 0x04],
    ppc_privattrib_xbar: PpcPrivattribXbar,
    _reserved36: [u8; 0x80],
    spc_privattrib0: SpcPrivattrib0,
    spc_privattrib1: SpcPrivattrib1,
    _reserved38: [u8; 0x0138],
    fpc_privattriba0: FpcPrivattriba0,
    fpc_privattriba1: FpcPrivattriba1,
    fpc_privattribb0: FpcPrivattribb0,
    fpc_privattribb1: FpcPrivattribb1,
    fpc_nonmain_privattrib0: FpcNonmainPrivattrib0,
    fpc_nonmain_privattrib1: FpcNonmainPrivattrib1,
    fpc_trim_privattrib0: FpcTrimPrivattrib0,
    fpc_trim_privattrib1: FpcTrimPrivattrib1,
    fpc_engr_privattrib0: FpcEngrPrivattrib0,
    fpc_engr_privattrib1: FpcEngrPrivattrib1,
    _reserved48: [u8; 0x01d8],
    fpc_weprota0: FpcWeprota0,
    fpc_weprota1: FpcWeprota1,
    fpc_weprotb0: FpcWeprotb0,
    fpc_weprotb1: FpcWeprotb1,
    fpc_weprotnonmain0: FpcWeprotnonmain0,
    fpc_weprotnonmain1: FpcWeprotnonmain1,
    fpc_weprottrim0: FpcWeprottrim0,
    fpc_weprottrim1: FpcWeprottrim1,
    fpc_weprotengr0: FpcWeprotengr0,
    fpc_weprotengr1: FpcWeprotengr1,
    _reserved58: [u8; 0x01d8],
    fpc_flsemreq: FpcFlsemreq,
    fpc_flsemclr: FpcFlsemclr,
    fpc_flsemstat: FpcFlsemstat,
    _reserved61: [u8; 0x0378],
    vtor_ns: VtorNs,
    _reserved62: [u8; 0xf8],
    dtb_muxsel: DtbMuxsel,
    _reserved63: [u8; 0xfc],
    lock: Lock,
    commit: Commit,
    _reserved65: [u8; 0x0274],
    revision: Revision,
}
impl RegisterBlock {
    #[doc = "0x1004 - SRAM Privilege Access Attribute Violation Configuration Register"]
    #[inline(always)]
    pub const fn spc_attribviolp_config(&self) -> &SpcAttribviolpConfig {
        &self.spc_attribviolp_config
    }
    #[doc = "0x1014 - Peripheral Privilege Access Attribute Violation Configuration Register"]
    #[inline(always)]
    pub const fn ppc_attribviolp_config(&self) -> &PpcAttribviolpConfig {
        &self.ppc_attribviolp_config
    }
    #[doc = "0x1024 - Flash Privilege Access Attribute Violation Configuration Register"]
    #[inline(always)]
    pub const fn fpc_attribviolp_config(&self) -> &FpcAttribviolpConfig {
        &self.fpc_attribviolp_config
    }
    #[doc = "0x1040 - Flash Hide Protection for Bank0 Configuration Register"]
    #[inline(always)]
    pub const fn fpc_hdpprot0_config(&self) -> &FpcHdpprot0Config {
        &self.fpc_hdpprot0_config
    }
    #[doc = "0x1044 - Flash Hide Protection for Bank1 Configuration Register"]
    #[inline(always)]
    pub const fn fpc_hdpprot1_config(&self) -> &FpcHdpprot1Config {
        &self.fpc_hdpprot1_config
    }
    #[doc = "0x104c - Flash Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_hdpprot_control(&self) -> &FpcHdpprotControl {
        &self.fpc_hdpprot_control
    }
    #[doc = "0x1050 - Flash Hide Protection Enable Control Register"]
    #[inline(always)]
    pub const fn fpc_hdpen_control(&self) -> &FpcHdpenControl {
        &self.fpc_hdpen_control
    }
    #[doc = "0x1054 - Flash NONMAIN Bank0 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_nonmain0_hdpprot_control(&self) -> &FpcNonmain0HdpprotControl {
        &self.fpc_nonmain0_hdpprot_control
    }
    #[doc = "0x1058 - Flash NONMAIN Bank1 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_nonmain1_hdpprot_control(&self) -> &FpcNonmain1HdpprotControl {
        &self.fpc_nonmain1_hdpprot_control
    }
    #[doc = "0x105c - Flash TRIM Bank0 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_trim0_hdpprot_control(&self) -> &FpcTrim0HdpprotControl {
        &self.fpc_trim0_hdpprot_control
    }
    #[doc = "0x1060 - Flash TRIM Bank1 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_trim1_hdpprot_control(&self) -> &FpcTrim1HdpprotControl {
        &self.fpc_trim1_hdpprot_control
    }
    #[doc = "0x1064 - Flash ENGR Bank0 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_engr0_hdpprot_control(&self) -> &FpcEngr0HdpprotControl {
        &self.fpc_engr0_hdpprot_control
    }
    #[doc = "0x1068 - Flash ENGR Bank1 Hide Protection Control Register"]
    #[inline(always)]
    pub const fn fpc_engr1_hdpprot_control(&self) -> &FpcEngr1HdpprotControl {
        &self.fpc_engr1_hdpprot_control
    }
    #[doc = "0x1104 - Peripheral Privilege Access ADC"]
    #[inline(always)]
    pub const fn ppc_privattrib_adc(&self) -> &PpcPrivattribAdc {
        &self.ppc_privattrib_adc
    }
    #[doc = "0x110c - Peripheral Privilege Access Timer"]
    #[inline(always)]
    pub const fn ppc_privattrib_timer(&self) -> &PpcPrivattribTimer {
        &self.ppc_privattrib_timer
    }
    #[doc = "0x1114 - Peripheral Privilege Access UniComm"]
    #[inline(always)]
    pub const fn ppc_privattrib_unicomm(&self) -> &PpcPrivattribUnicomm {
        &self.ppc_privattrib_unicomm
    }
    #[doc = "0x111c - Peripheral Privilege Access GPIO"]
    #[inline(always)]
    pub const fn ppc_privattrib_gpio(&self) -> &PpcPrivattribGpio {
        &self.ppc_privattrib_gpio
    }
    #[doc = "0x1124 - Peripheral Privilege Access CAN"]
    #[inline(always)]
    pub const fn ppc_privattrib_mcan(&self) -> &PpcPrivattribMcan {
        &self.ppc_privattrib_mcan
    }
    #[doc = "0x1134 - Peripheral Privilege Access CMPSS"]
    #[inline(always)]
    pub const fn ppc_privattrib_cmpss(&self) -> &PpcPrivattribCmpss {
        &self.ppc_privattrib_cmpss
    }
    #[doc = "0x113c - Peripheral Privilege Access Watchdog"]
    #[inline(always)]
    pub const fn ppc_privattrib_wdt(&self) -> &PpcPrivattribWdt {
        &self.ppc_privattrib_wdt
    }
    #[doc = "0x114c - Peripheral Privilege Access CRC"]
    #[inline(always)]
    pub const fn ppc_privattrib_crc(&self) -> &PpcPrivattribCrc {
        &self.ppc_privattrib_crc
    }
    #[doc = "0x1154 - Peripheral Privilege Access AES"]
    #[inline(always)]
    pub const fn ppc_privattrib_aes(&self) -> &PpcPrivattribAes {
        &self.ppc_privattrib_aes
    }
    #[doc = "0x116c - Peripheral Privilege Access IOMUX"]
    #[inline(always)]
    pub const fn ppc_privattrib_iomux(&self) -> &PpcPrivattribIomux {
        &self.ppc_privattrib_iomux
    }
    #[doc = "0x1184 - Peripheral Privilege Access DEBUGSS"]
    #[inline(always)]
    pub const fn ppc_privattrib_debugss(&self) -> &PpcPrivattribDebugss {
        &self.ppc_privattrib_debugss
    }
    #[doc = "0x1194 - Peripheral Privilege Access KEYSTORECTL"]
    #[inline(always)]
    pub const fn ppc_privattrib_keystorectl(&self) -> &PpcPrivattribKeystorectl {
        &self.ppc_privattrib_keystorectl
    }
    #[doc = "0x119c - Peripheral Privilege Access EAM"]
    #[inline(always)]
    pub const fn ppc_privattrib_eam(&self) -> &PpcPrivattribEam {
        &self.ppc_privattrib_eam
    }
    #[doc = "0x11a4 - Peripheral Privilege Access WUC"]
    #[inline(always)]
    pub const fn ppc_privattrib_wuc(&self) -> &PpcPrivattribWuc {
        &self.ppc_privattrib_wuc
    }
    #[doc = "0x11ac - Peripheral Privilege Access FRIREGS"]
    #[inline(always)]
    pub const fn ppc_privattrib_friregs(&self) -> &PpcPrivattribFriregs {
        &self.ppc_privattrib_friregs
    }
    #[doc = "0x11b4 - Peripheral Privilege Access MEMCFG"]
    #[inline(always)]
    pub const fn ppc_privattrib_memcfg(&self) -> &PpcPrivattribMemcfg {
        &self.ppc_privattrib_memcfg
    }
    #[doc = "0x120c - Peripheral Privilege Access System Control"]
    #[inline(always)]
    pub const fn ppc_privattrib_sysctl(&self) -> &PpcPrivattribSysctl {
        &self.ppc_privattrib_sysctl
    }
    #[doc = "0x1214 - Peripheral Privilege Access DMA"]
    #[inline(always)]
    pub const fn ppc_privattrib_dma(&self) -> &PpcPrivattribDma {
        &self.ppc_privattrib_dma
    }
    #[doc = "0x121c - Peripheral Privilege Access GSC"]
    #[inline(always)]
    pub const fn ppc_privattrib_gsc(&self) -> &PpcPrivattribGsc {
        &self.ppc_privattrib_gsc
    }
    #[doc = "0x1224 - Peripheral Privilege Access Co-processor"]
    #[inline(always)]
    pub const fn ppc_privattrib_coproc(&self) -> &PpcPrivattribCoproc {
        &self.ppc_privattrib_coproc
    }
    #[doc = "0x122c - Peripheral Privilege Access EPI"]
    #[inline(always)]
    pub const fn ppc_privattrib_epi(&self) -> &PpcPrivattribEpi {
        &self.ppc_privattrib_epi
    }
    #[doc = "0x1234 - Peripheral Privilege Access PGA"]
    #[inline(always)]
    pub const fn ppc_privattrib_pga(&self) -> &PpcPrivattribPga {
        &self.ppc_privattrib_pga
    }
    #[doc = "0x123c - Peripheral Privilege Access XBAR"]
    #[inline(always)]
    pub const fn ppc_privattrib_xbar(&self) -> &PpcPrivattribXbar {
        &self.ppc_privattrib_xbar
    }
    #[doc = "0x12c0 - SRAM Privilege Attribute-0 Register"]
    #[inline(always)]
    pub const fn spc_privattrib0(&self) -> &SpcPrivattrib0 {
        &self.spc_privattrib0
    }
    #[doc = "0x12c4 - SRAM Privilege Attribute-1 Register"]
    #[inline(always)]
    pub const fn spc_privattrib1(&self) -> &SpcPrivattrib1 {
        &self.spc_privattrib1
    }
    #[doc = "0x1400 - Flash Privilege Attribute Register A0"]
    #[inline(always)]
    pub const fn fpc_privattriba0(&self) -> &FpcPrivattriba0 {
        &self.fpc_privattriba0
    }
    #[doc = "0x1404 - Flash Privilege Attribute Register A1"]
    #[inline(always)]
    pub const fn fpc_privattriba1(&self) -> &FpcPrivattriba1 {
        &self.fpc_privattriba1
    }
    #[doc = "0x1408 - Flash Privilege Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_privattribb0(&self) -> &FpcPrivattribb0 {
        &self.fpc_privattribb0
    }
    #[doc = "0x140c - Flash Privilege Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_privattribb1(&self) -> &FpcPrivattribb1 {
        &self.fpc_privattribb1
    }
    #[doc = "0x1410 - Flash Non-Main Privilege Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_nonmain_privattrib0(&self) -> &FpcNonmainPrivattrib0 {
        &self.fpc_nonmain_privattrib0
    }
    #[doc = "0x1414 - Flash Non-Main Privilege Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_nonmain_privattrib1(&self) -> &FpcNonmainPrivattrib1 {
        &self.fpc_nonmain_privattrib1
    }
    #[doc = "0x1418 - Flash Trim Privilege Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_trim_privattrib0(&self) -> &FpcTrimPrivattrib0 {
        &self.fpc_trim_privattrib0
    }
    #[doc = "0x141c - Flash Trim Privilege Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_trim_privattrib1(&self) -> &FpcTrimPrivattrib1 {
        &self.fpc_trim_privattrib1
    }
    #[doc = "0x1420 - Flash Engr Privilege Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_engr_privattrib0(&self) -> &FpcEngrPrivattrib0 {
        &self.fpc_engr_privattrib0
    }
    #[doc = "0x1424 - Flash Engr Privilege Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_engr_privattrib1(&self) -> &FpcEngrPrivattrib1 {
        &self.fpc_engr_privattrib1
    }
    #[doc = "0x1600 - Flash Write Protect Attribute Register A0"]
    #[inline(always)]
    pub const fn fpc_weprota0(&self) -> &FpcWeprota0 {
        &self.fpc_weprota0
    }
    #[doc = "0x1604 - Flash Write Protect Attribute Register A1"]
    #[inline(always)]
    pub const fn fpc_weprota1(&self) -> &FpcWeprota1 {
        &self.fpc_weprota1
    }
    #[doc = "0x1608 - Flash Write Protect Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_weprotb0(&self) -> &FpcWeprotb0 {
        &self.fpc_weprotb0
    }
    #[doc = "0x160c - Flash Write Protect Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_weprotb1(&self) -> &FpcWeprotb1 {
        &self.fpc_weprotb1
    }
    #[doc = "0x1610 - Flash Non-Main Write Protect Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_weprotnonmain0(&self) -> &FpcWeprotnonmain0 {
        &self.fpc_weprotnonmain0
    }
    #[doc = "0x1614 - Flash Non-Main Write Protect Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_weprotnonmain1(&self) -> &FpcWeprotnonmain1 {
        &self.fpc_weprotnonmain1
    }
    #[doc = "0x1618 - Flash Trim Write Protect Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_weprottrim0(&self) -> &FpcWeprottrim0 {
        &self.fpc_weprottrim0
    }
    #[doc = "0x161c - Flash Trim Write Protect Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_weprottrim1(&self) -> &FpcWeprottrim1 {
        &self.fpc_weprottrim1
    }
    #[doc = "0x1620 - Flash Engr Write Protect Attribute Register B0"]
    #[inline(always)]
    pub const fn fpc_weprotengr0(&self) -> &FpcWeprotengr0 {
        &self.fpc_weprotengr0
    }
    #[doc = "0x1624 - Flash Engr Write Protect Attribute Register B1"]
    #[inline(always)]
    pub const fn fpc_weprotengr1(&self) -> &FpcWeprotengr1 {
        &self.fpc_weprotengr1
    }
    #[doc = "0x1800 - Flash semaphore request register"]
    #[inline(always)]
    pub const fn fpc_flsemreq(&self) -> &FpcFlsemreq {
        &self.fpc_flsemreq
    }
    #[doc = "0x1804 - Flash semaphore release register"]
    #[inline(always)]
    pub const fn fpc_flsemclr(&self) -> &FpcFlsemclr {
        &self.fpc_flsemclr
    }
    #[doc = "0x1808 - Flash semaphore statys registers"]
    #[inline(always)]
    pub const fn fpc_flsemstat(&self) -> &FpcFlsemstat {
        &self.fpc_flsemstat
    }
    #[doc = "0x1b84 - Non - Secure Vector Table Offset Register"]
    #[inline(always)]
    pub const fn vtor_ns(&self) -> &VtorNs {
        &self.vtor_ns
    }
    #[doc = "0x1c80 - Digital Test Bus MUX Selection"]
    #[inline(always)]
    pub const fn dtb_muxsel(&self) -> &DtbMuxsel {
        &self.dtb_muxsel
    }
    #[doc = "0x1d80 - GSC Lock configuration register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x1d84 - GSC commit configuration register"]
    #[inline(always)]
    pub const fn commit(&self) -> &Commit {
        &self.commit
    }
    #[doc = "0x1ffc - GSC Revision register"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
}
#[doc = "SPC_ATTRIBVIOLP_CONFIG (rw) register accessor: SRAM Privilege Access Attribute Violation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spc_attribviolp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spc_attribviolp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc_attribviolp_config`] module"]
#[doc(alias = "SPC_ATTRIBVIOLP_CONFIG")]
pub type SpcAttribviolpConfig = crate::Reg<spc_attribviolp_config::SpcAttribviolpConfigSpec>;
#[doc = "SRAM Privilege Access Attribute Violation Configuration Register"]
pub mod spc_attribviolp_config;
#[doc = "PPC_ATTRIBVIOLP_CONFIG (rw) register accessor: Peripheral Privilege Access Attribute Violation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_attribviolp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_attribviolp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_attribviolp_config`] module"]
#[doc(alias = "PPC_ATTRIBVIOLP_CONFIG")]
pub type PpcAttribviolpConfig = crate::Reg<ppc_attribviolp_config::PpcAttribviolpConfigSpec>;
#[doc = "Peripheral Privilege Access Attribute Violation Configuration Register"]
pub mod ppc_attribviolp_config;
#[doc = "FPC_ATTRIBVIOLP_CONFIG (rw) register accessor: Flash Privilege Access Attribute Violation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_attribviolp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_attribviolp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_attribviolp_config`] module"]
#[doc(alias = "FPC_ATTRIBVIOLP_CONFIG")]
pub type FpcAttribviolpConfig = crate::Reg<fpc_attribviolp_config::FpcAttribviolpConfigSpec>;
#[doc = "Flash Privilege Access Attribute Violation Configuration Register"]
pub mod fpc_attribviolp_config;
#[doc = "FPC_HDPPROT0_CONFIG (rw) register accessor: Flash Hide Protection for Bank0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_hdpprot0_config`] module"]
#[doc(alias = "FPC_HDPPROT0_CONFIG")]
pub type FpcHdpprot0Config = crate::Reg<fpc_hdpprot0_config::FpcHdpprot0ConfigSpec>;
#[doc = "Flash Hide Protection for Bank0 Configuration Register"]
pub mod fpc_hdpprot0_config;
#[doc = "FPC_HDPPROT1_CONFIG (rw) register accessor: Flash Hide Protection for Bank1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_hdpprot1_config`] module"]
#[doc(alias = "FPC_HDPPROT1_CONFIG")]
pub type FpcHdpprot1Config = crate::Reg<fpc_hdpprot1_config::FpcHdpprot1ConfigSpec>;
#[doc = "Flash Hide Protection for Bank1 Configuration Register"]
pub mod fpc_hdpprot1_config;
#[doc = "FPC_HDPPROT_CONTROL (rw) register accessor: Flash Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_hdpprot_control`] module"]
#[doc(alias = "FPC_HDPPROT_CONTROL")]
pub type FpcHdpprotControl = crate::Reg<fpc_hdpprot_control::FpcHdpprotControlSpec>;
#[doc = "Flash Hide Protection Control Register"]
pub mod fpc_hdpprot_control;
#[doc = "FPC_HDPEN_CONTROL (rw) register accessor: Flash Hide Protection Enable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpen_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpen_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_hdpen_control`] module"]
#[doc(alias = "FPC_HDPEN_CONTROL")]
pub type FpcHdpenControl = crate::Reg<fpc_hdpen_control::FpcHdpenControlSpec>;
#[doc = "Flash Hide Protection Enable Control Register"]
pub mod fpc_hdpen_control;
#[doc = "FPC_NONMAIN0_HDPPROT_CONTROL (rw) register accessor: Flash NONMAIN Bank0 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain0_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain0_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_nonmain0_hdpprot_control`] module"]
#[doc(alias = "FPC_NONMAIN0_HDPPROT_CONTROL")]
pub type FpcNonmain0HdpprotControl =
    crate::Reg<fpc_nonmain0_hdpprot_control::FpcNonmain0HdpprotControlSpec>;
#[doc = "Flash NONMAIN Bank0 Hide Protection Control Register"]
pub mod fpc_nonmain0_hdpprot_control;
#[doc = "FPC_NONMAIN1_HDPPROT_CONTROL (rw) register accessor: Flash NONMAIN Bank1 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain1_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain1_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_nonmain1_hdpprot_control`] module"]
#[doc(alias = "FPC_NONMAIN1_HDPPROT_CONTROL")]
pub type FpcNonmain1HdpprotControl =
    crate::Reg<fpc_nonmain1_hdpprot_control::FpcNonmain1HdpprotControlSpec>;
#[doc = "Flash NONMAIN Bank1 Hide Protection Control Register"]
pub mod fpc_nonmain1_hdpprot_control;
#[doc = "FPC_TRIM0_HDPPROT_CONTROL (rw) register accessor: Flash TRIM Bank0 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_trim0_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_trim0_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_trim0_hdpprot_control`] module"]
#[doc(alias = "FPC_TRIM0_HDPPROT_CONTROL")]
pub type FpcTrim0HdpprotControl = crate::Reg<fpc_trim0_hdpprot_control::FpcTrim0HdpprotControlSpec>;
#[doc = "Flash TRIM Bank0 Hide Protection Control Register"]
pub mod fpc_trim0_hdpprot_control;
#[doc = "FPC_TRIM1_HDPPROT_CONTROL (rw) register accessor: Flash TRIM Bank1 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_trim1_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_trim1_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_trim1_hdpprot_control`] module"]
#[doc(alias = "FPC_TRIM1_HDPPROT_CONTROL")]
pub type FpcTrim1HdpprotControl = crate::Reg<fpc_trim1_hdpprot_control::FpcTrim1HdpprotControlSpec>;
#[doc = "Flash TRIM Bank1 Hide Protection Control Register"]
pub mod fpc_trim1_hdpprot_control;
#[doc = "FPC_ENGR0_HDPPROT_CONTROL (rw) register accessor: Flash ENGR Bank0 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_engr0_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_engr0_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_engr0_hdpprot_control`] module"]
#[doc(alias = "FPC_ENGR0_HDPPROT_CONTROL")]
pub type FpcEngr0HdpprotControl = crate::Reg<fpc_engr0_hdpprot_control::FpcEngr0HdpprotControlSpec>;
#[doc = "Flash ENGR Bank0 Hide Protection Control Register"]
pub mod fpc_engr0_hdpprot_control;
#[doc = "FPC_ENGR1_HDPPROT_CONTROL (rw) register accessor: Flash ENGR Bank1 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_engr1_hdpprot_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_engr1_hdpprot_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_engr1_hdpprot_control`] module"]
#[doc(alias = "FPC_ENGR1_HDPPROT_CONTROL")]
pub type FpcEngr1HdpprotControl = crate::Reg<fpc_engr1_hdpprot_control::FpcEngr1HdpprotControlSpec>;
#[doc = "Flash ENGR Bank1 Hide Protection Control Register"]
pub mod fpc_engr1_hdpprot_control;
#[doc = "PPC_PRIVATTRIB_ADC (rw) register accessor: Peripheral Privilege Access ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_adc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_adc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_adc`] module"]
#[doc(alias = "PPC_PRIVATTRIB_ADC")]
pub type PpcPrivattribAdc = crate::Reg<ppc_privattrib_adc::PpcPrivattribAdcSpec>;
#[doc = "Peripheral Privilege Access ADC"]
pub mod ppc_privattrib_adc;
#[doc = "PPC_PRIVATTRIB_TIMER (rw) register accessor: Peripheral Privilege Access Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_timer`] module"]
#[doc(alias = "PPC_PRIVATTRIB_TIMER")]
pub type PpcPrivattribTimer = crate::Reg<ppc_privattrib_timer::PpcPrivattribTimerSpec>;
#[doc = "Peripheral Privilege Access Timer"]
pub mod ppc_privattrib_timer;
#[doc = "PPC_PRIVATTRIB_UNICOMM (rw) register accessor: Peripheral Privilege Access UniComm\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_unicomm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_unicomm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_unicomm`] module"]
#[doc(alias = "PPC_PRIVATTRIB_UNICOMM")]
pub type PpcPrivattribUnicomm = crate::Reg<ppc_privattrib_unicomm::PpcPrivattribUnicommSpec>;
#[doc = "Peripheral Privilege Access UniComm"]
pub mod ppc_privattrib_unicomm;
#[doc = "PPC_PRIVATTRIB_GPIO (rw) register accessor: Peripheral Privilege Access GPIO\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_gpio`] module"]
#[doc(alias = "PPC_PRIVATTRIB_GPIO")]
pub type PpcPrivattribGpio = crate::Reg<ppc_privattrib_gpio::PpcPrivattribGpioSpec>;
#[doc = "Peripheral Privilege Access GPIO"]
pub mod ppc_privattrib_gpio;
#[doc = "PPC_PRIVATTRIB_MCAN (rw) register accessor: Peripheral Privilege Access CAN\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_mcan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_mcan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_mcan`] module"]
#[doc(alias = "PPC_PRIVATTRIB_MCAN")]
pub type PpcPrivattribMcan = crate::Reg<ppc_privattrib_mcan::PpcPrivattribMcanSpec>;
#[doc = "Peripheral Privilege Access CAN"]
pub mod ppc_privattrib_mcan;
#[doc = "PPC_PRIVATTRIB_CMPSS (rw) register accessor: Peripheral Privilege Access CMPSS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_cmpss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_cmpss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_cmpss`] module"]
#[doc(alias = "PPC_PRIVATTRIB_CMPSS")]
pub type PpcPrivattribCmpss = crate::Reg<ppc_privattrib_cmpss::PpcPrivattribCmpssSpec>;
#[doc = "Peripheral Privilege Access CMPSS"]
pub mod ppc_privattrib_cmpss;
#[doc = "PPC_PRIVATTRIB_WDT (rw) register accessor: Peripheral Privilege Access Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_wdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_wdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_wdt`] module"]
#[doc(alias = "PPC_PRIVATTRIB_WDT")]
pub type PpcPrivattribWdt = crate::Reg<ppc_privattrib_wdt::PpcPrivattribWdtSpec>;
#[doc = "Peripheral Privilege Access Watchdog"]
pub mod ppc_privattrib_wdt;
#[doc = "PPC_PRIVATTRIB_CRC (rw) register accessor: Peripheral Privilege Access CRC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_crc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_crc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_crc`] module"]
#[doc(alias = "PPC_PRIVATTRIB_CRC")]
pub type PpcPrivattribCrc = crate::Reg<ppc_privattrib_crc::PpcPrivattribCrcSpec>;
#[doc = "Peripheral Privilege Access CRC"]
pub mod ppc_privattrib_crc;
#[doc = "PPC_PRIVATTRIB_AES (rw) register accessor: Peripheral Privilege Access AES\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_aes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_aes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_aes`] module"]
#[doc(alias = "PPC_PRIVATTRIB_AES")]
pub type PpcPrivattribAes = crate::Reg<ppc_privattrib_aes::PpcPrivattribAesSpec>;
#[doc = "Peripheral Privilege Access AES"]
pub mod ppc_privattrib_aes;
#[doc = "PPC_PRIVATTRIB_IOMUX (rw) register accessor: Peripheral Privilege Access IOMUX\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_iomux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_iomux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_iomux`] module"]
#[doc(alias = "PPC_PRIVATTRIB_IOMUX")]
pub type PpcPrivattribIomux = crate::Reg<ppc_privattrib_iomux::PpcPrivattribIomuxSpec>;
#[doc = "Peripheral Privilege Access IOMUX"]
pub mod ppc_privattrib_iomux;
#[doc = "PPC_PRIVATTRIB_DEBUGSS (rw) register accessor: Peripheral Privilege Access DEBUGSS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_debugss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_debugss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_debugss`] module"]
#[doc(alias = "PPC_PRIVATTRIB_DEBUGSS")]
pub type PpcPrivattribDebugss = crate::Reg<ppc_privattrib_debugss::PpcPrivattribDebugssSpec>;
#[doc = "Peripheral Privilege Access DEBUGSS"]
pub mod ppc_privattrib_debugss;
#[doc = "PPC_PRIVATTRIB_KEYSTORECTL (rw) register accessor: Peripheral Privilege Access KEYSTORECTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_keystorectl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_keystorectl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_keystorectl`] module"]
#[doc(alias = "PPC_PRIVATTRIB_KEYSTORECTL")]
pub type PpcPrivattribKeystorectl =
    crate::Reg<ppc_privattrib_keystorectl::PpcPrivattribKeystorectlSpec>;
#[doc = "Peripheral Privilege Access KEYSTORECTL"]
pub mod ppc_privattrib_keystorectl;
#[doc = "PPC_PRIVATTRIB_EAM (rw) register accessor: Peripheral Privilege Access EAM\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_eam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_eam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_eam`] module"]
#[doc(alias = "PPC_PRIVATTRIB_EAM")]
pub type PpcPrivattribEam = crate::Reg<ppc_privattrib_eam::PpcPrivattribEamSpec>;
#[doc = "Peripheral Privilege Access EAM"]
pub mod ppc_privattrib_eam;
#[doc = "PPC_PRIVATTRIB_WUC (rw) register accessor: Peripheral Privilege Access WUC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_wuc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_wuc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_wuc`] module"]
#[doc(alias = "PPC_PRIVATTRIB_WUC")]
pub type PpcPrivattribWuc = crate::Reg<ppc_privattrib_wuc::PpcPrivattribWucSpec>;
#[doc = "Peripheral Privilege Access WUC"]
pub mod ppc_privattrib_wuc;
#[doc = "PPC_PRIVATTRIB_FRIREGS (rw) register accessor: Peripheral Privilege Access FRIREGS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_friregs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_friregs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_friregs`] module"]
#[doc(alias = "PPC_PRIVATTRIB_FRIREGS")]
pub type PpcPrivattribFriregs = crate::Reg<ppc_privattrib_friregs::PpcPrivattribFriregsSpec>;
#[doc = "Peripheral Privilege Access FRIREGS"]
pub mod ppc_privattrib_friregs;
#[doc = "PPC_PRIVATTRIB_MEMCFG (rw) register accessor: Peripheral Privilege Access MEMCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_memcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_memcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_memcfg`] module"]
#[doc(alias = "PPC_PRIVATTRIB_MEMCFG")]
pub type PpcPrivattribMemcfg = crate::Reg<ppc_privattrib_memcfg::PpcPrivattribMemcfgSpec>;
#[doc = "Peripheral Privilege Access MEMCFG"]
pub mod ppc_privattrib_memcfg;
#[doc = "PPC_PRIVATTRIB_SYSCTL (rw) register accessor: Peripheral Privilege Access System Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_sysctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_sysctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_sysctl`] module"]
#[doc(alias = "PPC_PRIVATTRIB_SYSCTL")]
pub type PpcPrivattribSysctl = crate::Reg<ppc_privattrib_sysctl::PpcPrivattribSysctlSpec>;
#[doc = "Peripheral Privilege Access System Control"]
pub mod ppc_privattrib_sysctl;
#[doc = "PPC_PRIVATTRIB_DMA (rw) register accessor: Peripheral Privilege Access DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_dma`] module"]
#[doc(alias = "PPC_PRIVATTRIB_DMA")]
pub type PpcPrivattribDma = crate::Reg<ppc_privattrib_dma::PpcPrivattribDmaSpec>;
#[doc = "Peripheral Privilege Access DMA"]
pub mod ppc_privattrib_dma;
#[doc = "PPC_PRIVATTRIB_GSC (rw) register accessor: Peripheral Privilege Access GSC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_gsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_gsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_gsc`] module"]
#[doc(alias = "PPC_PRIVATTRIB_GSC")]
pub type PpcPrivattribGsc = crate::Reg<ppc_privattrib_gsc::PpcPrivattribGscSpec>;
#[doc = "Peripheral Privilege Access GSC"]
pub mod ppc_privattrib_gsc;
#[doc = "PPC_PRIVATTRIB_COPROC (rw) register accessor: Peripheral Privilege Access Co-processor\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_coproc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_coproc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_coproc`] module"]
#[doc(alias = "PPC_PRIVATTRIB_COPROC")]
pub type PpcPrivattribCoproc = crate::Reg<ppc_privattrib_coproc::PpcPrivattribCoprocSpec>;
#[doc = "Peripheral Privilege Access Co-processor"]
pub mod ppc_privattrib_coproc;
#[doc = "PPC_PRIVATTRIB_EPI (rw) register accessor: Peripheral Privilege Access EPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_epi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_epi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_epi`] module"]
#[doc(alias = "PPC_PRIVATTRIB_EPI")]
pub type PpcPrivattribEpi = crate::Reg<ppc_privattrib_epi::PpcPrivattribEpiSpec>;
#[doc = "Peripheral Privilege Access EPI"]
pub mod ppc_privattrib_epi;
#[doc = "PPC_PRIVATTRIB_PGA (rw) register accessor: Peripheral Privilege Access PGA\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_pga`] module"]
#[doc(alias = "PPC_PRIVATTRIB_PGA")]
pub type PpcPrivattribPga = crate::Reg<ppc_privattrib_pga::PpcPrivattribPgaSpec>;
#[doc = "Peripheral Privilege Access PGA"]
pub mod ppc_privattrib_pga;
#[doc = "PPC_PRIVATTRIB_XBAR (rw) register accessor: Peripheral Privilege Access XBAR\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_xbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_xbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppc_privattrib_xbar`] module"]
#[doc(alias = "PPC_PRIVATTRIB_XBAR")]
pub type PpcPrivattribXbar = crate::Reg<ppc_privattrib_xbar::PpcPrivattribXbarSpec>;
#[doc = "Peripheral Privilege Access XBAR"]
pub mod ppc_privattrib_xbar;
#[doc = "SPC_PRIVATTRIB0 (rw) register accessor: SRAM Privilege Attribute-0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spc_privattrib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spc_privattrib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc_privattrib0`] module"]
#[doc(alias = "SPC_PRIVATTRIB0")]
pub type SpcPrivattrib0 = crate::Reg<spc_privattrib0::SpcPrivattrib0Spec>;
#[doc = "SRAM Privilege Attribute-0 Register"]
pub mod spc_privattrib0;
#[doc = "SPC_PRIVATTRIB1 (rw) register accessor: SRAM Privilege Attribute-1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spc_privattrib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spc_privattrib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc_privattrib1`] module"]
#[doc(alias = "SPC_PRIVATTRIB1")]
pub type SpcPrivattrib1 = crate::Reg<spc_privattrib1::SpcPrivattrib1Spec>;
#[doc = "SRAM Privilege Attribute-1 Register"]
pub mod spc_privattrib1;
#[doc = "FPC_PRIVATTRIBA0 (rw) register accessor: Flash Privilege Attribute Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattriba0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattriba0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_privattriba0`] module"]
#[doc(alias = "FPC_PRIVATTRIBA0")]
pub type FpcPrivattriba0 = crate::Reg<fpc_privattriba0::FpcPrivattriba0Spec>;
#[doc = "Flash Privilege Attribute Register A0"]
pub mod fpc_privattriba0;
#[doc = "FPC_PRIVATTRIBA1 (rw) register accessor: Flash Privilege Attribute Register A1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattriba1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattriba1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_privattriba1`] module"]
#[doc(alias = "FPC_PRIVATTRIBA1")]
pub type FpcPrivattriba1 = crate::Reg<fpc_privattriba1::FpcPrivattriba1Spec>;
#[doc = "Flash Privilege Attribute Register A1"]
pub mod fpc_privattriba1;
#[doc = "FPC_PRIVATTRIBB0 (rw) register accessor: Flash Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattribb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattribb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_privattribb0`] module"]
#[doc(alias = "FPC_PRIVATTRIBB0")]
pub type FpcPrivattribb0 = crate::Reg<fpc_privattribb0::FpcPrivattribb0Spec>;
#[doc = "Flash Privilege Attribute Register B0"]
pub mod fpc_privattribb0;
#[doc = "FPC_PRIVATTRIBB1 (rw) register accessor: Flash Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattribb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattribb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_privattribb1`] module"]
#[doc(alias = "FPC_PRIVATTRIBB1")]
pub type FpcPrivattribb1 = crate::Reg<fpc_privattribb1::FpcPrivattribb1Spec>;
#[doc = "Flash Privilege Attribute Register B1"]
pub mod fpc_privattribb1;
#[doc = "FPC_NONMAIN_PRIVATTRIB0 (rw) register accessor: Flash Non-Main Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain_privattrib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain_privattrib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_nonmain_privattrib0`] module"]
#[doc(alias = "FPC_NONMAIN_PRIVATTRIB0")]
pub type FpcNonmainPrivattrib0 = crate::Reg<fpc_nonmain_privattrib0::FpcNonmainPrivattrib0Spec>;
#[doc = "Flash Non-Main Privilege Attribute Register B0"]
pub mod fpc_nonmain_privattrib0;
#[doc = "FPC_NONMAIN_PRIVATTRIB1 (rw) register accessor: Flash Non-Main Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain_privattrib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain_privattrib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_nonmain_privattrib1`] module"]
#[doc(alias = "FPC_NONMAIN_PRIVATTRIB1")]
pub type FpcNonmainPrivattrib1 = crate::Reg<fpc_nonmain_privattrib1::FpcNonmainPrivattrib1Spec>;
#[doc = "Flash Non-Main Privilege Attribute Register B1"]
pub mod fpc_nonmain_privattrib1;
#[doc = "FPC_TRIM_PRIVATTRIB0 (rw) register accessor: Flash Trim Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_trim_privattrib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_trim_privattrib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_trim_privattrib0`] module"]
#[doc(alias = "FPC_TRIM_PRIVATTRIB0")]
pub type FpcTrimPrivattrib0 = crate::Reg<fpc_trim_privattrib0::FpcTrimPrivattrib0Spec>;
#[doc = "Flash Trim Privilege Attribute Register B0"]
pub mod fpc_trim_privattrib0;
#[doc = "FPC_TRIM_PRIVATTRIB1 (rw) register accessor: Flash Trim Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_trim_privattrib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_trim_privattrib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_trim_privattrib1`] module"]
#[doc(alias = "FPC_TRIM_PRIVATTRIB1")]
pub type FpcTrimPrivattrib1 = crate::Reg<fpc_trim_privattrib1::FpcTrimPrivattrib1Spec>;
#[doc = "Flash Trim Privilege Attribute Register B1"]
pub mod fpc_trim_privattrib1;
#[doc = "FPC_ENGR_PRIVATTRIB0 (rw) register accessor: Flash Engr Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_engr_privattrib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_engr_privattrib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_engr_privattrib0`] module"]
#[doc(alias = "FPC_ENGR_PRIVATTRIB0")]
pub type FpcEngrPrivattrib0 = crate::Reg<fpc_engr_privattrib0::FpcEngrPrivattrib0Spec>;
#[doc = "Flash Engr Privilege Attribute Register B0"]
pub mod fpc_engr_privattrib0;
#[doc = "FPC_ENGR_PRIVATTRIB1 (rw) register accessor: Flash Engr Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_engr_privattrib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_engr_privattrib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_engr_privattrib1`] module"]
#[doc(alias = "FPC_ENGR_PRIVATTRIB1")]
pub type FpcEngrPrivattrib1 = crate::Reg<fpc_engr_privattrib1::FpcEngrPrivattrib1Spec>;
#[doc = "Flash Engr Privilege Attribute Register B1"]
pub mod fpc_engr_privattrib1;
#[doc = "FPC_WEPROTA0 (rw) register accessor: Flash Write Protect Attribute Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprota0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprota0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprota0`] module"]
#[doc(alias = "FPC_WEPROTA0")]
pub type FpcWeprota0 = crate::Reg<fpc_weprota0::FpcWeprota0Spec>;
#[doc = "Flash Write Protect Attribute Register A0"]
pub mod fpc_weprota0;
#[doc = "FPC_WEPROTA1 (rw) register accessor: Flash Write Protect Attribute Register A1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprota1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprota1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprota1`] module"]
#[doc(alias = "FPC_WEPROTA1")]
pub type FpcWeprota1 = crate::Reg<fpc_weprota1::FpcWeprota1Spec>;
#[doc = "Flash Write Protect Attribute Register A1"]
pub mod fpc_weprota1;
#[doc = "FPC_WEPROTB0 (rw) register accessor: Flash Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotb0`] module"]
#[doc(alias = "FPC_WEPROTB0")]
pub type FpcWeprotb0 = crate::Reg<fpc_weprotb0::FpcWeprotb0Spec>;
#[doc = "Flash Write Protect Attribute Register B0"]
pub mod fpc_weprotb0;
#[doc = "FPC_WEPROTB1 (rw) register accessor: Flash Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotb1`] module"]
#[doc(alias = "FPC_WEPROTB1")]
pub type FpcWeprotb1 = crate::Reg<fpc_weprotb1::FpcWeprotb1Spec>;
#[doc = "Flash Write Protect Attribute Register B1"]
pub mod fpc_weprotb1;
#[doc = "FPC_WEPROTNONMAIN0 (rw) register accessor: Flash Non-Main Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotnonmain0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotnonmain0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotnonmain0`] module"]
#[doc(alias = "FPC_WEPROTNONMAIN0")]
pub type FpcWeprotnonmain0 = crate::Reg<fpc_weprotnonmain0::FpcWeprotnonmain0Spec>;
#[doc = "Flash Non-Main Write Protect Attribute Register B0"]
pub mod fpc_weprotnonmain0;
#[doc = "FPC_WEPROTNONMAIN1 (rw) register accessor: Flash Non-Main Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotnonmain1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotnonmain1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotnonmain1`] module"]
#[doc(alias = "FPC_WEPROTNONMAIN1")]
pub type FpcWeprotnonmain1 = crate::Reg<fpc_weprotnonmain1::FpcWeprotnonmain1Spec>;
#[doc = "Flash Non-Main Write Protect Attribute Register B1"]
pub mod fpc_weprotnonmain1;
#[doc = "FPC_WEPROTTRIM0 (rw) register accessor: Flash Trim Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprottrim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprottrim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprottrim0`] module"]
#[doc(alias = "FPC_WEPROTTRIM0")]
pub type FpcWeprottrim0 = crate::Reg<fpc_weprottrim0::FpcWeprottrim0Spec>;
#[doc = "Flash Trim Write Protect Attribute Register B0"]
pub mod fpc_weprottrim0;
#[doc = "FPC_WEPROTTRIM1 (rw) register accessor: Flash Trim Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprottrim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprottrim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprottrim1`] module"]
#[doc(alias = "FPC_WEPROTTRIM1")]
pub type FpcWeprottrim1 = crate::Reg<fpc_weprottrim1::FpcWeprottrim1Spec>;
#[doc = "Flash Trim Write Protect Attribute Register B1"]
pub mod fpc_weprottrim1;
#[doc = "FPC_WEPROTENGR0 (rw) register accessor: Flash Engr Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotengr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotengr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotengr0`] module"]
#[doc(alias = "FPC_WEPROTENGR0")]
pub type FpcWeprotengr0 = crate::Reg<fpc_weprotengr0::FpcWeprotengr0Spec>;
#[doc = "Flash Engr Write Protect Attribute Register B0"]
pub mod fpc_weprotengr0;
#[doc = "FPC_WEPROTENGR1 (rw) register accessor: Flash Engr Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotengr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotengr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_weprotengr1`] module"]
#[doc(alias = "FPC_WEPROTENGR1")]
pub type FpcWeprotengr1 = crate::Reg<fpc_weprotengr1::FpcWeprotengr1Spec>;
#[doc = "Flash Engr Write Protect Attribute Register B1"]
pub mod fpc_weprotengr1;
#[doc = "FPC_FLSEMREQ (rw) register accessor: Flash semaphore request register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_flsemreq`] module"]
#[doc(alias = "FPC_FLSEMREQ")]
pub type FpcFlsemreq = crate::Reg<fpc_flsemreq::FpcFlsemreqSpec>;
#[doc = "Flash semaphore request register"]
pub mod fpc_flsemreq;
#[doc = "FPC_FLSEMCLR (rw) register accessor: Flash semaphore release register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_flsemclr`] module"]
#[doc(alias = "FPC_FLSEMCLR")]
pub type FpcFlsemclr = crate::Reg<fpc_flsemclr::FpcFlsemclrSpec>;
#[doc = "Flash semaphore release register"]
pub mod fpc_flsemclr;
#[doc = "FPC_FLSEMSTAT (rw) register accessor: Flash semaphore statys registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpc_flsemstat`] module"]
#[doc(alias = "FPC_FLSEMSTAT")]
pub type FpcFlsemstat = crate::Reg<fpc_flsemstat::FpcFlsemstatSpec>;
#[doc = "Flash semaphore statys registers"]
pub mod fpc_flsemstat;
#[doc = "VTOR_NS (rw) register accessor: Non - Secure Vector Table Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vtor_ns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor_ns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtor_ns`] module"]
#[doc(alias = "VTOR_NS")]
pub type VtorNs = crate::Reg<vtor_ns::VtorNsSpec>;
#[doc = "Non - Secure Vector Table Offset Register"]
pub mod vtor_ns;
#[doc = "DTB_MUXSEL (rw) register accessor: Digital Test Bus MUX Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_muxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_muxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb_muxsel`] module"]
#[doc(alias = "DTB_MUXSEL")]
pub type DtbMuxsel = crate::Reg<dtb_muxsel::DtbMuxselSpec>;
#[doc = "Digital Test Bus MUX Selection"]
pub mod dtb_muxsel;
#[doc = "LOCK (rw) register accessor: GSC Lock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "GSC Lock configuration register"]
pub mod lock;
#[doc = "COMMIT (rw) register accessor: GSC commit configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`commit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commit`] module"]
#[doc(alias = "COMMIT")]
pub type Commit = crate::Reg<commit::CommitSpec>;
#[doc = "GSC commit configuration register"]
pub mod commit;
#[doc = "REVISION (rw) register accessor: GSC Revision register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`] module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "GSC Revision register"]
pub mod revision;
