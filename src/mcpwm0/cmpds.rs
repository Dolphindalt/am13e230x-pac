#[doc = "Register `CMPDS` reader"]
pub type R = crate::R<CmpdsSpec>;
#[doc = "Register `CMPDS` writer"]
pub type W = crate::W<CmpdsSpec>;
#[doc = "Field `CMPDS` reader - Compare D shadow register"]
pub type CmpdsR = crate::FieldReader<u16>;
#[doc = "Field `CMPDS` writer - Compare D shadow register"]
pub type CmpdsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Compare D shadow register"]
    #[inline(always)]
    pub fn cmpds(&self) -> CmpdsR {
        CmpdsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPDS")
            .field("cmpds", &self.cmpds())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare D shadow register"]
    #[inline(always)]
    pub fn cmpds(&mut self) -> CmpdsW<'_, CmpdsSpec> {
        CmpdsW::new(self, 0)
    }
}
#[doc = "Counter compare D shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpdsSpec;
impl crate::RegisterSpec for CmpdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpds::R`](R) reader structure"]
impl crate::Readable for CmpdsSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpds::W`](W) writer structure"]
impl crate::Writable for CmpdsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPDS to value 0"]
impl crate::Resettable for CmpdsSpec {}
