#[doc = "Register `PWM3_CMPA` reader"]
pub type R = crate::R<Pwm3CmpaSpec>;
#[doc = "Register `PWM3_CMPA` writer"]
pub type W = crate::W<Pwm3CmpaSpec>;
#[doc = "Field `PWM3_CMPA` reader - PWM3 Compare A register"]
pub type Pwm3CmpaR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_CMPA` writer - PWM3 Compare A register"]
pub type Pwm3CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM3 Compare A register"]
    #[inline(always)]
    pub fn pwm3_cmpa(&self) -> Pwm3CmpaR {
        Pwm3CmpaR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM3_CMPA")
            .field("pwm3_cmpa", &self.pwm3_cmpa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM3 Compare A register"]
    #[inline(always)]
    pub fn pwm3_cmpa(&mut self) -> Pwm3CmpaW<'_, Pwm3CmpaSpec> {
        Pwm3CmpaW::new(self, 0)
    }
}
#[doc = "PWM3 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3CmpaSpec;
impl crate::RegisterSpec for Pwm3CmpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_cmpa::R`](R) reader structure"]
impl crate::Readable for Pwm3CmpaSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_cmpa::W`](W) writer structure"]
impl crate::Writable for Pwm3CmpaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_CMPA to value 0"]
impl crate::Resettable for Pwm3CmpaSpec {}
