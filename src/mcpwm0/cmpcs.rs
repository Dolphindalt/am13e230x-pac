#[doc = "Register `CMPCS` reader"]
pub type R = crate::R<CmpcsSpec>;
#[doc = "Register `CMPCS` writer"]
pub type W = crate::W<CmpcsSpec>;
#[doc = "Field `CMPCS` reader - Compare C shadow register"]
pub type CmpcsR = crate::FieldReader<u16>;
#[doc = "Field `CMPCS` writer - Compare C shadow register"]
pub type CmpcsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Compare C shadow register"]
    #[inline(always)]
    pub fn cmpcs(&self) -> CmpcsR {
        CmpcsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPCS")
            .field("cmpcs", &self.cmpcs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare C shadow register"]
    #[inline(always)]
    pub fn cmpcs(&mut self) -> CmpcsW<'_, CmpcsSpec> {
        CmpcsW::new(self, 0)
    }
}
#[doc = "Counter compare C shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpcsSpec;
impl crate::RegisterSpec for CmpcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpcs::R`](R) reader structure"]
impl crate::Readable for CmpcsSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpcs::W`](W) writer structure"]
impl crate::Writable for CmpcsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPCS to value 0"]
impl crate::Resettable for CmpcsSpec {}
