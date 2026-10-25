#[doc = "Register `DMADA` reader"]
pub type R = crate::R<DmadaSpec>;
#[doc = "Register `DMADA` writer"]
pub type W = crate::W<DmadaSpec>;
#[doc = "Field `ADDR` reader - DMA Channel Destination Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - DMA Channel Destination Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMADA").field("addr", &self.addr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DmadaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmada::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmada::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadaSpec;
impl crate::RegisterSpec for DmadaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmada::R`](R) reader structure"]
impl crate::Readable for DmadaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmada::W`](W) writer structure"]
impl crate::Writable for DmadaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMADA to value 0"]
impl crate::Resettable for DmadaSpec {}
