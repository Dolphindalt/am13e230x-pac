#[doc = "Register `FPC_WEPROTA0` reader"]
pub type R = crate::R<FpcWeprota0Spec>;
#[doc = "Register `FPC_WEPROTA0` writer"]
pub type W = crate::W<FpcWeprota0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Write Protect Attribute Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprota0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprota0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcWeprota0Spec;
impl crate::RegisterSpec for FpcWeprota0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_weprota0::R`](R) reader structure"]
impl crate::Readable for FpcWeprota0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_weprota0::W`](W) writer structure"]
impl crate::Writable for FpcWeprota0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_WEPROTA0 to value 0"]
impl crate::Resettable for FpcWeprota0Spec {}
