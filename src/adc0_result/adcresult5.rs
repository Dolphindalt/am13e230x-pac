#[doc = "Register `ADCRESULT5` reader"]
pub type R = crate::R<Adcresult5Spec>;
#[doc = "Register `ADCRESULT5` writer"]
pub type W = crate::W<Adcresult5Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult5Spec;
impl crate::RegisterSpec for Adcresult5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult5::R`](R) reader structure"]
impl crate::Readable for Adcresult5Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult5::W`](W) writer structure"]
impl crate::Writable for Adcresult5Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT5 to value 0"]
impl crate::Resettable for Adcresult5Spec {}
