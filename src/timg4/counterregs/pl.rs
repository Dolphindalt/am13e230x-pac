#[doc = "Register `PL` reader"]
pub type R = crate::R<PlSpec>;
#[doc = "Register `PL` writer"]
pub type W = crate::W<PlSpec>;
#[doc = "Field `PHASE` reader - Phase Load value"]
pub type PhaseR = crate::FieldReader<u32>;
#[doc = "Field `PHASE` writer - Phase Load value"]
pub type PhaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Phase Load value"]
    #[inline(always)]
    pub fn phase(&self) -> PhaseR {
        PhaseR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PL").field("phase", &self.phase()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Phase Load value"]
    #[inline(always)]
    pub fn phase(&mut self) -> PhaseW<'_, PlSpec> {
        PhaseW::new(self, 0)
    }
}
#[doc = "Phase Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlSpec;
impl crate::RegisterSpec for PlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl::R`](R) reader structure"]
impl crate::Readable for PlSpec {}
#[doc = "`write(|w| ..)` method takes [`pl::W`](W) writer structure"]
impl crate::Writable for PlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PL to value 0"]
impl crate::Resettable for PlSpec {}
