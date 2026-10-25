#[doc = "Register `TBSTS` reader"]
pub type R = crate::R<TbstsSpec>;
#[doc = "Register `TBSTS` writer"]
pub type W = crate::W<TbstsSpec>;
#[doc = "Field `CTRDIR` reader - Counter Direction Status"]
pub type CtrdirR = crate::BitReader;
#[doc = "Field `SYNCI` reader - External Input Sync Status"]
pub type SynciR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Direction Status"]
    #[inline(always)]
    pub fn ctrdir(&self) -> CtrdirR {
        CtrdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Input Sync Status"]
    #[inline(always)]
    pub fn synci(&self) -> SynciR {
        SynciR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBSTS")
            .field("ctrdir", &self.ctrdir())
            .field("synci", &self.synci())
            .finish()
    }
}
impl W {}
#[doc = "Time base status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbstsSpec;
impl crate::RegisterSpec for TbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbsts::R`](R) reader structure"]
impl crate::Readable for TbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbsts::W`](W) writer structure"]
impl crate::Writable for TbstsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBSTS to value 0"]
impl crate::Resettable for TbstsSpec {}
