#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `OUTPUTRDY` reader - AES ready"]
pub type OutputrdyR = crate::BitReader;
#[doc = "Field `OUTPUTRDY` writer - AES ready"]
pub type OutputrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUTRDY` reader - AES ready"]
pub type InputrdyR = crate::BitReader;
#[doc = "Field `INPUTRDY` writer - AES ready"]
pub type InputrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAVEDCNTXTRDY` reader - AES ready"]
pub type SavedcntxtrdyR = crate::BitReader;
#[doc = "Field `SAVEDCNTXTRDY` writer - AES ready"]
pub type SavedcntxtrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTXTRDY` reader - AES ready"]
pub type CntxtrdyR = crate::BitReader;
#[doc = "Field `CNTXTRDY` writer - AES ready"]
pub type CntxtrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES ready"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OutputrdyR {
        OutputrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES ready"]
    #[inline(always)]
    pub fn inputrdy(&self) -> InputrdyR {
        InputrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AES ready"]
    #[inline(always)]
    pub fn savedcntxtrdy(&self) -> SavedcntxtrdyR {
        SavedcntxtrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES ready"]
    #[inline(always)]
    pub fn cntxtrdy(&self) -> CntxtrdyR {
        CntxtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("outputrdy", &self.outputrdy())
            .field("inputrdy", &self.inputrdy())
            .field("savedcntxtrdy", &self.savedcntxtrdy())
            .field("cntxtrdy", &self.cntxtrdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AES ready"]
    #[inline(always)]
    pub fn outputrdy(&mut self) -> OutputrdyW<'_, ImaskSpec> {
        OutputrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - AES ready"]
    #[inline(always)]
    pub fn inputrdy(&mut self) -> InputrdyW<'_, ImaskSpec> {
        InputrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - AES ready"]
    #[inline(always)]
    pub fn savedcntxtrdy(&mut self) -> SavedcntxtrdyW<'_, ImaskSpec> {
        SavedcntxtrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - AES ready"]
    #[inline(always)]
    pub fn cntxtrdy(&mut self) -> CntxtrdyW<'_, ImaskSpec> {
        CntxtrdyW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
