#[doc = "Register `CANTX` reader"]
pub type R = crate::R<CantxSpec>;
#[doc = "Register `CANTX` writer"]
pub type W = crate::W<CantxSpec>;
#[doc = "Peripheral-Analog Channel State\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pstate {
    #[doc = "0: P-Channel is in Unassigned State"]
    Unassigned = 0,
    #[doc = "1: P-Channel is in Handover State"]
    Handover = 1,
    #[doc = "2: P-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    ConcNotLocked = 2,
    #[doc = "3: P-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    ConcLocked = 3,
}
impl From<Pstate> for u8 {
    #[inline(always)]
    fn from(variant: Pstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pstate {
    type Ux = u8;
}
impl crate::IsEnum for Pstate {}
#[doc = "Field `PSTATE` reader - Peripheral-Analog Channel State"]
pub type PstateR = crate::FieldReader<Pstate>;
impl PstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pstate {
        match self.bits {
            0 => Pstate::Unassigned,
            1 => Pstate::Handover,
            2 => Pstate::ConcNotLocked,
            3 => Pstate::ConcLocked,
            _ => unreachable!(),
        }
    }
    #[doc = "P-Channel is in Unassigned State"]
    #[inline(always)]
    pub fn is_unassigned(&self) -> bool {
        *self == Pstate::Unassigned
    }
    #[doc = "P-Channel is in Handover State"]
    #[inline(always)]
    pub fn is_handover(&self) -> bool {
        *self == Pstate::Handover
    }
    #[doc = "P-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    #[inline(always)]
    pub fn is_conc_not_locked(&self) -> bool {
        *self == Pstate::ConcNotLocked
    }
    #[doc = "P-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    #[inline(always)]
    pub fn is_conc_locked(&self) -> bool {
        *self == Pstate::ConcLocked
    }
}
#[doc = "Field `PSTATE` writer - Peripheral-Analog Channel State"]
pub type PstateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pstate, crate::Safe>;
impl<'a, REG> PstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "P-Channel is in Unassigned State"]
    #[inline(always)]
    pub fn unassigned(self) -> &'a mut crate::W<REG> {
        self.variant(Pstate::Unassigned)
    }
    #[doc = "P-Channel is in Handover State"]
    #[inline(always)]
    pub fn handover(self) -> &'a mut crate::W<REG> {
        self.variant(Pstate::Handover)
    }
    #[doc = "P-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    #[inline(always)]
    pub fn conc_not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Pstate::ConcNotLocked)
    }
    #[doc = "P-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    #[inline(always)]
    pub fn conc_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Pstate::ConcLocked)
    }
}
#[doc = "GPIO Channel State\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gstate {
    #[doc = "0: G-Channel is in Unassigned State"]
    Unassigned = 0,
    #[doc = "1: G-Channel is in Handover State"]
    Handover = 1,
    #[doc = "2: G-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    ConcNotLocked = 2,
    #[doc = "3: G-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    ConcLocked = 3,
}
impl From<Gstate> for u8 {
    #[inline(always)]
    fn from(variant: Gstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gstate {
    type Ux = u8;
}
impl crate::IsEnum for Gstate {}
#[doc = "Field `GSTATE` reader - GPIO Channel State"]
pub type GstateR = crate::FieldReader<Gstate>;
impl GstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gstate {
        match self.bits {
            0 => Gstate::Unassigned,
            1 => Gstate::Handover,
            2 => Gstate::ConcNotLocked,
            3 => Gstate::ConcLocked,
            _ => unreachable!(),
        }
    }
    #[doc = "G-Channel is in Unassigned State"]
    #[inline(always)]
    pub fn is_unassigned(&self) -> bool {
        *self == Gstate::Unassigned
    }
    #[doc = "G-Channel is in Handover State"]
    #[inline(always)]
    pub fn is_handover(&self) -> bool {
        *self == Gstate::Handover
    }
    #[doc = "G-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    #[inline(always)]
    pub fn is_conc_not_locked(&self) -> bool {
        *self == Gstate::ConcNotLocked
    }
    #[doc = "G-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    #[inline(always)]
    pub fn is_conc_locked(&self) -> bool {
        *self == Gstate::ConcLocked
    }
}
#[doc = "Field `GSTATE` writer - GPIO Channel State"]
pub type GstateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gstate, crate::Safe>;
impl<'a, REG> GstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "G-Channel is in Unassigned State"]
    #[inline(always)]
    pub fn unassigned(self) -> &'a mut crate::W<REG> {
        self.variant(Gstate::Unassigned)
    }
    #[doc = "G-Channel is in Handover State"]
    #[inline(always)]
    pub fn handover(self) -> &'a mut crate::W<REG> {
        self.variant(Gstate::Handover)
    }
    #[doc = "G-Channel is in Connected State and not Locked (That is F field is allowed to change without going back through Unassigned state)"]
    #[inline(always)]
    pub fn conc_not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Gstate::ConcNotLocked)
    }
    #[doc = "G-Channel is in Connected State and Locked (That is F field is not allowed to change to a different non-Zero value until both G and P channels go to Unassigned)"]
    #[inline(always)]
    pub fn conc_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Gstate::ConcLocked)
    }
}
#[doc = "Pull down enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipd {
    #[doc = "0: No pull down"]
    NoPull = 0,
    #[doc = "1: Pull down"]
    Pull = 1,
}
impl From<Pipd> for bool {
    #[inline(always)]
    fn from(variant: Pipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPD` reader - Pull down enable"]
pub type PipdR = crate::BitReader<Pipd>;
impl PipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipd {
        match self.bits {
            false => Pipd::NoPull,
            true => Pipd::Pull,
        }
    }
    #[doc = "No pull down"]
    #[inline(always)]
    pub fn is_no_pull(&self) -> bool {
        *self == Pipd::NoPull
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn is_pull(&self) -> bool {
        *self == Pipd::Pull
    }
}
#[doc = "Field `PIPD` writer - Pull down enable"]
pub type PipdW<'a, REG> = crate::BitWriter<'a, REG, Pipd>;
impl<'a, REG> PipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pull down"]
    #[inline(always)]
    pub fn no_pull(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::NoPull)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn pull(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::Pull)
    }
}
#[doc = "Pull up enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipu {
    #[doc = "0: No pull up"]
    NoPull = 0,
    #[doc = "1: Pull up"]
    Pull = 1,
}
impl From<Pipu> for bool {
    #[inline(always)]
    fn from(variant: Pipu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPU` reader - Pull up enable"]
pub type PipuR = crate::BitReader<Pipu>;
impl PipuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipu {
        match self.bits {
            false => Pipu::NoPull,
            true => Pipu::Pull,
        }
    }
    #[doc = "No pull up"]
    #[inline(always)]
    pub fn is_no_pull(&self) -> bool {
        *self == Pipu::NoPull
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn is_pull(&self) -> bool {
        *self == Pipu::Pull
    }
}
#[doc = "Field `PIPU` writer - Pull up enable"]
pub type PipuW<'a, REG> = crate::BitWriter<'a, REG, Pipu>;
impl<'a, REG> PipuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pull up"]
    #[inline(always)]
    pub fn no_pull(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::NoPull)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn pull(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::Pull)
    }
}
#[doc = "Input enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inena {
    #[doc = "0: Inputs 0 to connected core"]
    Disable = 0,
    #[doc = "1: Inputs IO pad value to connected core"]
    Enable = 1,
}
impl From<Inena> for bool {
    #[inline(always)]
    fn from(variant: Inena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INENA` reader - Input enable"]
pub type InenaR = crate::BitReader<Inena>;
impl InenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inena {
        match self.bits {
            false => Inena::Disable,
            true => Inena::Enable,
        }
    }
    #[doc = "Inputs 0 to connected core"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inena::Disable
    }
    #[doc = "Inputs IO pad value to connected core"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inena::Enable
    }
}
#[doc = "Field `INENA` writer - Input enable"]
pub type InenaW<'a, REG> = crate::BitWriter<'a, REG, Inena>;
impl<'a, REG> InenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs 0 to connected core"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Disable)
    }
    #[doc = "Inputs IO pad value to connected core"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Enable)
    }
}
#[doc = "Hysteresis enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hysten {
    #[doc = "0: No hysteresis"]
    Dis = 0,
    #[doc = "1: Hysteresis on"]
    En = 1,
}
impl From<Hysten> for bool {
    #[inline(always)]
    fn from(variant: Hysten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYSTEN` reader - Hysteresis enable"]
pub type HystenR = crate::BitReader<Hysten>;
impl HystenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hysten {
        match self.bits {
            false => Hysten::Dis,
            true => Hysten::En,
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hysten::Dis
    }
    #[doc = "Hysteresis on"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hysten::En
    }
}
#[doc = "Field `HYSTEN` writer - Hysteresis enable"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG, Hysten>;
impl<'a, REG> HystenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::Dis)
    }
    #[doc = "Hysteresis on"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::En)
    }
}
#[doc = "Drive strength options\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drv {
    #[doc = "0: Lowest drive strength"]
    Strength0 = 0,
    #[doc = "1: Drive strength 2/8"]
    Strength1 = 1,
    #[doc = "2: Drive strength 3/8"]
    Strength2 = 2,
    #[doc = "3: Drive strength 4/8"]
    Strength3 = 3,
    #[doc = "4: Drive strength 5/8"]
    Strength4 = 4,
    #[doc = "5: Drive strength 6/8"]
    Strength5 = 5,
    #[doc = "6: Drive strength 7/8"]
    Strength6 = 6,
    #[doc = "7: Highest drive strength"]
    Strength7 = 7,
}
impl From<Drv> for u8 {
    #[inline(always)]
    fn from(variant: Drv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drv {
    type Ux = u8;
}
impl crate::IsEnum for Drv {}
#[doc = "Field `DRV` reader - Drive strength options"]
pub type DrvR = crate::FieldReader<Drv>;
impl DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drv {
        match self.bits {
            0 => Drv::Strength0,
            1 => Drv::Strength1,
            2 => Drv::Strength2,
            3 => Drv::Strength3,
            4 => Drv::Strength4,
            5 => Drv::Strength5,
            6 => Drv::Strength6,
            7 => Drv::Strength7,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive strength"]
    #[inline(always)]
    pub fn is_strength_0(&self) -> bool {
        *self == Drv::Strength0
    }
    #[doc = "Drive strength 2/8"]
    #[inline(always)]
    pub fn is_strength_1(&self) -> bool {
        *self == Drv::Strength1
    }
    #[doc = "Drive strength 3/8"]
    #[inline(always)]
    pub fn is_strength_2(&self) -> bool {
        *self == Drv::Strength2
    }
    #[doc = "Drive strength 4/8"]
    #[inline(always)]
    pub fn is_strength_3(&self) -> bool {
        *self == Drv::Strength3
    }
    #[doc = "Drive strength 5/8"]
    #[inline(always)]
    pub fn is_strength_4(&self) -> bool {
        *self == Drv::Strength4
    }
    #[doc = "Drive strength 6/8"]
    #[inline(always)]
    pub fn is_strength_5(&self) -> bool {
        *self == Drv::Strength5
    }
    #[doc = "Drive strength 7/8"]
    #[inline(always)]
    pub fn is_strength_6(&self) -> bool {
        *self == Drv::Strength6
    }
    #[doc = "Highest drive strength"]
    #[inline(always)]
    pub fn is_strength_7(&self) -> bool {
        *self == Drv::Strength7
    }
}
#[doc = "Field `DRV` writer - Drive strength options"]
pub type DrvW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drv, crate::Safe>;
impl<'a, REG> DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive strength"]
    #[inline(always)]
    pub fn strength_0(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength0)
    }
    #[doc = "Drive strength 2/8"]
    #[inline(always)]
    pub fn strength_1(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength1)
    }
    #[doc = "Drive strength 3/8"]
    #[inline(always)]
    pub fn strength_2(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength2)
    }
    #[doc = "Drive strength 4/8"]
    #[inline(always)]
    pub fn strength_3(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength3)
    }
    #[doc = "Drive strength 5/8"]
    #[inline(always)]
    pub fn strength_4(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength4)
    }
    #[doc = "Drive strength 6/8"]
    #[inline(always)]
    pub fn strength_5(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength5)
    }
    #[doc = "Drive strength 7/8"]
    #[inline(always)]
    pub fn strength_6(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength6)
    }
    #[doc = "Highest drive strength"]
    #[inline(always)]
    pub fn strength_7(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Strength7)
    }
}
#[doc = "High-Z instead of low output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highz0 {
    #[doc = "0: Pin can be driven low"]
    Low = 0,
    #[doc = "1: Pin is tri-stated instead of driven low"]
    Hiz = 1,
}
impl From<Highz0> for bool {
    #[inline(always)]
    fn from(variant: Highz0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHZ0` reader - High-Z instead of low output"]
pub type Highz0R = crate::BitReader<Highz0>;
impl Highz0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highz0 {
        match self.bits {
            false => Highz0::Low,
            true => Highz0::Hiz,
        }
    }
    #[doc = "Pin can be driven low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Highz0::Low
    }
    #[doc = "Pin is tri-stated instead of driven low"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == Highz0::Hiz
    }
}
#[doc = "Field `HIGHZ0` writer - High-Z instead of low output"]
pub type Highz0W<'a, REG> = crate::BitWriter<'a, REG, Highz0>;
impl<'a, REG> Highz0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin can be driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Highz0::Low)
    }
    #[doc = "Pin is tri-stated instead of driven low"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut crate::W<REG> {
        self.variant(Highz0::Hiz)
    }
}
#[doc = "High-Z instead of high output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highz1 {
    #[doc = "0: Pin can be driven high"]
    High = 0,
    #[doc = "1: Pin is tri-stated instead of driven high"]
    Hiz = 1,
}
impl From<Highz1> for bool {
    #[inline(always)]
    fn from(variant: Highz1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHZ1` reader - High-Z instead of high output"]
pub type Highz1R = crate::BitReader<Highz1>;
impl Highz1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highz1 {
        match self.bits {
            false => Highz1::High,
            true => Highz1::Hiz,
        }
    }
    #[doc = "Pin can be driven high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Highz1::High
    }
    #[doc = "Pin is tri-stated instead of driven high"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == Highz1::Hiz
    }
}
#[doc = "Field `HIGHZ1` writer - High-Z instead of high output"]
pub type Highz1W<'a, REG> = crate::BitWriter<'a, REG, Highz1>;
impl<'a, REG> Highz1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin can be driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Highz1::High)
    }
    #[doc = "Pin is tri-stated instead of driven high"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut crate::W<REG> {
        self.variant(Highz1::Hiz)
    }
}
#[doc = "Invert digital input/output relative to peripheral/GPIO\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Input and output are non-inverted"]
    NoChng = 0,
    #[doc = "1: Input and output are inverted"]
    Chng = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert digital input/output relative to peripheral/GPIO"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::NoChng,
            true => Inv::Chng,
        }
    }
    #[doc = "Input and output are non-inverted"]
    #[inline(always)]
    pub fn is_no_chng(&self) -> bool {
        *self == Inv::NoChng
    }
    #[doc = "Input and output are inverted"]
    #[inline(always)]
    pub fn is_chng(&self) -> bool {
        *self == Inv::Chng
    }
}
#[doc = "Field `INV` writer - Invert digital input/output relative to peripheral/GPIO"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input and output are non-inverted"]
    #[inline(always)]
    pub fn no_chng(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::NoChng)
    }
    #[doc = "Input and output are inverted"]
    #[inline(always)]
    pub fn chng(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Chng)
    }
}
#[doc = "Wake up enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuen {
    #[doc = "0: Wake up not enabled"]
    NoWakeup = 0,
    #[doc = "1: Wake up enabled"]
    Wakeup = 1,
}
impl From<Wuen> for bool {
    #[inline(always)]
    fn from(variant: Wuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUEN` reader - Wake up enable"]
pub type WuenR = crate::BitReader<Wuen>;
impl WuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuen {
        match self.bits {
            false => Wuen::NoWakeup,
            true => Wuen::Wakeup,
        }
    }
    #[doc = "Wake up not enabled"]
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == Wuen::NoWakeup
    }
    #[doc = "Wake up enabled"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == Wuen::Wakeup
    }
}
#[doc = "Field `WUEN` writer - Wake up enable"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG, Wuen>;
impl<'a, REG> WuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake up not enabled"]
    #[inline(always)]
    pub fn no_wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::NoWakeup)
    }
    #[doc = "Wake up enabled"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::Wakeup)
    }
}
#[doc = "Wake up compare value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcomp {
    #[doc = "0: Match 0 will wake"]
    Zero = 0,
    #[doc = "1: Match 1 will wake"]
    One = 1,
}
impl From<Wcomp> for bool {
    #[inline(always)]
    fn from(variant: Wcomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCOMP` reader - Wake up compare value"]
pub type WcompR = crate::BitReader<Wcomp>;
impl WcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcomp {
        match self.bits {
            false => Wcomp::Zero,
            true => Wcomp::One,
        }
    }
    #[doc = "Match 0 will wake"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Wcomp::Zero
    }
    #[doc = "Match 1 will wake"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Wcomp::One
    }
}
#[doc = "Field `WCOMP` writer - Wake up compare value"]
pub type WcompW<'a, REG> = crate::BitWriter<'a, REG, Wcomp>;
impl<'a, REG> WcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match 0 will wake"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Wcomp::Zero)
    }
    #[doc = "Match 1 will wake"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Wcomp::One)
    }
}
#[doc = "Reserved Slew Rate Control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slew {
    #[doc = "0: No Slew Rate Control"]
    Disable = 0,
    #[doc = "1: Use Slew Rate Control"]
    Enable = 1,
}
impl From<Slew> for bool {
    #[inline(always)]
    fn from(variant: Slew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEW` reader - Reserved Slew Rate Control"]
pub type SlewR = crate::BitReader<Slew>;
impl SlewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slew {
        match self.bits {
            false => Slew::Disable,
            true => Slew::Enable,
        }
    }
    #[doc = "No Slew Rate Control"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Slew::Disable
    }
    #[doc = "Use Slew Rate Control"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Slew::Enable
    }
}
#[doc = "Field `SLEW` writer - Reserved Slew Rate Control"]
pub type SlewW<'a, REG> = crate::BitWriter<'a, REG, Slew>;
impl<'a, REG> SlewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Slew Rate Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Disable)
    }
    #[doc = "Use Slew Rate Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Enable)
    }
}
#[doc = "Glitch Filter Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gflt {
    #[doc = "0: No internal glitch filter"]
    Disable = 0,
    #[doc = "1: Use internal glitch filter"]
    Enable = 1,
}
impl From<Gflt> for bool {
    #[inline(always)]
    fn from(variant: Gflt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFLT` reader - Glitch Filter Enable"]
pub type GfltR = crate::BitReader<Gflt>;
impl GfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gflt {
        match self.bits {
            false => Gflt::Disable,
            true => Gflt::Enable,
        }
    }
    #[doc = "No internal glitch filter"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gflt::Disable
    }
    #[doc = "Use internal glitch filter"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gflt::Enable
    }
}
#[doc = "Field `GFLT` writer - Glitch Filter Enable"]
pub type GfltW<'a, REG> = crate::BitWriter<'a, REG, Gflt>;
impl<'a, REG> GfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No internal glitch filter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gflt::Disable)
    }
    #[doc = "Use internal glitch filter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gflt::Enable)
    }
}
impl R {
    #[doc = "Bits 6:7 - Peripheral-Analog Channel State"]
    #[inline(always)]
    pub fn pstate(&self) -> PstateR {
        PstateR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO Channel State"]
    #[inline(always)]
    pub fn gstate(&self) -> GstateR {
        GstateR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Pull down enable"]
    #[inline(always)]
    pub fn pipd(&self) -> PipdR {
        PipdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull up enable"]
    #[inline(always)]
    pub fn pipu(&self) -> PipuR {
        PipuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input enable"]
    #[inline(always)]
    pub fn inena(&self) -> InenaR {
        InenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hysteresis enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Drive strength options"]
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - High-Z instead of low output"]
    #[inline(always)]
    pub fn highz0(&self) -> Highz0R {
        Highz0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High-Z instead of high output"]
    #[inline(always)]
    pub fn highz1(&self) -> Highz1R {
        Highz1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert digital input/output relative to peripheral/GPIO"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake up enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake up compare value"]
    #[inline(always)]
    pub fn wcomp(&self) -> WcompR {
        WcompR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved Slew Rate Control"]
    #[inline(always)]
    pub fn slew(&self) -> SlewR {
        SlewR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Glitch Filter Enable"]
    #[inline(always)]
    pub fn gflt(&self) -> GfltR {
        GfltR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANTX")
            .field("pstate", &self.pstate())
            .field("gstate", &self.gstate())
            .field("pipd", &self.pipd())
            .field("pipu", &self.pipu())
            .field("inena", &self.inena())
            .field("hysten", &self.hysten())
            .field("highz0", &self.highz0())
            .field("highz1", &self.highz1())
            .field("inv", &self.inv())
            .field("wuen", &self.wuen())
            .field("wcomp", &self.wcomp())
            .field("slew", &self.slew())
            .field("gflt", &self.gflt())
            .field("drv", &self.drv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - Peripheral-Analog Channel State"]
    #[inline(always)]
    pub fn pstate(&mut self) -> PstateW<'_, CantxSpec> {
        PstateW::new(self, 6)
    }
    #[doc = "Bits 14:15 - GPIO Channel State"]
    #[inline(always)]
    pub fn gstate(&mut self) -> GstateW<'_, CantxSpec> {
        GstateW::new(self, 14)
    }
    #[doc = "Bit 16 - Pull down enable"]
    #[inline(always)]
    pub fn pipd(&mut self) -> PipdW<'_, CantxSpec> {
        PipdW::new(self, 16)
    }
    #[doc = "Bit 17 - Pull up enable"]
    #[inline(always)]
    pub fn pipu(&mut self) -> PipuW<'_, CantxSpec> {
        PipuW::new(self, 17)
    }
    #[doc = "Bit 18 - Input enable"]
    #[inline(always)]
    pub fn inena(&mut self) -> InenaW<'_, CantxSpec> {
        InenaW::new(self, 18)
    }
    #[doc = "Bit 19 - Hysteresis enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HystenW<'_, CantxSpec> {
        HystenW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Drive strength options"]
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, CantxSpec> {
        DrvW::new(self, 20)
    }
    #[doc = "Bit 24 - High-Z instead of low output"]
    #[inline(always)]
    pub fn highz0(&mut self) -> Highz0W<'_, CantxSpec> {
        Highz0W::new(self, 24)
    }
    #[doc = "Bit 25 - High-Z instead of high output"]
    #[inline(always)]
    pub fn highz1(&mut self) -> Highz1W<'_, CantxSpec> {
        Highz1W::new(self, 25)
    }
    #[doc = "Bit 26 - Invert digital input/output relative to peripheral/GPIO"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, CantxSpec> {
        InvW::new(self, 26)
    }
    #[doc = "Bit 27 - Wake up enable"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WuenW<'_, CantxSpec> {
        WuenW::new(self, 27)
    }
    #[doc = "Bit 28 - Wake up compare value"]
    #[inline(always)]
    pub fn wcomp(&mut self) -> WcompW<'_, CantxSpec> {
        WcompW::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved Slew Rate Control"]
    #[inline(always)]
    pub fn slew(&mut self) -> SlewW<'_, CantxSpec> {
        SlewW::new(self, 29)
    }
    #[doc = "Bit 30 - Glitch Filter Enable"]
    #[inline(always)]
    pub fn gflt(&mut self) -> GfltW<'_, CantxSpec> {
        GfltW::new(self, 30)
    }
}
#[doc = "CAN TX IO\n\nYou can [`read`](crate::Reg::read) this register and get [`cantx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cantx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CantxSpec;
impl crate::RegisterSpec for CantxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cantx::R`](R) reader structure"]
impl crate::Readable for CantxSpec {}
#[doc = "`write(|w| ..)` method takes [`cantx::W`](W) writer structure"]
impl crate::Writable for CantxSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CANTX to value 0"]
impl crate::Resettable for CantxSpec {}
