#[doc = "Register `ADCRESULT9` reader"]
pub type R = crate::R<Adcresult9Spec>;
#[doc = "Register `ADCRESULT9` writer"]
pub type W = crate::W<Adcresult9Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 9 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult9Spec;
impl crate::RegisterSpec for Adcresult9Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult9::R`](R) reader structure"]
impl crate::Readable for Adcresult9Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult9::W`](W) writer structure"]
impl crate::Writable for Adcresult9Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT9 to value 0"]
impl crate::Resettable for Adcresult9Spec {}
