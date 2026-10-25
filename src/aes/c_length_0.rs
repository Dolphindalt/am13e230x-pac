#[doc = "Register `C_LENGTH_0` writer"]
pub type W = crate::W<CLength0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLength0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Crypto data length (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c_length_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLength0Spec;
impl crate::RegisterSpec for CLength0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`c_length_0::W`](W) writer structure"]
impl crate::Writable for CLength0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C_LENGTH_0 to value 0"]
impl crate::Resettable for CLength0Spec {}
