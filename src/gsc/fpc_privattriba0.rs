#[doc = "Register `FPC_PRIVATTRIBA0` reader"]
pub type R = crate::R<FpcPrivattriba0Spec>;
#[doc = "Register `FPC_PRIVATTRIBA0` writer"]
pub type W = crate::W<FpcPrivattriba0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Privilege Attribute Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattriba0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattriba0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcPrivattriba0Spec;
impl crate::RegisterSpec for FpcPrivattriba0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_privattriba0::R`](R) reader structure"]
impl crate::Readable for FpcPrivattriba0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_privattriba0::W`](W) writer structure"]
impl crate::Writable for FpcPrivattriba0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_PRIVATTRIBA0 to value 0xffff_ffff"]
impl crate::Resettable for FpcPrivattriba0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
