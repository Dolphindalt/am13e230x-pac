#[doc = "Register `STAT_SYSPERIPH` reader"]
pub type R = crate::R<StatSysperiphSpec>;
#[doc = "Field `tinie` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type TinieR = crate::BitReader;
#[doc = "Field `pga0` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pga0R = crate::BitReader;
#[doc = "Field `pga1` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pga1R = crate::BitReader;
#[doc = "Field `pga2` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pga2R = crate::BitReader;
#[doc = "Field `epi` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type EpiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn tinie(&self) -> TinieR {
        TinieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pga0(&self) -> Pga0R {
        Pga0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pga1(&self) -> Pga1R {
        Pga1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pga2(&self) -> Pga2R {
        Pga2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn epi(&self) -> EpiR {
        EpiR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT_SYSPERIPH")
            .field("tinie", &self.tinie())
            .field("pga0", &self.pga0())
            .field("pga1", &self.pga1())
            .field("pga2", &self.pga2())
            .field("epi", &self.epi())
            .finish()
    }
}
#[doc = "IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_sysperiph::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSysperiphSpec;
impl crate::RegisterSpec for StatSysperiphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_sysperiph::R`](R) reader structure"]
impl crate::Readable for StatSysperiphSpec {}
#[doc = "`reset()` method sets STAT_SYSPERIPH to value 0"]
impl crate::Resettable for StatSysperiphSpec {}
