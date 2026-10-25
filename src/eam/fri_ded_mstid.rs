#[doc = "Register `FRI_DED_MSTID` reader"]
pub type R = crate::R<FriDedMstidSpec>;
#[doc = "Register `FRI_DED_MSTID` writer"]
pub type W = crate::W<FriDedMstidSpec>;
#[doc = "Field `MSTID` reader - Master ID"]
pub type MstidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Master ID"]
    #[inline(always)]
    pub fn mstid(&self) -> MstidR {
        MstidR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRI_DED_MSTID")
            .field("mstid", &self.mstid())
            .finish()
    }
}
impl W {}
#[doc = "FRI DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_mstid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_mstid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriDedMstidSpec;
impl crate::RegisterSpec for FriDedMstidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_ded_mstid::R`](R) reader structure"]
impl crate::Readable for FriDedMstidSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_ded_mstid::W`](W) writer structure"]
impl crate::Writable for FriDedMstidSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_DED_MSTID to value 0"]
impl crate::Resettable for FriDedMstidSpec {}
