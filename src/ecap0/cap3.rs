#[doc = "Register `CAP3` reader"]
pub type R = crate::R<Cap3Spec>;
#[doc = "Register `CAP3` writer"]
pub type W = crate::W<Cap3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Capture 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap3Spec;
impl crate::RegisterSpec for Cap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap3::R`](R) reader structure"]
impl crate::Readable for Cap3Spec {}
#[doc = "`write(|w| ..)` method takes [`cap3::W`](W) writer structure"]
impl crate::Writable for Cap3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAP3 to value 0"]
impl crate::Resettable for Cap3Spec {}
