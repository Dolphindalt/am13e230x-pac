#[doc = "Register `ADCRESULT8` reader"]
pub type R = crate::R<Adcresult8Spec>;
#[doc = "Register `ADCRESULT8` writer"]
pub type W = crate::W<Adcresult8Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult8Spec;
impl crate::RegisterSpec for Adcresult8Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult8::R`](R) reader structure"]
impl crate::Readable for Adcresult8Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult8::W`](W) writer structure"]
impl crate::Writable for Adcresult8Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT8 to value 0"]
impl crate::Resettable for Adcresult8Spec {}
