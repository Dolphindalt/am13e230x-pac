#[doc = "Register `CMPSSCTL` reader"]
pub type R = crate::R<CmpssctlSpec>;
#[doc = "Register `CMPSSCTL` writer"]
pub type W = crate::W<CmpssctlSpec>;
#[doc = "Field `CMP2LDACOUTEN` reader - Enable general purpose DAC functionality for CMPSS2 COMPDACL"]
pub type Cmp2ldacoutenR = crate::BitReader;
#[doc = "Field `CMP2LDACOUTEN` writer - Enable general purpose DAC functionality for CMPSS2 COMPDACL"]
pub type Cmp2ldacoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3LDACOUTEN` reader - Enable general purpose DAC functionality for CMPSS3 COMPDACL"]
pub type Cmp3ldacoutenR = crate::BitReader;
#[doc = "Field `CMP3LDACOUTEN` writer - Enable general purpose DAC functionality for CMPSS3 COMPDACL"]
pub type Cmp3ldacoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSSCTLEN` reader - Enable the CMPSSCTL Register"]
pub type CmpssctlenR = crate::BitReader;
#[doc = "Field `CMPSSCTLEN` writer - Enable the CMPSSCTL Register"]
pub type CmpssctlenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable general purpose DAC functionality for CMPSS2 COMPDACL"]
    #[inline(always)]
    pub fn cmp2ldacouten(&self) -> Cmp2ldacoutenR {
        Cmp2ldacoutenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable general purpose DAC functionality for CMPSS3 COMPDACL"]
    #[inline(always)]
    pub fn cmp3ldacouten(&self) -> Cmp3ldacoutenR {
        Cmp3ldacoutenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the CMPSSCTL Register"]
    #[inline(always)]
    pub fn cmpssctlen(&self) -> CmpssctlenR {
        CmpssctlenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPSSCTL")
            .field("cmpssctlen", &self.cmpssctlen())
            .field("cmp3ldacouten", &self.cmp3ldacouten())
            .field("cmp2ldacouten", &self.cmp2ldacouten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable general purpose DAC functionality for CMPSS2 COMPDACL"]
    #[inline(always)]
    pub fn cmp2ldacouten(&mut self) -> Cmp2ldacoutenW<'_, CmpssctlSpec> {
        Cmp2ldacoutenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable general purpose DAC functionality for CMPSS3 COMPDACL"]
    #[inline(always)]
    pub fn cmp3ldacouten(&mut self) -> Cmp3ldacoutenW<'_, CmpssctlSpec> {
        Cmp3ldacoutenW::new(self, 1)
    }
    #[doc = "Bit 31 - Enable the CMPSSCTL Register"]
    #[inline(always)]
    pub fn cmpssctlen(&mut self) -> CmpssctlenW<'_, CmpssctlSpec> {
        CmpssctlenW::new(self, 31)
    }
}
#[doc = "CMPSS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpssctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpssctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpssctlSpec;
impl crate::RegisterSpec for CmpssctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpssctl::R`](R) reader structure"]
impl crate::Readable for CmpssctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpssctl::W`](W) writer structure"]
impl crate::Writable for CmpssctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPSSCTL to value 0"]
impl crate::Resettable for CmpssctlSpec {}
