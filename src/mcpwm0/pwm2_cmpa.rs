#[doc = "Register `PWM2_CMPA` reader"]
pub type R = crate::R<Pwm2CmpaSpec>;
#[doc = "Register `PWM2_CMPA` writer"]
pub type W = crate::W<Pwm2CmpaSpec>;
#[doc = "Field `PWM2_CMPA` reader - PWM2 Compare A register"]
pub type Pwm2CmpaR = crate::FieldReader<u16>;
#[doc = "Field `PWM2_CMPA` writer - PWM2 Compare A register"]
pub type Pwm2CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM2 Compare A register"]
    #[inline(always)]
    pub fn pwm2_cmpa(&self) -> Pwm2CmpaR {
        Pwm2CmpaR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2_CMPA")
            .field("pwm2_cmpa", &self.pwm2_cmpa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM2 Compare A register"]
    #[inline(always)]
    pub fn pwm2_cmpa(&mut self) -> Pwm2CmpaW<'_, Pwm2CmpaSpec> {
        Pwm2CmpaW::new(self, 0)
    }
}
#[doc = "PWM2 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2CmpaSpec;
impl crate::RegisterSpec for Pwm2CmpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_cmpa::R`](R) reader structure"]
impl crate::Readable for Pwm2CmpaSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_cmpa::W`](W) writer structure"]
impl crate::Writable for Pwm2CmpaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM2_CMPA to value 0"]
impl crate::Resettable for Pwm2CmpaSpec {}
