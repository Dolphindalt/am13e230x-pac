#[doc = "Register `CAP1` reader"]
pub type R = crate::R<Cap1Spec>;
#[doc = "Register `CAP1` writer"]
pub type W = crate::W<Cap1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap1Spec;
impl crate::RegisterSpec for Cap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap1::R`](R) reader structure"]
impl crate::Readable for Cap1Spec {}
#[doc = "`write(|w| ..)` method takes [`cap1::W`](W) writer structure"]
impl crate::Writable for Cap1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAP1 to value 0"]
impl crate::Resettable for Cap1Spec {}
