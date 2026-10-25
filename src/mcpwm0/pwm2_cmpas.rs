#[doc = "Register `PWM2_CMPAS` reader"]
pub type R = crate::R<Pwm2CmpasSpec>;
#[doc = "Register `PWM2_CMPAS` writer"]
pub type W = crate::W<Pwm2CmpasSpec>;
#[doc = "Field `PWM2_CMPAS` reader - PWM2 Compare A shadow register"]
pub type Pwm2CmpasR = crate::FieldReader<u16>;
#[doc = "Field `PWM2_CMPAS` writer - PWM2 Compare A shadow register"]
pub type Pwm2CmpasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM2 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm2_cmpas(&self) -> Pwm2CmpasR {
        Pwm2CmpasR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2_CMPAS")
            .field("pwm2_cmpas", &self.pwm2_cmpas())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM2 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm2_cmpas(&mut self) -> Pwm2CmpasW<'_, Pwm2CmpasSpec> {
        Pwm2CmpasW::new(self, 0)
    }
}
#[doc = "PWM2 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2CmpasSpec;
impl crate::RegisterSpec for Pwm2CmpasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_cmpas::R`](R) reader structure"]
impl crate::Readable for Pwm2CmpasSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_cmpas::W`](W) writer structure"]
impl crate::Writable for Pwm2CmpasSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM2_CMPAS to value 0"]
impl crate::Resettable for Pwm2CmpasSpec {}
