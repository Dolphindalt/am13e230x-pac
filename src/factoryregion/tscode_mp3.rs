#[doc = "Register `TSCODE_MP3` reader"]
pub type R = crate::R<TscodeMp3Spec>;
#[doc = "Field `TSCODE_EXTREF_MP3` reader - Temperature sensor value at 125C stored at this location"]
pub type TscodeExtrefMp3R = crate::FieldReader<u16>;
#[doc = "Field `TSCODE_INTREF_MP3` reader - Temperature sensor value at 125C stored at this location"]
pub type TscodeIntrefMp3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature sensor value at 125C stored at this location"]
    #[inline(always)]
    pub fn tscode_extref_mp3(&self) -> TscodeExtrefMp3R {
        TscodeExtrefMp3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature sensor value at 125C stored at this location"]
    #[inline(always)]
    pub fn tscode_intref_mp3(&self) -> TscodeIntrefMp3R {
        TscodeIntrefMp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCODE_MP3")
            .field("tscode_extref_mp3", &self.tscode_extref_mp3())
            .field("tscode_intref_mp3", &self.tscode_intref_mp3())
            .finish()
    }
}
#[doc = "Tempsense code MP3\n\nYou can [`read`](crate::Reg::read) this register and get [`tscode_mp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscodeMp3Spec;
impl crate::RegisterSpec for TscodeMp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscode_mp3::R`](R) reader structure"]
impl crate::Readable for TscodeMp3Spec {}
#[doc = "`reset()` method sets TSCODE_MP3 to value 0"]
impl crate::Resettable for TscodeMp3Spec {}
