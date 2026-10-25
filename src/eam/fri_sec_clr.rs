#[doc = "Register `FRI_SEC_CLR` reader"]
pub type R = crate::R<FriSecClrSpec>;
#[doc = "Register `FRI_SEC_CLR` writer"]
pub type W = crate::W<FriSecClrSpec>;
#[doc = "Field `SEC` reader - Single error correction error"]
pub type SecR = crate::BitReader;
#[doc = "Field `SEC` writer - Single error correction error"]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("FRI_SEC_CLR")
            .field("sec", &self.sec())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Single error correction error"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<'_, FriSecClrSpec> {
        SecW::new(self, 0)
    }
}
#[doc = "FRI SEC clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_sec_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_sec_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriSecClrSpec;
impl crate::RegisterSpec for FriSecClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_sec_clr::R`](R) reader structure"]
impl crate::Readable for FriSecClrSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_sec_clr::W`](W) writer structure"]
impl crate::Writable for FriSecClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_SEC_CLR to value 0"]
impl crate::Resettable for FriSecClrSpec {}
