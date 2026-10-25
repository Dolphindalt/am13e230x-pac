#[doc = "Register `ICS` reader"]
pub type R = crate::R<IcsSpec>;
#[doc = "Register `ICS` writer"]
pub type W = crate::W<IcsSpec>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Status Clear"]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Status Clear"]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status Clear"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICS")
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status Clear"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<'_, IcsSpec> {
        ExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ics::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ics::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsSpec;
impl crate::RegisterSpec for IcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ics::R`](R) reader structure"]
impl crate::Readable for IcsSpec {}
#[doc = "`write(|w| ..)` method takes [`ics::W`](W) writer structure"]
impl crate::Writable for IcsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICS to value 0"]
impl crate::Resettable for IcsSpec {}
