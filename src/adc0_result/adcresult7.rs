#[doc = "Register `ADCRESULT7` reader"]
pub type R = crate::R<Adcresult7Spec>;
#[doc = "Register `ADCRESULT7` writer"]
pub type W = crate::W<Adcresult7Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult7Spec;
impl crate::RegisterSpec for Adcresult7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult7::R`](R) reader structure"]
impl crate::Readable for Adcresult7Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult7::W`](W) writer structure"]
impl crate::Writable for Adcresult7Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT7 to value 0"]
impl crate::Resettable for Adcresult7Spec {}
