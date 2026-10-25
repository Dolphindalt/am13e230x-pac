#[doc = "Register `CPU_CONNECT_0` reader"]
pub type R = crate::R<CpuConnect0Spec>;
#[doc = "Register `CPU_CONNECT_0` writer"]
pub type W = crate::W<CpuConnect0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CPU Connect\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_connect_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_connect_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuConnect0Spec;
impl crate::RegisterSpec for CpuConnect0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_connect_0::R`](R) reader structure"]
impl crate::Readable for CpuConnect0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_connect_0::W`](W) writer structure"]
impl crate::Writable for CpuConnect0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CPU_CONNECT_0 to value 0"]
impl crate::Resettable for CpuConnect0Spec {}
