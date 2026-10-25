#[doc = "Register `ADCCTL2` reader"]
pub type R = crate::R<Adcctl2Spec>;
#[doc = "Register `ADCCTL2` writer"]
pub type W = crate::W<Adcctl2Spec>;
#[doc = "Field `PRESCALE` reader - ADC Clock Prescaler"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - ADC Clock Prescaler"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - ADC Clock Prescaler"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCTL2")
            .field("prescale", &self.prescale())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Clock Prescaler"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, Adcctl2Spec> {
        PrescaleW::new(self, 0)
    }
}
#[doc = "ADC Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl2Spec;
impl crate::RegisterSpec for Adcctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctl2::R`](R) reader structure"]
impl crate::Readable for Adcctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl2::W`](W) writer structure"]
impl crate::Writable for Adcctl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for Adcctl2Spec {}
