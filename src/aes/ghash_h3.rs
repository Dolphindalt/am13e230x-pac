#[doc = "Register `GHASH_H3` writer"]
pub type W = crate::W<GhashH3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GhashH3Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashH3Spec;
impl crate::RegisterSpec for GhashH3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ghash_h3::W`](W) writer structure"]
impl crate::Writable for GhashH3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GHASH_H3 to value 0"]
impl crate::Resettable for GhashH3Spec {}
