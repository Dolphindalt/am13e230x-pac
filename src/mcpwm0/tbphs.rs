#[doc = "Register `TBPHS` reader"]
pub type R = crate::R<TbphsSpec>;
#[doc = "Register `TBPHS` writer"]
pub type W = crate::W<TbphsSpec>;
#[doc = "Field `TBPHS` reader - Phase Offset Register"]
pub type TbphsR = crate::FieldReader<u16>;
#[doc = "Field `TBPHS` writer - Phase Offset Register"]
pub type TbphsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Phase Offset Register"]
    #[inline(always)]
    pub fn tbphs(&self) -> TbphsR {
        TbphsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPHS")
            .field("tbphs", &self.tbphs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Phase Offset Register"]
    #[inline(always)]
    pub fn tbphs(&mut self) -> TbphsW<'_, TbphsSpec> {
        TbphsW::new(self, 0)
    }
}
#[doc = "Time base phase offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbphs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbphs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbphsSpec;
impl crate::RegisterSpec for TbphsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbphs::R`](R) reader structure"]
impl crate::Readable for TbphsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbphs::W`](W) writer structure"]
impl crate::Writable for TbphsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBPHS to value 0"]
impl crate::Resettable for TbphsSpec {}
