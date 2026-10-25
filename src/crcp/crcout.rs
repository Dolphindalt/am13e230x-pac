#[doc = "Register `CRCOUT` reader"]
pub type R = crate::R<CrcoutSpec>;
#[doc = "Field `RESULT` reader - Result"]
pub type ResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Result"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCOUT")
            .field("result", &self.result())
            .finish()
    }
}
#[doc = "CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcoutSpec;
impl crate::RegisterSpec for CrcoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcout::R`](R) reader structure"]
impl crate::Readable for CrcoutSpec {}
#[doc = "`reset()` method sets CRCOUT to value 0"]
impl crate::Resettable for CrcoutSpec {}
