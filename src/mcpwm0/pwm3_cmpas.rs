#[doc = "Register `PWM3_CMPAS` reader"]
pub type R = crate::R<Pwm3CmpasSpec>;
#[doc = "Register `PWM3_CMPAS` writer"]
pub type W = crate::W<Pwm3CmpasSpec>;
#[doc = "Field `PWM3_CMPAS` reader - PWM3 Compare A shadow register"]
pub type Pwm3CmpasR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_CMPAS` writer - PWM3 Compare A shadow register"]
pub type Pwm3CmpasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM3 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm3_cmpas(&self) -> Pwm3CmpasR {
        Pwm3CmpasR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM3_CMPAS")
            .field("pwm3_cmpas", &self.pwm3_cmpas())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM3 Compare A shadow register"]
    #[inline(always)]
    pub fn pwm3_cmpas(&mut self) -> Pwm3CmpasW<'_, Pwm3CmpasSpec> {
        Pwm3CmpasW::new(self, 0)
    }
}
#[doc = "PWM3 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3CmpasSpec;
impl crate::RegisterSpec for Pwm3CmpasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_cmpas::R`](R) reader structure"]
impl crate::Readable for Pwm3CmpasSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_cmpas::W`](W) writer structure"]
impl crate::Writable for Pwm3CmpasSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_CMPAS to value 0"]
impl crate::Resettable for Pwm3CmpasSpec {}
