#[doc = "Register `IV0` reader"]
pub type R = crate::R<Iv0Spec>;
#[doc = "Register `IV0` writer"]
pub type W = crate::W<Iv0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IV (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`iv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv0Spec;
impl crate::RegisterSpec for Iv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv0::R`](R) reader structure"]
impl crate::Readable for Iv0Spec {}
#[doc = "`write(|w| ..)` method takes [`iv0::W`](W) writer structure"]
impl crate::Writable for Iv0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IV0 to value 0"]
impl crate::Resettable for Iv0Spec {}
