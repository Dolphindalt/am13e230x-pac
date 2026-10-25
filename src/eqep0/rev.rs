#[doc = "Register `REV` reader"]
pub type R = crate::R<RevSpec>;
#[doc = "Register `REV` writer"]
pub type W = crate::W<RevSpec>;
#[doc = "Field `MAJOR` reader - Major Revision Number"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MINOR` reader - Minor Revision Number"]
pub type MinorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Major Revision Number"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Minor Revision Number"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REV")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .finish()
    }
}
impl W {}
#[doc = "QEP Revision Number\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevSpec;
impl crate::RegisterSpec for RevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rev::R`](R) reader structure"]
impl crate::Readable for RevSpec {}
#[doc = "`write(|w| ..)` method takes [`rev::W`](W) writer structure"]
impl crate::Writable for RevSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets REV to value 0x09"]
impl crate::Resettable for RevSpec {
    const RESET_VALUE: u32 = 0x09;
}
