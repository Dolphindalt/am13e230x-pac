#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    traceid: Traceid,
    deviceid: Deviceid,
    userid: Userid,
    bslpin_uart: BslpinUart,
    bslpin_i2c: BslpinI2c,
    bslpin_can: BslpinCan,
    bslpin_invoke: BslpinInvoke,
    rom_sramflash: RomSramflash,
    pllstartup0_4_8mhz: Pllstartup0_4_8mhz,
    pllstartup1_4_8mhz: Pllstartup1_4_8mhz,
    pllstartup0_8_16mhz: Pllstartup0_8_16mhz,
    pllstartup1_8_16mhz: Pllstartup1_8_16mhz,
    pllstartup0_16_32mhz: Pllstartup0_16_32mhz,
    pllstartup1_16_32mhz: Pllstartup1_16_32mhz,
    pllstartup0_32_48mhz: Pllstartup0_32_48mhz,
    pllstartup1_32_48mhz: Pllstartup1_32_48mhz,
    temp_sense0: TempSense0,
    bootcrc: Bootcrc,
    _reserved18: [u8; 0x04],
    rom_version: RomVersion,
    rom_inm_version: RomInmVersion,
    syspllparam2: Syspllparam2,
    syspllldoctl: Syspllldoctl,
    syspllldoprog: Syspllldoprog,
    extref_mp3: ExtrefMp3,
    intref_mp3: IntrefMp3,
    tscode_mp1: TscodeMp1,
    tscode_mp3: TscodeMp3,
    tstemp: Tstemp,
    sramflash: Sramflash,
}
impl RegisterBlock {
    #[doc = "0x00 - Defined by TI, during ATE, based on wafer"]
    #[inline(always)]
    pub const fn traceid(&self) -> &Traceid {
        &self.traceid
    }
    #[doc = "0x04 - DEVICEID"]
    #[inline(always)]
    pub const fn deviceid(&self) -> &Deviceid {
        &self.deviceid
    }
    #[doc = "0x08 - Defined by TI, depending on device spin"]
    #[inline(always)]
    pub const fn userid(&self) -> &Userid {
        &self.userid
    }
    #[doc = "0x0c - BSL UART Pin Configuration"]
    #[inline(always)]
    pub const fn bslpin_uart(&self) -> &BslpinUart {
        &self.bslpin_uart
    }
    #[doc = "0x10 - BSL I2C Pin Configuration"]
    #[inline(always)]
    pub const fn bslpin_i2c(&self) -> &BslpinI2c {
        &self.bslpin_i2c
    }
    #[doc = "0x14 - BSL CAN Pin Configuration"]
    #[inline(always)]
    pub const fn bslpin_can(&self) -> &BslpinCan {
        &self.bslpin_can
    }
    #[doc = "0x18 - BSL Pin Invocation Configuration"]
    #[inline(always)]
    pub const fn bslpin_invoke(&self) -> &BslpinInvoke {
        &self.bslpin_invoke
    }
    #[doc = "0x1c - ROM_SRAMFLASH"]
    #[inline(always)]
    pub const fn rom_sramflash(&self) -> &RomSramflash {
        &self.rom_sramflash
    }
    #[doc = "0x20 - PLLSTARTUP0_4_8MHZ"]
    #[inline(always)]
    pub const fn pllstartup0_4_8mhz(&self) -> &Pllstartup0_4_8mhz {
        &self.pllstartup0_4_8mhz
    }
    #[doc = "0x24 - PLLSTARTUP1_4_8MHZ"]
    #[inline(always)]
    pub const fn pllstartup1_4_8mhz(&self) -> &Pllstartup1_4_8mhz {
        &self.pllstartup1_4_8mhz
    }
    #[doc = "0x28 - PLLSTARTUP0_8_16MHZ"]
    #[inline(always)]
    pub const fn pllstartup0_8_16mhz(&self) -> &Pllstartup0_8_16mhz {
        &self.pllstartup0_8_16mhz
    }
    #[doc = "0x2c - PLLSTARTUP1_8_16MHZ"]
    #[inline(always)]
    pub const fn pllstartup1_8_16mhz(&self) -> &Pllstartup1_8_16mhz {
        &self.pllstartup1_8_16mhz
    }
    #[doc = "0x30 - PLLSTARTUP0_16_32MHZ"]
    #[inline(always)]
    pub const fn pllstartup0_16_32mhz(&self) -> &Pllstartup0_16_32mhz {
        &self.pllstartup0_16_32mhz
    }
    #[doc = "0x34 - PLLSTARTUP1_16_32MHZ"]
    #[inline(always)]
    pub const fn pllstartup1_16_32mhz(&self) -> &Pllstartup1_16_32mhz {
        &self.pllstartup1_16_32mhz
    }
    #[doc = "0x38 - PLLSTARTUP0_32_48MHZ"]
    #[inline(always)]
    pub const fn pllstartup0_32_48mhz(&self) -> &Pllstartup0_32_48mhz {
        &self.pllstartup0_32_48mhz
    }
    #[doc = "0x3c - PLLSTARTUP1_32_48MHZ"]
    #[inline(always)]
    pub const fn pllstartup1_32_48mhz(&self) -> &Pllstartup1_32_48mhz {
        &self.pllstartup1_32_48mhz
    }
    #[doc = "0x40 - TEMP_SENSE0"]
    #[inline(always)]
    pub const fn temp_sense0(&self) -> &TempSense0 {
        &self.temp_sense0
    }
    #[doc = "0x44 - BOOTCRC"]
    #[inline(always)]
    pub const fn bootcrc(&self) -> &Bootcrc {
        &self.bootcrc
    }
    #[doc = "0x4c - ROM_VERSION"]
    #[inline(always)]
    pub const fn rom_version(&self) -> &RomVersion {
        &self.rom_version
    }
    #[doc = "0x50 - ROM_INM_VERSION"]
    #[inline(always)]
    pub const fn rom_inm_version(&self) -> &RomInmVersion {
        &self.rom_inm_version
    }
    #[doc = "0x54 - SYSPLLPARAM2"]
    #[inline(always)]
    pub const fn syspllparam2(&self) -> &Syspllparam2 {
        &self.syspllparam2
    }
    #[doc = "0x58 - SYSPLLLDOCTL"]
    #[inline(always)]
    pub const fn syspllldoctl(&self) -> &Syspllldoctl {
        &self.syspllldoctl
    }
    #[doc = "0x5c - SYSPLLLDOPROG"]
    #[inline(always)]
    pub const fn syspllldoprog(&self) -> &Syspllldoprog {
        &self.syspllldoprog
    }
    #[doc = "0x60 - External Reference MP3 Temp Sense"]
    #[inline(always)]
    pub const fn extref_mp3(&self) -> &ExtrefMp3 {
        &self.extref_mp3
    }
    #[doc = "0x64 - Internal Refernce Temp Sene"]
    #[inline(always)]
    pub const fn intref_mp3(&self) -> &IntrefMp3 {
        &self.intref_mp3
    }
    #[doc = "0x68 - Tempsense Code MP1"]
    #[inline(always)]
    pub const fn tscode_mp1(&self) -> &TscodeMp1 {
        &self.tscode_mp1
    }
    #[doc = "0x6c - Tempsense code MP3"]
    #[inline(always)]
    pub const fn tscode_mp3(&self) -> &TscodeMp3 {
        &self.tscode_mp3
    }
    #[doc = "0x70 - Temperature sense TEMP"]
    #[inline(always)]
    pub const fn tstemp(&self) -> &Tstemp {
        &self.tstemp
    }
    #[doc = "0x74 - Number of banks , sram size and flash size"]
    #[inline(always)]
    pub const fn sramflash(&self) -> &Sramflash {
        &self.sramflash
    }
}
#[doc = "TRACEID (r) register accessor: Defined by TI, during ATE, based on wafer\n\nYou can [`read`](crate::Reg::read) this register and get [`traceid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceid`] module"]
#[doc(alias = "TRACEID")]
pub type Traceid = crate::Reg<traceid::TraceidSpec>;
#[doc = "Defined by TI, during ATE, based on wafer"]
pub mod traceid;
#[doc = "DEVICEID (r) register accessor: DEVICEID\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`] module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "DEVICEID"]
pub mod deviceid;
#[doc = "USERID (r) register accessor: Defined by TI, depending on device spin\n\nYou can [`read`](crate::Reg::read) this register and get [`userid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userid`] module"]
#[doc(alias = "USERID")]
pub type Userid = crate::Reg<userid::UseridSpec>;
#[doc = "Defined by TI, depending on device spin"]
pub mod userid;
#[doc = "BSLPIN_UART (r) register accessor: BSL UART Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_uart::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bslpin_uart`] module"]
#[doc(alias = "BSLPIN_UART")]
pub type BslpinUart = crate::Reg<bslpin_uart::BslpinUartSpec>;
#[doc = "BSL UART Pin Configuration"]
pub mod bslpin_uart;
#[doc = "BSLPIN_I2C (r) register accessor: BSL I2C Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_i2c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bslpin_i2c`] module"]
#[doc(alias = "BSLPIN_I2C")]
pub type BslpinI2c = crate::Reg<bslpin_i2c::BslpinI2cSpec>;
#[doc = "BSL I2C Pin Configuration"]
pub mod bslpin_i2c;
#[doc = "BSLPIN_CAN (r) register accessor: BSL CAN Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_can::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bslpin_can`] module"]
#[doc(alias = "BSLPIN_CAN")]
pub type BslpinCan = crate::Reg<bslpin_can::BslpinCanSpec>;
#[doc = "BSL CAN Pin Configuration"]
pub mod bslpin_can;
#[doc = "BSLPIN_INVOKE (r) register accessor: BSL Pin Invocation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_invoke::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bslpin_invoke`] module"]
#[doc(alias = "BSLPIN_INVOKE")]
pub type BslpinInvoke = crate::Reg<bslpin_invoke::BslpinInvokeSpec>;
#[doc = "BSL Pin Invocation Configuration"]
pub mod bslpin_invoke;
#[doc = "ROM_SRAMFLASH (r) register accessor: ROM_SRAMFLASH\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_sramflash::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_sramflash`] module"]
#[doc(alias = "ROM_SRAMFLASH")]
pub type RomSramflash = crate::Reg<rom_sramflash::RomSramflashSpec>;
#[doc = "ROM_SRAMFLASH"]
pub mod rom_sramflash;
#[doc = "PLLSTARTUP0_4_8MHZ (r) register accessor: PLLSTARTUP0_4_8MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup0_4_8mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup0_4_8mhz`] module"]
#[doc(alias = "PLLSTARTUP0_4_8MHZ")]
pub type Pllstartup0_4_8mhz = crate::Reg<pllstartup0_4_8mhz::Pllstartup0_4_8mhzSpec>;
#[doc = "PLLSTARTUP0_4_8MHZ"]
pub mod pllstartup0_4_8mhz;
#[doc = "PLLSTARTUP1_4_8MHZ (r) register accessor: PLLSTARTUP1_4_8MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup1_4_8mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup1_4_8mhz`] module"]
#[doc(alias = "PLLSTARTUP1_4_8MHZ")]
pub type Pllstartup1_4_8mhz = crate::Reg<pllstartup1_4_8mhz::Pllstartup1_4_8mhzSpec>;
#[doc = "PLLSTARTUP1_4_8MHZ"]
pub mod pllstartup1_4_8mhz;
#[doc = "PLLSTARTUP0_8_16MHZ (r) register accessor: PLLSTARTUP0_8_16MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup0_8_16mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup0_8_16mhz`] module"]
#[doc(alias = "PLLSTARTUP0_8_16MHZ")]
pub type Pllstartup0_8_16mhz = crate::Reg<pllstartup0_8_16mhz::Pllstartup0_8_16mhzSpec>;
#[doc = "PLLSTARTUP0_8_16MHZ"]
pub mod pllstartup0_8_16mhz;
#[doc = "PLLSTARTUP1_8_16MHZ (r) register accessor: PLLSTARTUP1_8_16MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup1_8_16mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup1_8_16mhz`] module"]
#[doc(alias = "PLLSTARTUP1_8_16MHZ")]
pub type Pllstartup1_8_16mhz = crate::Reg<pllstartup1_8_16mhz::Pllstartup1_8_16mhzSpec>;
#[doc = "PLLSTARTUP1_8_16MHZ"]
pub mod pllstartup1_8_16mhz;
#[doc = "PLLSTARTUP0_16_32MHZ (r) register accessor: PLLSTARTUP0_16_32MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup0_16_32mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup0_16_32mhz`] module"]
#[doc(alias = "PLLSTARTUP0_16_32MHZ")]
pub type Pllstartup0_16_32mhz = crate::Reg<pllstartup0_16_32mhz::Pllstartup0_16_32mhzSpec>;
#[doc = "PLLSTARTUP0_16_32MHZ"]
pub mod pllstartup0_16_32mhz;
#[doc = "PLLSTARTUP1_16_32MHZ (r) register accessor: PLLSTARTUP1_16_32MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup1_16_32mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup1_16_32mhz`] module"]
#[doc(alias = "PLLSTARTUP1_16_32MHZ")]
pub type Pllstartup1_16_32mhz = crate::Reg<pllstartup1_16_32mhz::Pllstartup1_16_32mhzSpec>;
#[doc = "PLLSTARTUP1_16_32MHZ"]
pub mod pllstartup1_16_32mhz;
#[doc = "PLLSTARTUP0_32_48MHZ (r) register accessor: PLLSTARTUP0_32_48MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup0_32_48mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup0_32_48mhz`] module"]
#[doc(alias = "PLLSTARTUP0_32_48MHZ")]
pub type Pllstartup0_32_48mhz = crate::Reg<pllstartup0_32_48mhz::Pllstartup0_32_48mhzSpec>;
#[doc = "PLLSTARTUP0_32_48MHZ"]
pub mod pllstartup0_32_48mhz;
#[doc = "PLLSTARTUP1_32_48MHZ (r) register accessor: PLLSTARTUP1_32_48MHZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstartup1_32_48mhz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstartup1_32_48mhz`] module"]
#[doc(alias = "PLLSTARTUP1_32_48MHZ")]
pub type Pllstartup1_32_48mhz = crate::Reg<pllstartup1_32_48mhz::Pllstartup1_32_48mhzSpec>;
#[doc = "PLLSTARTUP1_32_48MHZ"]
pub mod pllstartup1_32_48mhz;
#[doc = "TEMP_SENSE0 (r) register accessor: TEMP_SENSE0\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sense0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sense0`] module"]
#[doc(alias = "TEMP_SENSE0")]
pub type TempSense0 = crate::Reg<temp_sense0::TempSense0Spec>;
#[doc = "TEMP_SENSE0"]
pub mod temp_sense0;
#[doc = "BOOTCRC (r) register accessor: BOOTCRC\n\nYou can [`read`](crate::Reg::read) this register and get [`bootcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootcrc`] module"]
#[doc(alias = "BOOTCRC")]
pub type Bootcrc = crate::Reg<bootcrc::BootcrcSpec>;
#[doc = "BOOTCRC"]
pub mod bootcrc;
#[doc = "ROM_VERSION (r) register accessor: ROM_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_version`] module"]
#[doc(alias = "ROM_VERSION")]
pub type RomVersion = crate::Reg<rom_version::RomVersionSpec>;
#[doc = "ROM_VERSION"]
pub mod rom_version;
#[doc = "ROM_INM_VERSION (r) register accessor: ROM_INM_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_inm_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_inm_version`] module"]
#[doc(alias = "ROM_INM_VERSION")]
pub type RomInmVersion = crate::Reg<rom_inm_version::RomInmVersionSpec>;
#[doc = "ROM_INM_VERSION"]
pub mod rom_inm_version;
#[doc = "SYSPLLPARAM2 (r) register accessor: SYSPLLPARAM2\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam2`] module"]
#[doc(alias = "SYSPLLPARAM2")]
pub type Syspllparam2 = crate::Reg<syspllparam2::Syspllparam2Spec>;
#[doc = "SYSPLLPARAM2"]
pub mod syspllparam2;
#[doc = "SYSPLLLDOCTL (r) register accessor: SYSPLLLDOCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllldoctl`] module"]
#[doc(alias = "SYSPLLLDOCTL")]
pub type Syspllldoctl = crate::Reg<syspllldoctl::SyspllldoctlSpec>;
#[doc = "SYSPLLLDOCTL"]
pub mod syspllldoctl;
#[doc = "SYSPLLLDOPROG (r) register accessor: SYSPLLLDOPROG\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoprog::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllldoprog`] module"]
#[doc(alias = "SYSPLLLDOPROG")]
pub type Syspllldoprog = crate::Reg<syspllldoprog::SyspllldoprogSpec>;
#[doc = "SYSPLLLDOPROG"]
pub mod syspllldoprog;
#[doc = "EXTREF_MP3 (r) register accessor: External Reference MP3 Temp Sense\n\nYou can [`read`](crate::Reg::read) this register and get [`extref_mp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extref_mp3`] module"]
#[doc(alias = "EXTREF_MP3")]
pub type ExtrefMp3 = crate::Reg<extref_mp3::ExtrefMp3Spec>;
#[doc = "External Reference MP3 Temp Sense"]
pub mod extref_mp3;
#[doc = "INTREF_MP3 (r) register accessor: Internal Refernce Temp Sene\n\nYou can [`read`](crate::Reg::read) this register and get [`intref_mp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intref_mp3`] module"]
#[doc(alias = "INTREF_MP3")]
pub type IntrefMp3 = crate::Reg<intref_mp3::IntrefMp3Spec>;
#[doc = "Internal Refernce Temp Sene"]
pub mod intref_mp3;
#[doc = "TSCODE_MP1 (r) register accessor: Tempsense Code MP1\n\nYou can [`read`](crate::Reg::read) this register and get [`tscode_mp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscode_mp1`] module"]
#[doc(alias = "TSCODE_MP1")]
pub type TscodeMp1 = crate::Reg<tscode_mp1::TscodeMp1Spec>;
#[doc = "Tempsense Code MP1"]
pub mod tscode_mp1;
#[doc = "TSCODE_MP3 (r) register accessor: Tempsense code MP3\n\nYou can [`read`](crate::Reg::read) this register and get [`tscode_mp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscode_mp3`] module"]
#[doc(alias = "TSCODE_MP3")]
pub type TscodeMp3 = crate::Reg<tscode_mp3::TscodeMp3Spec>;
#[doc = "Tempsense code MP3"]
pub mod tscode_mp3;
#[doc = "TSTEMP (r) register accessor: Temperature sense TEMP\n\nYou can [`read`](crate::Reg::read) this register and get [`tstemp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstemp`] module"]
#[doc(alias = "TSTEMP")]
pub type Tstemp = crate::Reg<tstemp::TstempSpec>;
#[doc = "Temperature sense TEMP"]
pub mod tstemp;
#[doc = "SRAMFLASH (r) register accessor: Number of banks , sram size and flash size\n\nYou can [`read`](crate::Reg::read) this register and get [`sramflash::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramflash`] module"]
#[doc(alias = "SRAMFLASH")]
pub type Sramflash = crate::Reg<sramflash::SramflashSpec>;
#[doc = "Number of banks , sram size and flash size"]
pub mod sramflash;
