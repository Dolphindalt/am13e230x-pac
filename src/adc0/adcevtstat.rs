#[doc = "Register `ADCEVTSTAT` reader"]
pub type R = crate::R<AdcevtstatSpec>;
#[doc = "Register `ADCEVTSTAT` writer"]
pub type W = crate::W<AdcevtstatSpec>;
#[doc = "Field `PPB1TRIPHI` reader - Post Processing Block 1 Trip High Flag"]
pub type Ppb1triphiR = crate::BitReader;
#[doc = "Field `PPB1TRIPLO` reader - Post Processing Block 1 Trip Low Flag"]
pub type Ppb1triploR = crate::BitReader;
#[doc = "Field `PPB1ZERO` reader - Post Processing Block 1 Zero Crossing Flag"]
pub type Ppb1zeroR = crate::BitReader;
#[doc = "Field `PPB1INLIMIT` reader - Post Processing Block 1 Within trip limit Flag"]
pub type Ppb1inlimitR = crate::BitReader;
#[doc = "Field `PPB2TRIPHI` reader - Post Processing Block 2 Trip High Flag"]
pub type Ppb2triphiR = crate::BitReader;
#[doc = "Field `PPB2TRIPLO` reader - Post Processing Block 2 Trip Low Flag"]
pub type Ppb2triploR = crate::BitReader;
#[doc = "Field `PPB2ZERO` reader - Post Processing Block 2 Zero Crossing Flag"]
pub type Ppb2zeroR = crate::BitReader;
#[doc = "Field `PPB2INLIMIT` reader - Post Processing Block 2 Within trip limit Flag"]
pub type Ppb2inlimitR = crate::BitReader;
#[doc = "Field `PPB3TRIPHI` reader - Post Processing Block 3 Trip High Flag"]
pub type Ppb3triphiR = crate::BitReader;
#[doc = "Field `PPB3TRIPLO` reader - Post Processing Block 3 Trip Low Flag"]
pub type Ppb3triploR = crate::BitReader;
#[doc = "Field `PPB3ZERO` reader - Post Processing Block 3 Zero Crossing Flag"]
pub type Ppb3zeroR = crate::BitReader;
#[doc = "Field `PPB3INLIMIT` reader - Post Processing Block 3 Within trip limit Flag"]
pub type Ppb3inlimitR = crate::BitReader;
#[doc = "Field `PPB4TRIPHI` reader - Post Processing Block 4 Trip High Flag"]
pub type Ppb4triphiR = crate::BitReader;
#[doc = "Field `PPB4TRIPLO` reader - Post Processing Block 4 Trip Low Flag"]
pub type Ppb4triploR = crate::BitReader;
#[doc = "Field `PPB4ZERO` reader - Post Processing Block 4 Zero Crossing Flag"]
pub type Ppb4zeroR = crate::BitReader;
#[doc = "Field `PPB4INLIMIT` reader - Post Processing Block 4 Within trip limit Flag"]
pub type Ppb4inlimitR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Post Processing Block 1 Trip High Flag"]
    #[inline(always)]
    pub fn ppb1triphi(&self) -> Ppb1triphiR {
        Ppb1triphiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Post Processing Block 1 Trip Low Flag"]
    #[inline(always)]
    pub fn ppb1triplo(&self) -> Ppb1triploR {
        Ppb1triploR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Post Processing Block 1 Zero Crossing Flag"]
    #[inline(always)]
    pub fn ppb1zero(&self) -> Ppb1zeroR {
        Ppb1zeroR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Post Processing Block 1 Within trip limit Flag"]
    #[inline(always)]
    pub fn ppb1inlimit(&self) -> Ppb1inlimitR {
        Ppb1inlimitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Post Processing Block 2 Trip High Flag"]
    #[inline(always)]
    pub fn ppb2triphi(&self) -> Ppb2triphiR {
        Ppb2triphiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Post Processing Block 2 Trip Low Flag"]
    #[inline(always)]
    pub fn ppb2triplo(&self) -> Ppb2triploR {
        Ppb2triploR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Post Processing Block 2 Zero Crossing Flag"]
    #[inline(always)]
    pub fn ppb2zero(&self) -> Ppb2zeroR {
        Ppb2zeroR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Post Processing Block 2 Within trip limit Flag"]
    #[inline(always)]
    pub fn ppb2inlimit(&self) -> Ppb2inlimitR {
        Ppb2inlimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Post Processing Block 3 Trip High Flag"]
    #[inline(always)]
    pub fn ppb3triphi(&self) -> Ppb3triphiR {
        Ppb3triphiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Post Processing Block 3 Trip Low Flag"]
    #[inline(always)]
    pub fn ppb3triplo(&self) -> Ppb3triploR {
        Ppb3triploR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Post Processing Block 3 Zero Crossing Flag"]
    #[inline(always)]
    pub fn ppb3zero(&self) -> Ppb3zeroR {
        Ppb3zeroR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Post Processing Block 3 Within trip limit Flag"]
    #[inline(always)]
    pub fn ppb3inlimit(&self) -> Ppb3inlimitR {
        Ppb3inlimitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Post Processing Block 4 Trip High Flag"]
    #[inline(always)]
    pub fn ppb4triphi(&self) -> Ppb4triphiR {
        Ppb4triphiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Post Processing Block 4 Trip Low Flag"]
    #[inline(always)]
    pub fn ppb4triplo(&self) -> Ppb4triploR {
        Ppb4triploR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Post Processing Block 4 Zero Crossing Flag"]
    #[inline(always)]
    pub fn ppb4zero(&self) -> Ppb4zeroR {
        Ppb4zeroR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Post Processing Block 4 Within trip limit Flag"]
    #[inline(always)]
    pub fn ppb4inlimit(&self) -> Ppb4inlimitR {
        Ppb4inlimitR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCEVTSTAT")
            .field("ppb1triphi", &self.ppb1triphi())
            .field("ppb1triplo", &self.ppb1triplo())
            .field("ppb1zero", &self.ppb1zero())
            .field("ppb1inlimit", &self.ppb1inlimit())
            .field("ppb2triphi", &self.ppb2triphi())
            .field("ppb2triplo", &self.ppb2triplo())
            .field("ppb2zero", &self.ppb2zero())
            .field("ppb2inlimit", &self.ppb2inlimit())
            .field("ppb3triphi", &self.ppb3triphi())
            .field("ppb3triplo", &self.ppb3triplo())
            .field("ppb3zero", &self.ppb3zero())
            .field("ppb3inlimit", &self.ppb3inlimit())
            .field("ppb4triphi", &self.ppb4triphi())
            .field("ppb4triplo", &self.ppb4triplo())
            .field("ppb4zero", &self.ppb4zero())
            .field("ppb4inlimit", &self.ppb4inlimit())
            .finish()
    }
}
impl W {}
#[doc = "ADC Event Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcevtstatSpec;
impl crate::RegisterSpec for AdcevtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcevtstat::R`](R) reader structure"]
impl crate::Readable for AdcevtstatSpec {}
#[doc = "`write(|w| ..)` method takes [`adcevtstat::W`](W) writer structure"]
impl crate::Writable for AdcevtstatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCEVTSTAT to value 0"]
impl crate::Resettable for AdcevtstatSpec {}
