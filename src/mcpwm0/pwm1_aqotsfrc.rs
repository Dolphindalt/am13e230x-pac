#[doc = "Register `PWM1_AQOTSFRC` reader"]
pub type R = crate::R<Pwm1AqotsfrcSpec>;
#[doc = "Register `PWM1_AQOTSFRC` writer"]
pub type W = crate::W<Pwm1AqotsfrcSpec>;
#[doc = "Field `PWMA` reader - Action qualifier one time software force on PWMA"]
pub type PwmaR = crate::BitReader;
#[doc = "Field `PWMA` writer - Action qualifier one time software force on PWMA"]
pub type PwmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMB` reader - Action qualifier software force on PWMB"]
pub type PwmbR = crate::BitReader;
#[doc = "Field `PWMB` writer - Action qualifier software force on PWMB"]
pub type PwmbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Action qualifier one time software force on PWMA"]
    #[inline(always)]
    pub fn pwma(&self) -> PwmaR {
        PwmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Action qualifier software force on PWMB"]
    #[inline(always)]
    pub fn pwmb(&self) -> PwmbR {
        PwmbR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_AQOTSFRC")
            .field("pwma", &self.pwma())
            .field("pwmb", &self.pwmb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Action qualifier one time software force on PWMA"]
    #[inline(always)]
    pub fn pwma(&mut self) -> PwmaW<'_, Pwm1AqotsfrcSpec> {
        PwmaW::new(self, 0)
    }
    #[doc = "Bit 4 - Action qualifier software force on PWMB"]
    #[inline(always)]
    pub fn pwmb(&mut self) -> PwmbW<'_, Pwm1AqotsfrcSpec> {
        PwmbW::new(self, 4)
    }
}
#[doc = "PWM1 action qualifier one time software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqotsfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqotsfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1AqotsfrcSpec;
impl crate::RegisterSpec for Pwm1AqotsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_aqotsfrc::R`](R) reader structure"]
impl crate::Readable for Pwm1AqotsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1_aqotsfrc::W`](W) writer structure"]
impl crate::Writable for Pwm1AqotsfrcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM1_AQOTSFRC to value 0"]
impl crate::Resettable for Pwm1AqotsfrcSpec {}
