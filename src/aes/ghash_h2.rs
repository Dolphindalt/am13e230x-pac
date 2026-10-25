#[doc = "Register `GHASH_H2` writer"]
pub type W = crate::W<GhashH2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GhashH2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashH2Spec;
impl crate::RegisterSpec for GhashH2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ghash_h2::W`](W) writer structure"]
impl crate::Writable for GhashH2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GHASH_H2 to value 0"]
impl crate::Resettable for GhashH2Spec {}
