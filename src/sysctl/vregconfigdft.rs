#[doc = "Register `VREGCONFIGDFT` reader"]
pub type R = crate::R<VregconfigdftSpec>;
#[doc = "Register `VREGCONFIGDFT` writer"]
pub type W = crate::W<VregconfigdftSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "VREG Configuration DFT Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vregconfigdft::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregconfigdft::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregconfigdftSpec;
impl crate::RegisterSpec for VregconfigdftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vregconfigdft::R`](R) reader structure"]
impl crate::Readable for VregconfigdftSpec {}
#[doc = "`write(|w| ..)` method takes [`vregconfigdft::W`](W) writer structure"]
impl crate::Writable for VregconfigdftSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets VREGCONFIGDFT to value 0"]
impl crate::Resettable for VregconfigdftSpec {}
