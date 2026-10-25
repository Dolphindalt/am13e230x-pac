#[doc = "Register `MCLKCFG` reader"]
pub type R = crate::R<MclkcfgSpec>;
#[doc = "Register `MCLKCFG` writer"]
pub type W = crate::W<MclkcfgSpec>;
#[doc = "USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usehsclk {
    #[doc = "0: MCLK will not use the high speed clock (HSCLK)"]
    Disable = 0,
    #[doc = "1: MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    Enable = 1,
}
impl From<Usehsclk> for bool {
    #[inline(always)]
    fn from(variant: Usehsclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEHSCLK` reader - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
pub type UsehsclkR = crate::BitReader<Usehsclk>;
impl UsehsclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usehsclk {
        match self.bits {
            false => Usehsclk::Disable,
            true => Usehsclk::Enable,
        }
    }
    #[doc = "MCLK will not use the high speed clock (HSCLK)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usehsclk::Disable
    }
    #[doc = "MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usehsclk::Enable
    }
}
#[doc = "Field `USEHSCLK` writer - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
pub type UsehsclkW<'a, REG> = crate::BitWriter<'a, REG, Usehsclk>;
impl<'a, REG> UsehsclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK will not use the high speed clock (HSCLK)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usehsclk::Disable)
    }
    #[doc = "MCLK will use the high speed clock (HSCLK) in RUN and SLEEP mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usehsclk::Enable)
    }
}
#[doc = "LFCLK is not an MCLK sourcei in PD1, tied 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselfclk {
    #[doc = "0: MCLK will not use the low frequency clock (LFCLK)"]
    Disable = 0,
    #[doc = "1: MCLK will use the low frequency clock (LFCLK)"]
    Enable = 1,
}
impl From<Uselfclk> for bool {
    #[inline(always)]
    fn from(variant: Uselfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USELFCLK` reader - LFCLK is not an MCLK sourcei in PD1, tied 0."]
pub type UselfclkR = crate::BitReader<Uselfclk>;
impl UselfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uselfclk {
        match self.bits {
            false => Uselfclk::Disable,
            true => Uselfclk::Enable,
        }
    }
    #[doc = "MCLK will not use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uselfclk::Disable
    }
    #[doc = "MCLK will use the low frequency clock (LFCLK)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uselfclk::Enable
    }
}
#[doc = "STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopclkstby {
    #[doc = "0: ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    Disable = 0,
    #[doc = "1: ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    Enable = 1,
}
impl From<Stopclkstby> for bool {
    #[inline(always)]
    fn from(variant: Stopclkstby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCLKSTBY` reader - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
pub type StopclkstbyR = crate::BitReader<Stopclkstby>;
impl StopclkstbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopclkstby {
        match self.bits {
            false => Stopclkstby::Disable,
            true => Stopclkstby::Enable,
        }
    }
    #[doc = "ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopclkstby::Disable
    }
    #[doc = "ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopclkstby::Enable
    }
}
#[doc = "Field `STOPCLKSTBY` writer - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
pub type StopclkstbyW<'a, REG> = crate::BitWriter<'a, REG, Stopclkstby>;
impl<'a, REG> StopclkstbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ULPCLK/LFCLK runs to all PD0 peripherals in STANDBY mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopclkstby::Disable)
    }
    #[doc = "ULPCLK/LFCLK is disabled to all peripherals in STANDBY mode except TIMG0 and TIMG1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopclkstby::Enable)
    }
}
#[doc = "MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkdeadchk {
    #[doc = "0: The MCLK dead check monitor is disabled"]
    Disable = 0,
    #[doc = "1: The MCLK dead check monitor is enabled"]
    Enable = 1,
}
impl From<Mclkdeadchk> for bool {
    #[inline(always)]
    fn from(variant: Mclkdeadchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKDEADCHK` reader - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
pub type MclkdeadchkR = crate::BitReader<Mclkdeadchk>;
impl MclkdeadchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkdeadchk {
        match self.bits {
            false => Mclkdeadchk::Disable,
            true => Mclkdeadchk::Enable,
        }
    }
    #[doc = "The MCLK dead check monitor is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mclkdeadchk::Disable
    }
    #[doc = "The MCLK dead check monitor is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mclkdeadchk::Enable
    }
}
#[doc = "Field `MCLKDEADCHK` writer - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
pub type MclkdeadchkW<'a, REG> = crate::BitWriter<'a, REG, Mclkdeadchk>;
impl<'a, REG> MclkdeadchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK dead check monitor is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdeadchk::Disable)
    }
    #[doc = "The MCLK dead check monitor is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdeadchk::Enable)
    }
}
#[doc = "MCLK Divider Configuration bits \\[1:0\\] are defined as MCLK4 is Bypass, MCLK2 is Bypass\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mclkdivcfg {
    #[doc = "0: MCLK: No Divide, MCLK2: No Divide, MCLK4: No Divide"]
    Ratio1_1_1 = 0,
    #[doc = "1: MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 2"]
    Ratio1_1_2 = 1,
    #[doc = "3: MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 4"]
    Ratio1_1_4 = 3,
    #[doc = "5: MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: Divide MCLK by 2"]
    Ratio1_2_2 = 5,
    #[doc = "7: MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: MCLK by 4"]
    Ratio1_2_4 = 7,
}
impl From<Mclkdivcfg> for u8 {
    #[inline(always)]
    fn from(variant: Mclkdivcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mclkdivcfg {
    type Ux = u8;
}
impl crate::IsEnum for Mclkdivcfg {}
#[doc = "Field `MCLKDIVCFG` reader - MCLK Divider Configuration bits \\[1:0\\] are defined as MCLK4 is Bypass, MCLK2 is Bypass"]
pub type MclkdivcfgR = crate::FieldReader<Mclkdivcfg>;
impl MclkdivcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mclkdivcfg> {
        match self.bits {
            0 => Some(Mclkdivcfg::Ratio1_1_1),
            1 => Some(Mclkdivcfg::Ratio1_1_2),
            3 => Some(Mclkdivcfg::Ratio1_1_4),
            5 => Some(Mclkdivcfg::Ratio1_2_2),
            7 => Some(Mclkdivcfg::Ratio1_2_4),
            _ => None,
        }
    }
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: No Divide"]
    #[inline(always)]
    pub fn is_ratio_1_1_1(&self) -> bool {
        *self == Mclkdivcfg::Ratio1_1_1
    }
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 2"]
    #[inline(always)]
    pub fn is_ratio_1_1_2(&self) -> bool {
        *self == Mclkdivcfg::Ratio1_1_2
    }
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 4"]
    #[inline(always)]
    pub fn is_ratio_1_1_4(&self) -> bool {
        *self == Mclkdivcfg::Ratio1_1_4
    }
    #[doc = "MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: Divide MCLK by 2"]
    #[inline(always)]
    pub fn is_ratio_1_2_2(&self) -> bool {
        *self == Mclkdivcfg::Ratio1_2_2
    }
    #[doc = "MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: MCLK by 4"]
    #[inline(always)]
    pub fn is_ratio_1_2_4(&self) -> bool {
        *self == Mclkdivcfg::Ratio1_2_4
    }
}
#[doc = "Field `MCLKDIVCFG` writer - MCLK Divider Configuration bits \\[1:0\\] are defined as MCLK4 is Bypass, MCLK2 is Bypass"]
pub type MclkdivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mclkdivcfg, crate::Safe>;
impl<'a, REG> MclkdivcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: No Divide"]
    #[inline(always)]
    pub fn ratio_1_1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdivcfg::Ratio1_1_1)
    }
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 2"]
    #[inline(always)]
    pub fn ratio_1_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdivcfg::Ratio1_1_2)
    }
    #[doc = "MCLK: No Divide, MCLK2: No Divide, MCLK4: Divide MCLK by 4"]
    #[inline(always)]
    pub fn ratio_1_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdivcfg::Ratio1_1_4)
    }
    #[doc = "MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: Divide MCLK by 2"]
    #[inline(always)]
    pub fn ratio_1_2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdivcfg::Ratio1_2_2)
    }
    #[doc = "MCLK: No Divide, MCLK2: Divide MCLK by 2, MCLK4: MCLK by 4"]
    #[inline(always)]
    pub fn ratio_1_2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkdivcfg::Ratio1_2_4)
    }
}
impl R {
    #[doc = "Bit 16 - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
    #[inline(always)]
    pub fn usehsclk(&self) -> UsehsclkR {
        UsehsclkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - LFCLK is not an MCLK sourcei in PD1, tied 0."]
    #[inline(always)]
    pub fn uselfclk(&self) -> UselfclkR {
        UselfclkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
    #[inline(always)]
    pub fn stopclkstby(&self) -> StopclkstbyR {
        StopclkstbyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
    #[inline(always)]
    pub fn mclkdeadchk(&self) -> MclkdeadchkR {
        MclkdeadchkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - MCLK Divider Configuration bits \\[1:0\\] are defined as MCLK4 is Bypass, MCLK2 is Bypass"]
    #[inline(always)]
    pub fn mclkdivcfg(&self) -> MclkdivcfgR {
        MclkdivcfgR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCLKCFG")
            .field("usehsclk", &self.usehsclk())
            .field("uselfclk", &self.uselfclk())
            .field("mclkdivcfg", &self.mclkdivcfg())
            .field("stopclkstby", &self.stopclkstby())
            .field("mclkdeadchk", &self.mclkdeadchk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - USEHSCLK, together with USELFCLK, sets the MCLK source policy. Set USEHSCLK to use HSCLK (HFCLK or SYSPLL) as the MCLK source in RUN and SLEEP modes."]
    #[inline(always)]
    pub fn usehsclk(&mut self) -> UsehsclkW<'_, MclkcfgSpec> {
        UsehsclkW::new(self, 16)
    }
    #[doc = "Bit 21 - STOPCLKSTBY sets the STANDBY mode policy (STANDBY0 or STANDBY1). When set, ULPCLK and LFCLK are disabled to all peripherals in STANDBY mode, with the exception of TIMG0 and TIMG1 which continue to run. Wake-up is only possible via an asynchronous fast clock request."]
    #[inline(always)]
    pub fn stopclkstby(&mut self) -> StopclkstbyW<'_, MclkcfgSpec> {
        StopclkstbyW::new(self, 21)
    }
    #[doc = "Bit 22 - MCLKDEADCHK enables or disables the continuous MCLK dead check monitor. LFCLK must be running before MCLKDEADCHK is enabled."]
    #[inline(always)]
    pub fn mclkdeadchk(&mut self) -> MclkdeadchkW<'_, MclkcfgSpec> {
        MclkdeadchkW::new(self, 22)
    }
    #[doc = "Bits 24:26 - MCLK Divider Configuration bits \\[1:0\\] are defined as MCLK4 is Bypass, MCLK2 is Bypass"]
    #[inline(always)]
    pub fn mclkdivcfg(&mut self) -> MclkdivcfgW<'_, MclkcfgSpec> {
        MclkdivcfgW::new(self, 24)
    }
}
#[doc = "Main clock (MCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkcfgSpec;
impl crate::RegisterSpec for MclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkcfg::R`](R) reader structure"]
impl crate::Readable for MclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mclkcfg::W`](W) writer structure"]
impl crate::Writable for MclkcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MCLKCFG to value 0x0700_0200"]
impl crate::Resettable for MclkcfgSpec {
    const RESET_VALUE: u32 = 0x0700_0200;
}
