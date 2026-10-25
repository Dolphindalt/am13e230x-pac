#[doc = "Register `SEC_ENABLE_CLR` reader"]
pub type R = crate::R<SecEnableClrSpec>;
#[doc = "Register `SEC_ENABLE_CLR` writer"]
pub type W = crate::W<SecEnableClrSpec>;
#[doc = "Field `MSGMEM_ENABLE_CLR` reader - Message RAM SEC Interrupt Pending Enable Clear"]
pub type MsgmemEnableClrR = crate::BitReader;
#[doc = "Field `MSGMEM_ENABLE_CLR` writer - Message RAM SEC Interrupt Pending Enable Clear"]
pub type MsgmemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Clear"]
    #[inline(always)]
    pub fn msgmem_enable_clr(&self) -> MsgmemEnableClrR {
        MsgmemEnableClrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_ENABLE_CLR")
            .field("msgmem_enable_clr", &self.msgmem_enable_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Clear"]
    #[inline(always)]
    pub fn msgmem_enable_clr(&mut self) -> MsgmemEnableClrW<'_, SecEnableClrSpec> {
        MsgmemEnableClrW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableClrSpec;
impl crate::RegisterSpec for SecEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_clr::R`](R) reader structure"]
impl crate::Readable for SecEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_clr::W`](W) writer structure"]
impl crate::Writable for SecEnableClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SEC_ENABLE_CLR to value 0"]
impl crate::Resettable for SecEnableClrSpec {}
