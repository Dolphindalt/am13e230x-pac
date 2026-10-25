#[doc = "Register `I2VCTL` reader"]
pub type R = crate::R<I2vctlSpec>;
#[doc = "Register `I2VCTL` writer"]
pub type W = crate::W<I2vctlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "I2V Logic Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2vctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2vctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2vctlSpec;
impl crate::RegisterSpec for I2vctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2vctl::R`](R) reader structure"]
impl crate::Readable for I2vctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2vctl::W`](W) writer structure"]
impl crate::Writable for I2vctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets I2VCTL to value 0"]
impl crate::Resettable for I2vctlSpec {}
