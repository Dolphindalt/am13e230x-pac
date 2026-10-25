#[doc = "Register `AAD_LENGTH` writer"]
pub type W = crate::W<AadLengthSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AadLengthSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "AAD Data Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_length::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AadLengthSpec;
impl crate::RegisterSpec for AadLengthSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aad_length::W`](W) writer structure"]
impl crate::Writable for AadLengthSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AAD_LENGTH to value 0"]
impl crate::Resettable for AadLengthSpec {}
