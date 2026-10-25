#[doc = "Register `IV1` reader"]
pub type R = crate::R<Iv1Spec>;
#[doc = "Register `IV1` writer"]
pub type W = crate::W<Iv1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv1Spec;
impl crate::RegisterSpec for Iv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1::R`](R) reader structure"]
impl crate::Readable for Iv1Spec {}
#[doc = "`write(|w| ..)` method takes [`iv1::W`](W) writer structure"]
impl crate::Writable for Iv1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IV1 to value 0"]
impl crate::Resettable for Iv1Spec {}
