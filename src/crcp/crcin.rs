#[doc = "Register `CRCIN` writer"]
pub type W = crate::W<CrcinSpec>;
#[doc = "Field `DATA` writer - Input Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CrcinSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, CrcinSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcinSpec;
impl crate::RegisterSpec for CrcinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcin::W`](W) writer structure"]
impl crate::Writable for CrcinSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCIN to value 0"]
impl crate::Resettable for CrcinSpec {}
