#[doc = "Register `ADCRESULT6` reader"]
pub type R = crate::R<Adcresult6Spec>;
#[doc = "Register `ADCRESULT6` writer"]
pub type W = crate::W<Adcresult6Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult6Spec;
impl crate::RegisterSpec for Adcresult6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult6::R`](R) reader structure"]
impl crate::Readable for Adcresult6Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult6::W`](W) writer structure"]
impl crate::Writable for Adcresult6Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT6 to value 0"]
impl crate::Resettable for Adcresult6Spec {}
