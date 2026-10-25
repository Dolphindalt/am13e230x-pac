#[doc = "Register `CMPLPMXSEL` reader"]
pub type R = crate::R<CmplpmxselSpec>;
#[doc = "Register `CMPLPMXSEL` writer"]
pub type W = crate::W<CmplpmxselSpec>;
#[doc = "Field `CMP0LPMXSEL` reader - CMP0LPMXSEL bits"]
pub type Cmp0lpmxselR = crate::FieldReader;
#[doc = "Field `CMP0LPMXSEL` writer - CMP0LPMXSEL bits"]
pub type Cmp0lpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP1LPMXSEL` reader - CMP1LPMXSEL bits"]
pub type Cmp1lpmxselR = crate::FieldReader;
#[doc = "Field `CMP1LPMXSEL` writer - CMP1LPMXSEL bits"]
pub type Cmp1lpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP2LPMXSEL` reader - CMP2LPMXSEL bits"]
pub type Cmp2lpmxselR = crate::FieldReader;
#[doc = "Field `CMP2LPMXSEL` writer - CMP2LPMXSEL bits"]
pub type Cmp2lpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CMP3LPMXSEL` reader - CMP3LPMXSEL bits"]
pub type Cmp3lpmxselR = crate::FieldReader;
#[doc = "Field `CMP3LPMXSEL` writer - CMP3LPMXSEL bits"]
pub type Cmp3lpmxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - CMP0LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp0lpmxsel(&self) -> Cmp0lpmxselR {
        Cmp0lpmxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - CMP1LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp1lpmxsel(&self) -> Cmp1lpmxselR {
        Cmp1lpmxselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - CMP2LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp2lpmxsel(&self) -> Cmp2lpmxselR {
        Cmp2lpmxselR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - CMP3LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp3lpmxsel(&self) -> Cmp3lpmxselR {
        Cmp3lpmxselR::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPLPMXSEL")
            .field("cmp0lpmxsel", &self.cmp0lpmxsel())
            .field("cmp1lpmxsel", &self.cmp1lpmxsel())
            .field("cmp3lpmxsel", &self.cmp3lpmxsel())
            .field("cmp2lpmxsel", &self.cmp2lpmxsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - CMP0LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp0lpmxsel(&mut self) -> Cmp0lpmxselW<'_, CmplpmxselSpec> {
        Cmp0lpmxselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - CMP1LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp1lpmxsel(&mut self) -> Cmp1lpmxselW<'_, CmplpmxselSpec> {
        Cmp1lpmxselW::new(self, 3)
    }
    #[doc = "Bits 6:8 - CMP2LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp2lpmxsel(&mut self) -> Cmp2lpmxselW<'_, CmplpmxselSpec> {
        Cmp2lpmxselW::new(self, 6)
    }
    #[doc = "Bits 9:11 - CMP3LPMXSEL bits"]
    #[inline(always)]
    pub fn cmp3lpmxsel(&mut self) -> Cmp3lpmxselW<'_, CmplpmxselSpec> {
        Cmp3lpmxselW::new(self, 9)
    }
}
#[doc = "Bits to select one of the many sources on CompLP inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmplpmxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmplpmxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmplpmxselSpec;
impl crate::RegisterSpec for CmplpmxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmplpmxsel::R`](R) reader structure"]
impl crate::Readable for CmplpmxselSpec {}
#[doc = "`write(|w| ..)` method takes [`cmplpmxsel::W`](W) writer structure"]
impl crate::Writable for CmplpmxselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPLPMXSEL to value 0"]
impl crate::Resettable for CmplpmxselSpec {}
