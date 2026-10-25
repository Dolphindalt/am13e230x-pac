#[doc = "Register `PGATYPE` reader"]
pub type R = crate::R<PgatypeSpec>;
#[doc = "Register `PGATYPE` writer"]
pub type W = crate::W<PgatypeSpec>;
#[doc = "Field `REV` reader - PGA Revision Field"]
pub type RevR = crate::FieldReader;
#[doc = "Field `TYPE` reader - PGA Type Field"]
pub type TypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PGA Revision Field"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PGA Type Field"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGATYPE")
            .field("rev", &self.rev())
            .field("type_", &self.type_())
            .finish()
    }
}
impl W {}
#[doc = "PGA Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgatype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgatype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgatypeSpec;
impl crate::RegisterSpec for PgatypeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgatype::R`](R) reader structure"]
impl crate::Readable for PgatypeSpec {}
#[doc = "`write(|w| ..)` method takes [`pgatype::W`](W) writer structure"]
impl crate::Writable for PgatypeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PGATYPE to value 0x0201"]
impl crate::Resettable for PgatypeSpec {
    const RESET_VALUE: u16 = 0x0201;
}
