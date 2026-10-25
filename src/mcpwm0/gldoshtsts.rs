#[doc = "Register `GLDOSHTSTS` reader"]
pub type R = crate::R<GldoshtstsSpec>;
#[doc = "Register `GLDOSHTSTS` writer"]
pub type W = crate::W<GldoshtstsSpec>;
#[doc = "Field `OSHTLATCH` reader - One shot latch output"]
pub type OshtlatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - One shot latch output"]
    #[inline(always)]
    pub fn oshtlatch(&self) -> OshtlatchR {
        OshtlatchR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLDOSHTSTS")
            .field("oshtlatch", &self.oshtlatch())
            .finish()
    }
}
impl W {}
#[doc = "Global load one shot status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldoshtsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldoshtsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GldoshtstsSpec;
impl crate::RegisterSpec for GldoshtstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gldoshtsts::R`](R) reader structure"]
impl crate::Readable for GldoshtstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gldoshtsts::W`](W) writer structure"]
impl crate::Writable for GldoshtstsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GLDOSHTSTS to value 0"]
impl crate::Resettable for GldoshtstsSpec {}
