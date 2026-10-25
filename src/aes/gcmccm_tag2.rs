#[doc = "Register `GCMCCM_TAG2` writer"]
pub type W = crate::W<GcmccmTag2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GcmccmTag2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmccmTag2Spec;
impl crate::RegisterSpec for GcmccmTag2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcmccm_tag2::W`](W) writer structure"]
impl crate::Writable for GcmccmTag2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GCMCCM_TAG2 to value 0"]
impl crate::Resettable for GcmccmTag2Spec {}
