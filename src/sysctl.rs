#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    iidx: Iidx,
    _reserved1: [u8; 0x04],
    imask: Imask,
    _reserved2: [u8; 0x04],
    ris: Ris,
    _reserved3: [u8; 0x04],
    mis: Mis,
    _reserved4: [u8; 0x04],
    iset: Iset,
    _reserved5: [u8; 0x04],
    iclr: Iclr,
    _reserved6: [u8; 0x04],
    nmiiidx: Nmiiidx,
    _reserved7: [u8; 0x0c],
    nmiris: Nmiris,
    _reserved8: [u8; 0x0c],
    nmiiset: Nmiiset,
    _reserved9: [u8; 0x04],
    nmiiclr: Nmiiclr,
    _reserved10: [u8; 0x84],
    sysosccfg: Sysosccfg,
    mclkcfg: Mclkcfg,
    hsclken: Hsclken,
    hsclkcfg: Hsclkcfg,
    hfclkclkcfg: Hfclkclkcfg,
    _reserved15: [u8; 0x0c],
    syspllcfg0: Syspllcfg0,
    syspllcfg1: Syspllcfg1,
    syspllparam0: Syspllparam0,
    syspllparam1: Syspllparam1,
    syspllparam2: Syspllparam2,
    syspllldoctl: Syspllldoctl,
    syspllldoprog: Syspllldoprog,
    genclken: Genclken,
    genclkcfg: Genclkcfg,
    pmodecfg: Pmodecfg,
    mldolpencfg: Mldolpencfg,
    _reserved26: [u8; 0x04],
    fcc: Fcc,
    pmuldosparectl: Pmuldosparectl,
    eco_reg1: EcoReg1,
    eco_reg2: EcoReg2,
    _reserved30: [u8; 0x20],
    systemcfg: Systemcfg,
    sramcfg: Sramcfg,
    _reserved32: [u8; 0x78],
    writelock: Writelock,
    clkstatus: Clkstatus,
    sysstatus: Sysstatus,
    _reserved35: [u8; 0x14],
    rstcause: Rstcause,
    _reserved36: [u8; 0xdc],
    resetlevel: Resetlevel,
    resetcmd: Resetcmd,
    _reserved38: [u8; 0x08],
    sysoscfclctl: Sysoscfclctl,
    _reserved39: [u8; 0x08],
    shdniorel: Shdniorel,
    exrstpin: Exrstpin,
    sysstatusclr: Sysstatusclr,
    swdjcfg: Swdjcfg,
    fcccmd: Fcccmd,
    _reserved44: [u8; 0xd0],
    shutdnstore0: Shutdnstore0,
    shutdnstore1: Shutdnstore1,
    shutdnstore2: Shutdnstore2,
    shutdnstore3: Shutdnstore3,
    adcseqfrcgb: Adcseqfrcgb,
    adcseqfrcgbsel: Adcseqfrcgbsel,
    m33sparesoclock1: M33sparesoclock1,
    m33sparesoclock2: M33sparesoclock2,
    read_reg: ReadReg,
    pwren_mcperiph: PwrenMcperiph,
    rstctl_assert_mcperiph: RstctlAssertMcperiph,
    rstctl_clear_mcperiph: RstctlClearMcperiph,
    stat_mcperiph: StatMcperiph,
    pwren_sysperiph: PwrenSysperiph,
    rstctl_assert_sysperiph: RstctlAssertSysperiph,
    rstctl_clear_sysperiph: RstctlClearSysperiph,
    stat_sysperiph: StatSysperiph,
    cmphpmxsel: Cmphpmxsel,
    _reserved62: [u8; 0x04],
    cmplpmxsel: Cmplpmxsel,
    cmphnmxsel: Cmphnmxsel,
    cmplnmxsel: Cmplnmxsel,
    tsnscfg: Tsnscfg,
    tsnssctl: Tsnssctl,
    pgaconfig: Pgaconfig,
    refconfiga: Refconfiga,
    internaltestctl: Internaltestctl,
    i2vctl: I2vctl,
    adcdacloopback: Adcdacloopback,
    xtalcr: Xtalcr,
    xtalcr2: Xtalcr2,
    x1cnt: X1cnt,
    cmpssctl: Cmpssctl,
    cmpssdacbufconfig: Cmpssdacbufconfig,
    anarefctl: Anarefctl,
    perclkcr: Perclkcr,
    adc_mmr_ovrd_ctl: AdcMmrOvrdCtl,
    adc_mmr_ovrd_val: AdcMmrOvrdVal,
    vregconfigdebug: Vregconfigdebug,
    vregconfigdft: Vregconfigdft,
    am13spareirefensoclock: Am13spareirefensoclock,
    am13sparesoclock2: Am13sparesoclock2,
    am13sparesoclock3: Am13sparesoclock3,
    am13sparesoclock4: Am13sparesoclock4,
    _reserved87: [u8; 0x1b88],
    flbankswppolicy: Flbankswppolicy,
    flbankswp: Flbankswp,
    _reserved89: [u8; 0x08],
    secstatus: Secstatus,
    _reserved90: [u8; 0x14],
    initdone: Initdone,
}
impl RegisterBlock {
    #[doc = "0x1020 - SYSCTL interrupt index"]
    #[inline(always)]
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
    #[doc = "0x1028 - SYSCTL interrupt mask"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x1030 - SYSCTL raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x1038 - SYSCTL masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1040 - SYSCTL interrupt set"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x1048 - SYSCTL interrupt clear"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x1050 - NMI interrupt index"]
    #[inline(always)]
    pub const fn nmiiidx(&self) -> &Nmiiidx {
        &self.nmiiidx
    }
    #[doc = "0x1060 - NMI raw interrupt status"]
    #[inline(always)]
    pub const fn nmiris(&self) -> &Nmiris {
        &self.nmiris
    }
    #[doc = "0x1070 - NMI interrupt set"]
    #[inline(always)]
    pub const fn nmiiset(&self) -> &Nmiiset {
        &self.nmiiset
    }
    #[doc = "0x1078 - NMI interrupt clear"]
    #[inline(always)]
    pub const fn nmiiclr(&self) -> &Nmiiclr {
        &self.nmiiclr
    }
    #[doc = "0x1100 - SYSOSC configuration"]
    #[inline(always)]
    pub const fn sysosccfg(&self) -> &Sysosccfg {
        &self.sysosccfg
    }
    #[doc = "0x1104 - Main clock (MCLK) configuration"]
    #[inline(always)]
    pub const fn mclkcfg(&self) -> &Mclkcfg {
        &self.mclkcfg
    }
    #[doc = "0x1108 - High-speed clock (HSCLK) source enable/disable"]
    #[inline(always)]
    pub const fn hsclken(&self) -> &Hsclken {
        &self.hsclken
    }
    #[doc = "0x110c - High-speed clock (HSCLK) source selection"]
    #[inline(always)]
    pub const fn hsclkcfg(&self) -> &Hsclkcfg {
        &self.hsclkcfg
    }
    #[doc = "0x1110 - High-frequency clock (HFCLK) configuration"]
    #[inline(always)]
    pub const fn hfclkclkcfg(&self) -> &Hfclkclkcfg {
        &self.hfclkclkcfg
    }
    #[doc = "0x1120 - SYSPLL reference and output configuration"]
    #[inline(always)]
    pub const fn syspllcfg0(&self) -> &Syspllcfg0 {
        &self.syspllcfg0
    }
    #[doc = "0x1124 - SYSPLL reference and feedback divider"]
    #[inline(always)]
    pub const fn syspllcfg1(&self) -> &Syspllcfg1 {
        &self.syspllcfg1
    }
    #[doc = "0x1128 - SYSPLL PARAM0 (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllparam0(&self) -> &Syspllparam0 {
        &self.syspllparam0
    }
    #[doc = "0x112c - SYSPLL PARAM1 (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllparam1(&self) -> &Syspllparam1 {
        &self.syspllparam1
    }
    #[doc = "0x1130 - SYSPLL PARAM2 (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllparam2(&self) -> &Syspllparam2 {
        &self.syspllparam2
    }
    #[doc = "0x1134 - SYSPLL LDO CTL (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllldoctl(&self) -> &Syspllldoctl {
        &self.syspllldoctl
    }
    #[doc = "0x1138 - SYSPLL LDO VOUT PROG (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllldoprog(&self) -> &Syspllldoprog {
        &self.syspllldoprog
    }
    #[doc = "0x113c - General clock enable control"]
    #[inline(always)]
    pub const fn genclken(&self) -> &Genclken {
        &self.genclken
    }
    #[doc = "0x1140 - General clock configuration"]
    #[inline(always)]
    pub const fn genclkcfg(&self) -> &Genclkcfg {
        &self.genclkcfg
    }
    #[doc = "0x1144 - Power mode configuration"]
    #[inline(always)]
    pub const fn pmodecfg(&self) -> &Pmodecfg {
        &self.pmodecfg
    }
    #[doc = "0x1148 - LDO Configuration Control"]
    #[inline(always)]
    pub const fn mldolpencfg(&self) -> &Mldolpencfg {
        &self.mldolpencfg
    }
    #[doc = "0x1150 - Frequency clock counter (FCC) count"]
    #[inline(always)]
    pub const fn fcc(&self) -> &Fcc {
        &self.fcc
    }
    #[doc = "0x1154 - LDO Spare Control"]
    #[inline(always)]
    pub const fn pmuldosparectl(&self) -> &Pmuldosparectl {
        &self.pmuldosparectl
    }
    #[doc = "0x1158 - Sysctl ECO Reg 1"]
    #[inline(always)]
    pub const fn eco_reg1(&self) -> &EcoReg1 {
        &self.eco_reg1
    }
    #[doc = "0x115c - Sysctl ECO Reg 2"]
    #[inline(always)]
    pub const fn eco_reg2(&self) -> &EcoReg2 {
        &self.eco_reg2
    }
    #[doc = "0x1180 - System configuration"]
    #[inline(always)]
    pub const fn systemcfg(&self) -> &Systemcfg {
        &self.systemcfg
    }
    #[doc = "0x1184 - System SRAM configuration"]
    #[inline(always)]
    pub const fn sramcfg(&self) -> &Sramcfg {
        &self.sramcfg
    }
    #[doc = "0x1200 - SYSCTL register write lockout"]
    #[inline(always)]
    pub const fn writelock(&self) -> &Writelock {
        &self.writelock
    }
    #[doc = "0x1204 - Clock module (CKM) status"]
    #[inline(always)]
    pub const fn clkstatus(&self) -> &Clkstatus {
        &self.clkstatus
    }
    #[doc = "0x1208 - System status information"]
    #[inline(always)]
    pub const fn sysstatus(&self) -> &Sysstatus {
        &self.sysstatus
    }
    #[doc = "0x1220 - Reset cause"]
    #[inline(always)]
    pub const fn rstcause(&self) -> &Rstcause {
        &self.rstcause
    }
    #[doc = "0x1300 - Reset level for application-triggered reset command"]
    #[inline(always)]
    pub const fn resetlevel(&self) -> &Resetlevel {
        &self.resetlevel
    }
    #[doc = "0x1304 - Execute an application-triggered reset command"]
    #[inline(always)]
    pub const fn resetcmd(&self) -> &Resetcmd {
        &self.resetcmd
    }
    #[doc = "0x1310 - SYSOSC frequency correction loop (FCL) ROSC enable"]
    #[inline(always)]
    pub const fn sysoscfclctl(&self) -> &Sysoscfclctl {
        &self.sysoscfclctl
    }
    #[doc = "0x131c - SHUTDOWN IO release control"]
    #[inline(always)]
    pub const fn shdniorel(&self) -> &Shdniorel {
        &self.shdniorel
    }
    #[doc = "0x1320 - Disable the reset function of the NRST pin"]
    #[inline(always)]
    pub const fn exrstpin(&self) -> &Exrstpin {
        &self.exrstpin
    }
    #[doc = "0x1324 - Clear sticky bits of SYSSTATUS"]
    #[inline(always)]
    pub const fn sysstatusclr(&self) -> &Sysstatusclr {
        &self.sysstatusclr
    }
    #[doc = "0x1328 - Disable the SWD/JTAG function on the SWD/JTAG pins"]
    #[inline(always)]
    pub const fn swdjcfg(&self) -> &Swdjcfg {
        &self.swdjcfg
    }
    #[doc = "0x132c - Frequency clock counter start capture"]
    #[inline(always)]
    pub const fn fcccmd(&self) -> &Fcccmd {
        &self.fcccmd
    }
    #[doc = "0x1400 - Shutdown storage memory (byte 0)"]
    #[inline(always)]
    pub const fn shutdnstore0(&self) -> &Shutdnstore0 {
        &self.shutdnstore0
    }
    #[doc = "0x1404 - Shutdown storage memory (byte 1)"]
    #[inline(always)]
    pub const fn shutdnstore1(&self) -> &Shutdnstore1 {
        &self.shutdnstore1
    }
    #[doc = "0x1408 - Shutdown storage memory (byte 2)"]
    #[inline(always)]
    pub const fn shutdnstore2(&self) -> &Shutdnstore2 {
        &self.shutdnstore2
    }
    #[doc = "0x140c - Shutdown storage memory (byte 3)"]
    #[inline(always)]
    pub const fn shutdnstore3(&self) -> &Shutdnstore3 {
        &self.shutdnstore3
    }
    #[doc = "0x1410 - ADC Global Sequence Force"]
    #[inline(always)]
    pub const fn adcseqfrcgb(&self) -> &Adcseqfrcgb {
        &self.adcseqfrcgb
    }
    #[doc = "0x1414 - ADC Global Sequence Force Select"]
    #[inline(always)]
    pub const fn adcseqfrcgbsel(&self) -> &Adcseqfrcgbsel {
        &self.adcseqfrcgbsel
    }
    #[doc = "0x1418 - M33C1 Spare SOC LOCK Reg 1"]
    #[inline(always)]
    pub const fn m33sparesoclock1(&self) -> &M33sparesoclock1 {
        &self.m33sparesoclock1
    }
    #[doc = "0x141c - M33C1 Spare SOC LOCK Reg 2"]
    #[inline(always)]
    pub const fn m33sparesoclock2(&self) -> &M33sparesoclock2 {
        &self.m33sparesoclock2
    }
    #[doc = "0x1420 - Sysctl read only Reg"]
    #[inline(always)]
    pub const fn read_reg(&self) -> &ReadReg {
        &self.read_reg
    }
    #[doc = "0x1424 - Register to control the power state"]
    #[inline(always)]
    pub const fn pwren_mcperiph(&self) -> &PwrenMcperiph {
        &self.pwren_mcperiph
    }
    #[doc = "0x1428 - rstctl assert register to control reset assertion - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn rstctl_assert_mcperiph(&self) -> &RstctlAssertMcperiph {
        &self.rstctl_assert_mcperiph
    }
    #[doc = "0x142c - rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn rstctl_clear_mcperiph(&self) -> &RstctlClearMcperiph {
        &self.rstctl_clear_mcperiph
    }
    #[doc = "0x1430 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn stat_mcperiph(&self) -> &StatMcperiph {
        &self.stat_mcperiph
    }
    #[doc = "0x1434 - Register to control the power state"]
    #[inline(always)]
    pub const fn pwren_sysperiph(&self) -> &PwrenSysperiph {
        &self.pwren_sysperiph
    }
    #[doc = "0x1438 - rstcl assert Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn rstctl_assert_sysperiph(&self) -> &RstctlAssertSysperiph {
        &self.rstctl_assert_sysperiph
    }
    #[doc = "0x143c - rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn rstctl_clear_sysperiph(&self) -> &RstctlClearSysperiph {
        &self.rstctl_clear_sysperiph
    }
    #[doc = "0x1440 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn stat_sysperiph(&self) -> &StatSysperiph {
        &self.stat_sysperiph
    }
    #[doc = "0x1444 - Bits to select one of the many sources on CompHP inputs. Refer to Pinmux diagram for details."]
    #[inline(always)]
    pub const fn cmphpmxsel(&self) -> &Cmphpmxsel {
        &self.cmphpmxsel
    }
    #[doc = "0x144c - Bits to select one of the many sources on CompLP inputs. Refer to Pinmux diagram for details."]
    #[inline(always)]
    pub const fn cmplpmxsel(&self) -> &Cmplpmxsel {
        &self.cmplpmxsel
    }
    #[doc = "0x1450 - Bits to select one of the many sources on CompHN inputs. Refer to Pinmux diagram for details."]
    #[inline(always)]
    pub const fn cmphnmxsel(&self) -> &Cmphnmxsel {
        &self.cmphnmxsel
    }
    #[doc = "0x1454 - Bits to select one of the many sources on CompLN inputs. Refer to Pinmux diagram for details."]
    #[inline(always)]
    pub const fn cmplnmxsel(&self) -> &Cmplnmxsel {
        &self.cmplnmxsel
    }
    #[doc = "0x1458 - Temperature Sensor Config Register"]
    #[inline(always)]
    pub const fn tsnscfg(&self) -> &Tsnscfg {
        &self.tsnscfg
    }
    #[doc = "0x145c - Temperature Sensor Control Register"]
    #[inline(always)]
    pub const fn tsnssctl(&self) -> &Tsnssctl {
        &self.tsnssctl
    }
    #[doc = "0x1460 - PGA Configuration Register"]
    #[inline(always)]
    pub const fn pgaconfig(&self) -> &Pgaconfig {
        &self.pgaconfig
    }
    #[doc = "0x1464 - Reference Configuration Regsiter"]
    #[inline(always)]
    pub const fn refconfiga(&self) -> &Refconfiga {
        &self.refconfiga
    }
    #[doc = "0x1468 - Internal Test Node Control Register"]
    #[inline(always)]
    pub const fn internaltestctl(&self) -> &Internaltestctl {
        &self.internaltestctl
    }
    #[doc = "0x146c - I2V Logic Control"]
    #[inline(always)]
    pub const fn i2vctl(&self) -> &I2vctl {
        &self.i2vctl
    }
    #[doc = "0x1470 - Not used in AM13"]
    #[inline(always)]
    pub const fn adcdacloopback(&self) -> &Adcdacloopback {
        &self.adcdacloopback
    }
    #[doc = "0x1474 - XTAL Control Register"]
    #[inline(always)]
    pub const fn xtalcr(&self) -> &Xtalcr {
        &self.xtalcr
    }
    #[doc = "0x1478 - XTAL Control Register for pad init"]
    #[inline(always)]
    pub const fn xtalcr2(&self) -> &Xtalcr2 {
        &self.xtalcr2
    }
    #[doc = "0x147c - x1cnt status register"]
    #[inline(always)]
    pub const fn x1cnt(&self) -> &X1cnt {
        &self.x1cnt
    }
    #[doc = "0x1480 - CMPSS control register"]
    #[inline(always)]
    pub const fn cmpssctl(&self) -> &Cmpssctl {
        &self.cmpssctl
    }
    #[doc = "0x1484 - Config bits for CMPSS DAC buffer"]
    #[inline(always)]
    pub const fn cmpssdacbufconfig(&self) -> &Cmpssdacbufconfig {
        &self.cmpssdacbufconfig
    }
    #[doc = "0x1488 - Analog Reference Select"]
    #[inline(always)]
    pub const fn anarefctl(&self) -> &Anarefctl {
        &self.anarefctl
    }
    #[doc = "0x148c - PWM Time Base Clock sync"]
    #[inline(always)]
    pub const fn perclkcr(&self) -> &Perclkcr {
        &self.perclkcr
    }
    #[doc = "0x1490 - ADC MMR Override control register for DFT: Control ADC enable override"]
    #[inline(always)]
    pub const fn adc_mmr_ovrd_ctl(&self) -> &AdcMmrOvrdCtl {
        &self.adc_mmr_ovrd_ctl
    }
    #[doc = "0x1494 - ADC MMR Override value register for DFT : Value of ADC enable override"]
    #[inline(always)]
    pub const fn adc_mmr_ovrd_val(&self) -> &AdcMmrOvrdVal {
        &self.adc_mmr_ovrd_val
    }
    #[doc = "0x1498 - VREG Configuration Debug Register"]
    #[inline(always)]
    pub const fn vregconfigdebug(&self) -> &Vregconfigdebug {
        &self.vregconfigdebug
    }
    #[doc = "0x149c - VREG Configuration DFT Register"]
    #[inline(always)]
    pub const fn vregconfigdft(&self) -> &Vregconfigdft {
        &self.vregconfigdft
    }
    #[doc = "0x14a0 - AM13 Spare IREFEN SOC LOCK Reg"]
    #[inline(always)]
    pub const fn am13spareirefensoclock(&self) -> &Am13spareirefensoclock {
        &self.am13spareirefensoclock
    }
    #[doc = "0x14a4 - AM13 Spare SOC LOCK Reg 2"]
    #[inline(always)]
    pub const fn am13sparesoclock2(&self) -> &Am13sparesoclock2 {
        &self.am13sparesoclock2
    }
    #[doc = "0x14a8 - AM13 Spare SOC LOCK Reg 3"]
    #[inline(always)]
    pub const fn am13sparesoclock3(&self) -> &Am13sparesoclock3 {
        &self.am13sparesoclock3
    }
    #[doc = "0x14ac - AM13 Spare SOC LOCK Reg 4"]
    #[inline(always)]
    pub const fn am13sparesoclock4(&self) -> &Am13sparesoclock4 {
        &self.am13sparesoclock4
    }
    #[doc = "0x3038 - Flash Bank Swap Policy"]
    #[inline(always)]
    pub const fn flbankswppolicy(&self) -> &Flbankswppolicy {
        &self.flbankswppolicy
    }
    #[doc = "0x303c - Flash MAIN bank address swap"]
    #[inline(always)]
    pub const fn flbankswp(&self) -> &Flbankswp {
        &self.flbankswp
    }
    #[doc = "0x3048 - Security Configuration status"]
    #[inline(always)]
    pub const fn secstatus(&self) -> &Secstatus {
        &self.secstatus
    }
    #[doc = "0x3060 - INITCODE PASS"]
    #[inline(always)]
    pub const fn initdone(&self) -> &Initdone {
        &self.initdone
    }
}
#[doc = "IIDX (r) register accessor: SYSCTL interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`] module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "SYSCTL interrupt index"]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: SYSCTL interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`] module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "SYSCTL interrupt mask"]
pub mod imask;
#[doc = "RIS (r) register accessor: SYSCTL raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "SYSCTL raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "SYSCTL masked interrupt status"]
pub mod mis;
#[doc = "ISET (w) register accessor: SYSCTL interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`] module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "SYSCTL interrupt set"]
pub mod iset;
#[doc = "ICLR (w) register accessor: SYSCTL interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`] module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "SYSCTL interrupt clear"]
pub mod iclr;
#[doc = "NMIIIDX (r) register accessor: NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiiidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiidx`] module"]
#[doc(alias = "NMIIIDX")]
pub type Nmiiidx = crate::Reg<nmiiidx::NmiiidxSpec>;
#[doc = "NMI interrupt index"]
pub mod nmiiidx;
#[doc = "NMIRIS (r) register accessor: NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiris`] module"]
#[doc(alias = "NMIRIS")]
pub type Nmiris = crate::Reg<nmiris::NmirisSpec>;
#[doc = "NMI raw interrupt status"]
pub mod nmiris;
#[doc = "NMIISET (w) register accessor: NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiset`] module"]
#[doc(alias = "NMIISET")]
pub type Nmiiset = crate::Reg<nmiiset::NmiisetSpec>;
#[doc = "NMI interrupt set"]
pub mod nmiiset;
#[doc = "NMIICLR (w) register accessor: NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiclr`] module"]
#[doc(alias = "NMIICLR")]
pub type Nmiiclr = crate::Reg<nmiiclr::NmiiclrSpec>;
#[doc = "NMI interrupt clear"]
pub mod nmiiclr;
#[doc = "SYSOSCCFG (rw) register accessor: SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysosccfg`] module"]
#[doc(alias = "SYSOSCCFG")]
pub type Sysosccfg = crate::Reg<sysosccfg::SysosccfgSpec>;
#[doc = "SYSOSC configuration"]
pub mod sysosccfg;
#[doc = "MCLKCFG (rw) register accessor: Main clock (MCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkcfg`] module"]
#[doc(alias = "MCLKCFG")]
pub type Mclkcfg = crate::Reg<mclkcfg::MclkcfgSpec>;
#[doc = "Main clock (MCLK) configuration"]
pub mod mclkcfg;
#[doc = "HSCLKEN (rw) register accessor: High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsclken`] module"]
#[doc(alias = "HSCLKEN")]
pub type Hsclken = crate::Reg<hsclken::HsclkenSpec>;
#[doc = "High-speed clock (HSCLK) source enable/disable"]
pub mod hsclken;
#[doc = "HSCLKCFG (rw) register accessor: High-speed clock (HSCLK) source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsclkcfg`] module"]
#[doc(alias = "HSCLKCFG")]
pub type Hsclkcfg = crate::Reg<hsclkcfg::HsclkcfgSpec>;
#[doc = "High-speed clock (HSCLK) source selection"]
pub mod hsclkcfg;
#[doc = "HFCLKCLKCFG (rw) register accessor: High-frequency clock (HFCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfclkclkcfg`] module"]
#[doc(alias = "HFCLKCLKCFG")]
pub type Hfclkclkcfg = crate::Reg<hfclkclkcfg::HfclkclkcfgSpec>;
#[doc = "High-frequency clock (HFCLK) configuration"]
pub mod hfclkclkcfg;
#[doc = "SYSPLLCFG0 (rw) register accessor: SYSPLL reference and output configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllcfg0`] module"]
#[doc(alias = "SYSPLLCFG0")]
pub type Syspllcfg0 = crate::Reg<syspllcfg0::Syspllcfg0Spec>;
#[doc = "SYSPLL reference and output configuration"]
pub mod syspllcfg0;
#[doc = "SYSPLLCFG1 (rw) register accessor: SYSPLL reference and feedback divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllcfg1`] module"]
#[doc(alias = "SYSPLLCFG1")]
pub type Syspllcfg1 = crate::Reg<syspllcfg1::Syspllcfg1Spec>;
#[doc = "SYSPLL reference and feedback divider"]
pub mod syspllcfg1;
#[doc = "SYSPLLPARAM0 (rw) register accessor: SYSPLL PARAM0 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam0`] module"]
#[doc(alias = "SYSPLLPARAM0")]
pub type Syspllparam0 = crate::Reg<syspllparam0::Syspllparam0Spec>;
#[doc = "SYSPLL PARAM0 (load from FACTORY region)"]
pub mod syspllparam0;
#[doc = "SYSPLLPARAM1 (rw) register accessor: SYSPLL PARAM1 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam1`] module"]
#[doc(alias = "SYSPLLPARAM1")]
pub type Syspllparam1 = crate::Reg<syspllparam1::Syspllparam1Spec>;
#[doc = "SYSPLL PARAM1 (load from FACTORY region)"]
pub mod syspllparam1;
#[doc = "SYSPLLPARAM2 (rw) register accessor: SYSPLL PARAM2 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam2`] module"]
#[doc(alias = "SYSPLLPARAM2")]
pub type Syspllparam2 = crate::Reg<syspllparam2::Syspllparam2Spec>;
#[doc = "SYSPLL PARAM2 (load from FACTORY region)"]
pub mod syspllparam2;
#[doc = "SYSPLLLDOCTL (rw) register accessor: SYSPLL LDO CTL (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllldoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllldoctl`] module"]
#[doc(alias = "SYSPLLLDOCTL")]
pub type Syspllldoctl = crate::Reg<syspllldoctl::SyspllldoctlSpec>;
#[doc = "SYSPLL LDO CTL (load from FACTORY region)"]
pub mod syspllldoctl;
#[doc = "SYSPLLLDOPROG (rw) register accessor: SYSPLL LDO VOUT PROG (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllldoprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllldoprog`] module"]
#[doc(alias = "SYSPLLLDOPROG")]
pub type Syspllldoprog = crate::Reg<syspllldoprog::SyspllldoprogSpec>;
#[doc = "SYSPLL LDO VOUT PROG (load from FACTORY region)"]
pub mod syspllldoprog;
#[doc = "GENCLKEN (rw) register accessor: General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`genclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genclken`] module"]
#[doc(alias = "GENCLKEN")]
pub type Genclken = crate::Reg<genclken::GenclkenSpec>;
#[doc = "General clock enable control"]
pub mod genclken;
#[doc = "GENCLKCFG (rw) register accessor: General clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`genclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genclkcfg`] module"]
#[doc(alias = "GENCLKCFG")]
pub type Genclkcfg = crate::Reg<genclkcfg::GenclkcfgSpec>;
#[doc = "General clock configuration"]
pub mod genclkcfg;
#[doc = "PMODECFG (rw) register accessor: Power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pmodecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmodecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmodecfg`] module"]
#[doc(alias = "PMODECFG")]
pub type Pmodecfg = crate::Reg<pmodecfg::PmodecfgSpec>;
#[doc = "Power mode configuration"]
pub mod pmodecfg;
#[doc = "MLDOLPENCFG (rw) register accessor: LDO Configuration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mldolpencfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mldolpencfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mldolpencfg`] module"]
#[doc(alias = "MLDOLPENCFG")]
pub type Mldolpencfg = crate::Reg<mldolpencfg::MldolpencfgSpec>;
#[doc = "LDO Configuration Control"]
pub mod mldolpencfg;
#[doc = "FCC (r) register accessor: Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`fcc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcc`] module"]
#[doc(alias = "FCC")]
pub type Fcc = crate::Reg<fcc::FccSpec>;
#[doc = "Frequency clock counter (FCC) count"]
pub mod fcc;
#[doc = "PMULDOSPARECTL (rw) register accessor: LDO Spare Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuldosparectl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuldosparectl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuldosparectl`] module"]
#[doc(alias = "PMULDOSPARECTL")]
pub type Pmuldosparectl = crate::Reg<pmuldosparectl::PmuldosparectlSpec>;
#[doc = "LDO Spare Control"]
pub mod pmuldosparectl;
#[doc = "ECO_REG1 (rw) register accessor: Sysctl ECO Reg 1\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_reg1`] module"]
#[doc(alias = "ECO_REG1")]
pub type EcoReg1 = crate::Reg<eco_reg1::EcoReg1Spec>;
#[doc = "Sysctl ECO Reg 1"]
pub mod eco_reg1;
#[doc = "ECO_REG2 (rw) register accessor: Sysctl ECO Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eco_reg2`] module"]
#[doc(alias = "ECO_REG2")]
pub type EcoReg2 = crate::Reg<eco_reg2::EcoReg2Spec>;
#[doc = "Sysctl ECO Reg 2"]
pub mod eco_reg2;
#[doc = "SYSTEMCFG (rw) register accessor: System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`systemcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemcfg`] module"]
#[doc(alias = "SYSTEMCFG")]
pub type Systemcfg = crate::Reg<systemcfg::SystemcfgSpec>;
#[doc = "System configuration"]
pub mod systemcfg;
#[doc = "SRAMCFG (rw) register accessor: System SRAM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sramcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramcfg`] module"]
#[doc(alias = "SRAMCFG")]
pub type Sramcfg = crate::Reg<sramcfg::SramcfgSpec>;
#[doc = "System SRAM configuration"]
pub mod sramcfg;
#[doc = "WRITELOCK (rw) register accessor: SYSCTL register write lockout\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writelock`] module"]
#[doc(alias = "WRITELOCK")]
pub type Writelock = crate::Reg<writelock::WritelockSpec>;
#[doc = "SYSCTL register write lockout"]
pub mod writelock;
#[doc = "CLKSTATUS (r) register accessor: Clock module (CKM) status\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstatus`] module"]
#[doc(alias = "CLKSTATUS")]
pub type Clkstatus = crate::Reg<clkstatus::ClkstatusSpec>;
#[doc = "Clock module (CKM) status"]
pub mod clkstatus;
#[doc = "SYSSTATUS (r) register accessor: System status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysstatus`] module"]
#[doc(alias = "SYSSTATUS")]
pub type Sysstatus = crate::Reg<sysstatus::SysstatusSpec>;
#[doc = "System status information"]
pub mod sysstatus;
#[doc = "RSTCAUSE (r) register accessor: Reset cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcause`] module"]
#[doc(alias = "RSTCAUSE")]
pub type Rstcause = crate::Reg<rstcause::RstcauseSpec>;
#[doc = "Reset cause"]
pub mod rstcause;
#[doc = "RESETLEVEL (rw) register accessor: Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`resetlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetlevel`] module"]
#[doc(alias = "RESETLEVEL")]
pub type Resetlevel = crate::Reg<resetlevel::ResetlevelSpec>;
#[doc = "Reset level for application-triggered reset command"]
pub mod resetlevel;
#[doc = "RESETCMD (w) register accessor: Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetcmd`] module"]
#[doc(alias = "RESETCMD")]
pub type Resetcmd = crate::Reg<resetcmd::ResetcmdSpec>;
#[doc = "Execute an application-triggered reset command"]
pub mod resetcmd;
#[doc = "SYSOSCFCLCTL (w) register accessor: SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscfclctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysoscfclctl`] module"]
#[doc(alias = "SYSOSCFCLCTL")]
pub type Sysoscfclctl = crate::Reg<sysoscfclctl::SysoscfclctlSpec>;
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable"]
pub mod sysoscfclctl;
#[doc = "SHDNIOREL (w) register accessor: SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shdniorel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdniorel`] module"]
#[doc(alias = "SHDNIOREL")]
pub type Shdniorel = crate::Reg<shdniorel::ShdniorelSpec>;
#[doc = "SHUTDOWN IO release control"]
pub mod shdniorel;
#[doc = "EXRSTPIN (w) register accessor: Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrstpin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrstpin`] module"]
#[doc(alias = "EXRSTPIN")]
pub type Exrstpin = crate::Reg<exrstpin::ExrstpinSpec>;
#[doc = "Disable the reset function of the NRST pin"]
pub mod exrstpin;
#[doc = "SYSSTATUSCLR (w) register accessor: Clear sticky bits of SYSSTATUS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysstatusclr`] module"]
#[doc(alias = "SYSSTATUSCLR")]
pub type Sysstatusclr = crate::Reg<sysstatusclr::SysstatusclrSpec>;
#[doc = "Clear sticky bits of SYSSTATUS"]
pub mod sysstatusclr;
#[doc = "SWDJCFG (w) register accessor: Disable the SWD/JTAG function on the SWD/JTAG pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdjcfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdjcfg`] module"]
#[doc(alias = "SWDJCFG")]
pub type Swdjcfg = crate::Reg<swdjcfg::SwdjcfgSpec>;
#[doc = "Disable the SWD/JTAG function on the SWD/JTAG pins"]
pub mod swdjcfg;
#[doc = "FCCCMD (w) register accessor: Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcccmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcccmd`] module"]
#[doc(alias = "FCCCMD")]
pub type Fcccmd = crate::Reg<fcccmd::FcccmdSpec>;
#[doc = "Frequency clock counter start capture"]
pub mod fcccmd;
#[doc = "SHUTDNSTORE0 (rw) register accessor: Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore0`] module"]
#[doc(alias = "SHUTDNSTORE0")]
pub type Shutdnstore0 = crate::Reg<shutdnstore0::Shutdnstore0Spec>;
#[doc = "Shutdown storage memory (byte 0)"]
pub mod shutdnstore0;
#[doc = "SHUTDNSTORE1 (rw) register accessor: Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore1`] module"]
#[doc(alias = "SHUTDNSTORE1")]
pub type Shutdnstore1 = crate::Reg<shutdnstore1::Shutdnstore1Spec>;
#[doc = "Shutdown storage memory (byte 1)"]
pub mod shutdnstore1;
#[doc = "SHUTDNSTORE2 (rw) register accessor: Shutdown storage memory (byte 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore2`] module"]
#[doc(alias = "SHUTDNSTORE2")]
pub type Shutdnstore2 = crate::Reg<shutdnstore2::Shutdnstore2Spec>;
#[doc = "Shutdown storage memory (byte 2)"]
pub mod shutdnstore2;
#[doc = "SHUTDNSTORE3 (rw) register accessor: Shutdown storage memory (byte 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore3`] module"]
#[doc(alias = "SHUTDNSTORE3")]
pub type Shutdnstore3 = crate::Reg<shutdnstore3::Shutdnstore3Spec>;
#[doc = "Shutdown storage memory (byte 3)"]
pub mod shutdnstore3;
#[doc = "ADCSEQFRCGB (rw) register accessor: ADC Global Sequence Force\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqfrcgb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqfrcgb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseqfrcgb`] module"]
#[doc(alias = "ADCSEQFRCGB")]
pub type Adcseqfrcgb = crate::Reg<adcseqfrcgb::AdcseqfrcgbSpec>;
#[doc = "ADC Global Sequence Force"]
pub mod adcseqfrcgb;
#[doc = "ADCSEQFRCGBSEL (rw) register accessor: ADC Global Sequence Force Select\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqfrcgbsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqfrcgbsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcseqfrcgbsel`] module"]
#[doc(alias = "ADCSEQFRCGBSEL")]
pub type Adcseqfrcgbsel = crate::Reg<adcseqfrcgbsel::AdcseqfrcgbselSpec>;
#[doc = "ADC Global Sequence Force Select"]
pub mod adcseqfrcgbsel;
#[doc = "M33SPARESOCLOCK1 (rw) register accessor: M33C1 Spare SOC LOCK Reg 1\n\nYou can [`read`](crate::Reg::read) this register and get [`m33sparesoclock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33sparesoclock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m33sparesoclock1`] module"]
#[doc(alias = "M33SPARESOCLOCK1")]
pub type M33sparesoclock1 = crate::Reg<m33sparesoclock1::M33sparesoclock1Spec>;
#[doc = "M33C1 Spare SOC LOCK Reg 1"]
pub mod m33sparesoclock1;
#[doc = "M33SPARESOCLOCK2 (rw) register accessor: M33C1 Spare SOC LOCK Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`m33sparesoclock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33sparesoclock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m33sparesoclock2`] module"]
#[doc(alias = "M33SPARESOCLOCK2")]
pub type M33sparesoclock2 = crate::Reg<m33sparesoclock2::M33sparesoclock2Spec>;
#[doc = "M33C1 Spare SOC LOCK Reg 2"]
pub mod m33sparesoclock2;
#[doc = "READ_REG (rw) register accessor: Sysctl read only Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`read_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read_reg`] module"]
#[doc(alias = "READ_REG")]
pub type ReadReg = crate::Reg<read_reg::ReadRegSpec>;
#[doc = "Sysctl read only Reg"]
pub mod read_reg;
#[doc = "PWREN_MCPERIPH (rw) register accessor: Register to control the power state\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren_mcperiph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren_mcperiph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren_mcperiph`] module"]
#[doc(alias = "PWREN_MCPERIPH")]
pub type PwrenMcperiph = crate::Reg<pwren_mcperiph::PwrenMcperiphSpec>;
#[doc = "Register to control the power state"]
pub mod pwren_mcperiph;
#[doc = "RSTCTL_ASSERT_MCPERIPH (w) register accessor: rstctl assert register to control reset assertion - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_assert_mcperiph::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_assert_mcperiph`] module"]
#[doc(alias = "RSTCTL_ASSERT_MCPERIPH")]
pub type RstctlAssertMcperiph = crate::Reg<rstctl_assert_mcperiph::RstctlAssertMcperiphSpec>;
#[doc = "rstctl assert register to control reset assertion - Write Only Register, Always Read as 0"]
pub mod rstctl_assert_mcperiph;
#[doc = "RSTCTL_CLEAR_MCPERIPH (w) register accessor: rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_clear_mcperiph::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_clear_mcperiph`] module"]
#[doc(alias = "RSTCTL_CLEAR_MCPERIPH")]
pub type RstctlClearMcperiph = crate::Reg<rstctl_clear_mcperiph::RstctlClearMcperiphSpec>;
#[doc = "rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0"]
pub mod rstctl_clear_mcperiph;
#[doc = "STAT_MCPERIPH (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_mcperiph::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_mcperiph`] module"]
#[doc(alias = "STAT_MCPERIPH")]
pub type StatMcperiph = crate::Reg<stat_mcperiph::StatMcperiphSpec>;
#[doc = "IP State Register - Read Only"]
pub mod stat_mcperiph;
#[doc = "PWREN_SYSPERIPH (rw) register accessor: Register to control the power state\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren_sysperiph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren_sysperiph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren_sysperiph`] module"]
#[doc(alias = "PWREN_SYSPERIPH")]
pub type PwrenSysperiph = crate::Reg<pwren_sysperiph::PwrenSysperiphSpec>;
#[doc = "Register to control the power state"]
pub mod pwren_sysperiph;
#[doc = "RSTCTL_ASSERT_SYSPERIPH (w) register accessor: rstcl assert Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_assert_sysperiph::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_assert_sysperiph`] module"]
#[doc(alias = "RSTCTL_ASSERT_SYSPERIPH")]
pub type RstctlAssertSysperiph = crate::Reg<rstctl_assert_sysperiph::RstctlAssertSysperiphSpec>;
#[doc = "rstcl assert Register - Write Only Register, Always Read as 0"]
pub mod rstctl_assert_sysperiph;
#[doc = "RSTCTL_CLEAR_SYSPERIPH (w) register accessor: rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_clear_sysperiph::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_clear_sysperiph`] module"]
#[doc(alias = "RSTCTL_CLEAR_SYSPERIPH")]
pub type RstctlClearSysperiph = crate::Reg<rstctl_clear_sysperiph::RstctlClearSysperiphSpec>;
#[doc = "rstctl clear register to control reset de-assertion - Write Only Register, Always Read as 0"]
pub mod rstctl_clear_sysperiph;
#[doc = "STAT_SYSPERIPH (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_sysperiph::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_sysperiph`] module"]
#[doc(alias = "STAT_SYSPERIPH")]
pub type StatSysperiph = crate::Reg<stat_sysperiph::StatSysperiphSpec>;
#[doc = "IP State Register - Read Only"]
pub mod stat_sysperiph;
#[doc = "CMPHPMXSEL (rw) register accessor: Bits to select one of the many sources on CompHP inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmphpmxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmphpmxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmphpmxsel`] module"]
#[doc(alias = "CMPHPMXSEL")]
pub type Cmphpmxsel = crate::Reg<cmphpmxsel::CmphpmxselSpec>;
#[doc = "Bits to select one of the many sources on CompHP inputs. Refer to Pinmux diagram for details."]
pub mod cmphpmxsel;
#[doc = "CMPLPMXSEL (rw) register accessor: Bits to select one of the many sources on CompLP inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmplpmxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmplpmxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmplpmxsel`] module"]
#[doc(alias = "CMPLPMXSEL")]
pub type Cmplpmxsel = crate::Reg<cmplpmxsel::CmplpmxselSpec>;
#[doc = "Bits to select one of the many sources on CompLP inputs. Refer to Pinmux diagram for details."]
pub mod cmplpmxsel;
#[doc = "CMPHNMXSEL (rw) register accessor: Bits to select one of the many sources on CompHN inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmphnmxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmphnmxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmphnmxsel`] module"]
#[doc(alias = "CMPHNMXSEL")]
pub type Cmphnmxsel = crate::Reg<cmphnmxsel::CmphnmxselSpec>;
#[doc = "Bits to select one of the many sources on CompHN inputs. Refer to Pinmux diagram for details."]
pub mod cmphnmxsel;
#[doc = "CMPLNMXSEL (rw) register accessor: Bits to select one of the many sources on CompLN inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmplnmxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmplnmxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmplnmxsel`] module"]
#[doc(alias = "CMPLNMXSEL")]
pub type Cmplnmxsel = crate::Reg<cmplnmxsel::CmplnmxselSpec>;
#[doc = "Bits to select one of the many sources on CompLN inputs. Refer to Pinmux diagram for details."]
pub mod cmplnmxsel;
#[doc = "TSNSCFG (rw) register accessor: Temperature Sensor Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsnscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsnscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsnscfg`] module"]
#[doc(alias = "TSNSCFG")]
pub type Tsnscfg = crate::Reg<tsnscfg::TsnscfgSpec>;
#[doc = "Temperature Sensor Config Register"]
pub mod tsnscfg;
#[doc = "TSNSSCTL (rw) register accessor: Temperature Sensor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsnssctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsnssctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsnssctl`] module"]
#[doc(alias = "TSNSSCTL")]
pub type Tsnssctl = crate::Reg<tsnssctl::TsnssctlSpec>;
#[doc = "Temperature Sensor Control Register"]
pub mod tsnssctl;
#[doc = "PGACONFIG (rw) register accessor: PGA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgaconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgaconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgaconfig`] module"]
#[doc(alias = "PGACONFIG")]
pub type Pgaconfig = crate::Reg<pgaconfig::PgaconfigSpec>;
#[doc = "PGA Configuration Register"]
pub mod pgaconfig;
#[doc = "REFCONFIGA (rw) register accessor: Reference Configuration Regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`refconfiga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refconfiga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refconfiga`] module"]
#[doc(alias = "REFCONFIGA")]
pub type Refconfiga = crate::Reg<refconfiga::RefconfigaSpec>;
#[doc = "Reference Configuration Regsiter"]
pub mod refconfiga;
#[doc = "INTERNALTESTCTL (rw) register accessor: Internal Test Node Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`internaltestctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internaltestctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internaltestctl`] module"]
#[doc(alias = "INTERNALTESTCTL")]
pub type Internaltestctl = crate::Reg<internaltestctl::InternaltestctlSpec>;
#[doc = "Internal Test Node Control Register"]
pub mod internaltestctl;
#[doc = "I2VCTL (rw) register accessor: I2V Logic Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2vctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2vctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2vctl`] module"]
#[doc(alias = "I2VCTL")]
pub type I2vctl = crate::Reg<i2vctl::I2vctlSpec>;
#[doc = "I2V Logic Control"]
pub mod i2vctl;
#[doc = "ADCDACLOOPBACK (rw) register accessor: Not used in AM13\n\nYou can [`read`](crate::Reg::read) this register and get [`adcdacloopback::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcdacloopback::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcdacloopback`] module"]
#[doc(alias = "ADCDACLOOPBACK")]
pub type Adcdacloopback = crate::Reg<adcdacloopback::AdcdacloopbackSpec>;
#[doc = "Not used in AM13"]
pub mod adcdacloopback;
#[doc = "XTALCR (rw) register accessor: XTAL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcr`] module"]
#[doc(alias = "XTALCR")]
pub type Xtalcr = crate::Reg<xtalcr::XtalcrSpec>;
#[doc = "XTAL Control Register"]
pub mod xtalcr;
#[doc = "XTALCR2 (rw) register accessor: XTAL Control Register for pad init\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcr2`] module"]
#[doc(alias = "XTALCR2")]
pub type Xtalcr2 = crate::Reg<xtalcr2::Xtalcr2Spec>;
#[doc = "XTAL Control Register for pad init"]
pub mod xtalcr2;
#[doc = "X1CNT (rw) register accessor: x1cnt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`x1cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x1cnt`] module"]
#[doc(alias = "X1CNT")]
pub type X1cnt = crate::Reg<x1cnt::X1cntSpec>;
#[doc = "x1cnt status register"]
pub mod x1cnt;
#[doc = "CMPSSCTL (rw) register accessor: CMPSS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpssctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpssctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpssctl`] module"]
#[doc(alias = "CMPSSCTL")]
pub type Cmpssctl = crate::Reg<cmpssctl::CmpssctlSpec>;
#[doc = "CMPSS control register"]
pub mod cmpssctl;
#[doc = "CMPSSDACBUFCONFIG (rw) register accessor: Config bits for CMPSS DAC buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpssdacbufconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpssdacbufconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpssdacbufconfig`] module"]
#[doc(alias = "CMPSSDACBUFCONFIG")]
pub type Cmpssdacbufconfig = crate::Reg<cmpssdacbufconfig::CmpssdacbufconfigSpec>;
#[doc = "Config bits for CMPSS DAC buffer"]
pub mod cmpssdacbufconfig;
#[doc = "ANAREFCTL (rw) register accessor: Analog Reference Select\n\nYou can [`read`](crate::Reg::read) this register and get [`anarefctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anarefctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anarefctl`] module"]
#[doc(alias = "ANAREFCTL")]
pub type Anarefctl = crate::Reg<anarefctl::AnarefctlSpec>;
#[doc = "Analog Reference Select"]
pub mod anarefctl;
#[doc = "PERCLKCR (rw) register accessor: PWM Time Base Clock sync\n\nYou can [`read`](crate::Reg::read) this register and get [`perclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perclkcr`] module"]
#[doc(alias = "PERCLKCR")]
pub type Perclkcr = crate::Reg<perclkcr::PerclkcrSpec>;
#[doc = "PWM Time Base Clock sync"]
pub mod perclkcr;
#[doc = "ADC_MMR_OVRD_CTL (rw) register accessor: ADC MMR Override control register for DFT: Control ADC enable override\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_mmr_ovrd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_mmr_ovrd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_mmr_ovrd_ctl`] module"]
#[doc(alias = "ADC_MMR_OVRD_CTL")]
pub type AdcMmrOvrdCtl = crate::Reg<adc_mmr_ovrd_ctl::AdcMmrOvrdCtlSpec>;
#[doc = "ADC MMR Override control register for DFT: Control ADC enable override"]
pub mod adc_mmr_ovrd_ctl;
#[doc = "ADC_MMR_OVRD_VAL (rw) register accessor: ADC MMR Override value register for DFT : Value of ADC enable override\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_mmr_ovrd_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_mmr_ovrd_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_mmr_ovrd_val`] module"]
#[doc(alias = "ADC_MMR_OVRD_VAL")]
pub type AdcMmrOvrdVal = crate::Reg<adc_mmr_ovrd_val::AdcMmrOvrdValSpec>;
#[doc = "ADC MMR Override value register for DFT : Value of ADC enable override"]
pub mod adc_mmr_ovrd_val;
#[doc = "VREGCONFIGDEBUG (rw) register accessor: VREG Configuration Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vregconfigdebug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregconfigdebug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vregconfigdebug`] module"]
#[doc(alias = "VREGCONFIGDEBUG")]
pub type Vregconfigdebug = crate::Reg<vregconfigdebug::VregconfigdebugSpec>;
#[doc = "VREG Configuration Debug Register"]
pub mod vregconfigdebug;
#[doc = "VREGCONFIGDFT (rw) register accessor: VREG Configuration DFT Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vregconfigdft::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregconfigdft::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vregconfigdft`] module"]
#[doc(alias = "VREGCONFIGDFT")]
pub type Vregconfigdft = crate::Reg<vregconfigdft::VregconfigdftSpec>;
#[doc = "VREG Configuration DFT Register"]
pub mod vregconfigdft;
#[doc = "AM13SPAREIREFENSOCLOCK (rw) register accessor: AM13 Spare IREFEN SOC LOCK Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`am13spareirefensoclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13spareirefensoclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@am13spareirefensoclock`] module"]
#[doc(alias = "AM13SPAREIREFENSOCLOCK")]
pub type Am13spareirefensoclock = crate::Reg<am13spareirefensoclock::Am13spareirefensoclockSpec>;
#[doc = "AM13 Spare IREFEN SOC LOCK Reg"]
pub mod am13spareirefensoclock;
#[doc = "AM13SPARESOCLOCK2 (rw) register accessor: AM13 Spare SOC LOCK Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`am13sparesoclock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13sparesoclock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@am13sparesoclock2`] module"]
#[doc(alias = "AM13SPARESOCLOCK2")]
pub type Am13sparesoclock2 = crate::Reg<am13sparesoclock2::Am13sparesoclock2Spec>;
#[doc = "AM13 Spare SOC LOCK Reg 2"]
pub mod am13sparesoclock2;
#[doc = "AM13SPARESOCLOCK3 (rw) register accessor: AM13 Spare SOC LOCK Reg 3\n\nYou can [`read`](crate::Reg::read) this register and get [`am13sparesoclock3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13sparesoclock3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@am13sparesoclock3`] module"]
#[doc(alias = "AM13SPARESOCLOCK3")]
pub type Am13sparesoclock3 = crate::Reg<am13sparesoclock3::Am13sparesoclock3Spec>;
#[doc = "AM13 Spare SOC LOCK Reg 3"]
pub mod am13sparesoclock3;
#[doc = "AM13SPARESOCLOCK4 (rw) register accessor: AM13 Spare SOC LOCK Reg 4\n\nYou can [`read`](crate::Reg::read) this register and get [`am13sparesoclock4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13sparesoclock4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@am13sparesoclock4`] module"]
#[doc(alias = "AM13SPARESOCLOCK4")]
pub type Am13sparesoclock4 = crate::Reg<am13sparesoclock4::Am13sparesoclock4Spec>;
#[doc = "AM13 Spare SOC LOCK Reg 4"]
pub mod am13sparesoclock4;
#[doc = "FLBANKSWPPOLICY (w) register accessor: Flash Bank Swap Policy\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flbankswppolicy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flbankswppolicy`] module"]
#[doc(alias = "FLBANKSWPPOLICY")]
pub type Flbankswppolicy = crate::Reg<flbankswppolicy::FlbankswppolicySpec>;
#[doc = "Flash Bank Swap Policy"]
pub mod flbankswppolicy;
#[doc = "FLBANKSWP (w) register accessor: Flash MAIN bank address swap\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flbankswp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flbankswp`] module"]
#[doc(alias = "FLBANKSWP")]
pub type Flbankswp = crate::Reg<flbankswp::FlbankswpSpec>;
#[doc = "Flash MAIN bank address swap"]
pub mod flbankswp;
#[doc = "SECSTATUS (r) register accessor: Security Configuration status\n\nYou can [`read`](crate::Reg::read) this register and get [`secstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secstatus`] module"]
#[doc(alias = "SECSTATUS")]
pub type Secstatus = crate::Reg<secstatus::SecstatusSpec>;
#[doc = "Security Configuration status"]
pub mod secstatus;
#[doc = "INITDONE (w) register accessor: INITCODE PASS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initdone::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initdone`] module"]
#[doc(alias = "INITDONE")]
pub type Initdone = crate::Reg<initdone::InitdoneSpec>;
#[doc = "INITCODE PASS"]
pub mod initdone;
