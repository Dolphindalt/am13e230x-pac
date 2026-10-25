#[doc = "Register `CMPLNMXSEL` reader"]
pub type R = crate::R<CmplnmxselSpec>;
#[doc = "Register `CMPLNMXSEL` writer"]
pub type W = crate::W<CmplnmxselSpec>;
#[doc = "Field `CMP0LNMXSEL` reader - CMP0LNMXSEL bits"]
pub type Cmp0lnmxselR = crate::BitReader;
#[doc = "Field `CMP0LNMXSEL` writer - CMP0LNMXSEL bits"]
pub type Cmp0lnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1LNMXSEL` reader - CMP1LNMXSEL bits"]
pub type Cmp1lnmxselR = crate::BitReader;
#[doc = "Field `CMP1LNMXSEL` writer - CMP1LNMXSEL bits"]
pub type Cmp1lnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2LNMXSEL` reader - CMP2LNMXSEL bits"]
pub type Cmp2lnmxselR = crate::BitReader;
#[doc = "Field `CMP2LNMXSEL` writer - CMP2LNMXSEL bits"]
pub type Cmp2lnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3LNMXSEL` reader - CMP3LNMXSEL bits"]
pub type Cmp3lnmxselR = crate::BitReader;
#[doc = "Field `CMP3LNMXSEL` writer - CMP3LNMXSEL bits"]
pub type Cmp3lnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP0LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp0lnmxsel(&self) -> Cmp0lnmxselR {
        Cmp0lnmxselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP1LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp1lnmxsel(&self) -> Cmp1lnmxselR {
        Cmp1lnmxselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp2lnmxsel(&self) -> Cmp2lnmxselR {
        Cmp2lnmxselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP3LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp3lnmxsel(&self) -> Cmp3lnmxselR {
        Cmp3lnmxselR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPLNMXSEL")
            .field("cmp0lnmxsel", &self.cmp0lnmxsel())
            .field("cmp1lnmxsel", &self.cmp1lnmxsel())
            .field("cmp3lnmxsel", &self.cmp3lnmxsel())
            .field("cmp2lnmxsel", &self.cmp2lnmxsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CMP0LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp0lnmxsel(&mut self) -> Cmp0lnmxselW<'_, CmplnmxselSpec> {
        Cmp0lnmxselW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP1LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp1lnmxsel(&mut self) -> Cmp1lnmxselW<'_, CmplnmxselSpec> {
        Cmp1lnmxselW::new(self, 1)
    }
    #[doc = "Bit 2 - CMP2LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp2lnmxsel(&mut self) -> Cmp2lnmxselW<'_, CmplnmxselSpec> {
        Cmp2lnmxselW::new(self, 2)
    }
    #[doc = "Bit 3 - CMP3LNMXSEL bits"]
    #[inline(always)]
    pub fn cmp3lnmxsel(&mut self) -> Cmp3lnmxselW<'_, CmplnmxselSpec> {
        Cmp3lnmxselW::new(self, 3)
    }
}
#[doc = "Bits to select one of the many sources on CompLN inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmplnmxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmplnmxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmplnmxselSpec;
impl crate::RegisterSpec for CmplnmxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmplnmxsel::R`](R) reader structure"]
impl crate::Readable for CmplnmxselSpec {}
#[doc = "`write(|w| ..)` method takes [`cmplnmxsel::W`](W) writer structure"]
impl crate::Writable for CmplnmxselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPLNMXSEL to value 0"]
impl crate::Resettable for CmplnmxselSpec {}
