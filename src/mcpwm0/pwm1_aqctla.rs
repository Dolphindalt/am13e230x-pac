#[doc = "Register `PWM1_AQCTLA` reader"]
pub type R = crate::R<Pwm1AqctlaSpec>;
#[doc = "Register `PWM1_AQCTLA` writer"]
pub type W = crate::W<Pwm1AqctlaSpec>;
#[doc = "Field `ZRO` reader - Action When TBCTR = 0"]
pub type ZroR = crate::FieldReader;
#[doc = "Field `ZRO` writer - Action When TBCTR = 0"]
pub type ZroW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PRD` reader - Action When TBCTR = TBPRD"]
pub type PrdR = crate::FieldReader;
#[doc = "Field `PRD` writer - Action When TBCTR = TBPRD"]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CAU` reader - Action When TBCTR = PWM1_CMPA on Up Count"]
pub type CauR = crate::FieldReader;
#[doc = "Field `CAU` writer - Action When TBCTR = PWM1_CMPA on Up Count"]
pub type CauW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CAD` reader - Action When TBCTR = PWM1_CMPA on Down Count"]
pub type CadR = crate::FieldReader;
#[doc = "Field `CAD` writer - Action When TBCTR = PWM1_CMPA on Down Count"]
pub type CadW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CBU` reader - Action When TBCTR = PWM1_CMPB on Up Count"]
pub type CbuR = crate::FieldReader;
#[doc = "Field `CBU` writer - Action When TBCTR = PWM1_CMPB on Up Count"]
pub type CbuW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CBD` reader - Action When TBCTR = PWM1_CMPB on Down Count"]
pub type CbdR = crate::FieldReader;
#[doc = "Field `CBD` writer - Action When TBCTR = PWM1_CMPB on Down Count"]
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
    #[doc = "Bits 4:5 - Action When TBCTR = PWM1_CMPA on Up Count"]
    #[inline(always)]
    pub fn cau(&self) -> CauR {
        CauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action When TBCTR = PWM1_CMPA on Down Count"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action When TBCTR = PWM1_CMPB on Up Count"]
    #[inline(always)]
    pub fn cbu(&self) -> CbuR {
        CbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action When TBCTR = PWM1_CMPB on Down Count"]
    #[inline(always)]
    pub fn cbd(&self) -> CbdR {
        CbdR::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_AQCTLA")
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
    pub fn zro(&mut self) -> ZroW<'_, Pwm1AqctlaSpec> {
        ZroW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Action When TBCTR = TBPRD"]
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, Pwm1AqctlaSpec> {
        PrdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Action When TBCTR = PWM1_CMPA on Up Count"]
    #[inline(always)]
    pub fn cau(&mut self) -> CauW<'_, Pwm1AqctlaSpec> {
        CauW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Action When TBCTR = PWM1_CMPA on Down Count"]
    #[inline(always)]
    pub fn cad(&mut self) -> CadW<'_, Pwm1AqctlaSpec> {
        CadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Action When TBCTR = PWM1_CMPB on Up Count"]
    #[inline(always)]
    pub fn cbu(&mut self) -> CbuW<'_, Pwm1AqctlaSpec> {
        CbuW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Action When TBCTR = PWM1_CMPB on Down Count"]
    #[inline(always)]
    pub fn cbd(&mut self) -> CbdW<'_, Pwm1AqctlaSpec> {
        CbdW::new(self, 10)
    }
}
#[doc = "PWM1 action qualifier A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqctla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqctla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1AqctlaSpec;
impl crate::RegisterSpec for Pwm1AqctlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_aqctla::R`](R) reader structure"]
impl crate::Readable for Pwm1AqctlaSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_aqctla::W`](W) writer structure"]
impl crate::Writable for Pwm1AqctlaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_AQCTLA to value 0"]
impl crate::Resettable for Pwm1AqctlaSpec {}
