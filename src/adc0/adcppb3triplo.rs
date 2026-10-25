#[doc = "Register `ADCPPB3TRIPLO` reader"]
pub type R = crate::R<Adcppb3triploSpec>;
#[doc = "Register `ADCPPB3TRIPLO` writer"]
pub type W = crate::W<Adcppb3triploSpec>;
#[doc = "Field `LIMITLO` reader - ADC Post Processing Block 3 Trip Low Limit"]
pub type LimitloR = crate::FieldReader<u16>;
#[doc = "Field `LIMITLO` writer - ADC Post Processing Block 3 Trip Low Limit"]
pub type LimitloW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC Post Processing Block 3 Trip Low Limit"]
    #[inline(always)]
    pub fn limitlo(&self) -> LimitloR {
        LimitloR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB3TRIPLO")
            .field("limitlo", &self.limitlo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Post Processing Block 3 Trip Low Limit"]
    #[inline(always)]
    pub fn limitlo(&mut self) -> LimitloW<'_, Adcppb3triploSpec> {
        LimitloW::new(self, 0)
    }
}
#[doc = "ADC PPB3 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3triplo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3triplo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb3triploSpec;
impl crate::RegisterSpec for Adcppb3triploSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb3triplo::R`](R) reader structure"]
impl crate::Readable for Adcppb3triploSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb3triplo::W`](W) writer structure"]
impl crate::Writable for Adcppb3triploSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB3TRIPLO to value 0"]
impl crate::Resettable for Adcppb3triploSpec {}
