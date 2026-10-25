#[doc = "Register `PECSR` reader"]
pub type R = crate::R<PecsrSpec>;
#[doc = "Field `PECBYTECNT` reader - This is the current PEC Byte Count of the State Machine."]
pub type PecbytecntR = crate::FieldReader<u16>;
#[doc = "Field `PECSTS_CHECK` reader - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
pub type PecstsCheckR = crate::BitReader;
#[doc = "Field `PECSTS_ERROR` reader - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
pub type PecstsErrorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - This is the current PEC Byte Count of the State Machine."]
    #[inline(always)]
    pub fn pecbytecnt(&self) -> PecbytecntR {
        PecbytecntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn pecsts_check(&self) -> PecstsCheckR {
        PecstsCheckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn pecsts_error(&self) -> PecstsErrorR {
        PecstsErrorR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECSR")
            .field("pecbytecnt", &self.pecbytecnt())
            .field("pecsts_check", &self.pecsts_check())
            .field("pecsts_error", &self.pecsts_error())
            .finish()
    }
}
#[doc = "PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PecsrSpec;
impl crate::RegisterSpec for PecsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecsr::R`](R) reader structure"]
impl crate::Readable for PecsrSpec {}
#[doc = "`reset()` method sets PECSR to value 0"]
impl crate::Resettable for PecsrSpec {}
