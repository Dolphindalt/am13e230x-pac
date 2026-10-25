#[doc = "Register `FPC_PRIVATTRIBA1` reader"]
pub type R = crate::R<FpcPrivattriba1Spec>;
#[doc = "Register `FPC_PRIVATTRIBA1` writer"]
pub type W = crate::W<FpcPrivattriba1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Privilege Attribute Register A1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattriba1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattriba1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcPrivattriba1Spec;
impl crate::RegisterSpec for FpcPrivattriba1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_privattriba1::R`](R) reader structure"]
impl crate::Readable for FpcPrivattriba1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_privattriba1::W`](W) writer structure"]
impl crate::Writable for FpcPrivattriba1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_PRIVATTRIBA1 to value 0xffff_ffff"]
impl crate::Resettable for FpcPrivattriba1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
