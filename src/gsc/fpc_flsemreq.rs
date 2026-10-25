#[doc = "Register `FPC_FLSEMREQ` reader"]
pub type R = crate::R<FpcFlsemreqSpec>;
#[doc = "Register `FPC_FLSEMREQ` writer"]
pub type W = crate::W<FpcFlsemreqSpec>;
#[doc = "Field `REQ` reader - Request Semaphore Ownership"]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - Request Semaphore Ownership"]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Request Semaphore Ownership"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_FLSEMREQ")
            .field("req", &self.req())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Request Semaphore Ownership"]
    #[inline(always)]
    pub fn req(&mut self) -> ReqW<'_, FpcFlsemreqSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Flash semaphore request register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcFlsemreqSpec;
impl crate::RegisterSpec for FpcFlsemreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_flsemreq::R`](R) reader structure"]
impl crate::Readable for FpcFlsemreqSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_flsemreq::W`](W) writer structure"]
impl crate::Writable for FpcFlsemreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_FLSEMREQ to value 0"]
impl crate::Resettable for FpcFlsemreqSpec {}
