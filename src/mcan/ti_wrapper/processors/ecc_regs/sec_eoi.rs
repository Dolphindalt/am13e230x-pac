#[doc = "Register `SEC_EOI` reader"]
pub type R = crate::R<SecEoiSpec>;
#[doc = "Register `SEC_EOI` writer"]
pub type W = crate::W<SecEoiSpec>;
#[doc = "Field `EOI_WR` reader - End of Interrupt"]
pub type EoiWrR = crate::BitReader;
#[doc = "Field `EOI_WR` writer - End of Interrupt"]
pub type EoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of Interrupt"]
    #[inline(always)]
    pub fn eoi_wr(&self) -> EoiWrR {
        EoiWrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_EOI")
            .field("eoi_wr", &self.eoi_wr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - End of Interrupt"]
    #[inline(always)]
    pub fn eoi_wr(&mut self) -> EoiWrW<'_, SecEoiSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEoiSpec;
impl crate::RegisterSpec for SecEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_eoi::R`](R) reader structure"]
impl crate::Readable for SecEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_eoi::W`](W) writer structure"]
impl crate::Writable for SecEoiSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SEC_EOI to value 0"]
impl crate::Resettable for SecEoiSpec {}
