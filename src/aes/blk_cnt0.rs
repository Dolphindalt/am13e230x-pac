#[doc = "Register `BLK_CNT0` reader"]
pub type R = crate::R<BlkCnt0Spec>;
#[doc = "Register `BLK_CNT0` writer"]
pub type W = crate::W<BlkCnt0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal block counter (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkCnt0Spec;
impl crate::RegisterSpec for BlkCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_cnt0::R`](R) reader structure"]
impl crate::Readable for BlkCnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`blk_cnt0::W`](W) writer structure"]
impl crate::Writable for BlkCnt0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BLK_CNT0 to value 0"]
impl crate::Resettable for BlkCnt0Spec {}
