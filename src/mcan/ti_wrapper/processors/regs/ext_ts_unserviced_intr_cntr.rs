#[doc = "Register `EXT_TS_UNSERVICED_INTR_CNTR` reader"]
pub type R = crate::R<ExtTsUnservicedIntrCntrSpec>;
#[doc = "Field `EXT_TS_INTR_CNTR` reader - External Timestamp Counter Unserviced Rollover Interrupts"]
pub type ExtTsIntrCntrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - External Timestamp Counter Unserviced Rollover Interrupts"]
    #[inline(always)]
    pub fn ext_ts_intr_cntr(&self) -> ExtTsIntrCntrR {
        ExtTsIntrCntrR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_TS_UNSERVICED_INTR_CNTR")
            .field("ext_ts_intr_cntr", &self.ext_ts_intr_cntr())
            .finish()
    }
}
#[doc = "MCAN Subsystem External Timestamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ts_unserviced_intr_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtTsUnservicedIntrCntrSpec;
impl crate::RegisterSpec for ExtTsUnservicedIntrCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ts_unserviced_intr_cntr::R`](R) reader structure"]
impl crate::Readable for ExtTsUnservicedIntrCntrSpec {}
#[doc = "`reset()` method sets EXT_TS_UNSERVICED_INTR_CNTR to value 0"]
impl crate::Resettable for ExtTsUnservicedIntrCntrSpec {}
