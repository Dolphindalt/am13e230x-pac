#[doc = "Register `SYSMEM_ACC_CLR` reader"]
pub type R = crate::R<SysmemAccClrSpec>;
#[doc = "Register `SYSMEM_ACC_CLR` writer"]
pub type W = crate::W<SysmemAccClrSpec>;
#[doc = "Field `ERR` reader - writing '1' will clear SYSMEM_ACC_FLAG\\[ERR\\] register."]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - writing '1' will clear SYSMEM_ACC_FLAG\\[ERR\\] register."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - writing '1' will clear SYSMEM_ACC_FLAG\\[ERR\\] register."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_ACC_CLR")
            .field("err", &self.err())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - writing '1' will clear SYSMEM_ACC_FLAG\\[ERR\\] register."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, SysmemAccClrSpec> {
        ErrW::new(self, 0)
    }
}
#[doc = "SYSMEM access error clear\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemAccClrSpec;
impl crate::RegisterSpec for SysmemAccClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_acc_clr::R`](R) reader structure"]
impl crate::Readable for SysmemAccClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_acc_clr::W`](W) writer structure"]
impl crate::Writable for SysmemAccClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_ACC_CLR to value 0"]
impl crate::Resettable for SysmemAccClrSpec {}
