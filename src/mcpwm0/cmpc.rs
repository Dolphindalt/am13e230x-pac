#[doc = "Register `CMPC` reader"]
pub type R = crate::R<CmpcSpec>;
#[doc = "Register `CMPC` writer"]
pub type W = crate::W<CmpcSpec>;
#[doc = "Field `CMPC` reader - Compare C register"]
pub type CmpcR = crate::FieldReader<u16>;
#[doc = "Field `CMPC` writer - Compare C register"]
pub type CmpcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Compare C register"]
    #[inline(always)]
    pub fn cmpc(&self) -> CmpcR {
        CmpcR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPC").field("cmpc", &self.cmpc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare C register"]
    #[inline(always)]
    pub fn cmpc(&mut self) -> CmpcW<'_, CmpcSpec> {
        CmpcW::new(self, 0)
    }
}
#[doc = "Counter compare C register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpcSpec;
impl crate::RegisterSpec for CmpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpc::R`](R) reader structure"]
impl crate::Readable for CmpcSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpc::W`](W) writer structure"]
impl crate::Writable for CmpcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPC to value 0"]
impl crate::Resettable for CmpcSpec {}
