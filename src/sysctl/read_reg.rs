#[doc = "Register `READ_REG` reader"]
pub type R = crate::R<ReadRegSpec>;
#[doc = "Register `READ_REG` writer"]
pub type W = crate::W<ReadRegSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sysctl read only Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`read_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadRegSpec;
impl crate::RegisterSpec for ReadRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_reg::R`](R) reader structure"]
impl crate::Readable for ReadRegSpec {}
#[doc = "`write(|w| ..)` method takes [`read_reg::W`](W) writer structure"]
impl crate::Writable for ReadRegSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets READ_REG to value 0"]
impl crate::Resettable for ReadRegSpec {}
