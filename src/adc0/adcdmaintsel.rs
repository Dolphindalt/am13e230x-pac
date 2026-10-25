#[doc = "Register `ADCDMAINTSEL` reader"]
pub type R = crate::R<AdcdmaintselSpec>;
#[doc = "Register `ADCDMAINTSEL` writer"]
pub type W = crate::W<AdcdmaintselSpec>;
#[doc = "Field `DMAINT1SEL` reader - ADCDMAINT1 EOC Source Select"]
pub type Dmaint1selR = crate::FieldReader;
#[doc = "Field `DMAINT1SEL` writer - ADCDMAINT1 EOC Source Select"]
pub type Dmaint1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DMAINT1CONT` reader - ADCDMAINT1 Continue to Interrupt Mode"]
pub type Dmaint1contR = crate::BitReader;
#[doc = "Field `DMAINT1CONT` writer - ADCDMAINT1 Continue to Interrupt Mode"]
pub type Dmaint1contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT1E` reader - ADCDMAINT1 Interrupt Enable"]
pub type Dmaint1eR = crate::BitReader;
#[doc = "Field `DMAINT1E` writer - ADCDMAINT1 Interrupt Enable"]
pub type Dmaint1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT2SEL` reader - ADCDMAINT2 EOC Source Select"]
pub type Dmaint2selR = crate::FieldReader;
#[doc = "Field `DMAINT2SEL` writer - ADCDMAINT2 EOC Source Select"]
pub type Dmaint2selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DMAINT2CONT` reader - ADCDMAINT2 Continue to Interrupt Mode"]
pub type Dmaint2contR = crate::BitReader;
#[doc = "Field `DMAINT2CONT` writer - ADCDMAINT2 Continue to Interrupt Mode"]
pub type Dmaint2contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT2E` reader - ADCDMAINT2 Interrupt Enable"]
pub type Dmaint2eR = crate::BitReader;
#[doc = "Field `DMAINT2E` writer - ADCDMAINT2 Interrupt Enable"]
pub type Dmaint2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT3SEL` reader - ADCDMAINT3 EOC Source Select"]
pub type Dmaint3selR = crate::FieldReader;
#[doc = "Field `DMAINT3SEL` writer - ADCDMAINT3 EOC Source Select"]
pub type Dmaint3selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DMAINT3CONT` reader - ADCDMAINT3 Continue to Interrupt Mode"]
pub type Dmaint3contR = crate::BitReader;
#[doc = "Field `DMAINT3CONT` writer - ADCDMAINT3 Continue to Interrupt Mode"]
pub type Dmaint3contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT3E` reader - ADCDMAINT3 Interrupt Enable"]
pub type Dmaint3eR = crate::BitReader;
#[doc = "Field `DMAINT3E` writer - ADCDMAINT3 Interrupt Enable"]
pub type Dmaint3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT4SEL` reader - ADCDMAINT4 EOC Source Select"]
pub type Dmaint4selR = crate::FieldReader;
#[doc = "Field `DMAINT4SEL` writer - ADCDMAINT4 EOC Source Select"]
pub type Dmaint4selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DMAINT4CONT` reader - ADCDMAINT4 Continue to Interrupt Mode"]
pub type Dmaint4contR = crate::BitReader;
#[doc = "Field `DMAINT4CONT` writer - ADCDMAINT4 Continue to Interrupt Mode"]
pub type Dmaint4contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT4E` reader - ADCDMAINT4 Interrupt Enable"]
pub type Dmaint4eR = crate::BitReader;
#[doc = "Field `DMAINT4E` writer - ADCDMAINT4 Interrupt Enable"]
pub type Dmaint4eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ADCDMAINT1 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint1sel(&self) -> Dmaint1selR {
        Dmaint1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ADCDMAINT1 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint1cont(&self) -> Dmaint1contR {
        Dmaint1contR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADCDMAINT1 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint1e(&self) -> Dmaint1eR {
        Dmaint1eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADCDMAINT2 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint2sel(&self) -> Dmaint2selR {
        Dmaint2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - ADCDMAINT2 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint2cont(&self) -> Dmaint2contR {
        Dmaint2contR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADCDMAINT2 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint2e(&self) -> Dmaint2eR {
        Dmaint2eR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ADCDMAINT3 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint3sel(&self) -> Dmaint3selR {
        Dmaint3selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - ADCDMAINT3 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint3cont(&self) -> Dmaint3contR {
        Dmaint3contR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADCDMAINT3 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint3e(&self) -> Dmaint3eR {
        Dmaint3eR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - ADCDMAINT4 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint4sel(&self) -> Dmaint4selR {
        Dmaint4selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - ADCDMAINT4 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint4cont(&self) -> Dmaint4contR {
        Dmaint4contR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCDMAINT4 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint4e(&self) -> Dmaint4eR {
        Dmaint4eR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCDMAINTSEL")
            .field("dmaint1sel", &self.dmaint1sel())
            .field("dmaint1cont", &self.dmaint1cont())
            .field("dmaint1e", &self.dmaint1e())
            .field("dmaint2sel", &self.dmaint2sel())
            .field("dmaint2cont", &self.dmaint2cont())
            .field("dmaint2e", &self.dmaint2e())
            .field("dmaint3sel", &self.dmaint3sel())
            .field("dmaint3cont", &self.dmaint3cont())
            .field("dmaint3e", &self.dmaint3e())
            .field("dmaint4sel", &self.dmaint4sel())
            .field("dmaint4cont", &self.dmaint4cont())
            .field("dmaint4e", &self.dmaint4e())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ADCDMAINT1 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint1sel(&mut self) -> Dmaint1selW<'_, AdcdmaintselSpec> {
        Dmaint1selW::new(self, 0)
    }
    #[doc = "Bit 6 - ADCDMAINT1 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint1cont(&mut self) -> Dmaint1contW<'_, AdcdmaintselSpec> {
        Dmaint1contW::new(self, 6)
    }
    #[doc = "Bit 7 - ADCDMAINT1 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint1e(&mut self) -> Dmaint1eW<'_, AdcdmaintselSpec> {
        Dmaint1eW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADCDMAINT2 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint2sel(&mut self) -> Dmaint2selW<'_, AdcdmaintselSpec> {
        Dmaint2selW::new(self, 8)
    }
    #[doc = "Bit 14 - ADCDMAINT2 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint2cont(&mut self) -> Dmaint2contW<'_, AdcdmaintselSpec> {
        Dmaint2contW::new(self, 14)
    }
    #[doc = "Bit 15 - ADCDMAINT2 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint2e(&mut self) -> Dmaint2eW<'_, AdcdmaintselSpec> {
        Dmaint2eW::new(self, 15)
    }
    #[doc = "Bits 16:19 - ADCDMAINT3 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint3sel(&mut self) -> Dmaint3selW<'_, AdcdmaintselSpec> {
        Dmaint3selW::new(self, 16)
    }
    #[doc = "Bit 22 - ADCDMAINT3 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint3cont(&mut self) -> Dmaint3contW<'_, AdcdmaintselSpec> {
        Dmaint3contW::new(self, 22)
    }
    #[doc = "Bit 23 - ADCDMAINT3 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint3e(&mut self) -> Dmaint3eW<'_, AdcdmaintselSpec> {
        Dmaint3eW::new(self, 23)
    }
    #[doc = "Bits 24:27 - ADCDMAINT4 EOC Source Select"]
    #[inline(always)]
    pub fn dmaint4sel(&mut self) -> Dmaint4selW<'_, AdcdmaintselSpec> {
        Dmaint4selW::new(self, 24)
    }
    #[doc = "Bit 30 - ADCDMAINT4 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn dmaint4cont(&mut self) -> Dmaint4contW<'_, AdcdmaintselSpec> {
        Dmaint4contW::new(self, 30)
    }
    #[doc = "Bit 31 - ADCDMAINT4 Interrupt Enable"]
    #[inline(always)]
    pub fn dmaint4e(&mut self) -> Dmaint4eW<'_, AdcdmaintselSpec> {
        Dmaint4eW::new(self, 31)
    }
}
#[doc = "ADC DMA Interrupt 1, 2, 3 and 4 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcdmaintsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcdmaintsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcdmaintselSpec;
impl crate::RegisterSpec for AdcdmaintselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcdmaintsel::R`](R) reader structure"]
impl crate::Readable for AdcdmaintselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcdmaintsel::W`](W) writer structure"]
impl crate::Writable for AdcdmaintselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCDMAINTSEL to value 0"]
impl crate::Resettable for AdcdmaintselSpec {}
