#[doc = "Register `IES` reader"]
pub type R = crate::R<IesSpec>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Enable Status"]
pub type ExtTsCntrOvflR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable Status"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IES")
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .finish()
    }
}
#[doc = "MCAN Subsystem Interrupt Enable Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ies::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IesSpec;
impl crate::RegisterSpec for IesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ies::R`](R) reader structure"]
impl crate::Readable for IesSpec {}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IesSpec {}
