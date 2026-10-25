#[doc = "Register `COMPCTL` reader"]
pub type R = crate::R<CompctlSpec>;
#[doc = "Register `COMPCTL` writer"]
pub type W = crate::W<CompctlSpec>;
#[doc = "Field `COMPHSOURCE` reader - High Comparator Source Select"]
pub type ComphsourceR = crate::BitReader;
#[doc = "Field `COMPHSOURCE` writer - High Comparator Source Select"]
pub type ComphsourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPHINV` reader - High Comparator Invert Select"]
pub type ComphinvR = crate::BitReader;
#[doc = "Field `COMPHINV` writer - High Comparator Invert Select"]
pub type ComphinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRIPHSEL` reader - High Comparator Trip Select"]
pub type CtriphselR = crate::FieldReader;
#[doc = "Field `CTRIPHSEL` writer - High Comparator Trip Select"]
pub type CtriphselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CTRIPOUTHSEL` reader - High Comparator Trip Output Select"]
pub type CtripouthselR = crate::FieldReader;
#[doc = "Field `CTRIPOUTHSEL` writer - High Comparator Trip Output Select"]
pub type CtripouthselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `ASYNCHEN` reader - High Comparator Asynchronous Path Enable"]
pub type AsynchenR = crate::BitReader;
#[doc = "Field `ASYNCHEN` writer - High Comparator Asynchronous Path Enable"]
pub type AsynchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLSOURCE` reader - Low Comparator Source Select"]
pub type ComplsourceR = crate::BitReader;
#[doc = "Field `COMPLSOURCE` writer - Low Comparator Source Select"]
pub type ComplsourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLINV` reader - Low Comparator Invert Select"]
pub type ComplinvR = crate::BitReader;
#[doc = "Field `COMPLINV` writer - Low Comparator Invert Select"]
pub type ComplinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRIPLSEL` reader - Low Comparator Trip Select"]
pub type CtriplselR = crate::FieldReader;
#[doc = "Field `CTRIPLSEL` writer - Low Comparator Trip Select"]
pub type CtriplselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CTRIPOUTLSEL` reader - Low Comparator Trip Output Select"]
pub type CtripoutlselR = crate::FieldReader;
#[doc = "Field `CTRIPOUTLSEL` writer - Low Comparator Trip Output Select"]
pub type CtripoutlselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `ASYNCLEN` reader - Low Comparator Asynchronous Path Enable"]
pub type AsynclenR = crate::BitReader;
#[doc = "Field `ASYNCLEN` writer - Low Comparator Asynchronous Path Enable"]
pub type AsynclenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPDACE` reader - Comparator/DAC Enable"]
pub type CompdaceR = crate::BitReader;
#[doc = "Field `COMPDACE` writer - Comparator/DAC Enable"]
pub type CompdaceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High Comparator Source Select"]
    #[inline(always)]
    pub fn comphsource(&self) -> ComphsourceR {
        ComphsourceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Comparator Invert Select"]
    #[inline(always)]
    pub fn comphinv(&self) -> ComphinvR {
        ComphinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - High Comparator Trip Select"]
    #[inline(always)]
    pub fn ctriphsel(&self) -> CtriphselR {
        CtriphselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - High Comparator Trip Output Select"]
    #[inline(always)]
    pub fn ctripouthsel(&self) -> CtripouthselR {
        CtripouthselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - High Comparator Asynchronous Path Enable"]
    #[inline(always)]
    pub fn asynchen(&self) -> AsynchenR {
        AsynchenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Comparator Source Select"]
    #[inline(always)]
    pub fn complsource(&self) -> ComplsourceR {
        ComplsourceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Comparator Invert Select"]
    #[inline(always)]
    pub fn complinv(&self) -> ComplinvR {
        ComplinvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Low Comparator Trip Select"]
    #[inline(always)]
    pub fn ctriplsel(&self) -> CtriplselR {
        CtriplselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Low Comparator Trip Output Select"]
    #[inline(always)]
    pub fn ctripoutlsel(&self) -> CtripoutlselR {
        CtripoutlselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Low Comparator Asynchronous Path Enable"]
    #[inline(always)]
    pub fn asynclen(&self) -> AsynclenR {
        AsynclenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator/DAC Enable"]
    #[inline(always)]
    pub fn compdace(&self) -> CompdaceR {
        CompdaceR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPCTL")
            .field("comphsource", &self.comphsource())
            .field("comphinv", &self.comphinv())
            .field("ctriphsel", &self.ctriphsel())
            .field("ctripouthsel", &self.ctripouthsel())
            .field("asynchen", &self.asynchen())
            .field("complsource", &self.complsource())
            .field("complinv", &self.complinv())
            .field("ctriplsel", &self.ctriplsel())
            .field("ctripoutlsel", &self.ctripoutlsel())
            .field("asynclen", &self.asynclen())
            .field("compdace", &self.compdace())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - High Comparator Source Select"]
    #[inline(always)]
    pub fn comphsource(&mut self) -> ComphsourceW<'_, CompctlSpec> {
        ComphsourceW::new(self, 0)
    }
    #[doc = "Bit 1 - High Comparator Invert Select"]
    #[inline(always)]
    pub fn comphinv(&mut self) -> ComphinvW<'_, CompctlSpec> {
        ComphinvW::new(self, 1)
    }
    #[doc = "Bits 2:3 - High Comparator Trip Select"]
    #[inline(always)]
    pub fn ctriphsel(&mut self) -> CtriphselW<'_, CompctlSpec> {
        CtriphselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - High Comparator Trip Output Select"]
    #[inline(always)]
    pub fn ctripouthsel(&mut self) -> CtripouthselW<'_, CompctlSpec> {
        CtripouthselW::new(self, 4)
    }
    #[doc = "Bit 6 - High Comparator Asynchronous Path Enable"]
    #[inline(always)]
    pub fn asynchen(&mut self) -> AsynchenW<'_, CompctlSpec> {
        AsynchenW::new(self, 6)
    }
    #[doc = "Bit 8 - Low Comparator Source Select"]
    #[inline(always)]
    pub fn complsource(&mut self) -> ComplsourceW<'_, CompctlSpec> {
        ComplsourceW::new(self, 8)
    }
    #[doc = "Bit 9 - Low Comparator Invert Select"]
    #[inline(always)]
    pub fn complinv(&mut self) -> ComplinvW<'_, CompctlSpec> {
        ComplinvW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Low Comparator Trip Select"]
    #[inline(always)]
    pub fn ctriplsel(&mut self) -> CtriplselW<'_, CompctlSpec> {
        CtriplselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Low Comparator Trip Output Select"]
    #[inline(always)]
    pub fn ctripoutlsel(&mut self) -> CtripoutlselW<'_, CompctlSpec> {
        CtripoutlselW::new(self, 12)
    }
    #[doc = "Bit 14 - Low Comparator Asynchronous Path Enable"]
    #[inline(always)]
    pub fn asynclen(&mut self) -> AsynclenW<'_, CompctlSpec> {
        AsynclenW::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator/DAC Enable"]
    #[inline(always)]
    pub fn compdace(&mut self) -> CompdaceW<'_, CompctlSpec> {
        CompdaceW::new(self, 15)
    }
}
#[doc = "CMPSS Comparator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompctlSpec;
impl crate::RegisterSpec for CompctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compctl::R`](R) reader structure"]
impl crate::Readable for CompctlSpec {}
#[doc = "`write(|w| ..)` method takes [`compctl::W`](W) writer structure"]
impl crate::Writable for CompctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPCTL to value 0"]
impl crate::Resettable for CompctlSpec {}
