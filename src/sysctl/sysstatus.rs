#[doc = "Register `SYSSTATUS` reader"]
pub type R = crate::R<SysstatusSpec>;
#[doc = "Field `FLASHDED` reader - FLASHDED indicates if a flash ECC double bit error was detected (DED)."]
pub type FlashdedR = crate::BitReader;
#[doc = "Field `FLASHSEC` reader - FLASHSEC indicates if a flash ECC single bit error was detected and corrected (SEC)."]
pub type FlashsecR = crate::BitReader;
#[doc = "Field `PMUIREFGOOD` reader - PMUIREFGOOD is set by hardware when the PMU current reference is ready."]
pub type PmuirefgoodR = crate::BitReader;
#[doc = "Field `MCAN0READY` reader - MCAN0READY indicates when the MCAN0 peripheral is ready."]
pub type Mcan0readyR = crate::BitReader;
#[doc = "Field `EXTRSTPINDIS` reader - EXTRSTPINDIS indicates when user has disabled the use of external reset pin"]
pub type ExtrstpindisR = crate::BitReader;
#[doc = "Field `SWDJCFGDIS` reader - SWDJCFGDIS indicates when user has disabled the use of SWD/JTAG Port"]
pub type SwdjcfgdisR = crate::BitReader;
#[doc = "Field `SHDNIOLOCK` reader - SHDNIOLOCK indicates when IO is locked due to SHUTDOWN"]
pub type ShdniolockR = crate::BitReader;
#[doc = "Field `SRAMBANK0READY` reader - SRAM BANK0 READY STATE"]
pub type Srambank0readyR = crate::BitReader;
#[doc = "Field `SRAMBANK1READY` reader - SRAM BANK1 READY STATE"]
pub type Srambank1readyR = crate::BitReader;
#[doc = "Field `SRAMBANK2READY` reader - SRAM BANK2 READY STATE"]
pub type Srambank2readyR = crate::BitReader;
#[doc = "Field `SRAMBANK3READY` reader - SRAM BANK3 READY STATE"]
pub type Srambank3readyR = crate::BitReader;
#[doc = "Field `REBOOTATTEMPTS` reader - REBOOTATTEMPTS indicates the number of boot attempts taken before the user application starts."]
pub type RebootattemptsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - FLASHDED indicates if a flash ECC double bit error was detected (DED)."]
    #[inline(always)]
    pub fn flashded(&self) -> FlashdedR {
        FlashdedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASHSEC indicates if a flash ECC single bit error was detected and corrected (SEC)."]
    #[inline(always)]
    pub fn flashsec(&self) -> FlashsecR {
        FlashsecR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - PMUIREFGOOD is set by hardware when the PMU current reference is ready."]
    #[inline(always)]
    pub fn pmuirefgood(&self) -> PmuirefgoodR {
        PmuirefgoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MCAN0READY indicates when the MCAN0 peripheral is ready."]
    #[inline(always)]
    pub fn mcan0ready(&self) -> Mcan0readyR {
        Mcan0readyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTRSTPINDIS indicates when user has disabled the use of external reset pin"]
    #[inline(always)]
    pub fn extrstpindis(&self) -> ExtrstpindisR {
        ExtrstpindisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SWDJCFGDIS indicates when user has disabled the use of SWD/JTAG Port"]
    #[inline(always)]
    pub fn swdjcfgdis(&self) -> SwdjcfgdisR {
        SwdjcfgdisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SHDNIOLOCK indicates when IO is locked due to SHUTDOWN"]
    #[inline(always)]
    pub fn shdniolock(&self) -> ShdniolockR {
        ShdniolockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM BANK0 READY STATE"]
    #[inline(always)]
    pub fn srambank0ready(&self) -> Srambank0readyR {
        Srambank0readyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM BANK1 READY STATE"]
    #[inline(always)]
    pub fn srambank1ready(&self) -> Srambank1readyR {
        Srambank1readyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SRAM BANK2 READY STATE"]
    #[inline(always)]
    pub fn srambank2ready(&self) -> Srambank2readyR {
        Srambank2readyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM BANK3 READY STATE"]
    #[inline(always)]
    pub fn srambank3ready(&self) -> Srambank3readyR {
        Srambank3readyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 30:31 - REBOOTATTEMPTS indicates the number of boot attempts taken before the user application starts."]
    #[inline(always)]
    pub fn rebootattempts(&self) -> RebootattemptsR {
        RebootattemptsR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSSTATUS")
            .field("shdniolock", &self.shdniolock())
            .field("extrstpindis", &self.extrstpindis())
            .field("srambank3ready", &self.srambank3ready())
            .field("mcan0ready", &self.mcan0ready())
            .field("flashded", &self.flashded())
            .field("flashsec", &self.flashsec())
            .field("srambank1ready", &self.srambank1ready())
            .field("pmuirefgood", &self.pmuirefgood())
            .field("srambank2ready", &self.srambank2ready())
            .field("rebootattempts", &self.rebootattempts())
            .field("srambank0ready", &self.srambank0ready())
            .field("swdjcfgdis", &self.swdjcfgdis())
            .finish()
    }
}
#[doc = "System status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysstatusSpec;
impl crate::RegisterSpec for SysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysstatus::R`](R) reader structure"]
impl crate::Readable for SysstatusSpec {}
#[doc = "`reset()` method sets SYSSTATUS to value 0"]
impl crate::Resettable for SysstatusSpec {}
