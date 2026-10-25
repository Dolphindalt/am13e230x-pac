#[doc = "Register `SYSMEM_PAR_FLAG` reader"]
pub type R = crate::R<SysmemParFlagSpec>;
#[doc = "Register `SYSMEM_PAR_FLAG` writer"]
pub type W = crate::W<SysmemParFlagSpec>;
#[doc = "Field `RD_PAR` reader - Double error detect error during read"]
pub type RdParR = crate::BitReader;
#[doc = "Field `WR_PAR` reader - diagnostic Double error detect error during write"]
pub type WrParR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Double error detect error during read"]
    #[inline(always)]
    pub fn rd_par(&self) -> RdParR {
        RdParR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - diagnostic Double error detect error during write"]
    #[inline(always)]
    pub fn wr_par(&self) -> WrParR {
        WrParR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_PAR_FLAG")
            .field("rd_par", &self.rd_par())
            .field("wr_par", &self.wr_par())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemParFlagSpec;
impl crate::RegisterSpec for SysmemParFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_par_flag::R`](R) reader structure"]
impl crate::Readable for SysmemParFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_par_flag::W`](W) writer structure"]
impl crate::Writable for SysmemParFlagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_PAR_FLAG to value 0"]
impl crate::Resettable for SysmemParFlagSpec {}
