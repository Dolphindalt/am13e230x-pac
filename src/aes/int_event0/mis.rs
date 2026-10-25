#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `OUTPUTRDY` reader - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type OutputrdyR = crate::BitReader;
#[doc = "Field `INPUTRDY` reader - AES ready"]
pub type InputrdyR = crate::BitReader;
#[doc = "Field `SAVEDCNTXTRDY` reader - AES ready"]
pub type SavedcntxtrdyR = crate::BitReader;
#[doc = "Field `CNTXTRDY` reader - AES ready"]
pub type CntxtrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OutputrdyR {
        OutputrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES ready"]
    #[inline(always)]
    pub fn inputrdy(&self) -> InputrdyR {
        InputrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AES ready"]
    #[inline(always)]
    pub fn savedcntxtrdy(&self) -> SavedcntxtrdyR {
        SavedcntxtrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES ready"]
    #[inline(always)]
    pub fn cntxtrdy(&self) -> CntxtrdyR {
        CntxtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("outputrdy", &self.outputrdy())
            .field("inputrdy", &self.inputrdy())
            .field("savedcntxtrdy", &self.savedcntxtrdy())
            .field("cntxtrdy", &self.cntxtrdy())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
