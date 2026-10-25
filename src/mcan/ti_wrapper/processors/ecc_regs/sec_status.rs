#[doc = "Register `SEC_STATUS` reader"]
pub type R = crate::R<SecStatusSpec>;
#[doc = "Register `SEC_STATUS` writer"]
pub type W = crate::W<SecStatusSpec>;
#[doc = "Field `MSGMEM_PEND` reader - Message RAM SEC Interrupt Pending"]
pub type MsgmemPendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending"]
    #[inline(always)]
    pub fn msgmem_pend(&self) -> MsgmemPendR {
        MsgmemPendR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_STATUS")
            .field("msgmem_pend", &self.msgmem_pend())
            .finish()
    }
}
impl W {}
#[doc = "MCAN Single Error Corrected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecStatusSpec;
impl crate::RegisterSpec for SecStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_status::R`](R) reader structure"]
impl crate::Readable for SecStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_status::W`](W) writer structure"]
impl crate::Writable for SecStatusSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SEC_STATUS to value 0"]
impl crate::Resettable for SecStatusSpec {}
