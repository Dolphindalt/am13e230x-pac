#[doc = "Register `SYSMEM_ACC_FLAG` reader"]
pub type R = crate::R<SysmemAccFlagSpec>;
#[doc = "Register `SYSMEM_ACC_FLAG` writer"]
pub type W = crate::W<SysmemAccFlagSpec>;
#[doc = "Field `ERR` reader - "]
pub type ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_ACC_FLAG")
            .field("err", &self.err())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM access error flag\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemAccFlagSpec;
impl crate::RegisterSpec for SysmemAccFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_acc_flag::R`](R) reader structure"]
impl crate::Readable for SysmemAccFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_acc_flag::W`](W) writer structure"]
impl crate::Writable for SysmemAccFlagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_ACC_FLAG to value 0"]
impl crate::Resettable for SysmemAccFlagSpec {}
