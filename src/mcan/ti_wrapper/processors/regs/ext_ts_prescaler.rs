#[doc = "Register `EXT_TS_PRESCALER` reader"]
pub type R = crate::R<ExtTsPrescalerSpec>;
#[doc = "Register `EXT_TS_PRESCALER` writer"]
pub type W = crate::W<ExtTsPrescalerSpec>;
#[doc = "Field `PRESCALER` reader - External Timestamp Prescaler"]
pub type PrescalerR = crate::FieldReader<u32>;
#[doc = "Field `PRESCALER` writer - External Timestamp Prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:23 - External Timestamp Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_TS_PRESCALER")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - External Timestamp Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<'_, ExtTsPrescalerSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem External Timestamp Prescaler 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ts_prescaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ts_prescaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtTsPrescalerSpec;
impl crate::RegisterSpec for ExtTsPrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ts_prescaler::R`](R) reader structure"]
impl crate::Readable for ExtTsPrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_ts_prescaler::W`](W) writer structure"]
impl crate::Writable for ExtTsPrescalerSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXT_TS_PRESCALER to value 0"]
impl crate::Resettable for ExtTsPrescalerSpec {}
