#[doc = "Register `ADCPPB1LIMIT` reader"]
pub type R = crate::R<Adcppb1limitSpec>;
#[doc = "Register `ADCPPB1LIMIT` writer"]
pub type W = crate::W<Adcppb1limitSpec>;
#[doc = "Field `LIMIT` reader - Post Processing Block 1 Limit"]
pub type LimitR = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Post Processing Block 1 Limit"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Post Processing Block 1 Limit"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB1LIMIT")
            .field("limit", &self.limit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Post Processing Block 1 Limit"]
    #[inline(always)]
    pub fn limit(&mut self) -> LimitW<'_, Adcppb1limitSpec> {
        LimitW::new(self, 0)
    }
}
#[doc = "ADC PPB1Conversion Count Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb1limitSpec;
impl crate::RegisterSpec for Adcppb1limitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb1limit::R`](R) reader structure"]
impl crate::Readable for Adcppb1limitSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb1limit::W`](W) writer structure"]
impl crate::Writable for Adcppb1limitSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB1LIMIT to value 0"]
impl crate::Resettable for Adcppb1limitSpec {}
