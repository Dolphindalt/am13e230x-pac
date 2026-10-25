#[doc = "Register `FRI_DED_ADDR` reader"]
pub type R = crate::R<FriDedAddrSpec>;
#[doc = "Register `FRI_DED_ADDR` writer"]
pub type W = crate::W<FriDedAddrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FRI DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriDedAddrSpec;
impl crate::RegisterSpec for FriDedAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_ded_addr::R`](R) reader structure"]
impl crate::Readable for FriDedAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_ded_addr::W`](W) writer structure"]
impl crate::Writable for FriDedAddrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_DED_ADDR to value 0"]
impl crate::Resettable for FriDedAddrSpec {}
