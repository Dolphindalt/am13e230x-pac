#[doc = "Register `TI_WRAPPER_PROCESSORS_ECC_REGS_STAT` reader"]
pub type R = crate::R<TiWrapperProcessorsEccRegsStatSpec>;
#[doc = "Field `NUM_RAMS` reader - Number of RAMs"]
pub type NumRamsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Number of RAMs"]
    #[inline(always)]
    pub fn num_rams(&self) -> NumRamsR {
        NumRamsR::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI_WRAPPER_PROCESSORS_ECC_REGS_STAT")
            .field("num_rams", &self.num_rams())
            .finish()
    }
}
#[doc = "MCAN Error Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_ecc_regs_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiWrapperProcessorsEccRegsStatSpec;
impl crate::RegisterSpec for TiWrapperProcessorsEccRegsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti_wrapper_processors_ecc_regs_stat::R`](R) reader structure"]
impl crate::Readable for TiWrapperProcessorsEccRegsStatSpec {}
#[doc = "`reset()` method sets TI_WRAPPER_PROCESSORS_ECC_REGS_STAT to value 0x02"]
impl crate::Resettable for TiWrapperProcessorsEccRegsStatSpec {
    const RESET_VALUE: u32 = 0x02;
}
