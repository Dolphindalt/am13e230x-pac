#[doc = "Register `CMDDATAINDEX` reader"]
pub type R = crate::R<CmddataindexSpec>;
#[doc = "Register `CMDDATAINDEX` writer"]
pub type W = crate::W<CmddataindexSpec>;
#[doc = "Field `VAL` reader - Data register index"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Data register index"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Data register index"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDDATAINDEX")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Data register index"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmddataindexSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddataindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddataindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmddataindexSpec;
impl crate::RegisterSpec for CmddataindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddataindex::R`](R) reader structure"]
impl crate::Readable for CmddataindexSpec {}
#[doc = "`write(|w| ..)` method takes [`cmddataindex::W`](W) writer structure"]
impl crate::Writable for CmddataindexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDDATAINDEX to value 0"]
impl crate::Resettable for CmddataindexSpec {}
