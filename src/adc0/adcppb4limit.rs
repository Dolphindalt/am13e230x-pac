#[doc = "Register `ADCPPB4LIMIT` reader"]
pub type R = crate::R<Adcppb4limitSpec>;
#[doc = "Register `ADCPPB4LIMIT` writer"]
pub type W = crate::W<Adcppb4limitSpec>;
#[doc = "Field `LIMIT` reader - Post Processing Block 4 Limit"]
pub type LimitR = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Post Processing Block 4 Limit"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Post Processing Block 4 Limit"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB4LIMIT")
            .field("limit", &self.limit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Post Processing Block 4 Limit"]
    #[inline(always)]
    pub fn limit(&mut self) -> LimitW<'_, Adcppb4limitSpec> {
        LimitW::new(self, 0)
    }
}
#[doc = "ADC PPB4Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb4limitSpec;
impl crate::RegisterSpec for Adcppb4limitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb4limit::R`](R) reader structure"]
impl crate::Readable for Adcppb4limitSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb4limit::W`](W) writer structure"]
impl crate::Writable for Adcppb4limitSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB4LIMIT to value 0"]
impl crate::Resettable for Adcppb4limitSpec {}
