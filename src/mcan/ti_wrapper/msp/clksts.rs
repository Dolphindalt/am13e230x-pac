#[doc = "Register `CLKSTS` reader"]
pub type R = crate::R<ClkstsSpec>;
#[doc = "Field `CLKSTOP_ACKSTS` reader - Clock stop acknowledge status from MCAN IP"]
pub type ClkstopAckstsR = crate::BitReader;
#[doc = "Field `STOPREQ_HW_OVR` reader - MCANSS clock stop HW override status bit"]
pub type StopreqHwOvrR = crate::BitReader;
#[doc = "Field `CCLKDONE` reader - MCAN controller clock request status from GPRCM"]
pub type CclkdoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clock stop acknowledge status from MCAN IP"]
    #[inline(always)]
    pub fn clkstop_acksts(&self) -> ClkstopAckstsR {
        ClkstopAckstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MCANSS clock stop HW override status bit"]
    #[inline(always)]
    pub fn stopreq_hw_ovr(&self) -> StopreqHwOvrR {
        StopreqHwOvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MCAN controller clock request status from GPRCM"]
    #[inline(always)]
    pub fn cclkdone(&self) -> CclkdoneR {
        CclkdoneR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSTS")
            .field("clkstop_acksts", &self.clkstop_acksts())
            .field("stopreq_hw_ovr", &self.stopreq_hw_ovr())
            .field("cclkdone", &self.cclkdone())
            .finish()
    }
}
#[doc = "MCANSS clock stop status register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstsSpec;
impl crate::RegisterSpec for ClkstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksts::R`](R) reader structure"]
impl crate::Readable for ClkstsSpec {}
#[doc = "`reset()` method sets CLKSTS to value 0"]
impl crate::Resettable for ClkstsSpec {}
