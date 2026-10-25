#[doc = "Register `CAP2` reader"]
pub type R = crate::R<Cap2Spec>;
#[doc = "Register `CAP2` writer"]
pub type W = crate::W<Cap2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Capture 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap2Spec;
impl crate::RegisterSpec for Cap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap2::R`](R) reader structure"]
impl crate::Readable for Cap2Spec {}
#[doc = "`write(|w| ..)` method takes [`cap2::W`](W) writer structure"]
impl crate::Writable for Cap2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAP2 to value 0"]
impl crate::Resettable for Cap2Spec {}
