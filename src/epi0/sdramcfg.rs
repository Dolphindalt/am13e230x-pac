#[doc = "Register `SDRAMCFG` reader"]
pub type R = crate::R<SdramcfgSpec>;
#[doc = "Register `SDRAMCFG` writer"]
pub type W = crate::W<SdramcfgSpec>;
#[doc = "Size of SDRAM\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: 64 megabits (8MB)"]
    SdramcfgSize64mbits = 0,
    #[doc = "1: 128 megabits (16MB)"]
    SdramcfgSize128mbits = 1,
    #[doc = "2: 256 megabits (32MB)"]
    SdramcfgSize256mbits = 2,
    #[doc = "3: 512 megabits (64MB)"]
    SdramcfgSize512mbits = 3,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Size of SDRAM"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Size {
        match self.bits {
            0 => Size::SdramcfgSize64mbits,
            1 => Size::SdramcfgSize128mbits,
            2 => Size::SdramcfgSize256mbits,
            3 => Size::SdramcfgSize512mbits,
            _ => unreachable!(),
        }
    }
    #[doc = "64 megabits (8MB)"]
    #[inline(always)]
    pub fn is_sdramcfg_size_64mbits(&self) -> bool {
        *self == Size::SdramcfgSize64mbits
    }
    #[doc = "128 megabits (16MB)"]
    #[inline(always)]
    pub fn is_sdramcfg_size_128mbits(&self) -> bool {
        *self == Size::SdramcfgSize128mbits
    }
    #[doc = "256 megabits (32MB)"]
    #[inline(always)]
    pub fn is_sdramcfg_size_256mbits(&self) -> bool {
        *self == Size::SdramcfgSize256mbits
    }
    #[doc = "512 megabits (64MB)"]
    #[inline(always)]
    pub fn is_sdramcfg_size_512mbits(&self) -> bool {
        *self == Size::SdramcfgSize512mbits
    }
}
#[doc = "Field `SIZE` writer - Size of SDRAM"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Size, crate::Safe>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 megabits (8MB)"]
    #[inline(always)]
    pub fn sdramcfg_size_64mbits(self) -> &'a mut crate::W<REG> {
        self.variant(Size::SdramcfgSize64mbits)
    }
    #[doc = "128 megabits (16MB)"]
    #[inline(always)]
    pub fn sdramcfg_size_128mbits(self) -> &'a mut crate::W<REG> {
        self.variant(Size::SdramcfgSize128mbits)
    }
    #[doc = "256 megabits (32MB)"]
    #[inline(always)]
    pub fn sdramcfg_size_256mbits(self) -> &'a mut crate::W<REG> {
        self.variant(Size::SdramcfgSize256mbits)
    }
    #[doc = "512 megabits (64MB)"]
    #[inline(always)]
    pub fn sdramcfg_size_512mbits(self) -> &'a mut crate::W<REG> {
        self.variant(Size::SdramcfgSize512mbits)
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleep {
    #[doc = "0: No effect."]
    None = 0,
    #[doc = "1: The SDRAM is put into low power state, but is self-refreshed."]
    On = 1,
}
impl From<Sleep> for bool {
    #[inline(always)]
    fn from(variant: Sleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SleepR = crate::BitReader<Sleep>;
impl SleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleep {
        match self.bits {
            false => Sleep::None,
            true => Sleep::On,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sleep::None
    }
    #[doc = "The SDRAM is put into low power state, but is self-refreshed."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sleep::On
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG, Sleep>;
impl<'a, REG> SleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::None)
    }
    #[doc = "The SDRAM is put into low power state, but is self-refreshed."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::On)
    }
}
#[doc = "Field `RFSH` reader - Refresh Counter"]
pub type RfshR = crate::FieldReader<u16>;
#[doc = "Field `RFSH` writer - Refresh Counter"]
pub type RfshW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
#[doc = "EPI Frequency Range.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freq {
    #[doc = "0: 0 to 15 MHz"]
    Sdramcfg0To15Mhz = 0,
    #[doc = "1: 15 to 30 MHz"]
    Sdramcfg15To30Mhz = 1,
    #[doc = "2: 30 to 50 MHz"]
    Sdramcfg30To50Mhz = 2,
    #[doc = "3: 50 to 100 MHz"]
    Sdramcfg50To100Mhz = 3,
}
impl From<Freq> for u8 {
    #[inline(always)]
    fn from(variant: Freq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freq {
    type Ux = u8;
}
impl crate::IsEnum for Freq {}
#[doc = "Field `FREQ` reader - EPI Frequency Range."]
pub type FreqR = crate::FieldReader<Freq>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freq {
        match self.bits {
            0 => Freq::Sdramcfg0To15Mhz,
            1 => Freq::Sdramcfg15To30Mhz,
            2 => Freq::Sdramcfg30To50Mhz,
            3 => Freq::Sdramcfg50To100Mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "0 to 15 MHz"]
    #[inline(always)]
    pub fn is_sdramcfg_0_to_15_mhz(&self) -> bool {
        *self == Freq::Sdramcfg0To15Mhz
    }
    #[doc = "15 to 30 MHz"]
    #[inline(always)]
    pub fn is_sdramcfg_15_to_30_mhz(&self) -> bool {
        *self == Freq::Sdramcfg15To30Mhz
    }
    #[doc = "30 to 50 MHz"]
    #[inline(always)]
    pub fn is_sdramcfg_30_to_50_mhz(&self) -> bool {
        *self == Freq::Sdramcfg30To50Mhz
    }
    #[doc = "50 to 100 MHz"]
    #[inline(always)]
    pub fn is_sdramcfg_50_to_100_mhz(&self) -> bool {
        *self == Freq::Sdramcfg50To100Mhz
    }
}
#[doc = "Field `FREQ` writer - EPI Frequency Range."]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freq, crate::Safe>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 to 15 MHz"]
    #[inline(always)]
    pub fn sdramcfg_0_to_15_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sdramcfg0To15Mhz)
    }
    #[doc = "15 to 30 MHz"]
    #[inline(always)]
    pub fn sdramcfg_15_to_30_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sdramcfg15To30Mhz)
    }
    #[doc = "30 to 50 MHz"]
    #[inline(always)]
    pub fn sdramcfg_30_to_50_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sdramcfg30To50Mhz)
    }
    #[doc = "50 to 100 MHz"]
    #[inline(always)]
    pub fn sdramcfg_50_to_100_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Sdramcfg50To100Mhz)
    }
}
impl R {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn rfsh(&self) -> RfshR {
        RfshR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - EPI Frequency Range."]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDRAMCFG")
            .field("size", &self.size())
            .field("sleep", &self.sleep())
            .field("rfsh", &self.rfsh())
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, SdramcfgSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<'_, SdramcfgSpec> {
        SleepW::new(self, 9)
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn rfsh(&mut self) -> RfshW<'_, SdramcfgSpec> {
        RfshW::new(self, 16)
    }
    #[doc = "Bits 30:31 - EPI Frequency Range."]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, SdramcfgSpec> {
        FreqW::new(self, 30)
    }
}
#[doc = "EPI SDRAM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sdramcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdramcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdramcfgSpec;
impl crate::RegisterSpec for SdramcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdramcfg::R`](R) reader structure"]
impl crate::Readable for SdramcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sdramcfg::W`](W) writer structure"]
impl crate::Writable for SdramcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SDRAMCFG to value 0"]
impl crate::Resettable for SdramcfgSpec {}
