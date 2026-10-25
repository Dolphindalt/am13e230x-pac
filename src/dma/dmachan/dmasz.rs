#[doc = "Register `DMASZ` reader"]
pub type R = crate::R<DmaszSpec>;
#[doc = "Register `DMASZ` writer"]
pub type W = crate::W<DmaszSpec>;
#[doc = "Field `SIZE` reader - DMA Channel Size in number of transfers"]
pub type SizeR = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - DMA Channel Size in number of transfers"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA Channel Size in number of transfers"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASZ").field("size", &self.size()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Channel Size in number of transfers"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, DmaszSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "DMA Channel Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaszSpec;
impl crate::RegisterSpec for DmaszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasz::R`](R) reader structure"]
impl crate::Readable for DmaszSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasz::W`](W) writer structure"]
impl crate::Writable for DmaszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASZ to value 0"]
impl crate::Resettable for DmaszSpec {}
