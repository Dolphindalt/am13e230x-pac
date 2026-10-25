#[doc = "Register `FORCE_IN_AV` writer"]
pub type W = crate::W<ForceInAvSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ForceInAvSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data control register for input data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_in_av::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceInAvSpec;
impl crate::RegisterSpec for ForceInAvSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`force_in_av::W`](W) writer structure"]
impl crate::Writable for ForceInAvSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FORCE_IN_AV to value 0"]
impl crate::Resettable for ForceInAvSpec {}
