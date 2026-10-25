#[doc = "Register `DED_ENABLE_SET` reader"]
pub type R = crate::R<DedEnableSetSpec>;
#[doc = "Register `DED_ENABLE_SET` writer"]
pub type W = crate::W<DedEnableSetSpec>;
#[doc = "Field `MSGMEM_ENABLE_SET` reader - Message RAM DED Interrupt Pending Enable Set"]
pub type MsgmemEnableSetR = crate::BitReader;
#[doc = "Field `MSGMEM_ENABLE_SET` writer - Message RAM DED Interrupt Pending Enable Set"]
pub type MsgmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Set"]
    #[inline(always)]
    pub fn msgmem_enable_set(&self) -> MsgmemEnableSetR {
        MsgmemEnableSetR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DED_ENABLE_SET")
            .field("msgmem_enable_set", &self.msgmem_enable_set())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Set"]
    #[inline(always)]
    pub fn msgmem_enable_set(&mut self) -> MsgmemEnableSetW<'_, DedEnableSetSpec> {
        MsgmemEnableSetW::new(self, 0)
    }
}
#[doc = "MCAN Double Error Detected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableSetSpec;
impl crate::RegisterSpec for DedEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_set::R`](R) reader structure"]
impl crate::Readable for DedEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_set::W`](W) writer structure"]
impl crate::Writable for DedEnableSetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DED_ENABLE_SET to value 0"]
impl crate::Resettable for DedEnableSetSpec {}
