#[doc = "Register `ADCRESULT15` reader"]
pub type R = crate::R<Adcresult15Spec>;
#[doc = "Register `ADCRESULT15` writer"]
pub type W = crate::W<Adcresult15Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Result 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcresult15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcresult15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcresult15Spec;
impl crate::RegisterSpec for Adcresult15Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcresult15::R`](R) reader structure"]
impl crate::Readable for Adcresult15Spec {}
#[doc = "`write(|w| ..)` method takes [`adcresult15::W`](W) writer structure"]
impl crate::Writable for Adcresult15Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCRESULT15 to value 0"]
impl crate::Resettable for Adcresult15Spec {}
