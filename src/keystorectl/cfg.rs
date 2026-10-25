#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Number of 256 bit keys to be held in the key-store. Can not exceed the total number of slots present in the hardware / 2. For eg: if SYS_N_SLOTS = 4, then atmost 2 256-bit keys can be held in the key-store. Incorrect setting of this field will be reported via STATUS register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nk256 {
    #[doc = "0: No 256-bit keys"]
    Zero = 0,
    #[doc = "1: One 256-bit key"]
    One = 1,
    #[doc = "2: Two 256-bit keys"]
    Two = 2,
    #[doc = "3: Three 256-bit keys"]
    Three = 3,
    #[doc = "4: Four 256-bit keys"]
    Four = 4,
}
impl From<Nk256> for u8 {
    #[inline(always)]
    fn from(variant: Nk256) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nk256 {
    type Ux = u8;
}
impl crate::IsEnum for Nk256 {}
#[doc = "Field `NK256` reader - Number of 256 bit keys to be held in the key-store. Can not exceed the total number of slots present in the hardware / 2. For eg: if SYS_N_SLOTS = 4, then atmost 2 256-bit keys can be held in the key-store. Incorrect setting of this field will be reported via STATUS register"]
pub type Nk256R = crate::FieldReader<Nk256>;
impl Nk256R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nk256> {
        match self.bits {
            0 => Some(Nk256::Zero),
            1 => Some(Nk256::One),
            2 => Some(Nk256::Two),
            3 => Some(Nk256::Three),
            4 => Some(Nk256::Four),
            _ => None,
        }
    }
    #[doc = "No 256-bit keys"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Nk256::Zero
    }
    #[doc = "One 256-bit key"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Nk256::One
    }
    #[doc = "Two 256-bit keys"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Nk256::Two
    }
    #[doc = "Three 256-bit keys"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Nk256::Three
    }
    #[doc = "Four 256-bit keys"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Nk256::Four
    }
}
#[doc = "Field `NK256` writer - Number of 256 bit keys to be held in the key-store. Can not exceed the total number of slots present in the hardware / 2. For eg: if SYS_N_SLOTS = 4, then atmost 2 256-bit keys can be held in the key-store. Incorrect setting of this field will be reported via STATUS register"]
pub type Nk256W<'a, REG> = crate::FieldWriter<'a, REG, 4, Nk256, crate::Safe>;
impl<'a, REG> Nk256W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No 256-bit keys"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Nk256::Zero)
    }
    #[doc = "One 256-bit key"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Nk256::One)
    }
    #[doc = "Two 256-bit keys"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Nk256::Two)
    }
    #[doc = "Three 256-bit keys"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Nk256::Three)
    }
    #[doc = "Four 256-bit keys"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Nk256::Four)
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of 256 bit keys to be held in the key-store. Can not exceed the total number of slots present in the hardware / 2. For eg: if SYS_N_SLOTS = 4, then atmost 2 256-bit keys can be held in the key-store. Incorrect setting of this field will be reported via STATUS register"]
    #[inline(always)]
    pub fn nk256(&self) -> Nk256R {
        Nk256R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG").field("nk256", &self.nk256()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of 256 bit keys to be held in the key-store. Can not exceed the total number of slots present in the hardware / 2. For eg: if SYS_N_SLOTS = 4, then atmost 2 256-bit keys can be held in the key-store. Incorrect setting of this field will be reported via STATUS register"]
    #[inline(always)]
    pub fn nk256(&mut self) -> Nk256W<'_, CfgSpec> {
        Nk256W::new(self, 0)
    }
}
#[doc = "Keystore configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0x01"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
