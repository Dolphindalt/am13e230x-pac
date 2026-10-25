#[doc = "Register `DED_STATUS` reader"]
pub type R = crate::R<DedStatusSpec>;
#[doc = "Register `DED_STATUS` writer"]
pub type W = crate::W<DedStatusSpec>;
#[doc = "Field `MSGMEM_PEND` reader - Message RAM DED Interrupt Pending"]
pub type MsgmemPendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending"]
    #[inline(always)]
    pub fn msgmem_pend(&self) -> MsgmemPendR {
        MsgmemPendR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DED_STATUS")
            .field("msgmem_pend", &self.msgmem_pend())
            .finish()
    }
}
impl W {}
#[doc = "MCAN Double Error Detected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedStatusSpec;
impl crate::RegisterSpec for DedStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_status::R`](R) reader structure"]
impl crate::Readable for DedStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ded_status::W`](W) writer structure"]
impl crate::Writable for DedStatusSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DED_STATUS to value 0"]
impl crate::Resettable for DedStatusSpec {}
