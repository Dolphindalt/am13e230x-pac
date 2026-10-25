#[doc = "Register `ADCRESULT11` reader"]
pub type R = crate::R<Adcresult11Spec>;
#[doc = "Register `ADCRESULT11` writer"]
pub type W = crate::W<Adcresult11Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult11Spec;
impl crate::RegisterSpec for Adcresult11Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult11::R`](R) reader structure"]
impl crate::Readable for Adcresult11Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult11::W`](W) writer structure"]
impl crate::Writable for Adcresult11Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT11 to value 0"]
impl crate::Resettable for Adcresult11Spec {}
