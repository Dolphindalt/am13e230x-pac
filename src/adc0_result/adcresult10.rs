#[doc = "Register `ADCRESULT10` reader"]
pub type R = crate::R<Adcresult10Spec>;
#[doc = "Register `ADCRESULT10` writer"]
pub type W = crate::W<Adcresult10Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 10 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult10Spec;
impl crate::RegisterSpec for Adcresult10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult10::R`](R) reader structure"]
impl crate::Readable for Adcresult10Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult10::W`](W) writer structure"]
impl crate::Writable for Adcresult10Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT10 to value 0"]
impl crate::Resettable for Adcresult10Spec {}
