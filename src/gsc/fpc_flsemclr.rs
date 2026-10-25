#[doc = "Register `FPC_FLSEMCLR` reader"]
pub type R = crate::R<FpcFlsemclrSpec>;
#[doc = "Register `FPC_FLSEMCLR` writer"]
pub type W = crate::W<FpcFlsemclrSpec>;
#[doc = "Field `CLR` reader - Release Semaphore Ownership"]
pub type ClrR = crate::BitReader;
#[doc = "Field `CLR` writer - Release Semaphore Ownership"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Release Semaphore Ownership"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_FLSEMCLR")
            .field("clr", &self.clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Release Semaphore Ownership"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, FpcFlsemclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Flash semaphore release register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_flsemclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_flsemclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcFlsemclrSpec;
impl crate::RegisterSpec for FpcFlsemclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_flsemclr::R`](R) reader structure"]
impl crate::Readable for FpcFlsemclrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_flsemclr::W`](W) writer structure"]
impl crate::Writable for FpcFlsemclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_FLSEMCLR to value 0"]
impl crate::Resettable for FpcFlsemclrSpec {}
