#[doc = "Register `KEYWR` reader"]
pub type R = crate::R<KeywrSpec>;
#[doc = "Register `KEYWR` writer"]
pub type W = crate::W<KeywrSpec>;
#[doc = "Key size selection. Selection of 128 or 256 bit keys\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyszsel {
    #[doc = "0: 256 bit key"]
    K256 = 0,
    #[doc = "1: 128 bit key"]
    K128 = 1,
}
impl From<Keyszsel> for u8 {
    #[inline(always)]
    fn from(variant: Keyszsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyszsel {
    type Ux = u8;
}
impl crate::IsEnum for Keyszsel {}
#[doc = "Field `KEYSZSEL` reader - Key size selection. Selection of 128 or 256 bit keys"]
pub type KeyszselR = crate::FieldReader<Keyszsel>;
impl KeyszselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyszsel> {
        match self.bits {
            0 => Some(Keyszsel::K256),
            1 => Some(Keyszsel::K128),
            _ => None,
        }
    }
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Keyszsel::K256
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == Keyszsel::K128
    }
}
#[doc = "Field `KEYSZSEL` writer - Key size selection. Selection of 128 or 256 bit keys"]
pub type KeyszselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Keyszsel, crate::Safe>;
impl<'a, REG> KeyszselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn k256(self) -> &'a mut crate::W<REG> {
        self.variant(Keyszsel::K256)
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn k128(self) -> &'a mut crate::W<REG> {
        self.variant(Keyszsel::K128)
    }
}
#[doc = "Select the key slot to write the key into. NOTE: SW needs to ensure that it is writing to the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when writing a 256-bit key, two slots will need to budgeted. The selected slot for the 256-bit key needs to be an even numbered slot (eg: 0 or 2). Also all the 256-bit keys need to be stored in lower numbered slots and 128-bit keys stored in higher numbered slots.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyslotsel {
    #[doc = "0: Slot 0"]
    Slot0 = 0,
    #[doc = "1: Slot 1"]
    Slot1 = 1,
    #[doc = "2: Slot 2"]
    Slot2 = 2,
    #[doc = "3: Slot 3"]
    Slot3 = 3,
    #[doc = "4: Slot 4"]
    Slot4 = 4,
    #[doc = "5: Slot 5"]
    Slot5 = 5,
    #[doc = "6: Slot 6"]
    Slot6 = 6,
    #[doc = "7: Slot 7"]
    Slot7 = 7,
}
impl From<Keyslotsel> for u8 {
    #[inline(always)]
    fn from(variant: Keyslotsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyslotsel {
    type Ux = u8;
}
impl crate::IsEnum for Keyslotsel {}
#[doc = "Field `KEYSLOTSEL` reader - Select the key slot to write the key into. NOTE: SW needs to ensure that it is writing to the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when writing a 256-bit key, two slots will need to budgeted. The selected slot for the 256-bit key needs to be an even numbered slot (eg: 0 or 2). Also all the 256-bit keys need to be stored in lower numbered slots and 128-bit keys stored in higher numbered slots."]
pub type KeyslotselR = crate::FieldReader<Keyslotsel>;
impl KeyslotselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyslotsel> {
        match self.bits {
            0 => Some(Keyslotsel::Slot0),
            1 => Some(Keyslotsel::Slot1),
            2 => Some(Keyslotsel::Slot2),
            3 => Some(Keyslotsel::Slot3),
            4 => Some(Keyslotsel::Slot4),
            5 => Some(Keyslotsel::Slot5),
            6 => Some(Keyslotsel::Slot6),
            7 => Some(Keyslotsel::Slot7),
            _ => None,
        }
    }
    #[doc = "Slot 0"]
    #[inline(always)]
    pub fn is_slot0(&self) -> bool {
        *self == Keyslotsel::Slot0
    }
    #[doc = "Slot 1"]
    #[inline(always)]
    pub fn is_slot1(&self) -> bool {
        *self == Keyslotsel::Slot1
    }
    #[doc = "Slot 2"]
    #[inline(always)]
    pub fn is_slot2(&self) -> bool {
        *self == Keyslotsel::Slot2
    }
    #[doc = "Slot 3"]
    #[inline(always)]
    pub fn is_slot3(&self) -> bool {
        *self == Keyslotsel::Slot3
    }
    #[doc = "Slot 4"]
    #[inline(always)]
    pub fn is_slot4(&self) -> bool {
        *self == Keyslotsel::Slot4
    }
    #[doc = "Slot 5"]
    #[inline(always)]
    pub fn is_slot5(&self) -> bool {
        *self == Keyslotsel::Slot5
    }
    #[doc = "Slot 6"]
    #[inline(always)]
    pub fn is_slot6(&self) -> bool {
        *self == Keyslotsel::Slot6
    }
    #[doc = "Slot 7"]
    #[inline(always)]
    pub fn is_slot7(&self) -> bool {
        *self == Keyslotsel::Slot7
    }
}
#[doc = "Field `KEYSLOTSEL` writer - Select the key slot to write the key into. NOTE: SW needs to ensure that it is writing to the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when writing a 256-bit key, two slots will need to budgeted. The selected slot for the 256-bit key needs to be an even numbered slot (eg: 0 or 2). Also all the 256-bit keys need to be stored in lower numbered slots and 128-bit keys stored in higher numbered slots."]
pub type KeyslotselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Keyslotsel, crate::Safe>;
impl<'a, REG> KeyslotselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slot 0"]
    #[inline(always)]
    pub fn slot0(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot0)
    }
    #[doc = "Slot 1"]
    #[inline(always)]
    pub fn slot1(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot1)
    }
    #[doc = "Slot 2"]
    #[inline(always)]
    pub fn slot2(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot2)
    }
    #[doc = "Slot 3"]
    #[inline(always)]
    pub fn slot3(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot3)
    }
    #[doc = "Slot 4"]
    #[inline(always)]
    pub fn slot4(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot4)
    }
    #[doc = "Slot 5"]
    #[inline(always)]
    pub fn slot5(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot5)
    }
    #[doc = "Slot 6"]
    #[inline(always)]
    pub fn slot6(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot6)
    }
    #[doc = "Slot 7"]
    #[inline(always)]
    pub fn slot7(self) -> &'a mut crate::W<REG> {
        self.variant(Keyslotsel::Slot7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Key size selection. Selection of 128 or 256 bit keys"]
    #[inline(always)]
    pub fn keyszsel(&self) -> KeyszselR {
        KeyszselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Select the key slot to write the key into. NOTE: SW needs to ensure that it is writing to the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when writing a 256-bit key, two slots will need to budgeted. The selected slot for the 256-bit key needs to be an even numbered slot (eg: 0 or 2). Also all the 256-bit keys need to be stored in lower numbered slots and 128-bit keys stored in higher numbered slots."]
    #[inline(always)]
    pub fn keyslotsel(&self) -> KeyslotselR {
        KeyslotselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYWR")
            .field("keyszsel", &self.keyszsel())
            .field("keyslotsel", &self.keyslotsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Key size selection. Selection of 128 or 256 bit keys"]
    #[inline(always)]
    pub fn keyszsel(&mut self) -> KeyszselW<'_, KeywrSpec> {
        KeyszselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select the key slot to write the key into. NOTE: SW needs to ensure that it is writing to the correct slots. The slot numbering is from 0 through SYS_N_SLOTS-1. Each slot is a 128-bit slot. Therefore, when writing a 256-bit key, two slots will need to budgeted. The selected slot for the 256-bit key needs to be an even numbered slot (eg: 0 or 2). Also all the 256-bit keys need to be stored in lower numbered slots and 128-bit keys stored in higher numbered slots."]
    #[inline(always)]
    pub fn keyslotsel(&mut self) -> KeyslotselW<'_, KeywrSpec> {
        KeyslotselW::new(self, 4)
    }
}
#[doc = "Key write configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keywr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keywr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeywrSpec;
impl crate::RegisterSpec for KeywrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keywr::R`](R) reader structure"]
impl crate::Readable for KeywrSpec {}
#[doc = "`write(|w| ..)` method takes [`keywr::W`](W) writer structure"]
impl crate::Writable for KeywrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEYWR to value 0"]
impl crate::Resettable for KeywrSpec {}
