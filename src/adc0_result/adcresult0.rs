#[doc = "Register `ADCRESULT0` reader"]
pub type R = crate::R<Adcresult0Spec>;
#[doc = "Register `ADCRESULT0` writer"]
pub type W = crate::W<Adcresult0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult0Spec;
impl crate::RegisterSpec for Adcresult0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult0::R`](R) reader structure"]
impl crate::Readable for Adcresult0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult0::W`](W) writer structure"]
impl crate::Writable for Adcresult0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT0 to value 0"]
impl crate::Resettable for Adcresult0Spec {}
