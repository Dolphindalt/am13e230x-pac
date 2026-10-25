#[doc = "Register `VTOR_NS` reader"]
pub type R = crate::R<VtorNsSpec>;
#[doc = "Register `VTOR_NS` writer"]
pub type W = crate::W<VtorNsSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Non - Secure Vector Table Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vtor_ns::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor_ns::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VtorNsSpec;
impl crate::RegisterSpec for VtorNsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtor_ns::R`](R) reader structure"]
impl crate::Readable for VtorNsSpec {}
#[doc = "`write(|w| ..)` method takes [`vtor_ns::W`](W) writer structure"]
impl crate::Writable for VtorNsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VTOR_NS to value 0x0100_0001"]
impl crate::Resettable for VtorNsSpec {
    const RESET_VALUE: u32 = 0x0100_0001;
}
