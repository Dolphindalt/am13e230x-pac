#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Register `REVISION` writer"]
pub type W = crate::W<RevisionSpec>;
#[doc = "Field `MINREV` reader - Minor Revision Number"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major Revision Number"]
pub type MajrevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Minor Revision Number"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision Number"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REVISION")
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
impl W {}
#[doc = "GSC Revision register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`revision::W`](W) writer structure"]
impl crate::Writable for RevisionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for RevisionSpec {}
