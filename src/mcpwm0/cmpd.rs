#[doc = "Register `CMPD` reader"]
pub type R = crate::R<CmpdSpec>;
#[doc = "Register `CMPD` writer"]
pub type W = crate::W<CmpdSpec>;
#[doc = "Field `CMPD` reader - Compare D register"]
pub type CmpdR = crate::FieldReader<u16>;
#[doc = "Field `CMPD` writer - Compare D register"]
pub type CmpdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Compare D register"]
    #[inline(always)]
    pub fn cmpd(&self) -> CmpdR {
        CmpdR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPD").field("cmpd", &self.cmpd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare D register"]
    #[inline(always)]
    pub fn cmpd(&mut self) -> CmpdW<'_, CmpdSpec> {
        CmpdW::new(self, 0)
    }
}
#[doc = "Counter compare D register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpdSpec;
impl crate::RegisterSpec for CmpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpd::R`](R) reader structure"]
impl crate::Readable for CmpdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpd::W`](W) writer structure"]
impl crate::Writable for CmpdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPD to value 0"]
impl crate::Resettable for CmpdSpec {}
