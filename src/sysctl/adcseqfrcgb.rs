#[doc = "Register `ADCSEQFRCGB` reader"]
pub type R = crate::R<AdcseqfrcgbSpec>;
#[doc = "Register `ADCSEQFRCGB` writer"]
pub type W = crate::W<AdcseqfrcgbSpec>;
#[doc = "Field `SEQ0` reader - Generate synchronous SW trigger for SEQ0"]
pub type Seq0R = crate::BitReader;
#[doc = "Field `SEQ0` writer - Generate synchronous SW trigger for SEQ0"]
pub type Seq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ1` reader - Generate synchronous SW trigger for SEQ1"]
pub type Seq1R = crate::BitReader;
#[doc = "Field `SEQ1` writer - Generate synchronous SW trigger for SEQ1"]
pub type Seq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ2` reader - Generate synchronous SW trigger for SEQ2"]
pub type Seq2R = crate::BitReader;
#[doc = "Field `SEQ2` writer - Generate synchronous SW trigger for SEQ2"]
pub type Seq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ3` reader - Generate synchronous SW trigger for SEQ3"]
pub type Seq3R = crate::BitReader;
#[doc = "Field `SEQ3` writer - Generate synchronous SW trigger for SEQ3"]
pub type Seq3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Generate synchronous SW trigger for SEQ0"]
    #[inline(always)]
    pub fn seq0(&self) -> Seq0R {
        Seq0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate synchronous SW trigger for SEQ1"]
    #[inline(always)]
    pub fn seq1(&self) -> Seq1R {
        Seq1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate synchronous SW trigger for SEQ2"]
    #[inline(always)]
    pub fn seq2(&self) -> Seq2R {
        Seq2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generate synchronous SW trigger for SEQ3"]
    #[inline(always)]
    pub fn seq3(&self) -> Seq3R {
        Seq3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQFRCGB")
            .field("seq3", &self.seq3())
            .field("seq2", &self.seq2())
            .field("seq0", &self.seq0())
            .field("seq1", &self.seq1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Generate synchronous SW trigger for SEQ0"]
    #[inline(always)]
    pub fn seq0(&mut self) -> Seq0W<'_, AdcseqfrcgbSpec> {
        Seq0W::new(self, 0)
    }
    #[doc = "Bit 1 - Generate synchronous SW trigger for SEQ1"]
    #[inline(always)]
    pub fn seq1(&mut self) -> Seq1W<'_, AdcseqfrcgbSpec> {
        Seq1W::new(self, 1)
    }
    #[doc = "Bit 2 - Generate synchronous SW trigger for SEQ2"]
    #[inline(always)]
    pub fn seq2(&mut self) -> Seq2W<'_, AdcseqfrcgbSpec> {
        Seq2W::new(self, 2)
    }
    #[doc = "Bit 3 - Generate synchronous SW trigger for SEQ3"]
    #[inline(always)]
    pub fn seq3(&mut self) -> Seq3W<'_, AdcseqfrcgbSpec> {
        Seq3W::new(self, 3)
    }
}
#[doc = "ADC Global Sequence Force\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqfrcgb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqfrcgb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcseqfrcgbSpec;
impl crate::RegisterSpec for AdcseqfrcgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseqfrcgb::R`](R) reader structure"]
impl crate::Readable for AdcseqfrcgbSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseqfrcgb::W`](W) writer structure"]
impl crate::Writable for AdcseqfrcgbSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQFRCGB to value 0"]
impl crate::Resettable for AdcseqfrcgbSpec {}
