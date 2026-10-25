#[doc = "Register `ADCRESULT3` reader"]
pub type R = crate::R<Adcresult3Spec>;
#[doc = "Register `ADCRESULT3` writer"]
pub type W = crate::W<Adcresult3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult3Spec;
impl crate::RegisterSpec for Adcresult3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult3::R`](R) reader structure"]
impl crate::Readable for Adcresult3Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult3::W`](W) writer structure"]
impl crate::Writable for Adcresult3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT3 to value 0"]
impl crate::Resettable for Adcresult3Spec {}
