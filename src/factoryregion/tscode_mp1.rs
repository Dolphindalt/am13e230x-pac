#[doc = "Register `TSCODE_MP1` reader"]
pub type R = crate::R<TscodeMp1Spec>;
#[doc = "Field `TSCODE_EXTREF_MP1` reader - Temperature sensor value at 30C stored at this location"]
pub type TscodeExtrefMp1R = crate::FieldReader<u16>;
#[doc = "Field `TSCODE_INTREF_MP1` reader - Temperature sensor value at 30C stored at this location"]
pub type TscodeIntrefMp1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature sensor value at 30C stored at this location"]
    #[inline(always)]
    pub fn tscode_extref_mp1(&self) -> TscodeExtrefMp1R {
        TscodeExtrefMp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature sensor value at 30C stored at this location"]
    #[inline(always)]
    pub fn tscode_intref_mp1(&self) -> TscodeIntrefMp1R {
        TscodeIntrefMp1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCODE_MP1")
            .field("tscode_extref_mp1", &self.tscode_extref_mp1())
            .field("tscode_intref_mp1", &self.tscode_intref_mp1())
            .finish()
    }
}
#[doc = "Tempsense Code MP1\n\nYou can [`read`](crate::Reg::read) this register and get [`tscode_mp1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscodeMp1Spec;
impl crate::RegisterSpec for TscodeMp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscode_mp1::R`](R) reader structure"]
impl crate::Readable for TscodeMp1Spec {}
#[doc = "`reset()` method sets TSCODE_MP1 to value 0"]
impl crate::Resettable for TscodeMp1Spec {}
