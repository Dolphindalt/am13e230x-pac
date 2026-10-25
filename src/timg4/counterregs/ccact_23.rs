#[doc = "Register `CCACT_23[%s]` reader"]
pub type R = crate::R<Ccact23Spec>;
#[doc = "Register `CCACT_23[%s]` writer"]
pub type W = crate::W<Ccact23Spec>;
#[doc = "CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Zact> for u8 {
    #[inline(always)]
    fn from(variant: Zact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zact {
    type Ux = u8;
}
impl crate::IsEnum for Zact {}
#[doc = "Field `ZACT` reader - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type ZactR = crate::FieldReader<Zact>;
impl ZactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zact {
        match self.bits {
            0 => Zact::Disabled,
            1 => Zact::CcpHigh,
            2 => Zact::CcpLow,
            3 => Zact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Zact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Zact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Zact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Zact::CcpToggle
    }
}
#[doc = "Field `ZACT` writer - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type ZactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Zact, crate::Safe>;
impl<'a, REG> ZactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Zact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Zact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Zact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Zact::CcpToggle)
    }
}
#[doc = "CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Lact> for u8 {
    #[inline(always)]
    fn from(variant: Lact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lact {
    type Ux = u8;
}
impl crate::IsEnum for Lact {}
#[doc = "Field `LACT` reader - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type LactR = crate::FieldReader<Lact>;
impl LactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lact {
        match self.bits {
            0 => Lact::Disabled,
            1 => Lact::CcpHigh,
            2 => Lact::CcpLow,
            3 => Lact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Lact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Lact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Lact::CcpToggle
    }
}
#[doc = "Field `LACT` writer - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type LactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lact, crate::Safe>;
impl<'a, REG> LactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Lact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Lact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Lact::CcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Cdact> for u8 {
    #[inline(always)]
    fn from(variant: Cdact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdact {
    type Ux = u8;
}
impl crate::IsEnum for Cdact {}
#[doc = "Field `CDACT` reader - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type CdactR = crate::FieldReader<Cdact>;
impl CdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdact {
        match self.bits {
            0 => Cdact::Disabled,
            1 => Cdact::CcpHigh,
            2 => Cdact::CcpLow,
            3 => Cdact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cdact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Cdact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Cdact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Cdact::CcpToggle
    }
}
#[doc = "Field `CDACT` writer - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type CdactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cdact, crate::Safe>;
impl<'a, REG> CdactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cdact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cdact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cdact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cdact::CcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cuact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Cuact> for u8 {
    #[inline(always)]
    fn from(variant: Cuact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cuact {
    type Ux = u8;
}
impl crate::IsEnum for Cuact {}
#[doc = "Field `CUACT` reader - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type CuactR = crate::FieldReader<Cuact>;
impl CuactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cuact {
        match self.bits {
            0 => Cuact::Disabled,
            1 => Cuact::CcpHigh,
            2 => Cuact::CcpLow,
            3 => Cuact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cuact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Cuact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Cuact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Cuact::CcpToggle
    }
}
#[doc = "Field `CUACT` writer - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type CuactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cuact, crate::Safe>;
impl<'a, REG> CuactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cuact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cuact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cuact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cuact::CcpToggle)
    }
}
#[doc = "CCP Output Action on CC2D event.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2dact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Cc2dact> for u8 {
    #[inline(always)]
    fn from(variant: Cc2dact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2dact {
    type Ux = u8;
}
impl crate::IsEnum for Cc2dact {}
#[doc = "Field `CC2DACT` reader - CCP Output Action on CC2D event."]
pub type Cc2dactR = crate::FieldReader<Cc2dact>;
impl Cc2dactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2dact {
        match self.bits {
            0 => Cc2dact::Disabled,
            1 => Cc2dact::CcpHigh,
            2 => Cc2dact::CcpLow,
            3 => Cc2dact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cc2dact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Cc2dact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Cc2dact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Cc2dact::CcpToggle
    }
}
#[doc = "Field `CC2DACT` writer - CCP Output Action on CC2D event."]
pub type Cc2dactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2dact, crate::Safe>;
impl<'a, REG> Cc2dactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2dact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2dact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2dact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2dact::CcpToggle)
    }
}
#[doc = "CCP Output Action on CC2U event.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2uact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
}
impl From<Cc2uact> for u8 {
    #[inline(always)]
    fn from(variant: Cc2uact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2uact {
    type Ux = u8;
}
impl crate::IsEnum for Cc2uact {}
#[doc = "Field `CC2UACT` reader - CCP Output Action on CC2U event."]
pub type Cc2uactR = crate::FieldReader<Cc2uact>;
impl Cc2uactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2uact {
        match self.bits {
            0 => Cc2uact::Disabled,
            1 => Cc2uact::CcpHigh,
            2 => Cc2uact::CcpLow,
            3 => Cc2uact::CcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cc2uact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Cc2uact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Cc2uact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Cc2uact::CcpToggle
    }
}
#[doc = "Field `CC2UACT` writer - CCP Output Action on CC2U event."]
pub type Cc2uactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2uact, crate::Safe>;
impl<'a, REG> Cc2uactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2uact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2uact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2uact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2uact::CcpToggle)
    }
}
#[doc = "CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fenact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
    #[doc = "4: CCP output value is tristated"]
    CcpHighz = 4,
}
impl From<Fenact> for u8 {
    #[inline(always)]
    fn from(variant: Fenact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fenact {
    type Ux = u8;
}
impl crate::IsEnum for Fenact {}
#[doc = "Field `FENACT` reader - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
pub type FenactR = crate::FieldReader<Fenact>;
impl FenactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fenact> {
        match self.bits {
            0 => Some(Fenact::Disabled),
            1 => Some(Fenact::CcpHigh),
            2 => Some(Fenact::CcpLow),
            3 => Some(Fenact::CcpToggle),
            4 => Some(Fenact::CcpHighz),
            _ => None,
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fenact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Fenact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Fenact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Fenact::CcpToggle
    }
    #[doc = "CCP output value is tristated"]
    #[inline(always)]
    pub fn is_ccp_highz(&self) -> bool {
        *self == Fenact::CcpHighz
    }
}
#[doc = "Field `FENACT` writer - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
pub type FenactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fenact, crate::Safe>;
impl<'a, REG> FenactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fenact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Fenact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Fenact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Fenact::CcpToggle)
    }
    #[doc = "CCP output value is tristated"]
    #[inline(always)]
    pub fn ccp_highz(self) -> &'a mut crate::W<REG> {
        self.variant(Fenact::CcpHighz)
    }
}
#[doc = "CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fexact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
    #[doc = "3: CCP output value is toggled"]
    CcpToggle = 3,
    #[doc = "4: CCP output value is tristated"]
    CcpHighz = 4,
}
impl From<Fexact> for u8 {
    #[inline(always)]
    fn from(variant: Fexact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fexact {
    type Ux = u8;
}
impl crate::IsEnum for Fexact {}
#[doc = "Field `FEXACT` reader - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
pub type FexactR = crate::FieldReader<Fexact>;
impl FexactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fexact> {
        match self.bits {
            0 => Some(Fexact::Disabled),
            1 => Some(Fexact::CcpHigh),
            2 => Some(Fexact::CcpLow),
            3 => Some(Fexact::CcpToggle),
            4 => Some(Fexact::CcpHighz),
            _ => None,
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fexact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Fexact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Fexact::CcpLow
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn is_ccp_toggle(&self) -> bool {
        *self == Fexact::CcpToggle
    }
    #[doc = "CCP output value is tristated"]
    #[inline(always)]
    pub fn is_ccp_highz(&self) -> bool {
        *self == Fexact::CcpHighz
    }
}
#[doc = "Field `FEXACT` writer - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
pub type FexactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fexact, crate::Safe>;
impl<'a, REG> FexactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fexact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Fexact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Fexact::CcpLow)
    }
    #[doc = "CCP output value is toggled"]
    #[inline(always)]
    pub fn ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Fexact::CcpToggle)
    }
    #[doc = "CCP output value is tristated"]
    #[inline(always)]
    pub fn ccp_highz(self) -> &'a mut crate::W<REG> {
        self.variant(Fexact::CcpHighz)
    }
}
#[doc = "CCP Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swfrcact {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP output value is set low"]
    CcpLow = 2,
}
impl From<Swfrcact> for u8 {
    #[inline(always)]
    fn from(variant: Swfrcact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swfrcact {
    type Ux = u8;
}
impl crate::IsEnum for Swfrcact {}
#[doc = "Field `SWFRCACT` reader - CCP Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type SwfrcactR = crate::FieldReader<Swfrcact>;
impl SwfrcactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swfrcact> {
        match self.bits {
            0 => Some(Swfrcact::Disabled),
            1 => Some(Swfrcact::CcpHigh),
            2 => Some(Swfrcact::CcpLow),
            _ => None,
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Swfrcact::Disabled
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == Swfrcact::CcpHigh
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == Swfrcact::CcpLow
    }
}
#[doc = "Field `SWFRCACT` writer - CCP Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type SwfrcactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Swfrcact, crate::Safe>;
impl<'a, REG> SwfrcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Swfrcact::Disabled)
    }
    #[doc = "CCP output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Swfrcact::CcpHigh)
    }
    #[doc = "CCP output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Swfrcact::CcpLow)
    }
}
#[doc = "CCP Complimentary Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwfrcactCmpl {
    #[doc = "0: This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    Disabled = 0,
    #[doc = "1: CCP Complimentary output value is set high"]
    CcpHigh = 1,
    #[doc = "2: CCP Complimentary output value is set low"]
    CcpLow = 2,
}
impl From<SwfrcactCmpl> for u8 {
    #[inline(always)]
    fn from(variant: SwfrcactCmpl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwfrcactCmpl {
    type Ux = u8;
}
impl crate::IsEnum for SwfrcactCmpl {}
#[doc = "Field `SWFRCACT_CMPL` reader - CCP Complimentary Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type SwfrcactCmplR = crate::FieldReader<SwfrcactCmpl>;
impl SwfrcactCmplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwfrcactCmpl> {
        match self.bits {
            0 => Some(SwfrcactCmpl::Disabled),
            1 => Some(SwfrcactCmpl::CcpHigh),
            2 => Some(SwfrcactCmpl::CcpLow),
            _ => None,
        }
    }
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SwfrcactCmpl::Disabled
    }
    #[doc = "CCP Complimentary output value is set high"]
    #[inline(always)]
    pub fn is_ccp_high(&self) -> bool {
        *self == SwfrcactCmpl::CcpHigh
    }
    #[doc = "CCP Complimentary output value is set low"]
    #[inline(always)]
    pub fn is_ccp_low(&self) -> bool {
        *self == SwfrcactCmpl::CcpLow
    }
}
#[doc = "Field `SWFRCACT_CMPL` writer - CCP Complimentary Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type SwfrcactCmplW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwfrcactCmpl, crate::Safe>;
impl<'a, REG> SwfrcactCmplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is disabled and a lower priority event is selected if asserting. The CCP output value is unaffected by the event."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SwfrcactCmpl::Disabled)
    }
    #[doc = "CCP Complimentary output value is set high"]
    #[inline(always)]
    pub fn ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(SwfrcactCmpl::CcpHigh)
    }
    #[doc = "CCP Complimentary output value is set low"]
    #[inline(always)]
    pub fn ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(SwfrcactCmpl::CcpLow)
    }
}
impl R {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn zact(&self) -> ZactR {
        ZactR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn lact(&self) -> LactR {
        LactR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn cdact(&self) -> CdactR {
        CdactR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn cuact(&self) -> CuactR {
        CuactR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn cc2dact(&self) -> Cc2dactR {
        Cc2dactR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn cc2uact(&self) -> Cc2uactR {
        Cc2uactR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 22:24 - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
    #[inline(always)]
    pub fn fenact(&self) -> FenactR {
        FenactR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
    #[inline(always)]
    pub fn fexact(&self) -> FexactR {
        FexactR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn swfrcact(&self) -> SwfrcactR {
        SwfrcactR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CCP Complimentary Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn swfrcact_cmpl(&self) -> SwfrcactCmplR {
        SwfrcactCmplR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCACT_23")
            .field("zact", &self.zact())
            .field("lact", &self.lact())
            .field("cdact", &self.cdact())
            .field("cuact", &self.cuact())
            .field("cc2dact", &self.cc2dact())
            .field("cc2uact", &self.cc2uact())
            .field("fenact", &self.fenact())
            .field("fexact", &self.fexact())
            .field("swfrcact", &self.swfrcact())
            .field("swfrcact_cmpl", &self.swfrcact_cmpl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn zact(&mut self) -> ZactW<'_, Ccact23Spec> {
        ZactW::new(self, 0)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn lact(&mut self) -> LactW<'_, Ccact23Spec> {
        LactW::new(self, 3)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn cdact(&mut self) -> CdactW<'_, Ccact23Spec> {
        CdactW::new(self, 6)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn cuact(&mut self) -> CuactW<'_, Ccact23Spec> {
        CuactW::new(self, 9)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn cc2dact(&mut self) -> Cc2dactW<'_, Ccact23Spec> {
        Cc2dactW::new(self, 12)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn cc2uact(&mut self) -> Cc2uactW<'_, Ccact23Spec> {
        Cc2uactW::new(self, 15)
    }
    #[doc = "Bits 22:24 - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
    #[inline(always)]
    pub fn fenact(&mut self) -> FenactW<'_, Ccact23Spec> {
        FenactW::new(self, 22)
    }
    #[doc = "Bits 25:27 - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
    #[inline(always)]
    pub fn fexact(&mut self) -> FexactW<'_, Ccact23Spec> {
        FexactW::new(self, 25)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn swfrcact(&mut self) -> SwfrcactW<'_, Ccact23Spec> {
        SwfrcactW::new(self, 28)
    }
    #[doc = "Bits 30:31 - CCP Complimentary Output Action on Software Force Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn swfrcact_cmpl(&mut self) -> SwfrcactCmplW<'_, Ccact23Spec> {
        SwfrcactCmplW::new(self, 30)
    }
}
#[doc = "Capture or Compare Action Registers 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccact23Spec;
impl crate::RegisterSpec for Ccact23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccact_23::R`](R) reader structure"]
impl crate::Readable for Ccact23Spec {}
#[doc = "`write(|w| ..)` method takes [`ccact_23::W`](W) writer structure"]
impl crate::Writable for Ccact23Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCACT_23[%s] to value 0"]
impl crate::Resettable for Ccact23Spec {}
