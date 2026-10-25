#[doc = "Register `PWM1_CMPAS` reader"]
pub type R = crate::R<Pwm1CmpasSpec>;
#[doc = "Register `PWM1_CMPAS` writer"]
pub type W = crate::W<Pwm1CmpasSpec>;
#[doc = "Field `PWM1_CMPAS` reader - PWM1 Compare A shadow register"]
pub type Pwm1CmpasR = crate::FieldReader<u16>;
#[doc = "Field `PWM1_CMPAS` writer - PWM1 Compare A shadow register"]
pub type Pwm1CmpasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM1 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm1_cmpas(&self) -> Pwm1CmpasR {
        Pwm1CmpasR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_CMPAS")
            .field("pwm1_cmpas", &self.pwm1_cmpas())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM1 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm1_cmpas(&mut self) -> Pwm1CmpasW<'_, Pwm1CmpasSpec> {
        Pwm1CmpasW::new(self, 0)
    }
}
#[doc = "PWM1 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1CmpasSpec;
impl crate::RegisterSpec for Pwm1CmpasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_cmpas::R`](R) reader structure"]
impl crate::Readable for Pwm1CmpasSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_cmpas::W`](W) writer structure"]
impl crate::Writable for Pwm1CmpasSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_CMPAS to value 0"]
impl crate::Resettable for Pwm1CmpasSpec {}
