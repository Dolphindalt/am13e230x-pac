#[doc = "Register `CAP4` reader"]
pub type R = crate::R<Cap4Spec>;
#[doc = "Register `CAP4` writer"]
pub type W = crate::W<Cap4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Capture 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap4Spec;
impl crate::RegisterSpec for Cap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap4::R`](R) reader structure"]
impl crate::Readable for Cap4Spec {}
#[doc = "`write(|w| ..)` method takes [`cap4::W`](W) writer structure"]
impl crate::Writable for Cap4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAP4 to value 0"]
impl crate::Resettable for Cap4Spec {}
