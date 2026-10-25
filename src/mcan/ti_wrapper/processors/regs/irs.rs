#[doc = "Register `IRS` reader"]
pub type R = crate::R<IrsSpec>;
#[doc = "Register `IRS` writer"]
pub type W = crate::W<IrsSpec>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Status"]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Status"]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRS")
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<'_, IrsSpec> {
        ExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Raw Satus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrsSpec;
impl crate::RegisterSpec for IrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irs::R`](R) reader structure"]
impl crate::Readable for IrsSpec {}
#[doc = "`write(|w| ..)` method takes [`irs::W`](W) writer structure"]
impl crate::Writable for IrsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IRS to value 0"]
impl crate::Resettable for IrsSpec {}
