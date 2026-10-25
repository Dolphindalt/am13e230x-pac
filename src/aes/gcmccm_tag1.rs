#[doc = "Register `GCMCCM_TAG1` writer"]
pub type W = crate::W<GcmccmTag1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GcmccmTag1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmccmTag1Spec;
impl crate::RegisterSpec for GcmccmTag1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcmccm_tag1::W`](W) writer structure"]
impl crate::Writable for GcmccmTag1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GCMCCM_TAG1 to value 0"]
impl crate::Resettable for GcmccmTag1Spec {}
