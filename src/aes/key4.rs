#[doc = "Register `KEY4` writer"]
pub type W = crate::W<Key4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Key4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key4Spec;
impl crate::RegisterSpec for Key4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key4::W`](W) writer structure"]
impl crate::Writable for Key4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY4 to value 0"]
impl crate::Resettable for Key4Spec {}
