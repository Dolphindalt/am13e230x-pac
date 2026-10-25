#[doc = "Register `CMDADDR` reader"]
pub type R = crate::R<CmdaddrSpec>;
#[doc = "Register `CMDADDR` writer"]
pub type W = crate::W<CmdaddrSpec>;
#[doc = "Field `VAL` reader - Address value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Address value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDADDR").field("val", &self.val()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Address value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmdaddrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdaddrSpec;
impl crate::RegisterSpec for CmdaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdaddr::R`](R) reader structure"]
impl crate::Readable for CmdaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdaddr::W`](W) writer structure"]
impl crate::Writable for CmdaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDADDR to value 0"]
impl crate::Resettable for CmdaddrSpec {}
