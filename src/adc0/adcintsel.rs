#[doc = "Register `ADCINTSEL` reader"]
pub type R = crate::R<AdcintselSpec>;
#[doc = "Register `ADCINTSEL` writer"]
pub type W = crate::W<AdcintselSpec>;
#[doc = "Field `INT1SEL` reader - ADCINT1 EOC Source Select"]
pub type Int1selR = crate::FieldReader;
#[doc = "Field `INT1SEL` writer - ADCINT1 EOC Source Select"]
pub type Int1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `INT1CONT` reader - ADCINT1 Continue to Interrupt Mode"]
pub type Int1contR = crate::BitReader;
#[doc = "Field `INT1CONT` writer - ADCINT1 Continue to Interrupt Mode"]
pub type Int1contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1E` reader - ADCINT1 Interrupt Enable"]
pub type Int1eR = crate::BitReader;
#[doc = "Field `INT1E` writer - ADCINT1 Interrupt Enable"]
pub type Int1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2SEL` reader - ADCINT2 EOC Source Select"]
pub type Int2selR = crate::FieldReader;
#[doc = "Field `INT2SEL` writer - ADCINT2 EOC Source Select"]
pub type Int2selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `INT2CONT` reader - ADCINT2 Continue to Interrupt Mode"]
pub type Int2contR = crate::BitReader;
#[doc = "Field `INT2CONT` writer - ADCINT2 Continue to Interrupt Mode"]
pub type Int2contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2E` reader - ADCINT2 Interrupt Enable"]
pub type Int2eR = crate::BitReader;
#[doc = "Field `INT2E` writer - ADCINT2 Interrupt Enable"]
pub type Int2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3SEL` reader - ADCINT3 EOC Source Select"]
pub type Int3selR = crate::FieldReader;
#[doc = "Field `INT3SEL` writer - ADCINT3 EOC Source Select"]
pub type Int3selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `INT3CONT` reader - ADCINT3 Continue to Interrupt Mode"]
pub type Int3contR = crate::BitReader;
#[doc = "Field `INT3CONT` writer - ADCINT3 Continue to Interrupt Mode"]
pub type Int3contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3E` reader - ADCINT3 Interrupt Enable"]
pub type Int3eR = crate::BitReader;
#[doc = "Field `INT3E` writer - ADCINT3 Interrupt Enable"]
pub type Int3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT4SEL` reader - ADCINT4 EOC Source Select"]
pub type Int4selR = crate::FieldReader;
#[doc = "Field `INT4SEL` writer - ADCINT4 EOC Source Select"]
pub type Int4selW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `INT4CONT` reader - ADCINT4 Continue to Interrupt Mode"]
pub type Int4contR = crate::BitReader;
#[doc = "Field `INT4CONT` writer - ADCINT4 Continue to Interrupt Mode"]
pub type Int4contW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT4E` reader - ADCINT4 Interrupt Enable"]
pub type Int4eR = crate::BitReader;
#[doc = "Field `INT4E` writer - ADCINT4 Interrupt Enable"]
pub type Int4eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ADCINT1 EOC Source Select"]
    #[inline(always)]
    pub fn int1sel(&self) -> Int1selR {
        Int1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ADCINT1 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int1cont(&self) -> Int1contR {
        Int1contR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADCINT1 Interrupt Enable"]
    #[inline(always)]
    pub fn int1e(&self) -> Int1eR {
        Int1eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADCINT2 EOC Source Select"]
    #[inline(always)]
    pub fn int2sel(&self) -> Int2selR {
        Int2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - ADCINT2 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int2cont(&self) -> Int2contR {
        Int2contR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADCINT2 Interrupt Enable"]
    #[inline(always)]
    pub fn int2e(&self) -> Int2eR {
        Int2eR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ADCINT3 EOC Source Select"]
    #[inline(always)]
    pub fn int3sel(&self) -> Int3selR {
        Int3selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - ADCINT3 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int3cont(&self) -> Int3contR {
        Int3contR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADCINT3 Interrupt Enable"]
    #[inline(always)]
    pub fn int3e(&self) -> Int3eR {
        Int3eR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - ADCINT4 EOC Source Select"]
    #[inline(always)]
    pub fn int4sel(&self) -> Int4selR {
        Int4selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - ADCINT4 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int4cont(&self) -> Int4contR {
        Int4contR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCINT4 Interrupt Enable"]
    #[inline(always)]
    pub fn int4e(&self) -> Int4eR {
        Int4eR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCINTSEL")
            .field("int1sel", &self.int1sel())
            .field("int1cont", &self.int1cont())
            .field("int1e", &self.int1e())
            .field("int2sel", &self.int2sel())
            .field("int2cont", &self.int2cont())
            .field("int2e", &self.int2e())
            .field("int3sel", &self.int3sel())
            .field("int3cont", &self.int3cont())
            .field("int3e", &self.int3e())
            .field("int4sel", &self.int4sel())
            .field("int4cont", &self.int4cont())
            .field("int4e", &self.int4e())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ADCINT1 EOC Source Select"]
    #[inline(always)]
    pub fn int1sel(&mut self) -> Int1selW<'_, AdcintselSpec> {
        Int1selW::new(self, 0)
    }
    #[doc = "Bit 6 - ADCINT1 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int1cont(&mut self) -> Int1contW<'_, AdcintselSpec> {
        Int1contW::new(self, 6)
    }
    #[doc = "Bit 7 - ADCINT1 Interrupt Enable"]
    #[inline(always)]
    pub fn int1e(&mut self) -> Int1eW<'_, AdcintselSpec> {
        Int1eW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADCINT2 EOC Source Select"]
    #[inline(always)]
    pub fn int2sel(&mut self) -> Int2selW<'_, AdcintselSpec> {
        Int2selW::new(self, 8)
    }
    #[doc = "Bit 14 - ADCINT2 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int2cont(&mut self) -> Int2contW<'_, AdcintselSpec> {
        Int2contW::new(self, 14)
    }
    #[doc = "Bit 15 - ADCINT2 Interrupt Enable"]
    #[inline(always)]
    pub fn int2e(&mut self) -> Int2eW<'_, AdcintselSpec> {
        Int2eW::new(self, 15)
    }
    #[doc = "Bits 16:19 - ADCINT3 EOC Source Select"]
    #[inline(always)]
    pub fn int3sel(&mut self) -> Int3selW<'_, AdcintselSpec> {
        Int3selW::new(self, 16)
    }
    #[doc = "Bit 22 - ADCINT3 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int3cont(&mut self) -> Int3contW<'_, AdcintselSpec> {
        Int3contW::new(self, 22)
    }
    #[doc = "Bit 23 - ADCINT3 Interrupt Enable"]
    #[inline(always)]
    pub fn int3e(&mut self) -> Int3eW<'_, AdcintselSpec> {
        Int3eW::new(self, 23)
    }
    #[doc = "Bits 24:27 - ADCINT4 EOC Source Select"]
    #[inline(always)]
    pub fn int4sel(&mut self) -> Int4selW<'_, AdcintselSpec> {
        Int4selW::new(self, 24)
    }
    #[doc = "Bit 30 - ADCINT4 Continue to Interrupt Mode"]
    #[inline(always)]
    pub fn int4cont(&mut self) -> Int4contW<'_, AdcintselSpec> {
        Int4contW::new(self, 30)
    }
    #[doc = "Bit 31 - ADCINT4 Interrupt Enable"]
    #[inline(always)]
    pub fn int4e(&mut self) -> Int4eW<'_, AdcintselSpec> {
        Int4eW::new(self, 31)
    }
}
#[doc = "ADC Interrupt 1, 2, 3 and 4 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcintsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcintsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcintselSpec;
impl crate::RegisterSpec for AdcintselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcintsel::R`](R) reader structure"]
impl crate::Readable for AdcintselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcintsel::W`](W) writer structure"]
impl crate::Writable for AdcintselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCINTSEL to value 0"]
impl crate::Resettable for AdcintselSpec {}
