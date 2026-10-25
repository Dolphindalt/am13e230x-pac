#[doc = "Register `TI_WRAPPER_PROCESSORS_REGS_IE` reader"]
pub type R = crate::R<TiWrapperProcessorsRegsIeSpec>;
#[doc = "Register `TI_WRAPPER_PROCESSORS_REGS_IE` writer"]
pub type W = crate::W<TiWrapperProcessorsRegsIeSpec>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Enable"]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Enable"]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI_WRAPPER_PROCESSORS_REGS_IE")
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<'_, TiWrapperProcessorsRegsIeSpec> {
        ExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_processors_regs_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_processors_regs_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiWrapperProcessorsRegsIeSpec;
impl crate::RegisterSpec for TiWrapperProcessorsRegsIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti_wrapper_processors_regs_ie::R`](R) reader structure"]
impl crate::Readable for TiWrapperProcessorsRegsIeSpec {}
#[doc = "`write(|w| ..)` method takes [`ti_wrapper_processors_regs_ie::W`](W) writer structure"]
impl crate::Writable for TiWrapperProcessorsRegsIeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TI_WRAPPER_PROCESSORS_REGS_IE to value 0"]
impl crate::Resettable for TiWrapperProcessorsRegsIeSpec {}
