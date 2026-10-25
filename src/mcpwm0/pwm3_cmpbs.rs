#[doc = "Register `PWM3_CMPBS` reader"]
pub type R = crate::R<Pwm3CmpbsSpec>;
#[doc = "Register `PWM3_CMPBS` writer"]
pub type W = crate::W<Pwm3CmpbsSpec>;
#[doc = "Field `PWM3_CMPBS` reader - PWM3 Compare B shadow register"]
pub type Pwm3CmpbsR = crate::FieldReader<u16>;
#[doc = "Field `PWM3_CMPBS` writer - PWM3 Compare B shadow register"]
pub type Pwm3CmpbsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - PWM3 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm3_cmpbs(&self) -> Pwm3CmpbsR {
        Pwm3CmpbsR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM3_CMPBS")
            .field("pwm3_cmpbs", &self.pwm3_cmpbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM3 Compare B shadow register"]
    #[inline(always)]
    pub fn pwm3_cmpbs(&mut self) -> Pwm3CmpbsW<'_, Pwm3CmpbsSpec> {
        Pwm3CmpbsW::new(self, 0)
    }
}
#[doc = "PWM3 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3CmpbsSpec;
impl crate::RegisterSpec for Pwm3CmpbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_cmpbs::R`](R) reader structure"]
impl crate::Readable for Pwm3CmpbsSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_cmpbs::W`](W) writer structure"]
impl crate::Writable for Pwm3CmpbsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_CMPBS to value 0"]
impl crate::Resettable for Pwm3CmpbsSpec {}
