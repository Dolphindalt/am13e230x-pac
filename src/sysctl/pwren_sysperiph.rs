#[doc = "Register `PWREN_SYSPERIPH` reader"]
pub type R = crate::R<PwrenSysperiphSpec>;
#[doc = "Register `PWREN_SYSPERIPH` writer"]
pub type W = crate::W<PwrenSysperiphSpec>;
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tinie {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Tinie> for bool {
    #[inline(always)]
    fn from(variant: Tinie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tinie` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type TinieR = crate::BitReader<Tinie>;
impl TinieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tinie {
        match self.bits {
            false => Tinie::Disable,
            true => Tinie::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tinie::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tinie::Enable
    }
}
#[doc = "Field `tinie` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type TinieW<'a, REG> = crate::BitWriter<'a, REG, Tinie>;
impl<'a, REG> TinieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tinie::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tinie::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pga0 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pga0> for bool {
    #[inline(always)]
    fn from(variant: Pga0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pga0` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga0R = crate::BitReader<Pga0>;
impl Pga0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pga0 {
        match self.bits {
            false => Pga0::Disable,
            true => Pga0::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pga0::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pga0::Enable
    }
}
#[doc = "Field `pga0` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga0W<'a, REG> = crate::BitWriter<'a, REG, Pga0>;
impl<'a, REG> Pga0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga0::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga0::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pga1 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pga1> for bool {
    #[inline(always)]
    fn from(variant: Pga1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pga1` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga1R = crate::BitReader<Pga1>;
impl Pga1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pga1 {
        match self.bits {
            false => Pga1::Disable,
            true => Pga1::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pga1::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pga1::Enable
    }
}
#[doc = "Field `pga1` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga1W<'a, REG> = crate::BitWriter<'a, REG, Pga1>;
impl<'a, REG> Pga1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga1::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga1::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pga2 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pga2> for bool {
    #[inline(always)]
    fn from(variant: Pga2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pga2` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga2R = crate::BitReader<Pga2>;
impl Pga2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pga2 {
        match self.bits {
            false => Pga2::Disable,
            true => Pga2::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pga2::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pga2::Enable
    }
}
#[doc = "Field `pga2` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pga2W<'a, REG> = crate::BitWriter<'a, REG, Pga2>;
impl<'a, REG> Pga2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga2::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pga2::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epi {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Epi> for bool {
    #[inline(always)]
    fn from(variant: Epi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `epi` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type EpiR = crate::BitReader<Epi>;
impl EpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epi {
        match self.bits {
            false => Epi::Disable,
            true => Epi::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Epi::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Epi::Enable
    }
}
#[doc = "Field `epi` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type EpiW<'a, REG> = crate::BitWriter<'a, REG, Epi>;
impl<'a, REG> EpiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Epi::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Epi::Enable)
    }
}
#[doc = "Write key: the write only takes effect if KEY is 0x26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "38: KEY to allow write access to this register"]
    Unlock = 38,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x26"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "KEY to allow write access to this register"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn tinie(&self) -> TinieR {
        TinieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga0(&self) -> Pga0R {
        Pga0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga1(&self) -> Pga1R {
        Pga1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga2(&self) -> Pga2R {
        Pga2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn epi(&self) -> EpiR {
        EpiR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWREN_SYSPERIPH")
            .field("tinie", &self.tinie())
            .field("pga0", &self.pga0())
            .field("pga1", &self.pga1())
            .field("pga2", &self.pga2())
            .field("epi", &self.epi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn tinie(&mut self) -> TinieW<'_, PwrenSysperiphSpec> {
        TinieW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga0(&mut self) -> Pga0W<'_, PwrenSysperiphSpec> {
        Pga0W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga1(&mut self) -> Pga1W<'_, PwrenSysperiphSpec> {
        Pga1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pga2(&mut self) -> Pga2W<'_, PwrenSysperiphSpec> {
        Pga2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn epi(&mut self) -> EpiW<'_, PwrenSysperiphSpec> {
        EpiW::new(self, 4)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x26"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, PwrenSysperiphSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Register to control the power state\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren_sysperiph::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren_sysperiph::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenSysperiphSpec;
impl crate::RegisterSpec for PwrenSysperiphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren_sysperiph::R`](R) reader structure"]
impl crate::Readable for PwrenSysperiphSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren_sysperiph::W`](W) writer structure"]
impl crate::Writable for PwrenSysperiphSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWREN_SYSPERIPH to value 0"]
impl crate::Resettable for PwrenSysperiphSpec {}
