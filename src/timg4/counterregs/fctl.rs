#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FctlSpec>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FctlSpec>;
#[doc = "Fault Input Enable This bit enables the input for fault detection.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fien {
    #[doc = "0: Fault Input Disabled"]
    Disabled = 0,
    #[doc = "1: Fault Input Enabled"]
    Enabled = 1,
}
impl From<Fien> for bool {
    #[inline(always)]
    fn from(variant: Fien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIEN` reader - Fault Input Enable This bit enables the input for fault detection."]
pub type FienR = crate::BitReader<Fien>;
impl FienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fien {
        match self.bits {
            false => Fien::Disabled,
            true => Fien::Enabled,
        }
    }
    #[doc = "Fault Input Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fien::Disabled
    }
    #[doc = "Fault Input Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fien::Enabled
    }
}
#[doc = "Field `FIEN` writer - Fault Input Enable This bit enables the input for fault detection."]
pub type FienW<'a, REG> = crate::BitWriter<'a, REG, Fien>;
impl<'a, REG> FienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fien::Disabled)
    }
    #[doc = "Fault Input Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fien::Enabled)
    }
}
#[doc = "Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fi {
    #[doc = "0: Overall Fault condition is not dependent on sensed input."]
    Independent = 0,
    #[doc = "1: Overall Fault condition is dependent on sensed input."]
    Dependent = 1,
}
impl From<Fi> for bool {
    #[inline(always)]
    fn from(variant: Fi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FI` reader - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
pub type FiR = crate::BitReader<Fi>;
impl FiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fi {
        match self.bits {
            false => Fi::Independent,
            true => Fi::Dependent,
        }
    }
    #[doc = "Overall Fault condition is not dependent on sensed input."]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Fi::Independent
    }
    #[doc = "Overall Fault condition is dependent on sensed input."]
    #[inline(always)]
    pub fn is_dependent(&self) -> bool {
        *self == Fi::Dependent
    }
}
#[doc = "Field `FI` writer - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
pub type FiW<'a, REG> = crate::BitWriter<'a, REG, Fi>;
impl<'a, REG> FiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overall Fault condition is not dependent on sensed input."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Fi::Independent)
    }
    #[doc = "Overall Fault condition is dependent on sensed input."]
    #[inline(always)]
    pub fn dependent(self) -> &'a mut crate::W<REG> {
        self.variant(Fi::Dependent)
    }
}
#[doc = "Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fl {
    #[doc = "0: Overall fault condition is not dependent on the F bit in RIS"]
    NoLatch = 0,
    #[doc = "1: Overall fault condition is dependent on the F bit in RIS"]
    LatchSwClr = 1,
    #[doc = "2: Fault condition is latched. Fault condition is cleared on a zero event if the fault input is 0."]
    LatchZClr = 2,
    #[doc = "3: Fault condition is latched. Fault condition is cleared on a load event if the fault input is 0."]
    LatchLdClr = 3,
}
impl From<Fl> for u8 {
    #[inline(always)]
    fn from(variant: Fl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fl {
    type Ux = u8;
}
impl crate::IsEnum for Fl {}
#[doc = "Field `FL` reader - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
pub type FlR = crate::FieldReader<Fl>;
impl FlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fl {
        match self.bits {
            0 => Fl::NoLatch,
            1 => Fl::LatchSwClr,
            2 => Fl::LatchZClr,
            3 => Fl::LatchLdClr,
            _ => unreachable!(),
        }
    }
    #[doc = "Overall fault condition is not dependent on the F bit in RIS"]
    #[inline(always)]
    pub fn is_no_latch(&self) -> bool {
        *self == Fl::NoLatch
    }
    #[doc = "Overall fault condition is dependent on the F bit in RIS"]
    #[inline(always)]
    pub fn is_latch_sw_clr(&self) -> bool {
        *self == Fl::LatchSwClr
    }
    #[doc = "Fault condition is latched. Fault condition is cleared on a zero event if the fault input is 0."]
    #[inline(always)]
    pub fn is_latch_z_clr(&self) -> bool {
        *self == Fl::LatchZClr
    }
    #[doc = "Fault condition is latched. Fault condition is cleared on a load event if the fault input is 0."]
    #[inline(always)]
    pub fn is_latch_ld_clr(&self) -> bool {
        *self == Fl::LatchLdClr
    }
}
#[doc = "Field `FL` writer - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
pub type FlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fl, crate::Safe>;
impl<'a, REG> FlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Overall fault condition is not dependent on the F bit in RIS"]
    #[inline(always)]
    pub fn no_latch(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::NoLatch)
    }
    #[doc = "Overall fault condition is dependent on the F bit in RIS"]
    #[inline(always)]
    pub fn latch_sw_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::LatchSwClr)
    }
    #[doc = "Fault condition is latched. Fault condition is cleared on a zero event if the fault input is 0."]
    #[inline(always)]
    pub fn latch_z_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::LatchZClr)
    }
    #[doc = "Fault condition is latched. Fault condition is cleared on a load event if the fault input is 0."]
    #[inline(always)]
    pub fn latch_ld_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fl::LatchLdClr)
    }
}
#[doc = "Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfim {
    #[doc = "0: Selected trigger does not participate in fault condition generation"]
    Disabled = 0,
    #[doc = "1: Selected trigger participates in fault condition generation"]
    Enabled = 1,
}
impl From<Tfim> for bool {
    #[inline(always)]
    fn from(variant: Tfim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFIM` reader - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
pub type TfimR = crate::BitReader<Tfim>;
impl TfimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfim {
        match self.bits {
            false => Tfim::Disabled,
            true => Tfim::Enabled,
        }
    }
    #[doc = "Selected trigger does not participate in fault condition generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tfim::Disabled
    }
    #[doc = "Selected trigger participates in fault condition generation"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tfim::Enabled
    }
}
#[doc = "Field `TFIM` writer - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
pub type TfimW<'a, REG> = crate::BitWriter<'a, REG, Tfim>;
impl<'a, REG> TfimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected trigger does not participate in fault condition generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfim::Disabled)
    }
    #[doc = "Selected trigger participates in fault condition generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfim::Enabled)
    }
}
#[doc = "Specifies whether the COMP0 output high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenac0 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenac0> for bool {
    #[inline(always)]
    fn from(variant: Fsenac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENAC0` reader - Specifies whether the COMP0 output high/low is treated as fault condition."]
pub type Fsenac0R = crate::BitReader<Fsenac0>;
impl Fsenac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenac0 {
        match self.bits {
            false => Fsenac0::Lowctive,
            true => Fsenac0::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenac0::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenac0::Highactive
    }
}
#[doc = "Field `FSENAC0` writer - Specifies whether the COMP0 output high/low is treated as fault condition."]
pub type Fsenac0W<'a, REG> = crate::BitWriter<'a, REG, Fsenac0>;
impl<'a, REG> Fsenac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac0::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac0::Highactive)
    }
}
#[doc = "Specifies whether the COMP1 output high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenac1 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenac1> for bool {
    #[inline(always)]
    fn from(variant: Fsenac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENAC1` reader - Specifies whether the COMP1 output high/low is treated as fault condition."]
pub type Fsenac1R = crate::BitReader<Fsenac1>;
impl Fsenac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenac1 {
        match self.bits {
            false => Fsenac1::Lowctive,
            true => Fsenac1::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenac1::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenac1::Highactive
    }
}
#[doc = "Field `FSENAC1` writer - Specifies whether the COMP1 output high/low is treated as fault condition."]
pub type Fsenac1W<'a, REG> = crate::BitWriter<'a, REG, Fsenac1>;
impl<'a, REG> Fsenac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac1::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac1::Highactive)
    }
}
#[doc = "Specifies whether the COMP2 output high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenac2 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenac2> for bool {
    #[inline(always)]
    fn from(variant: Fsenac2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENAC2` reader - Specifies whether the COMP2 output high/low is treated as fault condition."]
pub type Fsenac2R = crate::BitReader<Fsenac2>;
impl Fsenac2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenac2 {
        match self.bits {
            false => Fsenac2::Lowctive,
            true => Fsenac2::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenac2::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenac2::Highactive
    }
}
#[doc = "Field `FSENAC2` writer - Specifies whether the COMP2 output high/low is treated as fault condition."]
pub type Fsenac2W<'a, REG> = crate::BitWriter<'a, REG, Fsenac2>;
impl<'a, REG> Fsenac2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac2::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenac2::Highactive)
    }
}
#[doc = "Specifies whether the external fault pin0 high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenext0 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenext0> for bool {
    #[inline(always)]
    fn from(variant: Fsenext0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENEXT0` reader - Specifies whether the external fault pin0 high/low is treated as fault condition."]
pub type Fsenext0R = crate::BitReader<Fsenext0>;
impl Fsenext0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenext0 {
        match self.bits {
            false => Fsenext0::Lowctive,
            true => Fsenext0::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenext0::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenext0::Highactive
    }
}
#[doc = "Field `FSENEXT0` writer - Specifies whether the external fault pin0 high/low is treated as fault condition."]
pub type Fsenext0W<'a, REG> = crate::BitWriter<'a, REG, Fsenext0>;
impl<'a, REG> Fsenext0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext0::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext0::Highactive)
    }
}
#[doc = "Specifies whether the external fault pin1 high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenext1 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenext1> for bool {
    #[inline(always)]
    fn from(variant: Fsenext1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENEXT1` reader - Specifies whether the external fault pin1 high/low is treated as fault condition."]
pub type Fsenext1R = crate::BitReader<Fsenext1>;
impl Fsenext1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenext1 {
        match self.bits {
            false => Fsenext1::Lowctive,
            true => Fsenext1::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenext1::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenext1::Highactive
    }
}
#[doc = "Field `FSENEXT1` writer - Specifies whether the external fault pin1 high/low is treated as fault condition."]
pub type Fsenext1W<'a, REG> = crate::BitWriter<'a, REG, Fsenext1>;
impl<'a, REG> Fsenext1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext1::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext1::Highactive)
    }
}
#[doc = "Specifies whether the external fault pin2 high/low is treated as fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsenext2 {
    #[doc = "0: Fault Input is active low."]
    Lowctive = 0,
    #[doc = "1: Fault Input is active high."]
    Highactive = 1,
}
impl From<Fsenext2> for bool {
    #[inline(always)]
    fn from(variant: Fsenext2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSENEXT2` reader - Specifies whether the external fault pin2 high/low is treated as fault condition."]
pub type Fsenext2R = crate::BitReader<Fsenext2>;
impl Fsenext2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsenext2 {
        match self.bits {
            false => Fsenext2::Lowctive,
            true => Fsenext2::Highactive,
        }
    }
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn is_lowctive(&self) -> bool {
        *self == Fsenext2::Lowctive
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn is_highactive(&self) -> bool {
        *self == Fsenext2::Highactive
    }
}
#[doc = "Field `FSENEXT2` writer - Specifies whether the external fault pin2 high/low is treated as fault condition."]
pub type Fsenext2W<'a, REG> = crate::BitWriter<'a, REG, Fsenext2>;
impl<'a, REG> Fsenext2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault Input is active low."]
    #[inline(always)]
    pub fn lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext2::Lowctive)
    }
    #[doc = "Fault Input is active high."]
    #[inline(always)]
    pub fn highactive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsenext2::Highactive)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Input Enable This bit enables the input for fault detection."]
    #[inline(always)]
    pub fn fien(&self) -> FienR {
        FienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
    #[inline(always)]
    pub fn fi(&self) -> FiR {
        FiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
    #[inline(always)]
    pub fn fl(&self) -> FlR {
        FlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
    #[inline(always)]
    pub fn tfim(&self) -> TfimR {
        TfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the COMP0 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac0(&self) -> Fsenac0R {
        Fsenac0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies whether the COMP1 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac1(&self) -> Fsenac1R {
        Fsenac1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Specifies whether the COMP2 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac2(&self) -> Fsenac2R {
        Fsenac2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Specifies whether the external fault pin0 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext0(&self) -> Fsenext0R {
        Fsenext0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies whether the external fault pin1 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext1(&self) -> Fsenext1R {
        Fsenext1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Specifies whether the external fault pin2 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext2(&self) -> Fsenext2R {
        Fsenext2R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCTL")
            .field("fien", &self.fien())
            .field("fi", &self.fi())
            .field("tfim", &self.tfim())
            .field("fl", &self.fl())
            .field("fsenac0", &self.fsenac0())
            .field("fsenac1", &self.fsenac1())
            .field("fsenac2", &self.fsenac2())
            .field("fsenext0", &self.fsenext0())
            .field("fsenext1", &self.fsenext1())
            .field("fsenext2", &self.fsenext2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input Enable This bit enables the input for fault detection."]
    #[inline(always)]
    pub fn fien(&mut self) -> FienW<'_, FctlSpec> {
        FienW::new(self, 0)
    }
    #[doc = "Bit 2 - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
    #[inline(always)]
    pub fn fi(&mut self) -> FiW<'_, FctlSpec> {
        FiW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
    #[inline(always)]
    pub fn fl(&mut self) -> FlW<'_, FctlSpec> {
        FlW::new(self, 3)
    }
    #[doc = "Bit 7 - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
    #[inline(always)]
    pub fn tfim(&mut self) -> TfimW<'_, FctlSpec> {
        TfimW::new(self, 7)
    }
    #[doc = "Bit 8 - Specifies whether the COMP0 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac0(&mut self) -> Fsenac0W<'_, FctlSpec> {
        Fsenac0W::new(self, 8)
    }
    #[doc = "Bit 9 - Specifies whether the COMP1 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac1(&mut self) -> Fsenac1W<'_, FctlSpec> {
        Fsenac1W::new(self, 9)
    }
    #[doc = "Bit 10 - Specifies whether the COMP2 output high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenac2(&mut self) -> Fsenac2W<'_, FctlSpec> {
        Fsenac2W::new(self, 10)
    }
    #[doc = "Bit 11 - Specifies whether the external fault pin0 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext0(&mut self) -> Fsenext0W<'_, FctlSpec> {
        Fsenext0W::new(self, 11)
    }
    #[doc = "Bit 12 - Specifies whether the external fault pin1 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext1(&mut self) -> Fsenext1W<'_, FctlSpec> {
        Fsenext1W::new(self, 12)
    }
    #[doc = "Bit 13 - Specifies whether the external fault pin2 high/low is treated as fault condition."]
    #[inline(always)]
    pub fn fsenext2(&mut self) -> Fsenext2W<'_, FctlSpec> {
        Fsenext2W::new(self, 13)
    }
}
#[doc = "Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctlSpec;
impl crate::RegisterSpec for FctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FctlSpec {}
