#[doc = "Register `BOOTCRC` reader"]
pub type R = crate::R<BootcrcSpec>;
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
        f.debug_struct("BOOTCRC")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "BOOTCRC\n\nYou can [`read`](crate::Reg::read) this register and get [`bootcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootcrcSpec;
impl crate::RegisterSpec for BootcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootcrc::R`](R) reader structure"]
impl crate::Readable for BootcrcSpec {}
#[doc = "`reset()` method sets BOOTCRC to value 0"]
impl crate::Resettable for BootcrcSpec {}
