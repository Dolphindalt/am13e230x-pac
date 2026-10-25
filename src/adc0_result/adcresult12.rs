#[doc = "Register `ADCRESULT12` reader"]
pub type R = crate::R<Adcresult12Spec>;
#[doc = "Register `ADCRESULT12` writer"]
pub type W = crate::W<Adcresult12Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 12 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult12Spec;
impl crate::RegisterSpec for Adcresult12Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult12::R`](R) reader structure"]
impl crate::Readable for Adcresult12Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult12::W`](W) writer structure"]
impl crate::Writable for Adcresult12Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT12 to value 0"]
impl crate::Resettable for Adcresult12Spec {}
