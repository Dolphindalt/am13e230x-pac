#[doc = "Register `TSTEMP` reader"]
pub type R = crate::R<TstempSpec>;
#[doc = "Field `TSTEMP_MP1` reader - Test temperature (30C) is recorded here"]
pub type TstempMp1R = crate::FieldReader<u16>;
#[doc = "Field `TSTEMP_MP3` reader - Test temperature (125C) is recorded here"]
pub type TstempMp3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Test temperature (30C) is recorded here"]
    #[inline(always)]
    pub fn tstemp_mp1(&self) -> TstempMp1R {
        TstempMp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Test temperature (125C) is recorded here"]
    #[inline(always)]
    pub fn tstemp_mp3(&self) -> TstempMp3R {
        TstempMp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTEMP")
            .field("tstemp_mp1", &self.tstemp_mp1())
            .field("tstemp_mp3", &self.tstemp_mp3())
            .finish()
    }
}
#[doc = "Temperature sense TEMP\n\nYou can [`read`](crate::Reg::read) this register and get [`tstemp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstempSpec;
impl crate::RegisterSpec for TstempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstemp::R`](R) reader structure"]
impl crate::Readable for TstempSpec {}
#[doc = "`reset()` method sets TSTEMP to value 0"]
impl crate::Resettable for TstempSpec {}
