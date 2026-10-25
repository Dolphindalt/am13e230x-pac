#[doc = "Register `ADCPPB3LIMIT` reader"]
pub type R = crate::R<Adcppb3limitSpec>;
#[doc = "Register `ADCPPB3LIMIT` writer"]
pub type W = crate::W<Adcppb3limitSpec>;
#[doc = "Field `LIMIT` reader - Post Processing Block 3 Limit"]
pub type LimitR = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Post Processing Block 3 Limit"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Post Processing Block 3 Limit"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB3LIMIT")
            .field("limit", &self.limit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Post Processing Block 3 Limit"]
    #[inline(always)]
    pub fn limit(&mut self) -> LimitW<'_, Adcppb3limitSpec> {
        LimitW::new(self, 0)
    }
}
#[doc = "ADC PPB3Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb3limitSpec;
impl crate::RegisterSpec for Adcppb3limitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb3limit::R`](R) reader structure"]
impl crate::Readable for Adcppb3limitSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb3limit::W`](W) writer structure"]
impl crate::Writable for Adcppb3limitSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB3LIMIT to value 0"]
impl crate::Resettable for Adcppb3limitSpec {}
