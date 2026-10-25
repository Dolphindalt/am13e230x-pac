#[doc = "Register `DMASA` reader"]
pub type R = crate::R<DmasaSpec>;
#[doc = "Register `DMASA` writer"]
pub type W = crate::W<DmasaSpec>;
#[doc = "Field `ADDR` reader - DMA Channel Source Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - DMA Channel Source Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASA").field("addr", &self.addr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DmasaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasa::R`](R) reader structure"]
impl crate::Readable for DmasaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASA to value 0"]
impl crate::Resettable for DmasaSpec {}
