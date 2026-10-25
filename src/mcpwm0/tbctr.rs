#[doc = "Register `TBCTR` reader"]
pub type R = crate::R<TbctrSpec>;
#[doc = "Register `TBCTR` writer"]
pub type W = crate::W<TbctrSpec>;
#[doc = "Field `TBCTR` reader - Counter Value"]
pub type TbctrR = crate::FieldReader<u16>;
#[doc = "Field `TBCTR` writer - Counter Value"]
pub type TbctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn tbctr(&self) -> TbctrR {
        TbctrR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBCTR")
            .field("tbctr", &self.tbctr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn tbctr(&mut self) -> TbctrW<'_, TbctrSpec> {
        TbctrW::new(self, 0)
    }
}
#[doc = "Time base counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctrSpec;
impl crate::RegisterSpec for TbctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctr::R`](R) reader structure"]
impl crate::Readable for TbctrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctr::W`](W) writer structure"]
impl crate::Writable for TbctrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TbctrSpec {}
