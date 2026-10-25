#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `LFOSCGOOD` reader - Masked status of the LFOSCGOOD interrupt."]
pub type LfoscgoodR = crate::BitReader;
#[doc = "Field `FLASHSEC` reader - Flash Single Error Correct"]
pub type FlashsecR = crate::BitReader;
#[doc = "Field `HFCLKGOOD` reader - HFCLK GOOD"]
pub type HfclkgoodR = crate::BitReader;
#[doc = "Field `SYSPLLGOOD` reader - SYSPLL GOOD"]
pub type SyspllgoodR = crate::BitReader;
#[doc = "Field `HSCLKGOOD` reader - HSCLK GOOD"]
pub type HsclkgoodR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked status of the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn lfoscgood(&self) -> LfoscgoodR {
        LfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&self) -> FlashsecR {
        FlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&self) -> HfclkgoodR {
        HfclkgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn syspllgood(&self) -> SyspllgoodR {
        SyspllgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&self) -> HsclkgoodR {
        HsclkgoodR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("lfoscgood", &self.lfoscgood())
            .field("hfclkgood", &self.hfclkgood())
            .field("syspllgood", &self.syspllgood())
            .field("hsclkgood", &self.hsclkgood())
            .field("flashsec", &self.flashsec())
            .finish()
    }
}
#[doc = "SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
