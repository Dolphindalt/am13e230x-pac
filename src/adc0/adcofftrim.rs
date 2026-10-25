#[doc = "Register `ADCOFFTRIM` reader"]
pub type R = crate::R<AdcofftrimSpec>;
#[doc = "Register `ADCOFFTRIM` writer"]
pub type W = crate::W<AdcofftrimSpec>;
#[doc = "Field `OFFTRIM` reader - ADC Offset Trim"]
pub type OfftrimR = crate::FieldReader;
#[doc = "Field `OFFTRIM` writer - ADC Offset Trim"]
pub type OfftrimW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - ADC Offset Trim"]
    #[inline(always)]
    pub fn offtrim(&self) -> OfftrimR {
        OfftrimR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCOFFTRIM")
            .field("offtrim", &self.offtrim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC Offset Trim"]
    #[inline(always)]
    pub fn offtrim(&mut self) -> OfftrimW<'_, AdcofftrimSpec> {
        OfftrimW::new(self, 0)
    }
}
#[doc = "ADC Offset Trim Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcofftrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcofftrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcofftrimSpec;
impl crate::RegisterSpec for AdcofftrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcofftrim::R`](R) reader structure"]
impl crate::Readable for AdcofftrimSpec {}
#[doc = "`write(|w| ..)` method takes [`adcofftrim::W`](W) writer structure"]
impl crate::Writable for AdcofftrimSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCOFFTRIM to value 0"]
impl crate::Resettable for AdcofftrimSpec {}
