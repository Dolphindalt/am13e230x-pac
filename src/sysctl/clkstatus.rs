#[doc = "Register `CLKSTATUS` reader"]
pub type R = crate::R<ClkstatusSpec>;
#[doc = "SYSOSCFREQ indicates the current SYSOSC operating frequency.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sysoscfreq {
    #[doc = "0: SYSOSC is at base frequency (32MHz)"]
    Sysosc32m = 0,
    #[doc = "1: SYSOSC is at low frequency (4MHz)"]
    Sysosc4m = 1,
}
impl From<Sysoscfreq> for u8 {
    #[inline(always)]
    fn from(variant: Sysoscfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysoscfreq {
    type Ux = u8;
}
impl crate::IsEnum for Sysoscfreq {}
#[doc = "Field `SYSOSCFREQ` reader - SYSOSCFREQ indicates the current SYSOSC operating frequency."]
pub type SysoscfreqR = crate::FieldReader<Sysoscfreq>;
impl SysoscfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysoscfreq> {
        match self.bits {
            0 => Some(Sysoscfreq::Sysosc32m),
            1 => Some(Sysoscfreq::Sysosc4m),
            _ => None,
        }
    }
    #[doc = "SYSOSC is at base frequency (32MHz)"]
    #[inline(always)]
    pub fn is_sysosc32m(&self) -> bool {
        *self == Sysoscfreq::Sysosc32m
    }
    #[doc = "SYSOSC is at low frequency (4MHz)"]
    #[inline(always)]
    pub fn is_sysosc4m(&self) -> bool {
        *self == Sysoscfreq::Sysosc4m
    }
}
#[doc = "HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkmux {
    #[doc = "0: MCLK is not sourced from HSCLK"]
    Sysosc = 0,
    #[doc = "1: MCLK is sourced from HSCLK"]
    Hsclk = 1,
}
impl From<Hsclkmux> for bool {
    #[inline(always)]
    fn from(variant: Hsclkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKMUX` reader - HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK)."]
pub type HsclkmuxR = crate::BitReader<Hsclkmux>;
impl HsclkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkmux {
        match self.bits {
            false => Hsclkmux::Sysosc,
            true => Hsclkmux::Hsclk,
        }
    }
    #[doc = "MCLK is not sourced from HSCLK"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Hsclkmux::Sysosc
    }
    #[doc = "MCLK is sourced from HSCLK"]
    #[inline(always)]
    pub fn is_hsclk(&self) -> bool {
        *self == Hsclkmux::Hsclk
    }
}
#[doc = "Field `HFCLKGOOD` reader - HFCLKGOOD indicates that the HFCLK started correctly. When the XTAL is started or HFCLK_IN is selected as the HFCLK source, this bit will be set by hardware if a valid HFCLK is detected, and cleared if HFCLK is not operating within the expected range."]
pub type HfclkgoodR = crate::BitReader;
#[doc = "Field `SYSPLLGOOD` reader - SYSPLLGOOD indicates if the SYSPLL started correctly. When the SYSPLL is started, SYSPLLGOOD is cleared by hardware. After the startup settling time has expired, the SYSPLL status is tested. If the SYSPLL started successfully the SYSPLLGOOD bit is set, else it is left cleared."]
pub type SyspllgoodR = crate::BitReader;
#[doc = "Field `LFOSCGOOD` reader - LFOSCGOOD indicates when the LFOSC startup has completed and the LFOSC is ready for use."]
pub type LfoscgoodR = crate::BitReader;
#[doc = "Field `HSCLKSOFF` reader - HSCLKSOFF is set when the high speed clock sources (SYSPLL, HFCLK) are disabled or dead. It is the logical AND of HFCLKOFF and SYSPLLOFF."]
pub type HsclksoffR = crate::BitReader;
#[doc = "Field `HFCLKOFF` reader - HFCLKOFF indicates if the HFCLK is disabled or was dead at startup. When the HFCLK is started, HFCLKOFF is cleared by hardware. Following startup of the HFCLK, if the HFCLK startup monitor determines that the HFCLK was not started correctly, HFCLKOFF is set."]
pub type HfclkoffR = crate::BitReader;
#[doc = "Field `SYSPLLOFF` reader - SYSPLLOFF indicates if the SYSPLL is disabled or was dead at startup. When the SYSPLL is started, SYSPLLOFF is cleared by hardware. Following startup of the SYSPLL, if the SYSPLL startup monitor determines that the SYSPLL was not started correctly, SYSPLLOFF is set."]
pub type SysplloffR = crate::BitReader;
#[doc = "CURHSCLKSEL indicates the current clock source for HSCLK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curhsclksel {
    #[doc = "0: HSCLK is currently sourced from the SYSPLL"]
    Syspll = 0,
    #[doc = "1: HSCLK is currently sourced from the HFCLK"]
    Hfclk = 1,
}
impl From<Curhsclksel> for bool {
    #[inline(always)]
    fn from(variant: Curhsclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURHSCLKSEL` reader - CURHSCLKSEL indicates the current clock source for HSCLK."]
pub type CurhsclkselR = crate::BitReader<Curhsclksel>;
impl CurhsclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curhsclksel {
        match self.bits {
            false => Curhsclksel::Syspll,
            true => Curhsclksel::Hfclk,
        }
    }
    #[doc = "HSCLK is currently sourced from the SYSPLL"]
    #[inline(always)]
    pub fn is_syspll(&self) -> bool {
        *self == Curhsclksel::Syspll
    }
    #[doc = "HSCLK is currently sourced from the HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Curhsclksel::Hfclk
    }
}
#[doc = "Field `HSCLKDEAD` reader - HSCLKDEAD is set by hardware if the selected source for HSCLK was started but did not start successfully."]
pub type HsclkdeadR = crate::BitReader;
#[doc = "Field `HSCLKGOOD` reader - HSCLKGOOD is set by hardware if the selected clock source for HSCLK started successfully."]
pub type HsclkgoodR = crate::BitReader;
#[doc = "FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fclmode {
    #[doc = "0: SYSOSC FCL is disabled"]
    Disabled = 0,
    #[doc = "1: SYSOSC FCL is enabled"]
    Enabled = 1,
}
impl From<Fclmode> for bool {
    #[inline(always)]
    fn from(variant: Fclmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCLMODE` reader - FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled."]
pub type FclmodeR = crate::BitReader<Fclmode>;
impl FclmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fclmode {
        match self.bits {
            false => Fclmode::Disabled,
            true => Fclmode::Enabled,
        }
    }
    #[doc = "SYSOSC FCL is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fclmode::Disabled
    }
    #[doc = "SYSOSC FCL is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fclmode::Enabled
    }
}
#[doc = "FCCDONE indicates when a frequency clock counter capture is complete.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fccdone {
    #[doc = "0: FCC capture is not done"]
    Notdone = 0,
    #[doc = "1: FCC capture is done"]
    Done = 1,
}
impl From<Fccdone> for bool {
    #[inline(always)]
    fn from(variant: Fccdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCDONE` reader - FCCDONE indicates when a frequency clock counter capture is complete."]
pub type FccdoneR = crate::BitReader<Fccdone>;
impl FccdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fccdone {
        match self.bits {
            false => Fccdone::Notdone,
            true => Fccdone::Done,
        }
    }
    #[doc = "FCC capture is not done"]
    #[inline(always)]
    pub fn is_notdone(&self) -> bool {
        *self == Fccdone::Notdone
    }
    #[doc = "FCC capture is done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Fccdone::Done
    }
}
#[doc = "Field `HFCLKBLKUPD` reader - HFCLKBLKUPD indicates when writes to the HFCLKCLKCFG register are blocked."]
pub type HfclkblkupdR = crate::BitReader;
#[doc = "Field `SYSPLLBLKUPD` reader - SYSPLLBLKUPD indicates when writes to SYSPLLCFG0/1 and SYSPLLPARAM0/1 are blocked."]
pub type SyspllblkupdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - SYSOSCFREQ indicates the current SYSOSC operating frequency."]
    #[inline(always)]
    pub fn sysoscfreq(&self) -> SysoscfreqR {
        SysoscfreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - HSCLKMUX indicates if MCLK is currently sourced from the high-speed clock (HSCLK)."]
    #[inline(always)]
    pub fn hsclkmux(&self) -> HsclkmuxR {
        HsclkmuxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - HFCLKGOOD indicates that the HFCLK started correctly. When the XTAL is started or HFCLK_IN is selected as the HFCLK source, this bit will be set by hardware if a valid HFCLK is detected, and cleared if HFCLK is not operating within the expected range."]
    #[inline(always)]
    pub fn hfclkgood(&self) -> HfclkgoodR {
        HfclkgoodR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYSPLLGOOD indicates if the SYSPLL started correctly. When the SYSPLL is started, SYSPLLGOOD is cleared by hardware. After the startup settling time has expired, the SYSPLL status is tested. If the SYSPLL started successfully the SYSPLLGOOD bit is set, else it is left cleared."]
    #[inline(always)]
    pub fn syspllgood(&self) -> SyspllgoodR {
        SyspllgoodR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LFOSCGOOD indicates when the LFOSC startup has completed and the LFOSC is ready for use."]
    #[inline(always)]
    pub fn lfoscgood(&self) -> LfoscgoodR {
        LfoscgoodR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HSCLKSOFF is set when the high speed clock sources (SYSPLL, HFCLK) are disabled or dead. It is the logical AND of HFCLKOFF and SYSPLLOFF."]
    #[inline(always)]
    pub fn hsclksoff(&self) -> HsclksoffR {
        HsclksoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HFCLKOFF indicates if the HFCLK is disabled or was dead at startup. When the HFCLK is started, HFCLKOFF is cleared by hardware. Following startup of the HFCLK, if the HFCLK startup monitor determines that the HFCLK was not started correctly, HFCLKOFF is set."]
    #[inline(always)]
    pub fn hfclkoff(&self) -> HfclkoffR {
        HfclkoffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYSPLLOFF indicates if the SYSPLL is disabled or was dead at startup. When the SYSPLL is started, SYSPLLOFF is cleared by hardware. Following startup of the SYSPLL, if the SYSPLL startup monitor determines that the SYSPLL was not started correctly, SYSPLLOFF is set."]
    #[inline(always)]
    pub fn sysplloff(&self) -> SysplloffR {
        SysplloffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - CURHSCLKSEL indicates the current clock source for HSCLK."]
    #[inline(always)]
    pub fn curhsclksel(&self) -> CurhsclkselR {
        CurhsclkselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HSCLKDEAD is set by hardware if the selected source for HSCLK was started but did not start successfully."]
    #[inline(always)]
    pub fn hsclkdead(&self) -> HsclkdeadR {
        HsclkdeadR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HSCLKGOOD is set by hardware if the selected clock source for HSCLK started successfully."]
    #[inline(always)]
    pub fn hsclkgood(&self) -> HsclkgoodR {
        HsclkgoodR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FCLMODE indicates if the SYSOSC frequency correction loop (FCL) is enabled."]
    #[inline(always)]
    pub fn fclmode(&self) -> FclmodeR {
        FclmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FCCDONE indicates when a frequency clock counter capture is complete."]
    #[inline(always)]
    pub fn fccdone(&self) -> FccdoneR {
        FccdoneR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - HFCLKBLKUPD indicates when writes to the HFCLKCLKCFG register are blocked."]
    #[inline(always)]
    pub fn hfclkblkupd(&self) -> HfclkblkupdR {
        HfclkblkupdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SYSPLLBLKUPD indicates when writes to SYSPLLCFG0/1 and SYSPLLPARAM0/1 are blocked."]
    #[inline(always)]
    pub fn syspllblkupd(&self) -> SyspllblkupdR {
        SyspllblkupdR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSTATUS")
            .field("lfoscgood", &self.lfoscgood())
            .field("hfclkgood", &self.hfclkgood())
            .field("hsclkdead", &self.hsclkdead())
            .field("syspllblkupd", &self.syspllblkupd())
            .field("hfclkoff", &self.hfclkoff())
            .field("hfclkblkupd", &self.hfclkblkupd())
            .field("hsclkgood", &self.hsclkgood())
            .field("syspllgood", &self.syspllgood())
            .field("hsclkmux", &self.hsclkmux())
            .field("sysoscfreq", &self.sysoscfreq())
            .field("hsclksoff", &self.hsclksoff())
            .field("fclmode", &self.fclmode())
            .field("curhsclksel", &self.curhsclksel())
            .field("fccdone", &self.fccdone())
            .field("sysplloff", &self.sysplloff())
            .finish()
    }
}
#[doc = "Clock module (CKM) status\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatusSpec;
impl crate::RegisterSpec for ClkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstatus::R`](R) reader structure"]
impl crate::Readable for ClkstatusSpec {}
#[doc = "`reset()` method sets CLKSTATUS to value 0x7000"]
impl crate::Resettable for ClkstatusSpec {
    const RESET_VALUE: u32 = 0x7000;
}
