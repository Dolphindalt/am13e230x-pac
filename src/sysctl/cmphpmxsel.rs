#[doc = "Register `CMPHPMXSEL` reader"]
pub type R = crate::R<CmphpmxselSpec>;
#[doc = "Register `CMPHPMXSEL` writer"]
pub type W = crate::W<CmphpmxselSpec>;
#[doc = "Field `CMP0HPMXSEL` reader - CMP0HPMXSEL bits"]
pub type Cmp0hpmxselR = crate::FieldReader;
#[doc = "Field `CMP0HPMXSEL` writer - CMP0HPMXSEL bits"]
pub type Cmp0hpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP1HPMXSEL` reader - CMP1HPMXSEL bits"]
pub type Cmp1hpmxselR = crate::FieldReader;
#[doc = "Field `CMP1HPMXSEL` writer - CMP1HPMXSEL bits"]
pub type Cmp1hpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP2HPMXSEL` reader - CMP2HPMXSEL bits"]
pub type Cmp2hpmxselR = crate::FieldReader;
#[doc = "Field `CMP2HPMXSEL` writer - CMP2HPMXSEL bits"]
pub type Cmp2hpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP3HPMXSEL` reader - CMP3HPMXSEL bits"]
pub type Cmp3hpmxselR = crate::FieldReader;
#[doc = "Field `CMP3HPMXSEL` writer - CMP3HPMXSEL bits"]
pub type Cmp3hpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - CMP0HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp0hpmxsel(&self) -> Cmp0hpmxselR {
        Cmp0hpmxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - CMP1HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp1hpmxsel(&self) -> Cmp1hpmxselR {
        Cmp1hpmxselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - CMP2HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp2hpmxsel(&self) -> Cmp2hpmxselR {
        Cmp2hpmxselR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - CMP3HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp3hpmxsel(&self) -> Cmp3hpmxselR {
        Cmp3hpmxselR::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPHPMXSEL")
            .field("cmp3hpmxsel", &self.cmp3hpmxsel())
            .field("cmp1hpmxsel", &self.cmp1hpmxsel())
            .field("cmp2hpmxsel", &self.cmp2hpmxsel())
            .field("cmp0hpmxsel", &self.cmp0hpmxsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - CMP0HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp0hpmxsel(&mut self) -> Cmp0hpmxselW<'_, CmphpmxselSpec> {
        Cmp0hpmxselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - CMP1HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp1hpmxsel(&mut self) -> Cmp1hpmxselW<'_, CmphpmxselSpec> {
        Cmp1hpmxselW::new(self, 3)
    }
    #[doc = "Bits 6:8 - CMP2HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp2hpmxsel(&mut self) -> Cmp2hpmxselW<'_, CmphpmxselSpec> {
        Cmp2hpmxselW::new(self, 6)
    }
    #[doc = "Bits 9:11 - CMP3HPMXSEL bits"]
    #[inline(always)]
    pub fn cmp3hpmxsel(&mut self) -> Cmp3hpmxselW<'_, CmphpmxselSpec> {
        Cmp3hpmxselW::new(self, 9)
    }
}
#[doc = "Bits to select one of the many sources on CompHP inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmphpmxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmphpmxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmphpmxselSpec;
impl crate::RegisterSpec for CmphpmxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmphpmxsel::R`](R) reader structure"]
impl crate::Readable for CmphpmxselSpec {}
#[doc = "`write(|w| ..)` method takes [`cmphpmxsel::W`](W) writer structure"]
impl crate::Writable for CmphpmxselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPHPMXSEL to value 0"]
impl crate::Resettable for CmphpmxselSpec {}
