#[doc = "Register `BLK_CNT1` reader"]
pub type R = crate::R<BlkCnt1Spec>;
#[doc = "Register `BLK_CNT1` writer"]
pub type W = crate::W<BlkCnt1Spec>;
#[doc = "Field `DATA` reader - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:23 - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK_CNT1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, BlkCnt1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Internal block counter (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkCnt1Spec;
impl crate::RegisterSpec for BlkCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_cnt1::R`](R) reader structure"]
impl crate::Readable for BlkCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`blk_cnt1::W`](W) writer structure"]
impl crate::Writable for BlkCnt1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BLK_CNT1 to value 0"]
impl crate::Resettable for BlkCnt1Spec {}
