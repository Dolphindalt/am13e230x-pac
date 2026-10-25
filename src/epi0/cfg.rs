#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: General Purpose. General-Purpose mode. Control, address, and data pins are configured using the EPIGPCFG and EPIGPCFG2 registers."]
    Gp = 0,
    #[doc = "1: SDRAM. Supports SDR SDRAM. Control, address, and data pins are configured using the EPISDRAMCFG register."]
    Sdram = 1,
    #[doc = "2: 8-Bit Host-Bus (HB8). Host-bus 8-bit interface (also known as the MCU interface). Control, address, and data pins are configured using the EPIHB8CFG and EPIHB8CFG2 registers."]
    Hb8 = 2,
    #[doc = "3: 16-Bit Host-Bus (HB16). Host-bus 16-bit interface (standard SRAM). Control, address, and data pins are configured using the EPIHB16CFG and EPIHB16CFG2 registers."]
    Hb16 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode Select"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Gp),
            1 => Some(Mode::Sdram),
            2 => Some(Mode::Hb8),
            3 => Some(Mode::Hb16),
            _ => None,
        }
    }
    #[doc = "General Purpose. General-Purpose mode. Control, address, and data pins are configured using the EPIGPCFG and EPIGPCFG2 registers."]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == Mode::Gp
    }
    #[doc = "SDRAM. Supports SDR SDRAM. Control, address, and data pins are configured using the EPISDRAMCFG register."]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == Mode::Sdram
    }
    #[doc = "8-Bit Host-Bus (HB8). Host-bus 8-bit interface (also known as the MCU interface). Control, address, and data pins are configured using the EPIHB8CFG and EPIHB8CFG2 registers."]
    #[inline(always)]
    pub fn is_hb8(&self) -> bool {
        *self == Mode::Hb8
    }
    #[doc = "16-Bit Host-Bus (HB16). Host-bus 16-bit interface (standard SRAM). Control, address, and data pins are configured using the EPIHB16CFG and EPIHB16CFG2 registers."]
    #[inline(always)]
    pub fn is_hb16(&self) -> bool {
        *self == Mode::Hb16
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General Purpose. General-Purpose mode. Control, address, and data pins are configured using the EPIGPCFG and EPIGPCFG2 registers."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Gp)
    }
    #[doc = "SDRAM. Supports SDR SDRAM. Control, address, and data pins are configured using the EPISDRAMCFG register."]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sdram)
    }
    #[doc = "8-Bit Host-Bus (HB8). Host-bus 8-bit interface (also known as the MCU interface). Control, address, and data pins are configured using the EPIHB8CFG and EPIHB8CFG2 registers."]
    #[inline(always)]
    pub fn hb8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hb8)
    }
    #[doc = "16-Bit Host-Bus (HB16). Host-bus 16-bit interface (standard SRAM). Control, address, and data pins are configured using the EPIHB16CFG and EPIHB16CFG2 registers."]
    #[inline(always)]
    pub fn hb16(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hb16)
    }
}
#[doc = "Block Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blken {
    #[doc = "0: The EPI controller is disabled."]
    Disable = 0,
    #[doc = "1: The EPI controller is enabled."]
    Enable = 1,
}
impl From<Blken> for bool {
    #[inline(always)]
    fn from(variant: Blken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKEN` reader - Block Enable"]
pub type BlkenR = crate::BitReader<Blken>;
impl BlkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blken {
        match self.bits {
            false => Blken::Disable,
            true => Blken::Enable,
        }
    }
    #[doc = "The EPI controller is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blken::Disable
    }
    #[doc = "The EPI controller is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blken::Enable
    }
}
#[doc = "Field `BLKEN` writer - Block Enable"]
pub type BlkenW<'a, REG> = crate::BitWriter<'a, REG, Blken>;
impl<'a, REG> BlkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EPI controller is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blken::Disable)
    }
    #[doc = "The EPI controller is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blken::Enable)
    }
}
#[doc = "Integer Clock Divider Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intdiv {
    #[doc = "0: EPIBAUD register values create formula clock divide."]
    Formula = 0,
    #[doc = "1: EPIBAUD register values create integer clock divide."]
    Integer = 1,
}
impl From<Intdiv> for bool {
    #[inline(always)]
    fn from(variant: Intdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTDIV` reader - Integer Clock Divider Enable"]
pub type IntdivR = crate::BitReader<Intdiv>;
impl IntdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intdiv {
        match self.bits {
            false => Intdiv::Formula,
            true => Intdiv::Integer,
        }
    }
    #[doc = "EPIBAUD register values create formula clock divide."]
    #[inline(always)]
    pub fn is_formula(&self) -> bool {
        *self == Intdiv::Formula
    }
    #[doc = "EPIBAUD register values create integer clock divide."]
    #[inline(always)]
    pub fn is_integer(&self) -> bool {
        *self == Intdiv::Integer
    }
}
#[doc = "Field `INTDIV` writer - Integer Clock Divider Enable"]
pub type IntdivW<'a, REG> = crate::BitWriter<'a, REG, Intdiv>;
impl<'a, REG> IntdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EPIBAUD register values create formula clock divide."]
    #[inline(always)]
    pub fn formula(self) -> &'a mut crate::W<REG> {
        self.variant(Intdiv::Formula)
    }
    #[doc = "EPIBAUD register values create integer clock divide."]
    #[inline(always)]
    pub fn integer(self) -> &'a mut crate::W<REG> {
        self.variant(Intdiv::Integer)
    }
}
impl R {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn blken(&self) -> BlkenR {
        BlkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn intdiv(&self) -> IntdivR {
        IntdivR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("mode", &self.mode())
            .field("blken", &self.blken())
            .field("intdiv", &self.intdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn blken(&mut self) -> BlkenW<'_, CfgSpec> {
        BlkenW::new(self, 4)
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn intdiv(&mut self) -> IntdivW<'_, CfgSpec> {
        IntdivW::new(self, 8)
    }
}
#[doc = "EPI Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
