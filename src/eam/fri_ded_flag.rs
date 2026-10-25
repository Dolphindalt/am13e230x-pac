#[doc = "Register `FRI_DED_FLAG` reader"]
pub type R = crate::R<FriDedFlagSpec>;
#[doc = "Register `FRI_DED_FLAG` writer"]
pub type W = crate::W<FriDedFlagSpec>;
#[doc = "Field `DED` reader - Double error detect error"]
pub type DedR = crate::BitReader;
#[doc = "Field `DIAG_DED` reader - diagnostic Double error detect error"]
pub type DiagDedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn ded(&self) -> DedR {
        DedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - diagnostic Double error detect error"]
    #[inline(always)]
    pub fn diag_ded(&self) -> DiagDedR {
        DiagDedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRI_DED_FLAG")
            .field("ded", &self.ded())
            .field("diag_ded", &self.diag_ded())
            .finish()
    }
}
impl W {}
#[doc = "FRI DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriDedFlagSpec;
impl crate::RegisterSpec for FriDedFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_ded_flag::R`](R) reader structure"]
impl crate::Readable for FriDedFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_ded_flag::W`](W) writer structure"]
impl crate::Writable for FriDedFlagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_DED_FLAG to value 0"]
impl crate::Resettable for FriDedFlagSpec {}
