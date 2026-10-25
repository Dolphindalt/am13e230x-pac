#[doc = "Register `FCC` reader"]
pub type R = crate::R<FccSpec>;
#[doc = "Field `DATA` reader - Frequency clock counter (FCC) count value."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency clock counter (FCC) count value."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCC").field("data", &self.data()).finish()
    }
}
#[doc = "Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`fcc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FccSpec;
impl crate::RegisterSpec for FccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcc::R`](R) reader structure"]
impl crate::Readable for FccSpec {}
#[doc = "`reset()` method sets FCC to value 0"]
impl crate::Resettable for FccSpec {}
