#[doc = "Register `DBREDS` reader"]
pub type R = crate::R<DbredsSpec>;
#[doc = "Register `DBREDS` writer"]
pub type W = crate::W<DbredsSpec>;
#[doc = "Field `DBREDS` reader - Rising edge delay shadow register"]
pub type DbredsR = crate::FieldReader<u16>;
#[doc = "Field `DBREDS` writer - Rising edge delay shadow register"]
pub type DbredsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:13 - Rising edge delay shadow register"]
    #[inline(always)]
    pub fn dbreds(&self) -> DbredsR {
        DbredsR::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBREDS")
            .field("dbreds", &self.dbreds())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Rising edge delay shadow register"]
    #[inline(always)]
    pub fn dbreds(&mut self) -> DbredsW<'_, DbredsSpec> {
        DbredsW::new(self, 0)
    }
}
#[doc = "Dead band rise edge delay shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbreds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbreds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbredsSpec;
impl crate::RegisterSpec for DbredsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbreds::R`](R) reader structure"]
impl crate::Readable for DbredsSpec {}
#[doc = "`write(|w| ..)` method takes [`dbreds::W`](W) writer structure"]
impl crate::Writable for DbredsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBREDS to value 0"]
impl crate::Resettable for DbredsSpec {}
