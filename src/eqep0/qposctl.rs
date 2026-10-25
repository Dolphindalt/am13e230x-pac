#[doc = "Register `QPOSCTL` reader"]
pub type R = crate::R<QposctlSpec>;
#[doc = "Register `QPOSCTL` writer"]
pub type W = crate::W<QposctlSpec>;
#[doc = "Position compare sync pulse width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pcspw {
    #[doc = "0: 1 * 4 * SYSCLKOUT cycles"]
    Sysclkout4 = 0,
    #[doc = "1: 2 * 4 * SYSCLKOUT cycles"]
    Sysclkout8 = 1,
    #[doc = "4095: 4096 * 4 * SYSCLKOUT cycles"]
    Sysclkout16384 = 4095,
}
impl From<Pcspw> for u16 {
    #[inline(always)]
    fn from(variant: Pcspw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcspw {
    type Ux = u16;
}
impl crate::IsEnum for Pcspw {}
#[doc = "Field `PCSPW` reader - Position compare sync pulse width"]
pub type PcspwR = crate::FieldReader<Pcspw>;
impl PcspwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pcspw> {
        match self.bits {
            0 => Some(Pcspw::Sysclkout4),
            1 => Some(Pcspw::Sysclkout8),
            4095 => Some(Pcspw::Sysclkout16384),
            _ => None,
        }
    }
    #[doc = "1 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn is_sysclkout4(&self) -> bool {
        *self == Pcspw::Sysclkout4
    }
    #[doc = "2 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn is_sysclkout8(&self) -> bool {
        *self == Pcspw::Sysclkout8
    }
    #[doc = "4096 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn is_sysclkout16384(&self) -> bool {
        *self == Pcspw::Sysclkout16384
    }
}
#[doc = "Field `PCSPW` writer - Position compare sync pulse width"]
pub type PcspwW<'a, REG> = crate::FieldWriter<'a, REG, 12, Pcspw, crate::Safe>;
impl<'a, REG> PcspwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "1 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn sysclkout4(self) -> &'a mut crate::W<REG> {
        self.variant(Pcspw::Sysclkout4)
    }
    #[doc = "2 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn sysclkout8(self) -> &'a mut crate::W<REG> {
        self.variant(Pcspw::Sysclkout8)
    }
    #[doc = "4096 * 4 * SYSCLKOUT cycles"]
    #[inline(always)]
    pub fn sysclkout16384(self) -> &'a mut crate::W<REG> {
        self.variant(Pcspw::Sysclkout16384)
    }
}
#[doc = "Position compare enable/disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: Disable position compare unit"]
    Disable = 0,
    #[doc = "1: Enable position compare unit"]
    Enable = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Position compare enable/disable"]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::Disable,
            true => Pce::Enable,
        }
    }
    #[doc = "Disable position compare unit"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pce::Disable
    }
    #[doc = "Enable position compare unit"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pce::Enable
    }
}
#[doc = "Field `PCE` writer - Position compare enable/disable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG, Pce>;
impl<'a, REG> PceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable position compare unit"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Disable)
    }
    #[doc = "Enable position compare unit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Enable)
    }
}
#[doc = "Polarity of sync output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcpol {
    #[doc = "0: Active HIGH pulse output"]
    High = 0,
    #[doc = "1: Active LOW pulse output"]
    Low = 1,
}
impl From<Pcpol> for bool {
    #[inline(always)]
    fn from(variant: Pcpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCPOL` reader - Polarity of sync output"]
pub type PcpolR = crate::BitReader<Pcpol>;
impl PcpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcpol {
        match self.bits {
            false => Pcpol::High,
            true => Pcpol::Low,
        }
    }
    #[doc = "Active HIGH pulse output"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pcpol::High
    }
    #[doc = "Active LOW pulse output"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pcpol::Low
    }
}
#[doc = "Field `PCPOL` writer - Polarity of sync output"]
pub type PcpolW<'a, REG> = crate::BitWriter<'a, REG, Pcpol>;
impl<'a, REG> PcpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active HIGH pulse output"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpol::High)
    }
    #[doc = "Active LOW pulse output"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpol::Low)
    }
}
#[doc = "Position compare of shadow load\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcload {
    #[doc = "0: Load on QPOSCNT = 0"]
    Pcload0 = 0,
    #[doc = "1: Load when QPOSCNT = QPOSCMP"]
    PcloadQposcmp = 1,
}
impl From<Pcload> for bool {
    #[inline(always)]
    fn from(variant: Pcload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLOAD` reader - Position compare of shadow load"]
pub type PcloadR = crate::BitReader<Pcload>;
impl PcloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcload {
        match self.bits {
            false => Pcload::Pcload0,
            true => Pcload::PcloadQposcmp,
        }
    }
    #[doc = "Load on QPOSCNT = 0"]
    #[inline(always)]
    pub fn is_pcload_0(&self) -> bool {
        *self == Pcload::Pcload0
    }
    #[doc = "Load when QPOSCNT = QPOSCMP"]
    #[inline(always)]
    pub fn is_pcload_qposcmp(&self) -> bool {
        *self == Pcload::PcloadQposcmp
    }
}
#[doc = "Field `PCLOAD` writer - Position compare of shadow load"]
pub type PcloadW<'a, REG> = crate::BitWriter<'a, REG, Pcload>;
impl<'a, REG> PcloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Load on QPOSCNT = 0"]
    #[inline(always)]
    pub fn pcload_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcload::Pcload0)
    }
    #[doc = "Load when QPOSCNT = QPOSCMP"]
    #[inline(always)]
    pub fn pcload_qposcmp(self) -> &'a mut crate::W<REG> {
        self.variant(Pcload::PcloadQposcmp)
    }
}
#[doc = "Position compare of shadow enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcshdw {
    #[doc = "0: Shadow disabled, load Immediate"]
    Disable = 0,
    #[doc = "1: Shadow enabled"]
    Enable = 1,
}
impl From<Pcshdw> for bool {
    #[inline(always)]
    fn from(variant: Pcshdw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCSHDW` reader - Position compare of shadow enable"]
pub type PcshdwR = crate::BitReader<Pcshdw>;
impl PcshdwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcshdw {
        match self.bits {
            false => Pcshdw::Disable,
            true => Pcshdw::Enable,
        }
    }
    #[doc = "Shadow disabled, load Immediate"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pcshdw::Disable
    }
    #[doc = "Shadow enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pcshdw::Enable
    }
}
#[doc = "Field `PCSHDW` writer - Position compare of shadow enable"]
pub type PcshdwW<'a, REG> = crate::BitWriter<'a, REG, Pcshdw>;
impl<'a, REG> PcshdwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow disabled, load Immediate"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcshdw::Disable)
    }
    #[doc = "Shadow enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pcshdw::Enable)
    }
}
impl R {
    #[doc = "Bits 0:11 - Position compare sync pulse width"]
    #[inline(always)]
    pub fn pcspw(&self) -> PcspwR {
        PcspwR::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 12 - Position compare enable/disable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polarity of sync output"]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        PcpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Position compare of shadow load"]
    #[inline(always)]
    pub fn pcload(&self) -> PcloadR {
        PcloadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Position compare of shadow enable"]
    #[inline(always)]
    pub fn pcshdw(&self) -> PcshdwR {
        PcshdwR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QPOSCTL")
            .field("pcspw", &self.pcspw())
            .field("pce", &self.pce())
            .field("pcpol", &self.pcpol())
            .field("pcload", &self.pcload())
            .field("pcshdw", &self.pcshdw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Position compare sync pulse width"]
    #[inline(always)]
    pub fn pcspw(&mut self) -> PcspwW<'_, QposctlSpec> {
        PcspwW::new(self, 0)
    }
    #[doc = "Bit 12 - Position compare enable/disable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, QposctlSpec> {
        PceW::new(self, 12)
    }
    #[doc = "Bit 13 - Polarity of sync output"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PcpolW<'_, QposctlSpec> {
        PcpolW::new(self, 13)
    }
    #[doc = "Bit 14 - Position compare of shadow load"]
    #[inline(always)]
    pub fn pcload(&mut self) -> PcloadW<'_, QposctlSpec> {
        PcloadW::new(self, 14)
    }
    #[doc = "Bit 15 - Position compare of shadow enable"]
    #[inline(always)]
    pub fn pcshdw(&mut self) -> PcshdwW<'_, QposctlSpec> {
        PcshdwW::new(self, 15)
    }
}
#[doc = "Position Compare Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qposctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposctlSpec;
impl crate::RegisterSpec for QposctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qposctl::R`](R) reader structure"]
impl crate::Readable for QposctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qposctl::W`](W) writer structure"]
impl crate::Writable for QposctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSCTL to value 0"]
impl crate::Resettable for QposctlSpec {}
