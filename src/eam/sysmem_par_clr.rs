#[doc = "Register `SYSMEM_PAR_CLR` reader"]
pub type R = crate::R<SysmemParClrSpec>;
#[doc = "Register `SYSMEM_PAR_CLR` writer"]
pub type W = crate::W<SysmemParClrSpec>;
#[doc = "Field `RD_PAR` reader - Double error detect error"]
pub type RdParR = crate::BitReader;
#[doc = "Field `RD_PAR` writer - Double error detect error"]
pub type RdParW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_PAR` reader - diagnostic Double error detect error"]
pub type WrParR = crate::BitReader;
#[doc = "Field `WR_PAR` writer - diagnostic Double error detect error"]
pub type WrParW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn rd_par(&self) -> RdParR {
        RdParR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - diagnostic Double error detect error"]
    #[inline(always)]
    pub fn wr_par(&self) -> WrParR {
        WrParR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_PAR_CLR")
            .field("rd_par", &self.rd_par())
            .field("wr_par", &self.wr_par())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn rd_par(&mut self) -> RdParW<'_, SysmemParClrSpec> {
        RdParW::new(self, 0)
    }
    #[doc = "Bit 1 - diagnostic Double error detect error"]
    #[inline(always)]
    pub fn wr_par(&mut self) -> WrParW<'_, SysmemParClrSpec> {
        WrParW::new(self, 1)
    }
}
#[doc = "SYSMEM DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemParClrSpec;
impl crate::RegisterSpec for SysmemParClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_par_clr::R`](R) reader structure"]
impl crate::Readable for SysmemParClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_par_clr::W`](W) writer structure"]
impl crate::Writable for SysmemParClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_PAR_CLR to value 0"]
impl crate::Resettable for SysmemParClrSpec {}
