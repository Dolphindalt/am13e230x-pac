#[doc = "Register `ADCPPB4TRIPLO` reader"]
pub type R = crate::R<Adcppb4triploSpec>;
#[doc = "Register `ADCPPB4TRIPLO` writer"]
pub type W = crate::W<Adcppb4triploSpec>;
#[doc = "Field `LIMITLO` reader - ADC Post Processing Block 4 Trip Low Limit"]
pub type LimitloR = crate::FieldReader<u16>;
#[doc = "Field `LIMITLO` writer - ADC Post Processing Block 4 Trip Low Limit"]
pub type LimitloW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC Post Processing Block 4 Trip Low Limit"]
    #[inline(always)]
    pub fn limitlo(&self) -> LimitloR {
        LimitloR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB4TRIPLO")
            .field("limitlo", &self.limitlo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Post Processing Block 4 Trip Low Limit"]
    #[inline(always)]
    pub fn limitlo(&mut self) -> LimitloW<'_, Adcppb4triploSpec> {
        LimitloW::new(self, 0)
    }
}
#[doc = "ADC PPB4 Trip Low/Trigger Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4triplo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4triplo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb4triploSpec;
impl crate::RegisterSpec for Adcppb4triploSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb4triplo::R`](R) reader structure"]
impl crate::Readable for Adcppb4triploSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb4triplo::W`](W) writer structure"]
impl crate::Writable for Adcppb4triploSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB4TRIPLO to value 0"]
impl crate::Resettable for Adcppb4triploSpec {}
