#[doc = "Register `TEMP_SENSE0` reader"]
pub type R = crate::R<TempSense0Spec>;
#[doc = "Field `DATA` reader - DATA"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP_SENSE0")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "TEMP_SENSE0\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sense0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSense0Spec;
impl crate::RegisterSpec for TempSense0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp_sense0::R`](R) reader structure"]
impl crate::Readable for TempSense0Spec {}
#[doc = "`reset()` method sets TEMP_SENSE0 to value 0"]
impl crate::Resettable for TempSense0Spec {}
