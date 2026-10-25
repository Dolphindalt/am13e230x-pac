#[doc = "Register `DATA_IN` writer"]
pub type W = crate::W<DataInSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DataInSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data in alias register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataInSpec;
impl crate::RegisterSpec for DataInSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_in::W`](W) writer structure"]
impl crate::Writable for DataInSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DATA_IN to value 0"]
impl crate::Resettable for DataInSpec {}
