#[doc = "Register `ADCRESULT14` reader"]
pub type R = crate::R<Adcresult14Spec>;
#[doc = "Register `ADCRESULT14` writer"]
pub type W = crate::W<Adcresult14Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult14Spec;
impl crate::RegisterSpec for Adcresult14Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult14::R`](R) reader structure"]
impl crate::Readable for Adcresult14Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult14::W`](W) writer structure"]
impl crate::Writable for Adcresult14Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT14 to value 0"]
impl crate::Resettable for Adcresult14Spec {}
