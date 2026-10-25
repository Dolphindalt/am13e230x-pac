#[doc = "Register `TRACEID` reader"]
pub type R = crate::R<TraceidSpec>;
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
        f.debug_struct("TRACEID")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "Defined by TI, during ATE, based on wafer\n\nYou can [`read`](crate::Reg::read) this register and get [`traceid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceidSpec;
impl crate::RegisterSpec for TraceidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceid::R`](R) reader structure"]
impl crate::Readable for TraceidSpec {}
#[doc = "`reset()` method sets TRACEID to value 0"]
impl crate::Resettable for TraceidSpec {}
