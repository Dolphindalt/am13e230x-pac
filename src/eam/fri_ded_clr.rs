#[doc = "Register `FRI_DED_CLR` reader"]
pub type R = crate::R<FriDedClrSpec>;
#[doc = "Register `FRI_DED_CLR` writer"]
pub type W = crate::W<FriDedClrSpec>;
#[doc = "Field `SEC` reader - Double error detect error"]
pub type SecR = crate::BitReader;
#[doc = "Field `SEC` writer - Double error detect error"]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIAG_DED` reader - diagnostic Double error detect error"]
pub type DiagDedR = crate::BitReader;
#[doc = "Field `DIAG_DED` writer - diagnostic Double error detect error"]
pub type DiagDedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 1) != 0)
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
        f.debug_struct("FRI_DED_CLR")
            .field("sec", &self.sec())
            .field("diag_ded", &self.diag_ded())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<'_, FriDedClrSpec> {
        SecW::new(self, 0)
    }
    #[doc = "Bit 1 - diagnostic Double error detect error"]
    #[inline(always)]
    pub fn diag_ded(&mut self) -> DiagDedW<'_, FriDedClrSpec> {
        DiagDedW::new(self, 1)
    }
}
#[doc = "FRI DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fri_ded_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fri_ded_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FriDedClrSpec;
impl crate::RegisterSpec for FriDedClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fri_ded_clr::R`](R) reader structure"]
impl crate::Readable for FriDedClrSpec {}
#[doc = "`write(|w| ..)` method takes [`fri_ded_clr::W`](W) writer structure"]
impl crate::Writable for FriDedClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FRI_DED_CLR to value 0"]
impl crate::Resettable for FriDedClrSpec {}
