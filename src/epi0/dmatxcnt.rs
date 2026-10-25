#[doc = "Register `DMATXCNT` reader"]
pub type R = crate::R<DmatxcntSpec>;
#[doc = "Register `DMATXCNT` writer"]
pub type W = crate::W<DmatxcntSpec>;
#[doc = "Field `TXCNT` reader - DMA Count."]
pub type TxcntR = crate::FieldReader<u16>;
#[doc = "Field `TXCNT` writer - DMA Count."]
pub type TxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - DMA Count."]
    #[inline(always)]
    pub fn txcnt(&self) -> TxcntR {
        TxcntR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATXCNT")
            .field("txcnt", &self.txcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Count."]
    #[inline(always)]
    pub fn txcnt(&mut self) -> TxcntW<'_, DmatxcntSpec> {
        TxcntW::new(self, 0)
    }
}
#[doc = "EPI DMA Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatxcntSpec;
impl crate::RegisterSpec for DmatxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxcnt::R`](R) reader structure"]
impl crate::Readable for DmatxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatxcnt::W`](W) writer structure"]
impl crate::Writable for DmatxcntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DMATXCNT to value 0"]
impl crate::Resettable for DmatxcntSpec {}
