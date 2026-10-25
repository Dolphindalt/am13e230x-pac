#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `OUTPUTRDY` writer - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type OutputrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUTRDY` writer - AES ready"]
pub type InputrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAVEDCNTXTRDY` writer - AES ready"]
pub type SavedcntxtrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTXTRDY` writer - AES ready"]
pub type CntxtrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn outputrdy(&mut self) -> OutputrdyW<'_, IsetSpec> {
        OutputrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - AES ready"]
    #[inline(always)]
    pub fn inputrdy(&mut self) -> InputrdyW<'_, IsetSpec> {
        InputrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - AES ready"]
    #[inline(always)]
    pub fn savedcntxtrdy(&mut self) -> SavedcntxtrdyW<'_, IsetSpec> {
        SavedcntxtrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - AES ready"]
    #[inline(always)]
    pub fn cntxtrdy(&mut self) -> CntxtrdyW<'_, IsetSpec> {
        CntxtrdyW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {}
