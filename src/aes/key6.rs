#[doc = "Register `KEY6` writer"]
pub type W = crate::W<Key6Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Key6Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key6Spec;
impl crate::RegisterSpec for Key6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key6::W`](W) writer structure"]
impl crate::Writable for Key6Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY6 to value 0"]
impl crate::Resettable for Key6Spec {}
