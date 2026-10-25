#[doc = "Register `ADCINTCYCLE` reader"]
pub type R = crate::R<AdcintcycleSpec>;
#[doc = "Register `ADCINTCYCLE` writer"]
pub type W = crate::W<AdcintcycleSpec>;
#[doc = "Field `DELAY` reader - Delay from ADCSOC fall edge to early Interrupt generation."]
pub type DelayR = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay from ADCSOC fall edge to early Interrupt generation."]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:5 - Delay from ADCSOC fall edge to early Interrupt generation."]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTCYCLE")
            .field("delay", &self.delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Delay from ADCSOC fall edge to early Interrupt generation."]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<'_, AdcintcycleSpec> {
        DelayW::new(self, 0)
    }
}
#[doc = "ADC Early Interrupt Generation Cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintcycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintcycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintcycleSpec;
impl crate::RegisterSpec for AdcintcycleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintcycle::R`](R) reader structure"]
impl crate::Readable for AdcintcycleSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintcycle::W`](W) writer structure"]
impl crate::Writable for AdcintcycleSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTCYCLE to value 0"]
impl crate::Resettable for AdcintcycleSpec {}
