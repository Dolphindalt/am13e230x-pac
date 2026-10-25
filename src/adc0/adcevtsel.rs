#[doc = "Register `ADCEVTSEL` reader"]
pub type R = crate::R<AdcevtselSpec>;
#[doc = "Register `ADCEVTSEL` writer"]
pub type W = crate::W<AdcevtselSpec>;
#[doc = "Field `PPB1TRIPHI` reader - Post Processing Block 1 Trip High Event Enable"]
pub type Ppb1triphiR = crate::BitReader;
#[doc = "Field `PPB1TRIPHI` writer - Post Processing Block 1 Trip High Event Enable"]
pub type Ppb1triphiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB1TRIPLO` reader - Post Processing Block 1 Trip Low Event Enable"]
pub type Ppb1triploR = crate::BitReader;
#[doc = "Field `PPB1TRIPLO` writer - Post Processing Block 1 Trip Low Event Enable"]
pub type Ppb1triploW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB1ZERO` reader - Post Processing Block 1 Zero Crossing Event Enable"]
pub type Ppb1zeroR = crate::BitReader;
#[doc = "Field `PPB1ZERO` writer - Post Processing Block 1 Zero Crossing Event Enable"]
pub type Ppb1zeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB1INLIMIT` reader - Post Processing Block 1 Within trip limit event enable"]
pub type Ppb1inlimitR = crate::BitReader;
#[doc = "Field `PPB1INLIMIT` writer - Post Processing Block 1 Within trip limit event enable"]
pub type Ppb1inlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB2TRIPHI` reader - Post Processing Block 2 Trip High Event Enable"]
pub type Ppb2triphiR = crate::BitReader;
#[doc = "Field `PPB2TRIPHI` writer - Post Processing Block 2 Trip High Event Enable"]
pub type Ppb2triphiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB2TRIPLO` reader - Post Processing Block 2 Trip Low Event Enable"]
pub type Ppb2triploR = crate::BitReader;
#[doc = "Field `PPB2TRIPLO` writer - Post Processing Block 2 Trip Low Event Enable"]
pub type Ppb2triploW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB2ZERO` reader - Post Processing Block 2 Zero Crossing Event Enable"]
pub type Ppb2zeroR = crate::BitReader;
#[doc = "Field `PPB2ZERO` writer - Post Processing Block 2 Zero Crossing Event Enable"]
pub type Ppb2zeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB2INLIMIT` reader - Post Processing Block 2 Within trip limit event enable"]
pub type Ppb2inlimitR = crate::BitReader;
#[doc = "Field `PPB2INLIMIT` writer - Post Processing Block 2 Within trip limit event enable"]
pub type Ppb2inlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB3TRIPHI` reader - Post Processing Block 3 Trip High Event Enable"]
pub type Ppb3triphiR = crate::BitReader;
#[doc = "Field `PPB3TRIPHI` writer - Post Processing Block 3 Trip High Event Enable"]
pub type Ppb3triphiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB3TRIPLO` reader - Post Processing Block 3 Trip Low Event Enable"]
pub type Ppb3triploR = crate::BitReader;
#[doc = "Field `PPB3TRIPLO` writer - Post Processing Block 3 Trip Low Event Enable"]
pub type Ppb3triploW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB3ZERO` reader - Post Processing Block 3 Zero Crossing Event Enable"]
pub type Ppb3zeroR = crate::BitReader;
#[doc = "Field `PPB3ZERO` writer - Post Processing Block 3 Zero Crossing Event Enable"]
pub type Ppb3zeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB3INLIMIT` reader - Post Processing Block 3 Within trip limit event enable"]
pub type Ppb3inlimitR = crate::BitReader;
#[doc = "Field `PPB3INLIMIT` writer - Post Processing Block 3 Within trip limit event enable"]
pub type Ppb3inlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB4TRIPHI` reader - Post Processing Block 4 Trip High Event Enable"]
pub type Ppb4triphiR = crate::BitReader;
#[doc = "Field `PPB4TRIPHI` writer - Post Processing Block 4 Trip High Event Enable"]
pub type Ppb4triphiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB4TRIPLO` reader - Post Processing Block 4 Trip Low Event Enable"]
pub type Ppb4triploR = crate::BitReader;
#[doc = "Field `PPB4TRIPLO` writer - Post Processing Block 4 Trip Low Event Enable"]
pub type Ppb4triploW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB4ZERO` reader - Post Processing Block 4 Zero Crossing Event Enable"]
pub type Ppb4zeroR = crate::BitReader;
#[doc = "Field `PPB4ZERO` writer - Post Processing Block 4 Zero Crossing Event Enable"]
pub type Ppb4zeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPB4INLIMIT` reader - Post Processing Block 4 Within trip limit event enable"]
pub type Ppb4inlimitR = crate::BitReader;
#[doc = "Field `PPB4INLIMIT` writer - Post Processing Block 4 Within trip limit event enable"]
pub type Ppb4inlimitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Post Processing Block 1 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb1triphi(&self) -> Ppb1triphiR {
        Ppb1triphiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Post Processing Block 1 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb1triplo(&self) -> Ppb1triploR {
        Ppb1triploR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Post Processing Block 1 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb1zero(&self) -> Ppb1zeroR {
        Ppb1zeroR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Post Processing Block 1 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb1inlimit(&self) -> Ppb1inlimitR {
        Ppb1inlimitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Post Processing Block 2 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb2triphi(&self) -> Ppb2triphiR {
        Ppb2triphiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Post Processing Block 2 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb2triplo(&self) -> Ppb2triploR {
        Ppb2triploR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Post Processing Block 2 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb2zero(&self) -> Ppb2zeroR {
        Ppb2zeroR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Post Processing Block 2 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb2inlimit(&self) -> Ppb2inlimitR {
        Ppb2inlimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Post Processing Block 3 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb3triphi(&self) -> Ppb3triphiR {
        Ppb3triphiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Post Processing Block 3 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb3triplo(&self) -> Ppb3triploR {
        Ppb3triploR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Post Processing Block 3 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb3zero(&self) -> Ppb3zeroR {
        Ppb3zeroR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Post Processing Block 3 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb3inlimit(&self) -> Ppb3inlimitR {
        Ppb3inlimitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Post Processing Block 4 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb4triphi(&self) -> Ppb4triphiR {
        Ppb4triphiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Post Processing Block 4 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb4triplo(&self) -> Ppb4triploR {
        Ppb4triploR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Post Processing Block 4 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb4zero(&self) -> Ppb4zeroR {
        Ppb4zeroR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Post Processing Block 4 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb4inlimit(&self) -> Ppb4inlimitR {
        Ppb4inlimitR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCEVTSEL")
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
impl W {
    #[doc = "Bit 0 - Post Processing Block 1 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb1triphi(&mut self) -> Ppb1triphiW<'_, AdcevtselSpec> {
        Ppb1triphiW::new(self, 0)
    }
    #[doc = "Bit 1 - Post Processing Block 1 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb1triplo(&mut self) -> Ppb1triploW<'_, AdcevtselSpec> {
        Ppb1triploW::new(self, 1)
    }
    #[doc = "Bit 2 - Post Processing Block 1 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb1zero(&mut self) -> Ppb1zeroW<'_, AdcevtselSpec> {
        Ppb1zeroW::new(self, 2)
    }
    #[doc = "Bit 3 - Post Processing Block 1 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb1inlimit(&mut self) -> Ppb1inlimitW<'_, AdcevtselSpec> {
        Ppb1inlimitW::new(self, 3)
    }
    #[doc = "Bit 4 - Post Processing Block 2 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb2triphi(&mut self) -> Ppb2triphiW<'_, AdcevtselSpec> {
        Ppb2triphiW::new(self, 4)
    }
    #[doc = "Bit 5 - Post Processing Block 2 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb2triplo(&mut self) -> Ppb2triploW<'_, AdcevtselSpec> {
        Ppb2triploW::new(self, 5)
    }
    #[doc = "Bit 6 - Post Processing Block 2 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb2zero(&mut self) -> Ppb2zeroW<'_, AdcevtselSpec> {
        Ppb2zeroW::new(self, 6)
    }
    #[doc = "Bit 7 - Post Processing Block 2 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb2inlimit(&mut self) -> Ppb2inlimitW<'_, AdcevtselSpec> {
        Ppb2inlimitW::new(self, 7)
    }
    #[doc = "Bit 8 - Post Processing Block 3 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb3triphi(&mut self) -> Ppb3triphiW<'_, AdcevtselSpec> {
        Ppb3triphiW::new(self, 8)
    }
    #[doc = "Bit 9 - Post Processing Block 3 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb3triplo(&mut self) -> Ppb3triploW<'_, AdcevtselSpec> {
        Ppb3triploW::new(self, 9)
    }
    #[doc = "Bit 10 - Post Processing Block 3 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb3zero(&mut self) -> Ppb3zeroW<'_, AdcevtselSpec> {
        Ppb3zeroW::new(self, 10)
    }
    #[doc = "Bit 11 - Post Processing Block 3 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb3inlimit(&mut self) -> Ppb3inlimitW<'_, AdcevtselSpec> {
        Ppb3inlimitW::new(self, 11)
    }
    #[doc = "Bit 12 - Post Processing Block 4 Trip High Event Enable"]
    #[inline(always)]
    pub fn ppb4triphi(&mut self) -> Ppb4triphiW<'_, AdcevtselSpec> {
        Ppb4triphiW::new(self, 12)
    }
    #[doc = "Bit 13 - Post Processing Block 4 Trip Low Event Enable"]
    #[inline(always)]
    pub fn ppb4triplo(&mut self) -> Ppb4triploW<'_, AdcevtselSpec> {
        Ppb4triploW::new(self, 13)
    }
    #[doc = "Bit 14 - Post Processing Block 4 Zero Crossing Event Enable"]
    #[inline(always)]
    pub fn ppb4zero(&mut self) -> Ppb4zeroW<'_, AdcevtselSpec> {
        Ppb4zeroW::new(self, 14)
    }
    #[doc = "Bit 15 - Post Processing Block 4 Within trip limit event enable"]
    #[inline(always)]
    pub fn ppb4inlimit(&mut self) -> Ppb4inlimitW<'_, AdcevtselSpec> {
        Ppb4inlimitW::new(self, 15)
    }
}
#[doc = "ADC Event Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcevtsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcevtsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcevtselSpec;
impl crate::RegisterSpec for AdcevtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcevtsel::R`](R) reader structure"]
impl crate::Readable for AdcevtselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcevtsel::W`](W) writer structure"]
impl crate::Writable for AdcevtselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCEVTSEL to value 0"]
impl crate::Resettable for AdcevtselSpec {}
