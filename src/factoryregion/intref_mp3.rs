#[doc = "Register `INTREF_MP3` reader"]
pub type R = crate::R<IntrefMp3Spec>;
#[doc = "Field `TSGAIN_INTREF_MP3` reader - Temperature sensor gain is recorded here"]
pub type TsgainIntrefMp3R = crate::FieldReader<u16>;
#[doc = "Field `TSOFFSET_INTREF_MP3` reader - Temperature sensor offset is recorded here"]
pub type TsoffsetIntrefMp3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature sensor gain is recorded here"]
    #[inline(always)]
    pub fn tsgain_intref_mp3(&self) -> TsgainIntrefMp3R {
        TsgainIntrefMp3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature sensor offset is recorded here"]
    #[inline(always)]
    pub fn tsoffset_intref_mp3(&self) -> TsoffsetIntrefMp3R {
        TsoffsetIntrefMp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTREF_MP3")
            .field("tsgain_intref_mp3", &self.tsgain_intref_mp3())
            .field("tsoffset_intref_mp3", &self.tsoffset_intref_mp3())
            .finish()
    }
}
#[doc = "Internal Refernce Temp Sene\n\nYou can [`read`](crate::Reg::read) this register and get [`intref_mp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrefMp3Spec;
impl crate::RegisterSpec for IntrefMp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intref_mp3::R`](R) reader structure"]
impl crate::Readable for IntrefMp3Spec {}
#[doc = "`reset()` method sets INTREF_MP3 to value 0"]
impl crate::Resettable for IntrefMp3Spec {}
