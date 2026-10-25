#[doc = "Register `SYSPLLPARAM2` reader"]
pub type R = crate::R<Syspllparam2Spec>;
#[doc = "Field `LPFCAPC` reader - LPFCAPC"]
pub type LpfcapcR = crate::FieldReader;
#[doc = "Field `RNGIXVCOIBIASCFG` reader - RNGIXVCOIBIASCFG"]
pub type RngixvcoibiascfgR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - LPFCAPC"]
    #[inline(always)]
    pub fn lpfcapc(&self) -> LpfcapcR {
        LpfcapcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - RNGIXVCOIBIASCFG"]
    #[inline(always)]
    pub fn rngixvcoibiascfg(&self) -> RngixvcoibiascfgR {
        RngixvcoibiascfgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLPARAM2")
            .field("lpfcapc", &self.lpfcapc())
            .field("rngixvcoibiascfg", &self.rngixvcoibiascfg())
            .finish()
    }
}
#[doc = "SYSPLLPARAM2\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam2Spec;
impl crate::RegisterSpec for Syspllparam2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam2::R`](R) reader structure"]
impl crate::Readable for Syspllparam2Spec {}
#[doc = "`reset()` method sets SYSPLLPARAM2 to value 0"]
impl crate::Resettable for Syspllparam2Spec {}
