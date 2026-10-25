#[doc = "Register `TI_WRAPPER_PROCESSORS_REGS_STAT` reader"]
pub type R = crate::R<TiWrapperProcessorsRegsStatSpec>;
#[doc = "Field `RESET` reader - Soft Reset Status"]
pub type ResetR = crate::BitReader;
#[doc = "Field `MEM_INIT_DONE` reader - Memory Initialization Done"]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `ENABLE_FDOE` reader - Flexible Datarate Operation Enable"]
pub type EnableFdoeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Soft Reset Status"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Memory Initialization Done"]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flexible Datarate Operation Enable"]
    #[inline(always)]
    pub fn enable_fdoe(&self) -> EnableFdoeR {
        EnableFdoeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI_WRAPPER_PROCESSORS_REGS_STAT")
            .field("reset", &self.reset())
            .field("mem_init_done", &self.mem_init_done())
            .field("enable_fdoe", &self.enable_fdoe())
            .finish()
    }
}
#[doc = "MCAN Subsystem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_regs_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiWrapperProcessorsRegsStatSpec;
impl crate::RegisterSpec for TiWrapperProcessorsRegsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti_wrapper_processors_regs_stat::R`](R) reader structure"]
impl crate::Readable for TiWrapperProcessorsRegsStatSpec {}
#[doc = "`reset()` method sets TI_WRAPPER_PROCESSORS_REGS_STAT to value 0"]
impl crate::Resettable for TiWrapperProcessorsRegsStatSpec {}
