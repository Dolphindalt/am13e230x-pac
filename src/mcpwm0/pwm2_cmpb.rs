#[doc = "Register `PWM2_CMPB` reader"]
pub type R = crate::R<Pwm2CmpbSpec>;
#[doc = "Register `PWM2_CMPB` writer"]
pub type W = crate::W<Pwm2CmpbSpec>;
#[doc = "Field `PWM2_CMPB` reader - PWM2 Compare B register"]
pub type Pwm2CmpbR = crate::FieldReader<u16>;
#[doc = "Field `PWM2_CMPB` writer - PWM2 Compare B register"]
pub type Pwm2CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM2 Compare B register"]
    #[inline(always)]
    pub fn pwm2_cmpb(&self) -> Pwm2CmpbR {
        Pwm2CmpbR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2_CMPB")
            .field("pwm2_cmpb", &self.pwm2_cmpb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM2 Compare B register"]
    #[inline(always)]
    pub fn pwm2_cmpb(&mut self) -> Pwm2CmpbW<'_, Pwm2CmpbSpec> {
        Pwm2CmpbW::new(self, 0)
    }
}
#[doc = "PWM2 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2CmpbSpec;
impl crate::RegisterSpec for Pwm2CmpbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_cmpb::R`](R) reader structure"]
impl crate::Readable for Pwm2CmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2_cmpb::W`](W) writer structure"]
impl crate::Writable for Pwm2CmpbSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM2_CMPB to value 0"]
impl crate::Resettable for Pwm2CmpbSpec {}
