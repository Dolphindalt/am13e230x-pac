#[doc = "Register `PWM1_CMPA` reader"]
pub type R = crate::R<Pwm1CmpaSpec>;
#[doc = "Register `PWM1_CMPA` writer"]
pub type W = crate::W<Pwm1CmpaSpec>;
#[doc = "Field `PWM1_CMPA` reader - PWM1 Compare A register"]
pub type Pwm1CmpaR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_CMPA` writer - PWM1 Compare A register"]
pub type Pwm1CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM1 Compare A register"]
    #[inline(always)]
    pub fn pwm1_cmpa(&self) -> Pwm1CmpaR {
        Pwm1CmpaR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_CMPA")
            .field("pwm1_cmpa", &self.pwm1_cmpa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM1 Compare A register"]
    #[inline(always)]
    pub fn pwm1_cmpa(&mut self) -> Pwm1CmpaW<'_, Pwm1CmpaSpec> {
        Pwm1CmpaW::new(self, 0)
    }
}
#[doc = "PWM1 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1CmpaSpec;
impl crate::RegisterSpec for Pwm1CmpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_cmpa::R`](R) reader structure"]
impl crate::Readable for Pwm1CmpaSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_cmpa::W`](W) writer structure"]
impl crate::Writable for Pwm1CmpaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_CMPA to value 0"]
impl crate::Resettable for Pwm1CmpaSpec {}
