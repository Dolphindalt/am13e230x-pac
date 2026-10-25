#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `LFOSCGOOD` writer - Clear the LFOSCGOOD interrupt."]
pub type LfoscgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSEC` writer - Flash Single Error Correct"]
pub type FlashsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFCLKGOOD` writer - HFCLK GOOD"]
pub type HfclkgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSPLLGOOD` writer - SYSPLL GOOD"]
pub type SyspllgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSCLKGOOD` writer - HSCLK GOOD"]
pub type HsclkgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn lfoscgood(&mut self) -> LfoscgoodW<'_, IclrSpec> {
        LfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn flashsec(&mut self) -> FlashsecW<'_, IclrSpec> {
        FlashsecW::new(self, 2)
    }
    #[doc = "Bit 4 - HFCLK GOOD"]
    #[inline(always)]
    pub fn hfclkgood(&mut self) -> HfclkgoodW<'_, IclrSpec> {
        HfclkgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn syspllgood(&mut self) -> SyspllgoodW<'_, IclrSpec> {
        SyspllgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - HSCLK GOOD"]
    #[inline(always)]
    pub fn hsclkgood(&mut self) -> HsclkgoodW<'_, IclrSpec> {
        HsclkgoodW::new(self, 6)
    }
}
#[doc = "SYSCTL interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
