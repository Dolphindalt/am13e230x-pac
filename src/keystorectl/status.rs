#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Status information\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: Valid configuration"]
    Valid = 0,
    #[doc = "1: Key-store has not been configured. NK256 has not been set."]
    NoCfg = 1,
    #[doc = "2: Invalid value for NK256 field in CFG."]
    InvalidNk256 = 2,
    #[doc = "3: Busy receiving a key deposit"]
    BusyReceive = 3,
    #[doc = "4: Busy transmitting a key to a crypto engine"]
    BusyTransmit = 4,
    #[doc = "5: Invalid keyslot selection for writing"]
    InvalidKeyslotselw = 5,
    #[doc = "6: Invalid keyslot selection for reading"]
    InvalidKeyslotselr = 6,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Status information"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::Valid),
            1 => Some(Stat::NoCfg),
            2 => Some(Stat::InvalidNk256),
            3 => Some(Stat::BusyReceive),
            4 => Some(Stat::BusyTransmit),
            5 => Some(Stat::InvalidKeyslotselw),
            6 => Some(Stat::InvalidKeyslotselr),
            _ => None,
        }
    }
    #[doc = "Valid configuration"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Stat::Valid
    }
    #[doc = "Key-store has not been configured. NK256 has not been set."]
    #[inline(always)]
    pub fn is_no_cfg(&self) -> bool {
        *self == Stat::NoCfg
    }
    #[doc = "Invalid value for NK256 field in CFG."]
    #[inline(always)]
    pub fn is_invalid_nk256(&self) -> bool {
        *self == Stat::InvalidNk256
    }
    #[doc = "Busy receiving a key deposit"]
    #[inline(always)]
    pub fn is_busy_receive(&self) -> bool {
        *self == Stat::BusyReceive
    }
    #[doc = "Busy transmitting a key to a crypto engine"]
    #[inline(always)]
    pub fn is_busy_transmit(&self) -> bool {
        *self == Stat::BusyTransmit
    }
    #[doc = "Invalid keyslot selection for writing"]
    #[inline(always)]
    pub fn is_invalid_keyslotselw(&self) -> bool {
        *self == Stat::InvalidKeyslotselw
    }
    #[doc = "Invalid keyslot selection for reading"]
    #[inline(always)]
    pub fn is_invalid_keyslotselr(&self) -> bool {
        *self == Stat::InvalidKeyslotselr
    }
}
#[doc = "Field `VALID` reader - Bitvector of valid bits to indicate which slots have been configured"]
pub type ValidR = crate::FieldReader;
#[doc = "Size of keystorage: Number of 128-bit key slots\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nkeyslots {
    #[doc = "0: Two slots"]
    Two = 0,
    #[doc = "1: Three slots"]
    Three = 1,
    #[doc = "2: Four slots"]
    Four = 2,
}
impl From<Nkeyslots> for u8 {
    #[inline(always)]
    fn from(variant: Nkeyslots) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nkeyslots {
    type Ux = u8;
}
impl crate::IsEnum for Nkeyslots {}
#[doc = "Field `NKEYSLOTS` reader - Size of keystorage: Number of 128-bit key slots"]
pub type NkeyslotsR = crate::FieldReader<Nkeyslots>;
impl NkeyslotsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nkeyslots> {
        match self.bits {
            0 => Some(Nkeyslots::Two),
            1 => Some(Nkeyslots::Three),
            2 => Some(Nkeyslots::Four),
            _ => None,
        }
    }
    #[doc = "Two slots"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Nkeyslots::Two
    }
    #[doc = "Three slots"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Nkeyslots::Three
    }
    #[doc = "Four slots"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Nkeyslots::Four
    }
}
impl R {
    #[doc = "Bits 0:3 - Status information"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Bitvector of valid bits to indicate which slots have been configured"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Size of keystorage: Number of 128-bit key slots"]
    #[inline(always)]
    pub fn nkeyslots(&self) -> NkeyslotsR {
        NkeyslotsR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("stat", &self.stat())
            .field("valid", &self.valid())
            .field("nkeyslots", &self.nkeyslots())
            .finish()
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0002_0001"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0002_0001;
}
