#[doc = "Register `SYSMEM_ACC_TYPE` reader"]
pub type R = crate::R<SysmemAccTypeSpec>;
#[doc = "Register `SYSMEM_ACC_TYPE` writer"]
pub type W = crate::W<SysmemAccTypeSpec>;
#[doc = "Field `TYPE` reader - Access error type. This field will be cleared along with the corresponding flag clear. \\[4\\] - CBUS Access Error \\[3\\] - SBUS Access Error \\[2\\] - Write Error \\[1\\] - Read Error \\[0\\] - Fetch Error"]
pub type TypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Access error type. This field will be cleared along with the corresponding flag clear. \\[4\\] - CBUS Access Error \\[3\\] - SBUS Access Error \\[2\\] - Write Error \\[1\\] - Read Error \\[0\\] - Fetch Error"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_ACC_TYPE")
            .field("type_", &self.type_())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM access error type\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_acc_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_acc_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemAccTypeSpec;
impl crate::RegisterSpec for SysmemAccTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_acc_type::R`](R) reader structure"]
impl crate::Readable for SysmemAccTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_acc_type::W`](W) writer structure"]
impl crate::Writable for SysmemAccTypeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_ACC_TYPE to value 0"]
impl crate::Resettable for SysmemAccTypeSpec {}
