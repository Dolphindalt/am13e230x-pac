#[doc = "Register `SEC_ENABLE_SET` reader"]
pub type R = crate::R<SecEnableSetSpec>;
#[doc = "Register `SEC_ENABLE_SET` writer"]
pub type W = crate::W<SecEnableSetSpec>;
#[doc = "Field `MSGMEM_ENABLE_SET` reader - Message RAM SEC Interrupt Pending Enable Set"]
pub type MsgmemEnableSetR = crate::BitReader;
#[doc = "Field `MSGMEM_ENABLE_SET` writer - Message RAM SEC Interrupt Pending Enable Set"]
pub type MsgmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Set"]
    #[inline(always)]
    pub fn msgmem_enable_set(&self) -> MsgmemEnableSetR {
        MsgmemEnableSetR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_ENABLE_SET")
            .field("msgmem_enable_set", &self.msgmem_enable_set())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Set"]
    #[inline(always)]
    pub fn msgmem_enable_set(&mut self) -> MsgmemEnableSetW<'_, SecEnableSetSpec> {
        MsgmemEnableSetW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableSetSpec;
impl crate::RegisterSpec for SecEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_set::R`](R) reader structure"]
impl crate::Readable for SecEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_set::W`](W) writer structure"]
impl crate::Writable for SecEnableSetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SEC_ENABLE_SET to value 0"]
impl crate::Resettable for SecEnableSetSpec {}
