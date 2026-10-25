#[doc = "Register `ADCRESULT2` reader"]
pub type R = crate::R<Adcresult2Spec>;
#[doc = "Register `ADCRESULT2` writer"]
pub type W = crate::W<Adcresult2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult2Spec;
impl crate::RegisterSpec for Adcresult2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult2::R`](R) reader structure"]
impl crate::Readable for Adcresult2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult2::W`](W) writer structure"]
impl crate::Writable for Adcresult2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT2 to value 0"]
impl crate::Resettable for Adcresult2Spec {}
