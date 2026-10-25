#[doc = "Register `PWM3_AQOTSFRC` reader"]
pub type R = crate::R<Pwm3AqotsfrcSpec>;
#[doc = "Register `PWM3_AQOTSFRC` writer"]
pub type W = crate::W<Pwm3AqotsfrcSpec>;
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
        f.debug_struct("PWM3_AQOTSFRC")
            .field("pwma", &self.pwma())
            .field("pwmb", &self.pwmb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Action qualifier one time software force on PWMA"]
    #[inline(always)]
    pub fn pwma(&mut self) -> PwmaW<'_, Pwm3AqotsfrcSpec> {
        PwmaW::new(self, 0)
    }
    #[doc = "Bit 4 - Action qualifier software force on PWMB"]
    #[inline(always)]
    pub fn pwmb(&mut self) -> PwmbW<'_, Pwm3AqotsfrcSpec> {
        PwmbW::new(self, 4)
    }
}
#[doc = "PWM3 action qualifier one time software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqotsfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqotsfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm3AqotsfrcSpec;
impl crate::RegisterSpec for Pwm3AqotsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm3_aqotsfrc::R`](R) reader structure"]
impl crate::Readable for Pwm3AqotsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm3_aqotsfrc::W`](W) writer structure"]
impl crate::Writable for Pwm3AqotsfrcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWM3_AQOTSFRC to value 0"]
impl crate::Resettable for Pwm3AqotsfrcSpec {}
