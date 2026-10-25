#[doc = "Register `AM13SPAREIREFENSOCLOCK` reader"]
pub type R = crate::R<Am13spareirefensoclockSpec>;
#[doc = "Register `AM13SPAREIREFENSOCLOCK` writer"]
pub type W = crate::W<Am13spareirefensoclockSpec>;
#[doc = "Field `SPARE` reader - Spare IREFEN SOC LOCK Register"]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Spare IREFEN SOC LOCK Register"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:5 - Spare IREFEN SOC LOCK Register"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AM13SPAREIREFENSOCLOCK")
            .field("spare", &self.spare())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Spare IREFEN SOC LOCK Register"]
    #[inline(always)]
    pub fn spare(&mut self) -> SpareW<'_, Am13spareirefensoclockSpec> {
        SpareW::new(self, 0)
    }
}
#[doc = "AM13 Spare IREFEN SOC LOCK Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`am13spareirefensoclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13spareirefensoclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Am13spareirefensoclockSpec;
impl crate::RegisterSpec for Am13spareirefensoclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`am13spareirefensoclock::R`](R) reader structure"]
impl crate::Readable for Am13spareirefensoclockSpec {}
#[doc = "`write(|w| ..)` method takes [`am13spareirefensoclock::W`](W) writer structure"]
impl crate::Writable for Am13spareirefensoclockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AM13SPAREIREFENSOCLOCK to value 0"]
impl crate::Resettable for Am13spareirefensoclockSpec {}
