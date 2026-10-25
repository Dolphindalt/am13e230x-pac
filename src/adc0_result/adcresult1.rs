#[doc = "Register `ADCRESULT1` reader"]
pub type R = crate::R<Adcresult1Spec>;
#[doc = "Register `ADCRESULT1` writer"]
pub type W = crate::W<Adcresult1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult1Spec;
impl crate::RegisterSpec for Adcresult1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult1::R`](R) reader structure"]
impl crate::Readable for Adcresult1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult1::W`](W) writer structure"]
impl crate::Writable for Adcresult1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT1 to value 0"]
impl crate::Resettable for Adcresult1Spec {}
