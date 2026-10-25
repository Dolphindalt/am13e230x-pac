#[doc = "Register `SECURITY_ERR_CLR` reader"]
pub type R = crate::R<SecurityErrClrSpec>;
#[doc = "Register `SECURITY_ERR_CLR` writer"]
pub type W = crate::W<SecurityErrClrSpec>;
#[doc = "Field `PRIV` reader - Privilege error"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - Privilege error"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP` reader - Hide protection error"]
pub type HdpR = crate::BitReader;
#[doc = "Field `HDP` writer - Hide protection error"]
pub type HdpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLADDR` reader - FPI ILLADDR"]
pub type FpiIlladdrR = crate::BitReader;
#[doc = "Field `FPI_ILLADDR` writer - FPI ILLADDR"]
pub type FpiIlladdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLPROG` reader - FPI ILLPROG"]
pub type FpiIllprogR = crate::BitReader;
#[doc = "Field `FPI_ILLPROG` writer - FPI ILLPROG"]
pub type FpiIllprogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLERASE` reader - FPI ILLERASE"]
pub type FpiIlleraseR = crate::BitReader;
#[doc = "Field `FPI_ILLERASE` writer - FPI ILLERASE"]
pub type FpiIlleraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLRDVER` reader - FPI ILLRDVER"]
pub type FpiIllrdverR = crate::BitReader;
#[doc = "Field `FPI_ILLRDVER` writer - FPI ILLRDVER"]
pub type FpiIllrdverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLMODECH` reader - FPI ILLMODECH"]
pub type FpiIllmodechR = crate::BitReader;
#[doc = "Field `FPI_ILLMODECH` writer - FPI ILLMODECH"]
pub type FpiIllmodechW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLCMD` reader - FPI ILLCMD"]
pub type FpiIllcmdR = crate::BitReader;
#[doc = "Field `FPI_ILLCMD` writer - FPI ILLCMD"]
pub type FpiIllcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPI_ILLSIZE` reader - FPI ILLSIZE"]
pub type FpiIllsizeR = crate::BitReader;
#[doc = "Field `FPI_ILLSIZE` writer - FPI ILLSIZE"]
pub type FpiIllsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLC_MMR_ACCESS_ERROR` reader - FLC MMR ACCESS ERROR"]
pub type FlcMmrAccessErrorR = crate::BitReader;
#[doc = "Field `FLC_MMR_ACCESS_ERROR` writer - FLC MMR ACCESS ERROR"]
pub type FlcMmrAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLSEM_ACCESS_ERROR` reader - FLSEM ACCESS ERROR"]
pub type FlsemAccessErrorR = crate::BitReader;
#[doc = "Field `FLSEM_ACCESS_ERROR` writer - FLSEM ACCESS ERROR"]
pub type FlsemAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Privilege error"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hide protection error"]
    #[inline(always)]
    pub fn hdp(&self) -> HdpR {
        HdpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPI ILLADDR"]
    #[inline(always)]
    pub fn fpi_illaddr(&self) -> FpiIlladdrR {
        FpiIlladdrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPI ILLPROG"]
    #[inline(always)]
    pub fn fpi_illprog(&self) -> FpiIllprogR {
        FpiIllprogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPI ILLERASE"]
    #[inline(always)]
    pub fn fpi_illerase(&self) -> FpiIlleraseR {
        FpiIlleraseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FPI ILLRDVER"]
    #[inline(always)]
    pub fn fpi_illrdver(&self) -> FpiIllrdverR {
        FpiIllrdverR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FPI ILLMODECH"]
    #[inline(always)]
    pub fn fpi_illmodech(&self) -> FpiIllmodechR {
        FpiIllmodechR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FPI ILLCMD"]
    #[inline(always)]
    pub fn fpi_illcmd(&self) -> FpiIllcmdR {
        FpiIllcmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPI ILLSIZE"]
    #[inline(always)]
    pub fn fpi_illsize(&self) -> FpiIllsizeR {
        FpiIllsizeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLC MMR ACCESS ERROR"]
    #[inline(always)]
    pub fn flc_mmr_access_error(&self) -> FlcMmrAccessErrorR {
        FlcMmrAccessErrorR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FLSEM ACCESS ERROR"]
    #[inline(always)]
    pub fn flsem_access_error(&self) -> FlsemAccessErrorR {
        FlsemAccessErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURITY_ERR_CLR")
            .field("priv_", &self.priv_())
            .field("hdp", &self.hdp())
            .field("fpi_illaddr", &self.fpi_illaddr())
            .field("fpi_illprog", &self.fpi_illprog())
            .field("fpi_illerase", &self.fpi_illerase())
            .field("fpi_illrdver", &self.fpi_illrdver())
            .field("fpi_illmodech", &self.fpi_illmodech())
            .field("fpi_illcmd", &self.fpi_illcmd())
            .field("fpi_illsize", &self.fpi_illsize())
            .field("flc_mmr_access_error", &self.flc_mmr_access_error())
            .field("flsem_access_error", &self.flsem_access_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Privilege error"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, SecurityErrClrSpec> {
        PrivW::new(self, 1)
    }
    #[doc = "Bit 2 - Hide protection error"]
    #[inline(always)]
    pub fn hdp(&mut self) -> HdpW<'_, SecurityErrClrSpec> {
        HdpW::new(self, 2)
    }
    #[doc = "Bit 3 - FPI ILLADDR"]
    #[inline(always)]
    pub fn fpi_illaddr(&mut self) -> FpiIlladdrW<'_, SecurityErrClrSpec> {
        FpiIlladdrW::new(self, 3)
    }
    #[doc = "Bit 4 - FPI ILLPROG"]
    #[inline(always)]
    pub fn fpi_illprog(&mut self) -> FpiIllprogW<'_, SecurityErrClrSpec> {
        FpiIllprogW::new(self, 4)
    }
    #[doc = "Bit 5 - FPI ILLERASE"]
    #[inline(always)]
    pub fn fpi_illerase(&mut self) -> FpiIlleraseW<'_, SecurityErrClrSpec> {
        FpiIlleraseW::new(self, 5)
    }
    #[doc = "Bit 6 - FPI ILLRDVER"]
    #[inline(always)]
    pub fn fpi_illrdver(&mut self) -> FpiIllrdverW<'_, SecurityErrClrSpec> {
        FpiIllrdverW::new(self, 6)
    }
    #[doc = "Bit 7 - FPI ILLMODECH"]
    #[inline(always)]
    pub fn fpi_illmodech(&mut self) -> FpiIllmodechW<'_, SecurityErrClrSpec> {
        FpiIllmodechW::new(self, 7)
    }
    #[doc = "Bit 8 - FPI ILLCMD"]
    #[inline(always)]
    pub fn fpi_illcmd(&mut self) -> FpiIllcmdW<'_, SecurityErrClrSpec> {
        FpiIllcmdW::new(self, 8)
    }
    #[doc = "Bit 9 - FPI ILLSIZE"]
    #[inline(always)]
    pub fn fpi_illsize(&mut self) -> FpiIllsizeW<'_, SecurityErrClrSpec> {
        FpiIllsizeW::new(self, 9)
    }
    #[doc = "Bit 10 - FLC MMR ACCESS ERROR"]
    #[inline(always)]
    pub fn flc_mmr_access_error(&mut self) -> FlcMmrAccessErrorW<'_, SecurityErrClrSpec> {
        FlcMmrAccessErrorW::new(self, 10)
    }
    #[doc = "Bit 11 - FLSEM ACCESS ERROR"]
    #[inline(always)]
    pub fn flsem_access_error(&mut self) -> FlsemAccessErrorW<'_, SecurityErrClrSpec> {
        FlsemAccessErrorW::new(self, 11)
    }
}
#[doc = "Security error clear\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecurityErrClrSpec;
impl crate::RegisterSpec for SecurityErrClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`security_err_clr::R`](R) reader structure"]
impl crate::Readable for SecurityErrClrSpec {}
#[doc = "`write(|w| ..)` method takes [`security_err_clr::W`](W) writer structure"]
impl crate::Writable for SecurityErrClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SECURITY_ERR_CLR to value 0"]
impl crate::Resettable for SecurityErrClrSpec {}
