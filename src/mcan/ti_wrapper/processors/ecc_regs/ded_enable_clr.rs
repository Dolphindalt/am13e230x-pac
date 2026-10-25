#[doc = "Register `DED_ENABLE_CLR` reader"]
pub type R = crate::R<DedEnableClrSpec>;
#[doc = "Register `DED_ENABLE_CLR` writer"]
pub type W = crate::W<DedEnableClrSpec>;
#[doc = "Field `MSGMEM_ENABLE_CLR` reader - Message RAM DED Interrupt Pending Enable Clear"]
pub type MsgmemEnableClrR = crate::BitReader;
#[doc = "Field `MSGMEM_ENABLE_CLR` writer - Message RAM DED Interrupt Pending Enable Clear"]
pub type MsgmemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Clear"]
    #[inline(always)]
    pub fn msgmem_enable_clr(&self) -> MsgmemEnableClrR {
        MsgmemEnableClrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DED_ENABLE_CLR")
            .field("msgmem_enable_clr", &self.msgmem_enable_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Clear"]
    #[inline(always)]
    pub fn msgmem_enable_clr(&mut self) -> MsgmemEnableClrW<'_, DedEnableClrSpec> {
        MsgmemEnableClrW::new(self, 0)
    }
}
#[doc = "MCAN Double Error Detected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableClrSpec;
impl crate::RegisterSpec for DedEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_clr::R`](R) reader structure"]
impl crate::Readable for DedEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_clr::W`](W) writer structure"]
impl crate::Writable for DedEnableClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DED_ENABLE_CLR to value 0"]
impl crate::Resettable for DedEnableClrSpec {}
