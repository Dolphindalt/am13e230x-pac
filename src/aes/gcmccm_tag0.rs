#[doc = "Register `GCMCCM_TAG0` writer"]
pub type W = crate::W<GcmccmTag0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GcmccmTag0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmccm_tag0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmccmTag0Spec;
impl crate::RegisterSpec for GcmccmTag0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gcmccm_tag0::W`](W) writer structure"]
impl crate::Writable for GcmccmTag0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GCMCCM_TAG0 to value 0"]
impl crate::Resettable for GcmccmTag0Spec {}
