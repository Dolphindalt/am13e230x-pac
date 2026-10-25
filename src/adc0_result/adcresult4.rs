#[doc = "Register `ADCRESULT4` reader"]
pub type R = crate::R<Adcresult4Spec>;
#[doc = "Register `ADCRESULT4` writer"]
pub type W = crate::W<Adcresult4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult4Spec;
impl crate::RegisterSpec for Adcresult4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult4::R`](R) reader structure"]
impl crate::Readable for Adcresult4Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult4::W`](W) writer structure"]
impl crate::Writable for Adcresult4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT4 to value 0"]
impl crate::Resettable for Adcresult4Spec {}
