#[doc = "Register `TBPRDS` reader"]
pub type R = crate::R<TbprdsSpec>;
#[doc = "Register `TBPRDS` writer"]
pub type W = crate::W<TbprdsSpec>;
#[doc = "Field `TBPRDS` reader - Time base period shadow register"]
pub type TbprdsR = crate::FieldReader<u16>;
#[doc = "Field `TBPRDS` writer - Time base period shadow register"]
pub type TbprdsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Time base period shadow register"]
    #[inline(always)]
    pub fn tbprds(&self) -> TbprdsR {
        TbprdsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPRDS")
            .field("tbprds", &self.tbprds())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Time base period shadow register"]
    #[inline(always)]
    pub fn tbprds(&mut self) -> TbprdsW<'_, TbprdsSpec> {
        TbprdsW::new(self, 0)
    }
}
#[doc = "Time base period shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbprds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbprds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprdsSpec;
impl crate::RegisterSpec for TbprdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbprds::R`](R) reader structure"]
impl crate::Readable for TbprdsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbprds::W`](W) writer structure"]
impl crate::Writable for TbprdsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBPRDS to value 0"]
impl crate::Resettable for TbprdsSpec {}
