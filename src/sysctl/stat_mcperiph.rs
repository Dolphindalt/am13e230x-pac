#[doc = "Register `STAT_MCPERIPH` reader"]
pub type R = crate::R<StatMcperiphSpec>;
#[doc = "Field `eqep0` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Eqep0R = crate::BitReader;
#[doc = "Field `eqep1` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Eqep1R = crate::BitReader;
#[doc = "Field `eqep2` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Eqep2R = crate::BitReader;
#[doc = "Field `ecap0` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Ecap0R = crate::BitReader;
#[doc = "Field `ecap1` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Ecap1R = crate::BitReader;
#[doc = "Field `pwm0` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pwm0R = crate::BitReader;
#[doc = "Field `pwm1` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pwm1R = crate::BitReader;
#[doc = "Field `pwm2` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pwm2R = crate::BitReader;
#[doc = "Field `pwm3` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pwm3R = crate::BitReader;
#[doc = "Field `pwm4` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Pwm4R = crate::BitReader;
#[doc = "Field `cmpss0` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Cmpss0R = crate::BitReader;
#[doc = "Field `cmpss1` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Cmpss1R = crate::BitReader;
#[doc = "Field `cmpss2` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Cmpss2R = crate::BitReader;
#[doc = "Field `cmpss3` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type Cmpss3R = crate::BitReader;
#[doc = "Field `xbar` reader - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
pub type XbarR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn eqep0(&self) -> Eqep0R {
        Eqep0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn eqep1(&self) -> Eqep1R {
        Eqep1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn eqep2(&self) -> Eqep2R {
        Eqep2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn ecap0(&self) -> Ecap0R {
        Ecap0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn ecap1(&self) -> Ecap1R {
        Ecap1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pwm0(&self) -> Pwm0R {
        Pwm0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pwm1(&self) -> Pwm1R {
        Pwm1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pwm2(&self) -> Pwm2R {
        Pwm2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pwm3(&self) -> Pwm3R {
        Pwm3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn pwm4(&self) -> Pwm4R {
        Pwm4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn cmpss0(&self) -> Cmpss0R {
        Cmpss0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn cmpss1(&self) -> Cmpss1R {
        Cmpss1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn cmpss2(&self) -> Cmpss2R {
        Cmpss2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn cmpss3(&self) -> Cmpss3R {
        Cmpss3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit indicates, if the peripheral was reset, since this bit was cleared by RESETSTKYCLR in the RSTCTL register"]
    #[inline(always)]
    pub fn xbar(&self) -> XbarR {
        XbarR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT_MCPERIPH")
            .field("eqep0", &self.eqep0())
            .field("eqep1", &self.eqep1())
            .field("eqep2", &self.eqep2())
            .field("ecap0", &self.ecap0())
            .field("ecap1", &self.ecap1())
            .field("pwm0", &self.pwm0())
            .field("pwm1", &self.pwm1())
            .field("pwm2", &self.pwm2())
            .field("pwm3", &self.pwm3())
            .field("pwm4", &self.pwm4())
            .field("cmpss0", &self.cmpss0())
            .field("cmpss1", &self.cmpss1())
            .field("cmpss2", &self.cmpss2())
            .field("cmpss3", &self.cmpss3())
            .field("xbar", &self.xbar())
            .finish()
    }
}
#[doc = "IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_mcperiph::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatMcperiphSpec;
impl crate::RegisterSpec for StatMcperiphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_mcperiph::R`](R) reader structure"]
impl crate::Readable for StatMcperiphSpec {}
#[doc = "`reset()` method sets STAT_MCPERIPH to value 0"]
impl crate::Resettable for StatMcperiphSpec {}
