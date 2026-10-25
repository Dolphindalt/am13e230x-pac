#[doc = "Register `CMPCTL` reader"]
pub type R = crate::R<CmpctlSpec>;
#[doc = "Register `CMPCTL` writer"]
pub type W = crate::W<CmpctlSpec>;
#[doc = "Field `PWM1_LOADAMODE` reader - Shadow to active load of PWM1_CMPA"]
pub type Pwm1LoadamodeR = crate::FieldReader;
#[doc = "Field `PWM1_LOADAMODE` writer - Shadow to active load of PWM1_CMPA"]
pub type Pwm1LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM1_LOADBMODE` reader - Shadow to active load of PWM1_CMPB"]
pub type Pwm1LoadbmodeR = crate::FieldReader;
#[doc = "Field `PWM1_LOADBMODE` writer - Shadow to active load of PWM1_CMPB"]
pub type Pwm1LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM2_LOADAMODE` reader - Shadow to active load of PWM2_CMPA"]
pub type Pwm2LoadamodeR = crate::FieldReader;
#[doc = "Field `PWM2_LOADAMODE` writer - Shadow to active load of PWM2_CMPA"]
pub type Pwm2LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM2_LOADBMODE` reader - Shadow to active load of PWM2_CMPB"]
pub type Pwm2LoadbmodeR = crate::FieldReader;
#[doc = "Field `PWM2_LOADBMODE` writer - Shadow to active load of PWM2_CMPB"]
pub type Pwm2LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM3_LOADAMODE` reader - Shadow to active load of PWM3_CMPA"]
pub type Pwm3LoadamodeR = crate::FieldReader;
#[doc = "Field `PWM3_LOADAMODE` writer - Shadow to active load of PWM3_CMPA"]
pub type Pwm3LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PWM3_LOADBMODE` reader - Shadow to active load of PWM3_CMPB"]
pub type Pwm3LoadbmodeR = crate::FieldReader;
#[doc = "Field `PWM3_LOADBMODE` writer - Shadow to active load of PWM3_CMPB"]
pub type Pwm3LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `LOADCMODE` reader - Shadow to active load of CMPC"]
pub type LoadcmodeR = crate::FieldReader;
#[doc = "Field `LOADCMODE` writer - Shadow to active load of CMPC"]
pub type LoadcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `LOADDMODE` reader - Shadow to active load of CMPD"]
pub type LoaddmodeR = crate::FieldReader;
#[doc = "Field `LOADDMODE` writer - Shadow to active load of CMPD"]
pub type LoaddmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Shadow to active load of PWM1_CMPA"]
    #[inline(always)]
    pub fn pwm1_loadamode(&self) -> Pwm1LoadamodeR {
        Pwm1LoadamodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Shadow to active load of PWM1_CMPB"]
    #[inline(always)]
    pub fn pwm1_loadbmode(&self) -> Pwm1LoadbmodeR {
        Pwm1LoadbmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Shadow to active load of PWM2_CMPA"]
    #[inline(always)]
    pub fn pwm2_loadamode(&self) -> Pwm2LoadamodeR {
        Pwm2LoadamodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Shadow to active load of PWM2_CMPB"]
    #[inline(always)]
    pub fn pwm2_loadbmode(&self) -> Pwm2LoadbmodeR {
        Pwm2LoadbmodeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Shadow to active load of PWM3_CMPA"]
    #[inline(always)]
    pub fn pwm3_loadamode(&self) -> Pwm3LoadamodeR {
        Pwm3LoadamodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Shadow to active load of PWM3_CMPB"]
    #[inline(always)]
    pub fn pwm3_loadbmode(&self) -> Pwm3LoadbmodeR {
        Pwm3LoadbmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Shadow to active load of CMPC"]
    #[inline(always)]
    pub fn loadcmode(&self) -> LoadcmodeR {
        LoadcmodeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Shadow to active load of CMPD"]
    #[inline(always)]
    pub fn loaddmode(&self) -> LoaddmodeR {
        LoaddmodeR::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPCTL")
            .field("pwm1_loadamode", &self.pwm1_loadamode())
            .field("pwm1_loadbmode", &self.pwm1_loadbmode())
            .field("pwm2_loadamode", &self.pwm2_loadamode())
            .field("pwm2_loadbmode", &self.pwm2_loadbmode())
            .field("pwm3_loadamode", &self.pwm3_loadamode())
            .field("pwm3_loadbmode", &self.pwm3_loadbmode())
            .field("loadcmode", &self.loadcmode())
            .field("loaddmode", &self.loaddmode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow to active load of PWM1_CMPA"]
    #[inline(always)]
    pub fn pwm1_loadamode(&mut self) -> Pwm1LoadamodeW<'_, CmpctlSpec> {
        Pwm1LoadamodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Shadow to active load of PWM1_CMPB"]
    #[inline(always)]
    pub fn pwm1_loadbmode(&mut self) -> Pwm1LoadbmodeW<'_, CmpctlSpec> {
        Pwm1LoadbmodeW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Shadow to active load of PWM2_CMPA"]
    #[inline(always)]
    pub fn pwm2_loadamode(&mut self) -> Pwm2LoadamodeW<'_, CmpctlSpec> {
        Pwm2LoadamodeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Shadow to active load of PWM2_CMPB"]
    #[inline(always)]
    pub fn pwm2_loadbmode(&mut self) -> Pwm2LoadbmodeW<'_, CmpctlSpec> {
        Pwm2LoadbmodeW::new(self, 10)
    }
    #[doc = "Bits 16:17 - Shadow to active load of PWM3_CMPA"]
    #[inline(always)]
    pub fn pwm3_loadamode(&mut self) -> Pwm3LoadamodeW<'_, CmpctlSpec> {
        Pwm3LoadamodeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Shadow to active load of PWM3_CMPB"]
    #[inline(always)]
    pub fn pwm3_loadbmode(&mut self) -> Pwm3LoadbmodeW<'_, CmpctlSpec> {
        Pwm3LoadbmodeW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Shadow to active load of CMPC"]
    #[inline(always)]
    pub fn loadcmode(&mut self) -> LoadcmodeW<'_, CmpctlSpec> {
        LoadcmodeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Shadow to active load of CMPD"]
    #[inline(always)]
    pub fn loaddmode(&mut self) -> LoaddmodeW<'_, CmpctlSpec> {
        LoaddmodeW::new(self, 26)
    }
}
#[doc = "Counter compare control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpctlSpec;
impl crate::RegisterSpec for CmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctl::R`](R) reader structure"]
impl crate::Readable for CmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure"]
impl crate::Writable for CmpctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CmpctlSpec {}
