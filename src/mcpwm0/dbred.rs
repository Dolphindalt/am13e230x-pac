#[doc = "Register `DBRED` reader"]
pub type R = crate::R<DbredSpec>;
#[doc = "Register `DBRED` writer"]
pub type W = crate::W<DbredSpec>;
#[doc = "Field `DBRED` reader - Rising edge delay value"]
pub type DbredR = crate::FieldReader<u16>;
#[doc = "Field `DBRED` writer - Rising edge delay value"]
pub type DbredW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:13 - Rising edge delay value"]
    #[inline(always)]
    pub fn dbred(&self) -> DbredR {
        DbredR::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBRED")
            .field("dbred", &self.dbred())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Rising edge delay value"]
    #[inline(always)]
    pub fn dbred(&mut self) -> DbredW<'_, DbredSpec> {
        DbredW::new(self, 0)
    }
}
#[doc = "Dead band rise edge delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dbred::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbred::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbredSpec;
impl crate::RegisterSpec for DbredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbred::R`](R) reader structure"]
impl crate::Readable for DbredSpec {}
#[doc = "`write(|w| ..)` method takes [`dbred::W`](W) writer structure"]
impl crate::Writable for DbredSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBRED to value 0"]
impl crate::Resettable for DbredSpec {}
