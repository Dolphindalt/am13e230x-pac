#[doc = "Register `KEY7` writer"]
pub type W = crate::W<Key7Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Key7Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key7Spec;
impl crate::RegisterSpec for Key7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key7::W`](W) writer structure"]
impl crate::Writable for Key7Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY7 to value 0"]
impl crate::Resettable for Key7Spec {}
