#[doc = "Register `ADDRMAP` reader"]
pub type R = crate::R<AddrmapSpec>;
#[doc = "Register `ADDRMAP` writer"]
pub type W = crate::W<AddrmapSpec>;
#[doc = "External RAM Address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eradr {
    #[doc = "0: Not mapped"]
    NoMap = 0,
    #[doc = "1: At 0x60000000"]
    Map60000000 = 1,
    #[doc = "2: At 0x80000000"]
    Map80000000 = 2,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x60000000 and CS1n maps to 0x80000000."]
    DualMap = 3,
}
impl From<Eradr> for u8 {
    #[inline(always)]
    fn from(variant: Eradr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eradr {
    type Ux = u8;
}
impl crate::IsEnum for Eradr {}
#[doc = "Field `ERADR` reader - External RAM Address"]
pub type EradrR = crate::FieldReader<Eradr>;
impl EradrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eradr {
        match self.bits {
            0 => Eradr::NoMap,
            1 => Eradr::Map60000000,
            2 => Eradr::Map80000000,
            3 => Eradr::DualMap,
            _ => unreachable!(),
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn is_no_map(&self) -> bool {
        *self == Eradr::NoMap
    }
    #[doc = "At 0x60000000"]
    #[inline(always)]
    pub fn is_map_60000000(&self) -> bool {
        *self == Eradr::Map60000000
    }
    #[doc = "At 0x80000000"]
    #[inline(always)]
    pub fn is_map_80000000(&self) -> bool {
        *self == Eradr::Map80000000
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x60000000 and CS1n maps to 0x80000000."]
    #[inline(always)]
    pub fn is_dual_map(&self) -> bool {
        *self == Eradr::DualMap
    }
}
#[doc = "Field `ERADR` writer - External RAM Address"]
pub type EradrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eradr, crate::Safe>;
impl<'a, REG> EradrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn no_map(self) -> &'a mut crate::W<REG> {
        self.variant(Eradr::NoMap)
    }
    #[doc = "At 0x60000000"]
    #[inline(always)]
    pub fn map_60000000(self) -> &'a mut crate::W<REG> {
        self.variant(Eradr::Map60000000)
    }
    #[doc = "At 0x80000000"]
    #[inline(always)]
    pub fn map_80000000(self) -> &'a mut crate::W<REG> {
        self.variant(Eradr::Map80000000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x60000000 and CS1n maps to 0x80000000."]
    #[inline(always)]
    pub fn dual_map(self) -> &'a mut crate::W<REG> {
        self.variant(Eradr::DualMap)
    }
}
#[doc = "External RAM Size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ersz {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    AddrMemSize256 = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    AddrMemSize64k = 1,
    #[doc = "2: 16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    AddrMemSize16m = 2,
    #[doc = "3: 256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    AddrMemSize256m = 3,
}
impl From<Ersz> for u8 {
    #[inline(always)]
    fn from(variant: Ersz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ersz {
    type Ux = u8;
}
impl crate::IsEnum for Ersz {}
#[doc = "Field `ERSZ` reader - External RAM Size"]
pub type ErszR = crate::FieldReader<Ersz>;
impl ErszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ersz {
        match self.bits {
            0 => Ersz::AddrMemSize256,
            1 => Ersz::AddrMemSize64k,
            2 => Ersz::AddrMemSize16m,
            3 => Ersz::AddrMemSize256m,
            _ => unreachable!(),
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn is_addr_mem_size_256(&self) -> bool {
        *self == Ersz::AddrMemSize256
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn is_addr_mem_size_64k(&self) -> bool {
        *self == Ersz::AddrMemSize64k
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn is_addr_mem_size_16m(&self) -> bool {
        *self == Ersz::AddrMemSize16m
    }
    #[doc = "256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    #[inline(always)]
    pub fn is_addr_mem_size_256m(&self) -> bool {
        *self == Ersz::AddrMemSize256m
    }
}
#[doc = "Field `ERSZ` writer - External RAM Size"]
pub type ErszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ersz, crate::Safe>;
impl<'a, REG> ErszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn addr_mem_size_256(self) -> &'a mut crate::W<REG> {
        self.variant(Ersz::AddrMemSize256)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn addr_mem_size_64k(self) -> &'a mut crate::W<REG> {
        self.variant(Ersz::AddrMemSize64k)
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn addr_mem_size_16m(self) -> &'a mut crate::W<REG> {
        self.variant(Ersz::AddrMemSize16m)
    }
    #[doc = "256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    #[inline(always)]
    pub fn addr_mem_size_256m(self) -> &'a mut crate::W<REG> {
        self.variant(Ersz::AddrMemSize256m)
    }
}
#[doc = "External Peripheral Address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epadr {
    #[doc = "0: Not mapped"]
    NoMap = 0,
    #[doc = "1: External Peripheral (EPADR): maps to 0xA0000000 | External RAM (ERADR): same bit value maps to 0x60000000 (not supported on AM13E2x)"]
    Map60000000 = 1,
    #[doc = "2: External Peripheral (EPADR): maps to 0xC0000000 | External RAM (ERADR): same bit value maps to 0x80000000"]
    Map80000000 = 2,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA0000000 and CS3n maps to 0xC0000000."]
    DualMap = 3,
}
impl From<Epadr> for u8 {
    #[inline(always)]
    fn from(variant: Epadr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epadr {
    type Ux = u8;
}
impl crate::IsEnum for Epadr {}
#[doc = "Field `EPADR` reader - External Peripheral Address"]
pub type EpadrR = crate::FieldReader<Epadr>;
impl EpadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epadr {
        match self.bits {
            0 => Epadr::NoMap,
            1 => Epadr::Map60000000,
            2 => Epadr::Map80000000,
            3 => Epadr::DualMap,
            _ => unreachable!(),
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn is_no_map(&self) -> bool {
        *self == Epadr::NoMap
    }
    #[doc = "External Peripheral (EPADR): maps to 0xA0000000 | External RAM (ERADR): same bit value maps to 0x60000000 (not supported on AM13E2x)"]
    #[inline(always)]
    pub fn is_map_60000000(&self) -> bool {
        *self == Epadr::Map60000000
    }
    #[doc = "External Peripheral (EPADR): maps to 0xC0000000 | External RAM (ERADR): same bit value maps to 0x80000000"]
    #[inline(always)]
    pub fn is_map_80000000(&self) -> bool {
        *self == Epadr::Map80000000
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA0000000 and CS3n maps to 0xC0000000."]
    #[inline(always)]
    pub fn is_dual_map(&self) -> bool {
        *self == Epadr::DualMap
    }
}
#[doc = "Field `EPADR` writer - External Peripheral Address"]
pub type EpadrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Epadr, crate::Safe>;
impl<'a, REG> EpadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn no_map(self) -> &'a mut crate::W<REG> {
        self.variant(Epadr::NoMap)
    }
    #[doc = "External Peripheral (EPADR): maps to 0xA0000000 | External RAM (ERADR): same bit value maps to 0x60000000 (not supported on AM13E2x)"]
    #[inline(always)]
    pub fn map_60000000(self) -> &'a mut crate::W<REG> {
        self.variant(Epadr::Map60000000)
    }
    #[doc = "External Peripheral (EPADR): maps to 0xC0000000 | External RAM (ERADR): same bit value maps to 0x80000000"]
    #[inline(always)]
    pub fn map_80000000(self) -> &'a mut crate::W<REG> {
        self.variant(Epadr::Map80000000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA0000000 and CS3n maps to 0xC0000000."]
    #[inline(always)]
    pub fn dual_map(self) -> &'a mut crate::W<REG> {
        self.variant(Epadr::DualMap)
    }
}
#[doc = "External Peripheral Size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epsz {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    AddrPeriphSize256 = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    AddrPeriphSize64k = 1,
    #[doc = "2: 16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    AddrPeriphSize16m = 2,
    #[doc = "3: 256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    AddrPeriphSize256m = 3,
}
impl From<Epsz> for u8 {
    #[inline(always)]
    fn from(variant: Epsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epsz {
    type Ux = u8;
}
impl crate::IsEnum for Epsz {}
#[doc = "Field `EPSZ` reader - External Peripheral Size"]
pub type EpszR = crate::FieldReader<Epsz>;
impl EpszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epsz {
        match self.bits {
            0 => Epsz::AddrPeriphSize256,
            1 => Epsz::AddrPeriphSize64k,
            2 => Epsz::AddrPeriphSize16m,
            3 => Epsz::AddrPeriphSize256m,
            _ => unreachable!(),
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn is_addr_periph_size_256(&self) -> bool {
        *self == Epsz::AddrPeriphSize256
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn is_addr_periph_size_64k(&self) -> bool {
        *self == Epsz::AddrPeriphSize64k
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn is_addr_periph_size_16m(&self) -> bool {
        *self == Epsz::AddrPeriphSize16m
    }
    #[doc = "256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    #[inline(always)]
    pub fn is_addr_periph_size_256m(&self) -> bool {
        *self == Epsz::AddrPeriphSize256m
    }
}
#[doc = "Field `EPSZ` writer - External Peripheral Size"]
pub type EpszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Epsz, crate::Safe>;
impl<'a, REG> EpszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn addr_periph_size_256(self) -> &'a mut crate::W<REG> {
        self.variant(Epsz::AddrPeriphSize256)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn addr_periph_size_64k(self) -> &'a mut crate::W<REG> {
        self.variant(Epsz::AddrPeriphSize64k)
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn addr_periph_size_16m(self) -> &'a mut crate::W<REG> {
        self.variant(Epsz::AddrPeriphSize16m)
    }
    #[doc = "256 MB; lower address range: 0x0000000 to 0xFFFFFFF"]
    #[inline(always)]
    pub fn addr_periph_size_256m(self) -> &'a mut crate::W<REG> {
        self.variant(Epsz::AddrPeriphSize256m)
    }
}
#[doc = "External Code Address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ecadr {
    #[doc = "0: Not mapped"]
    NoMap = 0,
    #[doc = "1: At 0x10000000"]
    Map10000000 = 1,
}
impl From<Ecadr> for u8 {
    #[inline(always)]
    fn from(variant: Ecadr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ecadr {
    type Ux = u8;
}
impl crate::IsEnum for Ecadr {}
#[doc = "Field `ECADR` reader - External Code Address"]
pub type EcadrR = crate::FieldReader<Ecadr>;
impl EcadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ecadr> {
        match self.bits {
            0 => Some(Ecadr::NoMap),
            1 => Some(Ecadr::Map10000000),
            _ => None,
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn is_no_map(&self) -> bool {
        *self == Ecadr::NoMap
    }
    #[doc = "At 0x10000000"]
    #[inline(always)]
    pub fn is_map_10000000(&self) -> bool {
        *self == Ecadr::Map10000000
    }
}
#[doc = "Field `ECADR` writer - External Code Address"]
pub type EcadrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ecadr>;
impl<'a, REG> EcadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn no_map(self) -> &'a mut crate::W<REG> {
        self.variant(Ecadr::NoMap)
    }
    #[doc = "At 0x10000000"]
    #[inline(always)]
    pub fn map_10000000(self) -> &'a mut crate::W<REG> {
        self.variant(Ecadr::Map10000000)
    }
}
#[doc = "External Code Size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ecsz {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    AddrCodeSize256 = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    AddrCodeSize64k = 1,
    #[doc = "2: 16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    AddrCodeSize16m = 2,
    #[doc = "3: 256MB; lower address range: 0x0000000 to 0x0FFFFFFF"]
    AddrCodeSize256m = 3,
}
impl From<Ecsz> for u8 {
    #[inline(always)]
    fn from(variant: Ecsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ecsz {
    type Ux = u8;
}
impl crate::IsEnum for Ecsz {}
#[doc = "Field `ECSZ` reader - External Code Size"]
pub type EcszR = crate::FieldReader<Ecsz>;
impl EcszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecsz {
        match self.bits {
            0 => Ecsz::AddrCodeSize256,
            1 => Ecsz::AddrCodeSize64k,
            2 => Ecsz::AddrCodeSize16m,
            3 => Ecsz::AddrCodeSize256m,
            _ => unreachable!(),
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn is_addr_code_size_256(&self) -> bool {
        *self == Ecsz::AddrCodeSize256
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn is_addr_code_size_64k(&self) -> bool {
        *self == Ecsz::AddrCodeSize64k
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn is_addr_code_size_16m(&self) -> bool {
        *self == Ecsz::AddrCodeSize16m
    }
    #[doc = "256MB; lower address range: 0x0000000 to 0x0FFFFFFF"]
    #[inline(always)]
    pub fn is_addr_code_size_256m(&self) -> bool {
        *self == Ecsz::AddrCodeSize256m
    }
}
#[doc = "Field `ECSZ` writer - External Code Size"]
pub type EcszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ecsz, crate::Safe>;
impl<'a, REG> EcszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn addr_code_size_256(self) -> &'a mut crate::W<REG> {
        self.variant(Ecsz::AddrCodeSize256)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn addr_code_size_64k(self) -> &'a mut crate::W<REG> {
        self.variant(Ecsz::AddrCodeSize64k)
    }
    #[doc = "16 MB; lower address range: 0x000000 to 0xFFFFFF"]
    #[inline(always)]
    pub fn addr_code_size_16m(self) -> &'a mut crate::W<REG> {
        self.variant(Ecsz::AddrCodeSize16m)
    }
    #[doc = "256MB; lower address range: 0x0000000 to 0x0FFFFFFF"]
    #[inline(always)]
    pub fn addr_code_size_256m(self) -> &'a mut crate::W<REG> {
        self.variant(Ecsz::AddrCodeSize256m)
    }
}
impl R {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn eradr(&self) -> EradrR {
        EradrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn ersz(&self) -> ErszR {
        ErszR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epadr(&self) -> EpadrR {
        EpadrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epsz(&self) -> EpszR {
        EpszR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn ecadr(&self) -> EcadrR {
        EcadrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn ecsz(&self) -> EcszR {
        EcszR::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP")
            .field("eradr", &self.eradr())
            .field("ersz", &self.ersz())
            .field("epadr", &self.epadr())
            .field("epsz", &self.epsz())
            .field("ecadr", &self.ecadr())
            .field("ecsz", &self.ecsz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn eradr(&mut self) -> EradrW<'_, AddrmapSpec> {
        EradrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn ersz(&mut self) -> ErszW<'_, AddrmapSpec> {
        ErszW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epadr(&mut self) -> EpadrW<'_, AddrmapSpec> {
        EpadrW::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epsz(&mut self) -> EpszW<'_, AddrmapSpec> {
        EpszW::new(self, 6)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn ecadr(&mut self) -> EcadrW<'_, AddrmapSpec> {
        EcadrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn ecsz(&mut self) -> EcszW<'_, AddrmapSpec> {
        EcszW::new(self, 10)
    }
}
#[doc = "EPI Address Map\n\nYou can [`read`](crate::Reg::read) this register and get [`addrmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrmapSpec;
impl crate::RegisterSpec for AddrmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrmap::R`](R) reader structure"]
impl crate::Readable for AddrmapSpec {}
#[doc = "`write(|w| ..)` method takes [`addrmap::W`](W) writer structure"]
impl crate::Writable for AddrmapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRMAP to value 0"]
impl crate::Resettable for AddrmapSpec {}
