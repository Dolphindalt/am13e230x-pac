#[doc = "Register `GHASH_H0` writer"]
pub type W = crate::W<GhashH0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GhashH0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash_h0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashH0Spec;
impl crate::RegisterSpec for GhashH0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ghash_h0::W`](W) writer structure"]
impl crate::Writable for GhashH0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GHASH_H0 to value 0"]
impl crate::Resettable for GhashH0Spec {}
