#[doc = "Register `PWM3_CMPB` reader"]
pub type R = crate::R<Pwm3CmpbSpec>;
#[doc = "Register `PWM3_CMPB` writer"]
pub type W = crate::W<Pwm3CmpbSpec>;
#[doc = "Field `PWM3_CMPB` reader - PWM3 Compare B register"]
pub type Pwm3CmpbR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_CMPB` writer - PWM3 Compare B register"]
pub type Pwm3CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM3 Compare B register"]
    #[inline(always)]
    pub fn pwm3_cmpb(&self) -> Pwm3CmpbR {
        Pwm3CmpbR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM3_CMPB")
            .field("pwm3_cmpb", &self.pwm3_cmpb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM3 Compare B register"]
    #[inline(always)]
    pub fn pwm3_cmpb(&mut self) -> Pwm3CmpbW<'_, Pwm3CmpbSpec> {
        Pwm3CmpbW::new(self, 0)
    }
}
#[doc = "PWM3 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3CmpbSpec;
impl crate::RegisterSpec for Pwm3CmpbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_cmpb::R`](R) reader structure"]
impl crate::Readable for Pwm3CmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_cmpb::W`](W) writer structure"]
impl crate::Writable for Pwm3CmpbSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_CMPB to value 0"]
impl crate::Resettable for Pwm3CmpbSpec {}
