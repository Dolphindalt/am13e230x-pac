#[doc = "Register `ADCRESULT13` reader"]
pub type R = crate::R<Adcresult13Spec>;
#[doc = "Register `ADCRESULT13` writer"]
pub type W = crate::W<Adcresult13Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult13Spec;
impl crate::RegisterSpec for Adcresult13Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult13::R`](R) reader structure"]
impl crate::Readable for Adcresult13Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult13::W`](W) writer structure"]
impl crate::Writable for Adcresult13Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT13 to value 0"]
impl crate::Resettable for Adcresult13Spec {}
