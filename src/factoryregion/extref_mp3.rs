#[doc = "Register `EXTREF_MP3` reader"]
pub type R = crate::R<ExtrefMp3Spec>;
#[doc = "Field `TSGAIN_EXTREF_MP3` reader - Temperature sensor gain is recorded here"]
pub type TsgainExtrefMp3R = crate::FieldReader<u16>;
#[doc = "Field `TSOFFSET_EXTREF_MP3` reader - Temperature sensor offset is recorded here"]
pub type TsoffsetExtrefMp3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature sensor gain is recorded here"]
    #[inline(always)]
    pub fn tsgain_extref_mp3(&self) -> TsgainExtrefMp3R {
        TsgainExtrefMp3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature sensor offset is recorded here"]
    #[inline(always)]
    pub fn tsoffset_extref_mp3(&self) -> TsoffsetExtrefMp3R {
        TsoffsetExtrefMp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTREF_MP3")
            .field("tsgain_extref_mp3", &self.tsgain_extref_mp3())
            .field("tsoffset_extref_mp3", &self.tsoffset_extref_mp3())
            .finish()
    }
}
#[doc = "External Reference MP3 Temp Sense\n\nYou can [`read`](crate::Reg::read) this register and get [`extref_mp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtrefMp3Spec;
impl crate::RegisterSpec for ExtrefMp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extref_mp3::R`](R) reader structure"]
impl crate::Readable for ExtrefMp3Spec {}
#[doc = "`reset()` method sets EXTREF_MP3 to value 0"]
impl crate::Resettable for ExtrefMp3Spec {}
