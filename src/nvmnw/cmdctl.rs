#[doc = "Register `CMDCTL` reader"]
pub type R = crate::R<CmdctlSpec>;
#[doc = "Register `CMDCTL` writer"]
pub type W = crate::W<CmdctlSpec>;
#[doc = "Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly through the NW hardware.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modesel {
    #[doc = "0: Read Mode"]
    Read = 0,
    #[doc = "2: Read Margin 0 Mode"]
    Rdmarg0 = 2,
    #[doc = "4: Read Margin 1 Mode"]
    Rdmarg1 = 4,
    #[doc = "6: Read Margin 0B Mode"]
    Rdmarg0b = 6,
    #[doc = "7: Read Margin 1B Mode"]
    Rdmarg1b = 7,
    #[doc = "9: Program Verify Mode"]
    Pgmver = 9,
    #[doc = "10: Program Single Word"]
    Pgmsw = 10,
    #[doc = "11: Erase Verify Mode"]
    Erasever = 11,
    #[doc = "12: Erase Sector"]
    Erasesect = 12,
    #[doc = "14: Program Multiple Word"]
    Pgmmw = 14,
    #[doc = "15: Erase Bank"]
    Erasebnk = 15,
}
impl From<Modesel> for u8 {
    #[inline(always)]
    fn from(variant: Modesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modesel {
    type Ux = u8;
}
impl crate::IsEnum for Modesel {}
#[doc = "Field `MODESEL` reader - Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly through the NW hardware."]
pub type ModeselR = crate::FieldReader<Modesel>;
impl ModeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modesel> {
        match self.bits {
            0 => Some(Modesel::Read),
            2 => Some(Modesel::Rdmarg0),
            4 => Some(Modesel::Rdmarg1),
            6 => Some(Modesel::Rdmarg0b),
            7 => Some(Modesel::Rdmarg1b),
            9 => Some(Modesel::Pgmver),
            10 => Some(Modesel::Pgmsw),
            11 => Some(Modesel::Erasever),
            12 => Some(Modesel::Erasesect),
            14 => Some(Modesel::Pgmmw),
            15 => Some(Modesel::Erasebnk),
            _ => None,
        }
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Modesel::Read
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn is_rdmarg0(&self) -> bool {
        *self == Modesel::Rdmarg0
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn is_rdmarg1(&self) -> bool {
        *self == Modesel::Rdmarg1
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn is_rdmarg0b(&self) -> bool {
        *self == Modesel::Rdmarg0b
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn is_rdmarg1b(&self) -> bool {
        *self == Modesel::Rdmarg1b
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn is_pgmver(&self) -> bool {
        *self == Modesel::Pgmver
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn is_pgmsw(&self) -> bool {
        *self == Modesel::Pgmsw
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn is_erasever(&self) -> bool {
        *self == Modesel::Erasever
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn is_erasesect(&self) -> bool {
        *self == Modesel::Erasesect
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn is_pgmmw(&self) -> bool {
        *self == Modesel::Pgmmw
    }
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn is_erasebnk(&self) -> bool {
        *self == Modesel::Erasebnk
    }
}
#[doc = "Field `MODESEL` writer - Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly through the NW hardware."]
pub type ModeselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Modesel>;
impl<'a, REG> ModeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Read)
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn rdmarg0(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg0)
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn rdmarg1(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg1)
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn rdmarg0b(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg0b)
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn rdmarg1b(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg1b)
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn pgmver(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmver)
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn pgmsw(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmsw)
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn erasever(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasever)
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn erasesect(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasesect)
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn pgmmw(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmmw)
    }
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn erasebnk(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasebnk)
    }
}
#[doc = "Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation is to be applied if CMDCTL.ADDRXLATEOVR is set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Banksel {
    #[doc = "1: Bank 0"]
    Bank0 = 1,
    #[doc = "2: Bank 1"]
    Bank1 = 2,
}
impl From<Banksel> for u8 {
    #[inline(always)]
    fn from(variant: Banksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Banksel {
    type Ux = u8;
}
impl crate::IsEnum for Banksel {}
#[doc = "Field `BANKSEL` reader - Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type BankselR = crate::FieldReader<Banksel>;
impl BankselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Banksel> {
        match self.bits {
            1 => Some(Banksel::Bank0),
            2 => Some(Banksel::Bank1),
            _ => None,
        }
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Banksel::Bank0
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Banksel::Bank1
    }
}
#[doc = "Field `BANKSEL` writer - Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type BankselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Banksel>;
impl<'a, REG> BankselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank0)
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank1)
    }
}
#[doc = "Bank Region A specific region ID can be written to this field to indicate to which region an operation is to be applied if CMDCTL.ADDRXLATEOVR is set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Regionsel {
    #[doc = "1: Main Region"]
    Main = 1,
    #[doc = "2: Non-Main Region"]
    Nonmain = 2,
    #[doc = "4: Trim Region"]
    Trim = 4,
    #[doc = "8: Engr Region"]
    Engr = 8,
}
impl From<Regionsel> for u8 {
    #[inline(always)]
    fn from(variant: Regionsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Regionsel {
    type Ux = u8;
}
impl crate::IsEnum for Regionsel {}
#[doc = "Field `REGIONSEL` reader - Bank Region A specific region ID can be written to this field to indicate to which region an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type RegionselR = crate::FieldReader<Regionsel>;
impl RegionselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Regionsel> {
        match self.bits {
            1 => Some(Regionsel::Main),
            2 => Some(Regionsel::Nonmain),
            4 => Some(Regionsel::Trim),
            8 => Some(Regionsel::Engr),
            _ => None,
        }
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == Regionsel::Main
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == Regionsel::Nonmain
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == Regionsel::Trim
    }
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == Regionsel::Engr
    }
}
#[doc = "Field `REGIONSEL` writer - Bank Region A specific region ID can be written to this field to indicate to which region an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type RegionselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Regionsel>;
impl<'a, REG> RegionselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn main(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Main)
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn nonmain(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Nonmain)
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn trim(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Trim)
    }
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn engr(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Engr)
    }
}
#[doc = "Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrxlateovr {
    #[doc = "0: Do not override"]
    Nooverride = 0,
    #[doc = "1: Override"]
    Override = 1,
}
impl From<Addrxlateovr> for bool {
    #[inline(always)]
    fn from(variant: Addrxlateovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRXLATEOVR` reader - Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type AddrxlateovrR = crate::BitReader<Addrxlateovr>;
impl AddrxlateovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrxlateovr {
        match self.bits {
            false => Addrxlateovr::Nooverride,
            true => Addrxlateovr::Override,
        }
    }
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn is_nooverride(&self) -> bool {
        *self == Addrxlateovr::Nooverride
    }
    #[doc = "Override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Addrxlateovr::Override
    }
}
#[doc = "Field `ADDRXLATEOVR` writer - Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type AddrxlateovrW<'a, REG> = crate::BitWriter<'a, REG, Addrxlateovr>;
impl<'a, REG> AddrxlateovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn nooverride(self) -> &'a mut crate::W<REG> {
        self.variant(Addrxlateovr::Nooverride)
    }
    #[doc = "Override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Addrxlateovr::Override)
    }
}
#[doc = "Override hardware generation of ECC data for program. Use data written to CMDDATAECC*.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccgenovr {
    #[doc = "0: Do not override"]
    Nooverride = 0,
    #[doc = "1: Override"]
    Override = 1,
}
impl From<Eccgenovr> for bool {
    #[inline(always)]
    fn from(variant: Eccgenovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCGENOVR` reader - Override hardware generation of ECC data for program. Use data written to CMDDATAECC*."]
pub type EccgenovrR = crate::BitReader<Eccgenovr>;
impl EccgenovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccgenovr {
        match self.bits {
            false => Eccgenovr::Nooverride,
            true => Eccgenovr::Override,
        }
    }
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn is_nooverride(&self) -> bool {
        *self == Eccgenovr::Nooverride
    }
    #[doc = "Override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Eccgenovr::Override
    }
}
#[doc = "Field `ECCGENOVR` writer - Override hardware generation of ECC data for program. Use data written to CMDDATAECC*."]
pub type EccgenovrW<'a, REG> = crate::BitWriter<'a, REG, Eccgenovr>;
impl<'a, REG> EccgenovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn nooverride(self) -> &'a mut crate::W<REG> {
        self.variant(Eccgenovr::Nooverride)
    }
    #[doc = "Override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Eccgenovr::Override)
    }
}
#[doc = "Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sserasedis {
    #[doc = "0: Enable"]
    Enable = 0,
    #[doc = "1: Disable"]
    Disable = 1,
}
impl From<Sserasedis> for bool {
    #[inline(always)]
    fn from(variant: Sserasedis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSERASEDIS` reader - Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SserasedisR = crate::BitReader<Sserasedis>;
impl SserasedisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sserasedis {
        match self.bits {
            false => Sserasedis::Enable,
            true => Sserasedis::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sserasedis::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sserasedis::Disable
    }
}
#[doc = "Field `SSERASEDIS` writer - Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SserasedisW<'a, REG> = crate::BitWriter<'a, REG, Sserasedis>;
impl<'a, REG> SserasedisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sserasedis::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sserasedis::Disable)
    }
}
#[doc = "Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without executing the program.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataveren {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Dataveren> for bool {
    #[inline(always)]
    fn from(variant: Dataveren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAVEREN` reader - Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without executing the program."]
pub type DataverenR = crate::BitReader<Dataveren>;
impl DataverenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataveren {
        match self.bits {
            false => Dataveren::Disable,
            true => Dataveren::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dataveren::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dataveren::Enable
    }
}
#[doc = "Field `DATAVEREN` writer - Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without executing the program."]
pub type DataverenW<'a, REG> = crate::BitWriter<'a, REG, Dataveren>;
impl<'a, REG> DataverenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dataveren::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dataveren::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly through the NW hardware."]
    #[inline(always)]
    pub fn modesel(&self) -> ModeselR {
        ModeselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn banksel(&self) -> BankselR {
        BankselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 9:12 - Bank Region A specific region ID can be written to this field to indicate to which region an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn regionsel(&self) -> RegionselR {
        RegionselR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    pub fn addrxlateovr(&self) -> AddrxlateovrR {
        AddrxlateovrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Override hardware generation of ECC data for program. Use data written to CMDDATAECC*."]
    #[inline(always)]
    pub fn eccgenovr(&self) -> EccgenovrR {
        EccgenovrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    pub fn sserasedis(&self) -> SserasedisR {
        SserasedisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without executing the program."]
    #[inline(always)]
    pub fn dataveren(&self) -> DataverenR {
        DataverenR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCTL")
            .field("modesel", &self.modesel())
            .field("banksel", &self.banksel())
            .field("regionsel", &self.regionsel())
            .field("eccgenovr", &self.eccgenovr())
            .field("addrxlateovr", &self.addrxlateovr())
            .field("sserasedis", &self.sserasedis())
            .field("dataveren", &self.dataveren())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly through the NW hardware."]
    #[inline(always)]
    pub fn modesel(&mut self) -> ModeselW<'_, CmdctlSpec> {
        ModeselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn banksel(&mut self) -> BankselW<'_, CmdctlSpec> {
        BankselW::new(self, 4)
    }
    #[doc = "Bits 9:12 - Bank Region A specific region ID can be written to this field to indicate to which region an operation is to be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn regionsel(&mut self) -> RegionselW<'_, CmdctlSpec> {
        RegionselW::new(self, 9)
    }
    #[doc = "Bit 16 - Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    pub fn addrxlateovr(&mut self) -> AddrxlateovrW<'_, CmdctlSpec> {
        AddrxlateovrW::new(self, 16)
    }
    #[doc = "Bit 17 - Override hardware generation of ECC data for program. Use data written to CMDDATAECC*."]
    #[inline(always)]
    pub fn eccgenovr(&mut self) -> EccgenovrW<'_, CmdctlSpec> {
        EccgenovrW::new(self, 17)
    }
    #[doc = "Bit 20 - Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    pub fn sserasedis(&mut self) -> SserasedisW<'_, CmdctlSpec> {
        SserasedisW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without executing the program."]
    #[inline(always)]
    pub fn dataveren(&mut self) -> DataverenW<'_, CmdctlSpec> {
        DataverenW::new(self, 21)
    }
}
#[doc = "Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdctlSpec;
impl crate::RegisterSpec for CmdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctl::R`](R) reader structure"]
impl crate::Readable for CmdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdctl::W`](W) writer structure"]
impl crate::Writable for CmdctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDCTL to value 0"]
impl crate::Resettable for CmdctlSpec {}
