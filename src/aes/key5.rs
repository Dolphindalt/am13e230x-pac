#[doc = "Register `KEY5` writer"]
pub type W = crate::W<Key5Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Key5Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key5Spec;
impl crate::RegisterSpec for Key5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key5::W`](W) writer structure"]
impl crate::Writable for Key5Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEY5 to value 0"]
impl crate::Resettable for Key5Spec {}
