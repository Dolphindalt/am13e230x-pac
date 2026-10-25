#[doc = "Register `PWM3_AQCTLBS` reader"]
pub type R = crate::R<Pwm3AqctlbsSpec>;
#[doc = "Register `PWM3_AQCTLBS` writer"]
pub type W = crate::W<Pwm3AqctlbsSpec>;
#[doc = "Field `ZRO` reader - Action When TBCTR = 0"]
pub type ZroR = crate::FieldReader;
#[doc = "Field `ZRO` writer - Action When TBCTR = 0"]
pub type ZroW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PRD` reader - Action When TBCTR = TBPRD"]
pub type PrdR = crate::FieldReader;
#[doc = "Field `PRD` writer - Action When TBCTR = TBPRD"]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CAU` reader - Action When TBCTR = PWM3_CMPA on Up Count"]
pub type CauR = crate::FieldReader;
#[doc = "Field `CAU` writer - Action When TBCTR = PWM3_CMPA on Up Count"]
pub type CauW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CAD` reader - Action When TBCTR = PWM3_CMPA on Down Count"]
pub type CadR = crate::FieldReader;
#[doc = "Field `CAD` writer - Action When TBCTR = PWM3_CMPA on Down Count"]
pub type CadW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CBU` reader - Action When TBCTR = PWM3_CMPB on Up Count"]
pub type CbuR = crate::FieldReader;
#[doc = "Field `CBU` writer - Action When TBCTR = PWM3_CMPB on Up Count"]
pub type CbuW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CBD` reader - Action When TBCTR = PWM3_CMPB on Down Count"]
pub type CbdR = crate::FieldReader;
#[doc = "Field `CBD` writer - Action When TBCTR = PWM3_CMPB on Down Count"]
pub type CbdW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Action When TBCTR = 0"]
    #[inline(always)]
    pub fn zro(&self) -> ZroR {
        ZroR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action When TBCTR = TBPRD"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action When TBCTR = PWM3_CMPA on Up Count"]
    #[inline(always)]
    pub fn cau(&self) -> CauR {
        CauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action When TBCTR = PWM3_CMPA on Down Count"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action When TBCTR = PWM3_CMPB on Up Count"]
    #[inline(always)]
    pub fn cbu(&self) -> CbuR {
        CbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action When TBCTR = PWM3_CMPB on Down Count"]
    #[inline(always)]
    pub fn cbd(&self) -> CbdR {
        CbdR::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM3_AQCTLBS")
            .field("zro", &self.zro())
            .field("prd", &self.prd())
            .field("cau", &self.cau())
            .field("cad", &self.cad())
            .field("cbu", &self.cbu())
            .field("cbd", &self.cbd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Action When TBCTR = 0"]
    #[inline(always)]
    pub fn zro(&mut self) -> ZroW<'_, Pwm3AqctlbsSpec> {
        ZroW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Action When TBCTR = TBPRD"]
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, Pwm3AqctlbsSpec> {
        PrdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Action When TBCTR = PWM3_CMPA on Up Count"]
    #[inline(always)]
    pub fn cau(&mut self) -> CauW<'_, Pwm3AqctlbsSpec> {
        CauW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Action When TBCTR = PWM3_CMPA on Down Count"]
    #[inline(always)]
    pub fn cad(&mut self) -> CadW<'_, Pwm3AqctlbsSpec> {
        CadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Action When TBCTR = PWM3_CMPB on Up Count"]
    #[inline(always)]
    pub fn cbu(&mut self) -> CbuW<'_, Pwm3AqctlbsSpec> {
        CbuW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Action When TBCTR = PWM3_CMPB on Down Count"]
    #[inline(always)]
    pub fn cbd(&mut self) -> CbdW<'_, Pwm3AqctlbsSpec> {
        CbdW::new(self, 10)
    }
}
#[doc = "PWM3 action qualifier B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqctlbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqctlbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3AqctlbsSpec;
impl crate::RegisterSpec for Pwm3AqctlbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_aqctlbs::R`](R) reader structure"]
impl crate::Readable for Pwm3AqctlbsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_aqctlbs::W`](W) writer structure"]
impl crate::Writable for Pwm3AqctlbsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_AQCTLBS to value 0"]
impl crate::Resettable for Pwm3AqctlbsSpec {}
