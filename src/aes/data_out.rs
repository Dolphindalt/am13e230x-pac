#[doc = "Register `DATA_OUT` reader"]
pub type R = crate::R<DataOutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Data out alias register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataOutSpec;
impl crate::RegisterSpec for DataOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_out::R`](R) reader structure"]
impl crate::Readable for DataOutSpec {}
#[doc = "`reset()` method sets DATA_OUT to value 0"]
impl crate::Resettable for DataOutSpec {}
