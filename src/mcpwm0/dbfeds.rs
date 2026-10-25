#[doc = "Register `DBFEDS` reader"]
pub type R = crate::R<DbfedsSpec>;
#[doc = "Register `DBFEDS` writer"]
pub type W = crate::W<DbfedsSpec>;
#[doc = "Field `DBFEDS` reader - Falling edge delay shadow register"]
pub type DbfedsR = crate::FieldReader<u16>;
#[doc = "Field `DBFEDS` writer - Falling edge delay shadow register"]
pub type DbfedsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:13 - Falling edge delay shadow register"]
    #[inline(always)]
    pub fn dbfeds(&self) -> DbfedsR {
        DbfedsR::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBFEDS")
            .field("dbfeds", &self.dbfeds())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Falling edge delay shadow register"]
    #[inline(always)]
    pub fn dbfeds(&mut self) -> DbfedsW<'_, DbfedsSpec> {
        DbfedsW::new(self, 0)
    }
}
#[doc = "Dead band fall edge delay shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfeds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfeds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbfedsSpec;
impl crate::RegisterSpec for DbfedsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbfeds::R`](R) reader structure"]
impl crate::Readable for DbfedsSpec {}
#[doc = "`write(|w| ..)` method takes [`dbfeds::W`](W) writer structure"]
impl crate::Writable for DbfedsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBFEDS to value 0"]
impl crate::Resettable for DbfedsSpec {}
