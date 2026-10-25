#[doc = "Register `TBSTSCLR` reader"]
pub type R = crate::R<TbstsclrSpec>;
#[doc = "Register `TBSTSCLR` writer"]
pub type W = crate::W<TbstsclrSpec>;
#[doc = "Field `SYNCI` reader - External Input Sync Status clear"]
pub type SynciR = crate::BitReader;
#[doc = "Field `SYNCI` writer - External Input Sync Status clear"]
pub type SynciW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - External Input Sync Status clear"]
    #[inline(always)]
    pub fn synci(&self) -> SynciR {
        SynciR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBSTSCLR")
            .field("synci", &self.synci())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - External Input Sync Status clear"]
    #[inline(always)]
    pub fn synci(&mut self) -> SynciW<'_, TbstsclrSpec> {
        SynciW::new(self, 1)
    }
}
#[doc = "Time base status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbstsclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbstsclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbstsclrSpec;
impl crate::RegisterSpec for TbstsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbstsclr::R`](R) reader structure"]
impl crate::Readable for TbstsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbstsclr::W`](W) writer structure"]
impl crate::Writable for TbstsclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TBSTSCLR to value 0"]
impl crate::Resettable for TbstsclrSpec {}
