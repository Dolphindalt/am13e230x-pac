#[doc = "Register `EOI` reader"]
pub type R = crate::R<EoiSpec>;
#[doc = "Register `EOI` writer"]
pub type W = crate::W<EoiSpec>;
#[doc = "Field `EOI` reader - External Timestamp Counter Overflow End of Interrupt"]
pub type EoiR = crate::FieldReader;
#[doc = "Field `EOI` writer - External Timestamp Counter Overflow End of Interrupt"]
pub type EoiW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - External Timestamp Counter Overflow End of Interrupt"]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EOI").field("eoi", &self.eoi()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - External Timestamp Counter Overflow End of Interrupt"]
    #[inline(always)]
    pub fn eoi(&mut self) -> EoiW<'_, EoiSpec> {
        EoiW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem End of Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EoiSpec;
impl crate::RegisterSpec for EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eoi::R`](R) reader structure"]
impl crate::Readable for EoiSpec {}
#[doc = "`write(|w| ..)` method takes [`eoi::W`](W) writer structure"]
impl crate::Writable for EoiSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EOI to value 0"]
impl crate::Resettable for EoiSpec {}
