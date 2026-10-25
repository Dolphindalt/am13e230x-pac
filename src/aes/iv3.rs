#[doc = "Register `IV3` reader"]
pub type R = crate::R<Iv3Spec>;
#[doc = "Register `IV3` writer"]
pub type W = crate::W<Iv3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv3Spec;
impl crate::RegisterSpec for Iv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv3::R`](R) reader structure"]
impl crate::Readable for Iv3Spec {}
#[doc = "`write(|w| ..)` method takes [`iv3::W`](W) writer structure"]
impl crate::Writable for Iv3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IV3 to value 0"]
impl crate::Resettable for Iv3Spec {}
