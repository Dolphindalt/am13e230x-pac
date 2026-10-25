#[doc = "Register `PPC_PRIVATTRIB_DMA` reader"]
pub type R = crate::R<PpcPrivattribDmaSpec>;
#[doc = "Register `PPC_PRIVATTRIB_DMA` writer"]
pub type W = crate::W<PpcPrivattribDmaSpec>;
#[doc = "Field `PRIV_DMA` reader - Privilege access for DMA"]
pub type PrivDmaR = crate::BitReader;
#[doc = "Field `PRIV_DMA` writer - Privilege access for DMA"]
pub type PrivDmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for DMA"]
    #[inline(always)]
    pub fn priv_dma(&self) -> PrivDmaR {
        PrivDmaR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_DMA")
            .field("priv_dma", &self.priv_dma())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for DMA"]
    #[inline(always)]
    pub fn priv_dma(&mut self) -> PrivDmaW<'_, PpcPrivattribDmaSpec> {
        PrivDmaW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribDmaSpec;
impl crate::RegisterSpec for PpcPrivattribDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_dma::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_dma::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_DMA to value 0x01"]
impl crate::Resettable for PpcPrivattribDmaSpec {
    const RESET_VALUE: u32 = 0x01;
}
