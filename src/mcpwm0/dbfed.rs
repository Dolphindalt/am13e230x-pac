#[doc = "Register `DBFED` reader"]
pub type R = crate::R<DbfedSpec>;
#[doc = "Register `DBFED` writer"]
pub type W = crate::W<DbfedSpec>;
#[doc = "Field `DBFED` reader - Falling edge delay value"]
pub type DbfedR = crate::FieldReader<u16>;
#[doc = "Field `DBFED` writer - Falling edge delay value"]
pub type DbfedW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:13 - Falling edge delay value"]
    #[inline(always)]
    pub fn dbfed(&self) -> DbfedR {
        DbfedR::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBFED")
            .field("dbfed", &self.dbfed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Falling edge delay value"]
    #[inline(always)]
    pub fn dbfed(&mut self) -> DbfedW<'_, DbfedSpec> {
        DbfedW::new(self, 0)
    }
}
#[doc = "Dead band fall edge delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbfedSpec;
impl crate::RegisterSpec for DbfedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbfed::R`](R) reader structure"]
impl crate::Readable for DbfedSpec {}
#[doc = "`write(|w| ..)` method takes [`dbfed::W`](W) writer structure"]
impl crate::Writable for DbfedSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBFED to value 0"]
impl crate::Resettable for DbfedSpec {}
