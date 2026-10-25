#[doc = "Register `AQCTL` reader"]
pub type R = crate::R<AqctlSpec>;
#[doc = "Register `AQCTL` writer"]
pub type W = crate::W<AqctlSpec>;
#[doc = "Field `PWM1_LDAQAMODE` reader - Shadow to active load of PWM1_AQCTLA"]
pub type Pwm1LdaqamodeR = crate::FieldReader;
#[doc = "Field `PWM1_LDAQAMODE` writer - Shadow to active load of PWM1_AQCTLA"]
pub type Pwm1LdaqamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM1_LDAQBMODE` reader - Shadow to active load of PWM1_AQCTLB"]
pub type Pwm1LdaqbmodeR = crate::FieldReader;
#[doc = "Field `PWM1_LDAQBMODE` writer - Shadow to active load of PWM1_AQCTLB"]
pub type Pwm1LdaqbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM2_LDAQAMODE` reader - Shadow to active load of PWM2_AQCTLA"]
pub type Pwm2LdaqamodeR = crate::FieldReader;
#[doc = "Field `PWM2_LDAQAMODE` writer - Shadow to active load of PWM2_AQCTLA"]
pub type Pwm2LdaqamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM2_LDAQBMODE` reader - Shadow to active load of PWM2_AQCTLB"]
pub type Pwm2LdaqbmodeR = crate::FieldReader;
#[doc = "Field `PWM2_LDAQBMODE` writer - Shadow to active load of PWM2_AQCTLB"]
pub type Pwm2LdaqbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM3_LDAQAMODE` reader - Shadow to active load of PWM3_AQCTLA"]
pub type Pwm3LdaqamodeR = crate::FieldReader;
#[doc = "Field `PWM3_LDAQAMODE` writer - Shadow to active load of PWM3_AQCTLA"]
pub type Pwm3LdaqamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM3_LDAQBMODE` reader - Shadow to active load of PWM3_AQCTLB"]
pub type Pwm3LdaqbmodeR = crate::FieldReader;
#[doc = "Field `PWM3_LDAQBMODE` writer - Shadow to active load of PWM3_AQCTLB"]
pub type Pwm3LdaqbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Shadow to active load of PWM1_AQCTLA"]
    #[inline(always)]
    pub fn pwm1_ldaqamode(&self) -> Pwm1LdaqamodeR {
        Pwm1LdaqamodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Shadow to active load of PWM1_AQCTLB"]
    #[inline(always)]
    pub fn pwm1_ldaqbmode(&self) -> Pwm1LdaqbmodeR {
        Pwm1LdaqbmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Shadow to active load of PWM2_AQCTLA"]
    #[inline(always)]
    pub fn pwm2_ldaqamode(&self) -> Pwm2LdaqamodeR {
        Pwm2LdaqamodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Shadow to active load of PWM2_AQCTLB"]
    #[inline(always)]
    pub fn pwm2_ldaqbmode(&self) -> Pwm2LdaqbmodeR {
        Pwm2LdaqbmodeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Shadow to active load of PWM3_AQCTLA"]
    #[inline(always)]
    pub fn pwm3_ldaqamode(&self) -> Pwm3LdaqamodeR {
        Pwm3LdaqamodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Shadow to active load of PWM3_AQCTLB"]
    #[inline(always)]
    pub fn pwm3_ldaqbmode(&self) -> Pwm3LdaqbmodeR {
        Pwm3LdaqbmodeR::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AQCTL")
            .field("pwm1_ldaqamode", &self.pwm1_ldaqamode())
            .field("pwm1_ldaqbmode", &self.pwm1_ldaqbmode())
            .field("pwm2_ldaqamode", &self.pwm2_ldaqamode())
            .field("pwm2_ldaqbmode", &self.pwm2_ldaqbmode())
            .field("pwm3_ldaqamode", &self.pwm3_ldaqamode())
            .field("pwm3_ldaqbmode", &self.pwm3_ldaqbmode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow to active load of PWM1_AQCTLA"]
    #[inline(always)]
    pub fn pwm1_ldaqamode(&mut self) -> Pwm1LdaqamodeW<'_, AqctlSpec> {
        Pwm1LdaqamodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Shadow to active load of PWM1_AQCTLB"]
    #[inline(always)]
    pub fn pwm1_ldaqbmode(&mut self) -> Pwm1LdaqbmodeW<'_, AqctlSpec> {
        Pwm1LdaqbmodeW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Shadow to active load of PWM2_AQCTLA"]
    #[inline(always)]
    pub fn pwm2_ldaqamode(&mut self) -> Pwm2LdaqamodeW<'_, AqctlSpec> {
        Pwm2LdaqamodeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Shadow to active load of PWM2_AQCTLB"]
    #[inline(always)]
    pub fn pwm2_ldaqbmode(&mut self) -> Pwm2LdaqbmodeW<'_, AqctlSpec> {
        Pwm2LdaqbmodeW::new(self, 10)
    }
    #[doc = "Bits 16:17 - Shadow to active load of PWM3_AQCTLA"]
    #[inline(always)]
    pub fn pwm3_ldaqamode(&mut self) -> Pwm3LdaqamodeW<'_, AqctlSpec> {
        Pwm3LdaqamodeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Shadow to active load of PWM3_AQCTLB"]
    #[inline(always)]
    pub fn pwm3_ldaqbmode(&mut self) -> Pwm3LdaqbmodeW<'_, AqctlSpec> {
        Pwm3LdaqbmodeW::new(self, 18)
    }
}
#[doc = "Action qualifier control register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqctlSpec;
impl crate::RegisterSpec for AqctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqctl::R`](R) reader structure"]
impl crate::Readable for AqctlSpec {}
#[doc = "`write(|w| ..)` method takes [`aqctl::W`](W) writer structure"]
impl crate::Writable for AqctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AQCTL to value 0"]
impl crate::Resettable for AqctlSpec {}
