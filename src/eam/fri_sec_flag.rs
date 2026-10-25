#[doc = "Register `FRI_SEC_FLAG` reader"]
pub type R = crate::R<FriSecFlagSpec>;
#[doc = "Register `FRI_SEC_FLAG` writer"]
pub type W = crate::W<FriSecFlagSpec>;
#[doc = "Field `SEC` reader - Single error correction error"]
pub type SecR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single error correction error"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRI_SEC_FLAG")
            .field("sec", &self.sec())
            .finish()
    }
}
impl W {}
#[doc = "FRI SEC flag\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriSecFlagSpec;
impl crate::RegisterSpec for FriSecFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_sec_flag::R`](R) reader structure"]
impl crate::Readable for FriSecFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_sec_flag::W`](W) writer structure"]
impl crate::Writable for FriSecFlagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_SEC_FLAG to value 0"]
impl crate::Resettable for FriSecFlagSpec {}
