#[doc = "Register `IV2` reader"]
pub type R = crate::R<Iv2Spec>;
#[doc = "Register `IV2` writer"]
pub type W = crate::W<Iv2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IV\n\nYou can [`read`](crate::Reg::read) this register and get [`iv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv2Spec;
impl crate::RegisterSpec for Iv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv2::R`](R) reader structure"]
impl crate::Readable for Iv2Spec {}
#[doc = "`write(|w| ..)` method takes [`iv2::W`](W) writer structure"]
impl crate::Writable for Iv2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IV2 to value 0"]
impl crate::Resettable for Iv2Spec {}
