#[doc = "Register `TBPRD` reader"]
pub type R = crate::R<TbprdSpec>;
#[doc = "Register `TBPRD` writer"]
pub type W = crate::W<TbprdSpec>;
#[doc = "Field `TBPRD` reader - Time base period register"]
pub type TbprdR = crate::FieldReader<u16>;
#[doc = "Field `TBPRD` writer - Time base period register"]
pub type TbprdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Time base period register"]
    #[inline(always)]
    pub fn tbprd(&self) -> TbprdR {
        TbprdR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPRD")
            .field("tbprd", &self.tbprd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Time base period register"]
    #[inline(always)]
    pub fn tbprd(&mut self) -> TbprdW<'_, TbprdSpec> {
        TbprdW::new(self, 0)
    }
}
#[doc = "Time base period register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprdSpec;
impl crate::RegisterSpec for TbprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbprd::R`](R) reader structure"]
impl crate::Readable for TbprdSpec {}
#[doc = "`write(|w| ..)` method takes [`tbprd::W`](W) writer structure"]
impl crate::Writable for TbprdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBPRD to value 0"]
impl crate::Resettable for TbprdSpec {}
