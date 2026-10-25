#[doc = "Register `CMPHNMXSEL` reader"]
pub type R = crate::R<CmphnmxselSpec>;
#[doc = "Register `CMPHNMXSEL` writer"]
pub type W = crate::W<CmphnmxselSpec>;
#[doc = "Field `CMP0HNMXSEL` reader - CMP0HNMXSEL bits"]
pub type Cmp0hnmxselR = crate::BitReader;
#[doc = "Field `CMP0HNMXSEL` writer - CMP0HNMXSEL bits"]
pub type Cmp0hnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1HNMXSEL` reader - CMP1HNMXSEL bits"]
pub type Cmp1hnmxselR = crate::BitReader;
#[doc = "Field `CMP1HNMXSEL` writer - CMP1HNMXSEL bits"]
pub type Cmp1hnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2HNMXSEL` reader - CMP2HNMXSEL bits"]
pub type Cmp2hnmxselR = crate::BitReader;
#[doc = "Field `CMP2HNMXSEL` writer - CMP2HNMXSEL bits"]
pub type Cmp2hnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3HNMXSEL` reader - CMP3HNMXSEL bits"]
pub type Cmp3hnmxselR = crate::BitReader;
#[doc = "Field `CMP3HNMXSEL` writer - CMP3HNMXSEL bits"]
pub type Cmp3hnmxselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP0HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp0hnmxsel(&self) -> Cmp0hnmxselR {
        Cmp0hnmxselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP1HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp1hnmxsel(&self) -> Cmp1hnmxselR {
        Cmp1hnmxselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp2hnmxsel(&self) -> Cmp2hnmxselR {
        Cmp2hnmxselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP3HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp3hnmxsel(&self) -> Cmp3hnmxselR {
        Cmp3hnmxselR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPHNMXSEL")
            .field("cmp3hnmxsel", &self.cmp3hnmxsel())
            .field("cmp2hnmxsel", &self.cmp2hnmxsel())
            .field("cmp1hnmxsel", &self.cmp1hnmxsel())
            .field("cmp0hnmxsel", &self.cmp0hnmxsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CMP0HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp0hnmxsel(&mut self) -> Cmp0hnmxselW<'_, CmphnmxselSpec> {
        Cmp0hnmxselW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP1HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp1hnmxsel(&mut self) -> Cmp1hnmxselW<'_, CmphnmxselSpec> {
        Cmp1hnmxselW::new(self, 1)
    }
    #[doc = "Bit 2 - CMP2HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp2hnmxsel(&mut self) -> Cmp2hnmxselW<'_, CmphnmxselSpec> {
        Cmp2hnmxselW::new(self, 2)
    }
    #[doc = "Bit 3 - CMP3HNMXSEL bits"]
    #[inline(always)]
    pub fn cmp3hnmxsel(&mut self) -> Cmp3hnmxselW<'_, CmphnmxselSpec> {
        Cmp3hnmxselW::new(self, 3)
    }
}
#[doc = "Bits to select one of the many sources on CompHN inputs. Refer to Pinmux diagram for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmphnmxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmphnmxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmphnmxselSpec;
impl crate::RegisterSpec for CmphnmxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmphnmxsel::R`](R) reader structure"]
impl crate::Readable for CmphnmxselSpec {}
#[doc = "`write(|w| ..)` method takes [`cmphnmxsel::W`](W) writer structure"]
impl crate::Writable for CmphnmxselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CMPHNMXSEL to value 0"]
impl crate::Resettable for CmphnmxselSpec {}
