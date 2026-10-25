#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Register `REVISION` writer"]
pub type W = crate::W<RevisionSpec>;
#[doc = "Field `PWM2_PRESENT` reader - PWM2 Present"]
pub type Pwm2PresentR = crate::BitReader;
#[doc = "Field `PWM3_PRESENT` reader - PWM3 Present"]
pub type Pwm3PresentR = crate::BitReader;
#[doc = "Field `CMPCD_PRESENT` reader - Compare C and D Present"]
pub type CmpcdPresentR = crate::BitReader;
#[doc = "Field `CAP_PRESENT` reader - Capture mode Present"]
pub type CapPresentR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PWM2 Present"]
    #[inline(always)]
    pub fn pwm2_present(&self) -> Pwm2PresentR {
        Pwm2PresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM3 Present"]
    #[inline(always)]
    pub fn pwm3_present(&self) -> Pwm3PresentR {
        Pwm3PresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare C and D Present"]
    #[inline(always)]
    pub fn cmpcd_present(&self) -> CmpcdPresentR {
        CmpcdPresentR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture mode Present"]
    #[inline(always)]
    pub fn cap_present(&self) -> CapPresentR {
        CapPresentR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REVISION")
            .field("pwm2_present", &self.pwm2_present())
            .field("pwm3_present", &self.pwm3_present())
            .field("cmpcd_present", &self.cmpcd_present())
            .field("cap_present", &self.cap_present())
            .finish()
    }
}
impl W {}
#[doc = "IP revision id register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`revision::W`](W) writer structure"]
impl crate::Writable for RevisionSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets REVISION to value 0x07"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u32 = 0x07;
}
