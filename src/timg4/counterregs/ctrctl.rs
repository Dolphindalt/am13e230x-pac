#[doc = "Register `CTRCTL` reader"]
pub type R = crate::R<CtrctlSpec>;
#[doc = "Register `CTRCTL` writer"]
pub type W = crate::W<CtrctlSpec>;
#[doc = "Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
#[doc = "Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repeat {
    #[doc = "0: Does not automatically advance following a zero event."]
    Repeat0 = 0,
    #[doc = "1: Continues to advance following a zero event."]
    Repeat1 = 1,
    #[doc = "2: Reserved"]
    Repeat2 = 2,
    #[doc = "3: Continues to advance following a zero event if the debug mode is not in effect, or following the release of the debug mode."]
    Repeat3 = 3,
    #[doc = "4: Reserved"]
    Repeat4 = 4,
}
impl From<Repeat> for u8 {
    #[inline(always)]
    fn from(variant: Repeat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repeat {
    type Ux = u8;
}
impl crate::IsEnum for Repeat {}
#[doc = "Field `REPEAT` reader - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
pub type RepeatR = crate::FieldReader<Repeat>;
impl RepeatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Repeat> {
        match self.bits {
            0 => Some(Repeat::Repeat0),
            1 => Some(Repeat::Repeat1),
            2 => Some(Repeat::Repeat2),
            3 => Some(Repeat::Repeat3),
            4 => Some(Repeat::Repeat4),
            _ => None,
        }
    }
    #[doc = "Does not automatically advance following a zero event."]
    #[inline(always)]
    pub fn is_repeat_0(&self) -> bool {
        *self == Repeat::Repeat0
    }
    #[doc = "Continues to advance following a zero event."]
    #[inline(always)]
    pub fn is_repeat_1(&self) -> bool {
        *self == Repeat::Repeat1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_repeat_2(&self) -> bool {
        *self == Repeat::Repeat2
    }
    #[doc = "Continues to advance following a zero event if the debug mode is not in effect, or following the release of the debug mode."]
    #[inline(always)]
    pub fn is_repeat_3(&self) -> bool {
        *self == Repeat::Repeat3
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_repeat_4(&self) -> bool {
        *self == Repeat::Repeat4
    }
}
#[doc = "Field `REPEAT` writer - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
pub type RepeatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Repeat, crate::Safe>;
impl<'a, REG> RepeatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Does not automatically advance following a zero event."]
    #[inline(always)]
    pub fn repeat_0(self) -> &'a mut crate::W<REG> {
        self.variant(Repeat::Repeat0)
    }
    #[doc = "Continues to advance following a zero event."]
    #[inline(always)]
    pub fn repeat_1(self) -> &'a mut crate::W<REG> {
        self.variant(Repeat::Repeat1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn repeat_2(self) -> &'a mut crate::W<REG> {
        self.variant(Repeat::Repeat2)
    }
    #[doc = "Continues to advance following a zero event if the debug mode is not in effect, or following the release of the debug mode."]
    #[inline(always)]
    pub fn repeat_3(self) -> &'a mut crate::W<REG> {
        self.variant(Repeat::Repeat3)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn repeat_4(self) -> &'a mut crate::W<REG> {
        self.variant(Repeat::Repeat4)
    }
}
#[doc = "Count Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: Down"]
    Down = 0,
    #[doc = "1: Up/Down"]
    UpDown = 1,
    #[doc = "2: Counter counts up."]
    Up = 2,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - Count Mode"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cm> {
        match self.bits {
            0 => Some(Cm::Down),
            1 => Some(Cm::UpDown),
            2 => Some(Cm::Up),
            _ => None,
        }
    }
    #[doc = "Down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cm::Down
    }
    #[doc = "Up/Down"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == Cm::UpDown
    }
    #[doc = "Counter counts up."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cm::Up
    }
}
#[doc = "Field `CM` writer - Count Mode"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Down)
    }
    #[doc = "Up/Down"]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::UpDown)
    }
    #[doc = "Counter counts up."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Up)
    }
}
#[doc = "Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clc {
    #[doc = "0: CCCTL_0 LCOND"]
    Ccctl0Lcond = 0,
    #[doc = "1: CCCTL_1 LCOND"]
    Ccctl1Lcond = 1,
    #[doc = "2: CCCTL_2 LCOND This value exists when there are 4 channels."]
    Ccctl2Lcond = 2,
    #[doc = "3: CCCTL_3 LCOND This value exists when there are 4 channels."]
    Ccctl3Lcond = 3,
    #[doc = "4: Controlled by 2 input QEI mode. This value exists when TIMER support QEI feature."]
    Qei2inp = 4,
    #[doc = "5: Controlled by 3 input QEI mode. This value exists when TIMER support QEI feature."]
    Qei3inp = 5,
}
impl From<Clc> for u8 {
    #[inline(always)]
    fn from(variant: Clc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clc {
    type Ux = u8;
}
impl crate::IsEnum for Clc {}
#[doc = "Field `CLC` reader - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type ClcR = crate::FieldReader<Clc>;
impl ClcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clc> {
        match self.bits {
            0 => Some(Clc::Ccctl0Lcond),
            1 => Some(Clc::Ccctl1Lcond),
            2 => Some(Clc::Ccctl2Lcond),
            3 => Some(Clc::Ccctl3Lcond),
            4 => Some(Clc::Qei2inp),
            5 => Some(Clc::Qei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL_0 LCOND"]
    #[inline(always)]
    pub fn is_ccctl0_lcond(&self) -> bool {
        *self == Clc::Ccctl0Lcond
    }
    #[doc = "CCCTL_1 LCOND"]
    #[inline(always)]
    pub fn is_ccctl1_lcond(&self) -> bool {
        *self == Clc::Ccctl1Lcond
    }
    #[doc = "CCCTL_2 LCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl2_lcond(&self) -> bool {
        *self == Clc::Ccctl2Lcond
    }
    #[doc = "CCCTL_3 LCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl3_lcond(&self) -> bool {
        *self == Clc::Ccctl3Lcond
    }
    #[doc = "Controlled by 2 input QEI mode. This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_2inp(&self) -> bool {
        *self == Clc::Qei2inp
    }
    #[doc = "Controlled by 3 input QEI mode. This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_3inp(&self) -> bool {
        *self == Clc::Qei3inp
    }
}
#[doc = "Field `CLC` writer - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type ClcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clc, crate::Safe>;
impl<'a, REG> ClcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL_0 LCOND"]
    #[inline(always)]
    pub fn ccctl0_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Ccctl0Lcond)
    }
    #[doc = "CCCTL_1 LCOND"]
    #[inline(always)]
    pub fn ccctl1_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Ccctl1Lcond)
    }
    #[doc = "CCCTL_2 LCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl2_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Ccctl2Lcond)
    }
    #[doc = "CCCTL_3 LCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl3_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Ccctl3Lcond)
    }
    #[doc = "Controlled by 2 input QEI mode. This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Qei2inp)
    }
    #[doc = "Controlled by 3 input QEI mode. This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(Clc::Qei3inp)
    }
}
#[doc = "Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cac {
    #[doc = "0: CCCTL_0 ACOND"]
    Ccctl0Acond = 0,
    #[doc = "1: CCCTL_1 ACOND"]
    Ccctl1Acond = 1,
    #[doc = "2: CCCTL_2 ACOND This value exists when there are 4 channels."]
    Ccctl2Acond = 2,
    #[doc = "3: CCCTL_3 ACOND This value exists when there are 4 channels."]
    Ccctl3Acond = 3,
    #[doc = "4: Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    Qei2inp = 4,
    #[doc = "5: Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    Qei3inp = 5,
}
impl From<Cac> for u8 {
    #[inline(always)]
    fn from(variant: Cac) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cac {
    type Ux = u8;
}
impl crate::IsEnum for Cac {}
#[doc = "Field `CAC` reader - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CacR = crate::FieldReader<Cac>;
impl CacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cac> {
        match self.bits {
            0 => Some(Cac::Ccctl0Acond),
            1 => Some(Cac::Ccctl1Acond),
            2 => Some(Cac::Ccctl2Acond),
            3 => Some(Cac::Ccctl3Acond),
            4 => Some(Cac::Qei2inp),
            5 => Some(Cac::Qei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL_0 ACOND"]
    #[inline(always)]
    pub fn is_ccctl0_acond(&self) -> bool {
        *self == Cac::Ccctl0Acond
    }
    #[doc = "CCCTL_1 ACOND"]
    #[inline(always)]
    pub fn is_ccctl1_acond(&self) -> bool {
        *self == Cac::Ccctl1Acond
    }
    #[doc = "CCCTL_2 ACOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl2_acond(&self) -> bool {
        *self == Cac::Ccctl2Acond
    }
    #[doc = "CCCTL_3 ACOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl3_acond(&self) -> bool {
        *self == Cac::Ccctl3Acond
    }
    #[doc = "Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_2inp(&self) -> bool {
        *self == Cac::Qei2inp
    }
    #[doc = "Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_3inp(&self) -> bool {
        *self == Cac::Qei3inp
    }
}
#[doc = "Field `CAC` writer - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CacW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cac, crate::Safe>;
impl<'a, REG> CacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL_0 ACOND"]
    #[inline(always)]
    pub fn ccctl0_acond(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Ccctl0Acond)
    }
    #[doc = "CCCTL_1 ACOND"]
    #[inline(always)]
    pub fn ccctl1_acond(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Ccctl1Acond)
    }
    #[doc = "CCCTL_2 ACOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl2_acond(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Ccctl2Acond)
    }
    #[doc = "CCCTL_3 ACOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl3_acond(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Ccctl3Acond)
    }
    #[doc = "Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Qei2inp)
    }
    #[doc = "Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(Cac::Qei3inp)
    }
}
#[doc = "Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Czc {
    #[doc = "0: CCCTL_0 ZCOND"]
    Ccctl0Zcond = 0,
    #[doc = "1: CCCTL_1 ZCOND"]
    Ccctl1Zcond = 1,
    #[doc = "2: CCCTL_2 ZCOND This value exists when there are 4 channels."]
    Ccctl2Zcond = 2,
    #[doc = "3: CCCTL_3 ZCOND This value exists when there are 4 channels."]
    Ccctl3Zcond = 3,
    #[doc = "4: Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    Qei2inp = 4,
    #[doc = "5: Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    Qei3inp = 5,
}
impl From<Czc> for u8 {
    #[inline(always)]
    fn from(variant: Czc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Czc {
    type Ux = u8;
}
impl crate::IsEnum for Czc {}
#[doc = "Field `CZC` reader - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CzcR = crate::FieldReader<Czc>;
impl CzcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Czc> {
        match self.bits {
            0 => Some(Czc::Ccctl0Zcond),
            1 => Some(Czc::Ccctl1Zcond),
            2 => Some(Czc::Ccctl2Zcond),
            3 => Some(Czc::Ccctl3Zcond),
            4 => Some(Czc::Qei2inp),
            5 => Some(Czc::Qei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL_0 ZCOND"]
    #[inline(always)]
    pub fn is_ccctl0_zcond(&self) -> bool {
        *self == Czc::Ccctl0Zcond
    }
    #[doc = "CCCTL_1 ZCOND"]
    #[inline(always)]
    pub fn is_ccctl1_zcond(&self) -> bool {
        *self == Czc::Ccctl1Zcond
    }
    #[doc = "CCCTL_2 ZCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl2_zcond(&self) -> bool {
        *self == Czc::Ccctl2Zcond
    }
    #[doc = "CCCTL_3 ZCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn is_ccctl3_zcond(&self) -> bool {
        *self == Czc::Ccctl3Zcond
    }
    #[doc = "Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_2inp(&self) -> bool {
        *self == Czc::Qei2inp
    }
    #[doc = "Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn is_qei_3inp(&self) -> bool {
        *self == Czc::Qei3inp
    }
}
#[doc = "Field `CZC` writer - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CzcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Czc, crate::Safe>;
impl<'a, REG> CzcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL_0 ZCOND"]
    #[inline(always)]
    pub fn ccctl0_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Ccctl0Zcond)
    }
    #[doc = "CCCTL_1 ZCOND"]
    #[inline(always)]
    pub fn ccctl1_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Ccctl1Zcond)
    }
    #[doc = "CCCTL_2 ZCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl2_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Ccctl2Zcond)
    }
    #[doc = "CCCTL_3 ZCOND This value exists when there are 4 channels."]
    #[inline(always)]
    pub fn ccctl3_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Ccctl3Zcond)
    }
    #[doc = "Controlled by 2-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Qei2inp)
    }
    #[doc = "Controlled by 3-input QEI mode This value exists when TIMER support QEI feature."]
    #[inline(always)]
    pub fn qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(Czc::Qei3inp)
    }
}
#[doc = "Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drb {
    #[doc = "0: Resume counting"]
    Resume = 0,
    #[doc = "1: Perform the action as specified by the CVAE field."]
    CvaeAction = 1,
}
impl From<Drb> for bool {
    #[inline(always)]
    fn from(variant: Drb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRB` reader - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
pub type DrbR = crate::BitReader<Drb>;
impl DrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drb {
        match self.bits {
            false => Drb::Resume,
            true => Drb::CvaeAction,
        }
    }
    #[doc = "Resume counting"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Drb::Resume
    }
    #[doc = "Perform the action as specified by the CVAE field."]
    #[inline(always)]
    pub fn is_cvae_action(&self) -> bool {
        *self == Drb::CvaeAction
    }
}
#[doc = "Field `DRB` writer - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
pub type DrbW<'a, REG> = crate::BitWriter<'a, REG, Drb>;
impl<'a, REG> DrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume counting"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Drb::Resume)
    }
    #[doc = "Perform the action as specified by the CVAE field."]
    #[inline(always)]
    pub fn cvae_action(self) -> &'a mut crate::W<REG> {
        self.variant(Drb::CvaeAction)
    }
}
#[doc = "Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fb {
    #[doc = "0: Continues counting"]
    Cont = 0,
    #[doc = "1: Suspends counting"]
    Susp = 1,
}
impl From<Fb> for bool {
    #[inline(always)]
    fn from(variant: Fb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FB` reader - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
pub type FbR = crate::BitReader<Fb>;
impl FbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fb {
        match self.bits {
            false => Fb::Cont,
            true => Fb::Susp,
        }
    }
    #[doc = "Continues counting"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Fb::Cont
    }
    #[doc = "Suspends counting"]
    #[inline(always)]
    pub fn is_susp(&self) -> bool {
        *self == Fb::Susp
    }
}
#[doc = "Field `FB` writer - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG, Fb>;
impl<'a, REG> FbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continues counting"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::Cont)
    }
    #[doc = "Suspends counting"]
    #[inline(always)]
    pub fn susp(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::Susp)
    }
}
#[doc = "Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frb {
    #[doc = "0: Resume counting"]
    Resume = 0,
    #[doc = "1: Perform the action as specified by the CVAE field."]
    CvaeAction = 1,
}
impl From<Frb> for bool {
    #[inline(always)]
    fn from(variant: Frb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRB` reader - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
pub type FrbR = crate::BitReader<Frb>;
impl FrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frb {
        match self.bits {
            false => Frb::Resume,
            true => Frb::CvaeAction,
        }
    }
    #[doc = "Resume counting"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Frb::Resume
    }
    #[doc = "Perform the action as specified by the CVAE field."]
    #[inline(always)]
    pub fn is_cvae_action(&self) -> bool {
        *self == Frb::CvaeAction
    }
}
#[doc = "Field `FRB` writer - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
pub type FrbW<'a, REG> = crate::BitWriter<'a, REG, Frb>;
impl<'a, REG> FrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume counting"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Frb::Resume)
    }
    #[doc = "Perform the action as specified by the CVAE field."]
    #[inline(always)]
    pub fn cvae_action(self) -> &'a mut crate::W<REG> {
        self.variant(Frb::CvaeAction)
    }
}
#[doc = "Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slzercnez {
    #[doc = "0: Disabled. Z and L events are always generated from the counter when their conditions are generated."]
    Disabled = 0,
    #[doc = "1: Enabled. Z and L events are generated from the counter when their conditions are generated and the RC register value is 0."]
    Enabled = 1,
}
impl From<Slzercnez> for bool {
    #[inline(always)]
    fn from(variant: Slzercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLZERCNEZ` reader - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
pub type SlzercnezR = crate::BitReader<Slzercnez>;
impl SlzercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slzercnez {
        match self.bits {
            false => Slzercnez::Disabled,
            true => Slzercnez::Enabled,
        }
    }
    #[doc = "Disabled. Z and L events are always generated from the counter when their conditions are generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slzercnez::Disabled
    }
    #[doc = "Enabled. Z and L events are generated from the counter when their conditions are generated and the RC register value is 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slzercnez::Enabled
    }
}
#[doc = "Field `SLZERCNEZ` writer - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
pub type SlzercnezW<'a, REG> = crate::BitWriter<'a, REG, Slzercnez>;
impl<'a, REG> SlzercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Z and L events are always generated from the counter when their conditions are generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slzercnez::Disabled)
    }
    #[doc = "Enabled. Z and L events are generated from the counter when their conditions are generated and the RC register value is 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slzercnez::Enabled)
    }
}
#[doc = "Phase Load Enable. This bit allows the timer to have phase load feature.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plen {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Plen> for bool {
    #[inline(always)]
    fn from(variant: Plen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLEN` reader - Phase Load Enable. This bit allows the timer to have phase load feature."]
pub type PlenR = crate::BitReader<Plen>;
impl PlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plen {
        match self.bits {
            false => Plen::Disabled,
            true => Plen::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Plen::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Plen::Enabled
    }
}
#[doc = "Field `PLEN` writer - Phase Load Enable. This bit allows the timer to have phase load feature."]
pub type PlenW<'a, REG> = crate::BitWriter<'a, REG, Plen>;
impl<'a, REG> PlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::Enabled)
    }
}
#[doc = "Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cvae {
    #[doc = "0: The counter is set to the LOAD register value"]
    Ldval = 0,
    #[doc = "1: The counter value is unchanged from its current value which could have been initialized by software"]
    Nochange = 1,
    #[doc = "2: The counter is set to zero"]
    Zeroval = 2,
}
impl From<Cvae> for u8 {
    #[inline(always)]
    fn from(variant: Cvae) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cvae {
    type Ux = u8;
}
impl crate::IsEnum for Cvae {}
#[doc = "Field `CVAE` reader - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
pub type CvaeR = crate::FieldReader<Cvae>;
impl CvaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cvae> {
        match self.bits {
            0 => Some(Cvae::Ldval),
            1 => Some(Cvae::Nochange),
            2 => Some(Cvae::Zeroval),
            _ => None,
        }
    }
    #[doc = "The counter is set to the LOAD register value"]
    #[inline(always)]
    pub fn is_ldval(&self) -> bool {
        *self == Cvae::Ldval
    }
    #[doc = "The counter value is unchanged from its current value which could have been initialized by software"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == Cvae::Nochange
    }
    #[doc = "The counter is set to zero"]
    #[inline(always)]
    pub fn is_zeroval(&self) -> bool {
        *self == Cvae::Zeroval
    }
}
#[doc = "Field `CVAE` writer - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
pub type CvaeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cvae, crate::Safe>;
impl<'a, REG> CvaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The counter is set to the LOAD register value"]
    #[inline(always)]
    pub fn ldval(self) -> &'a mut crate::W<REG> {
        self.variant(Cvae::Ldval)
    }
    #[doc = "The counter value is unchanged from its current value which could have been initialized by software"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut crate::W<REG> {
        self.variant(Cvae::Nochange)
    }
    #[doc = "The counter is set to zero"]
    #[inline(always)]
    pub fn zeroval(self) -> &'a mut crate::W<REG> {
        self.variant(Cvae::Zeroval)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
    #[inline(always)]
    pub fn repeat(&self) -> RepeatR {
        RepeatR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Count Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn clc(&self) -> ClcR {
        ClcR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn cac(&self) -> CacR {
        CacR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn czc(&self) -> CzcR {
        CzcR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 17 - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
    #[inline(always)]
    pub fn drb(&self) -> DrbR {
        DrbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
    #[inline(always)]
    pub fn frb(&self) -> FrbR {
        FrbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn slzercnez(&self) -> SlzercnezR {
        SlzercnezR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Phase Load Enable. This bit allows the timer to have phase load feature."]
    #[inline(always)]
    pub fn plen(&self) -> PlenR {
        PlenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
    #[inline(always)]
    pub fn cvae(&self) -> CvaeR {
        CvaeR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRCTL")
            .field("en", &self.en())
            .field("repeat", &self.repeat())
            .field("cm", &self.cm())
            .field("cvae", &self.cvae())
            .field("drb", &self.drb())
            .field("fb", &self.fb())
            .field("frb", &self.frb())
            .field("plen", &self.plen())
            .field("slzercnez", &self.slzercnez())
            .field("clc", &self.clc())
            .field("cac", &self.cac())
            .field("czc", &self.czc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
    #[inline(always)]
    pub fn repeat(&mut self) -> RepeatW<'_, CtrctlSpec> {
        RepeatW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Count Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, CtrctlSpec> {
        CmW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn clc(&mut self) -> ClcW<'_, CtrctlSpec> {
        ClcW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn cac(&mut self) -> CacW<'_, CtrctlSpec> {
        CacW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn czc(&mut self) -> CzcW<'_, CtrctlSpec> {
        CzcW::new(self, 13)
    }
    #[doc = "Bit 17 - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
    #[inline(always)]
    pub fn drb(&mut self) -> DrbW<'_, CtrctlSpec> {
        DrbW::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<'_, CtrctlSpec> {
        FbW::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
    #[inline(always)]
    pub fn frb(&mut self) -> FrbW<'_, CtrctlSpec> {
        FrbW::new(self, 19)
    }
    #[doc = "Bit 23 - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn slzercnez(&mut self) -> SlzercnezW<'_, CtrctlSpec> {
        SlzercnezW::new(self, 23)
    }
    #[doc = "Bit 24 - Phase Load Enable. This bit allows the timer to have phase load feature."]
    #[inline(always)]
    pub fn plen(&mut self) -> PlenW<'_, CtrctlSpec> {
        PlenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
    #[inline(always)]
    pub fn cvae(&mut self) -> CvaeW<'_, CtrctlSpec> {
        CvaeW::new(self, 28)
    }
}
#[doc = "Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrctlSpec;
impl crate::RegisterSpec for CtrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrctl::R`](R) reader structure"]
impl crate::Readable for CtrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrctl::W`](W) writer structure"]
impl crate::Writable for CtrctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRCTL to value 0xff80"]
impl crate::Resettable for CtrctlSpec {
    const RESET_VALUE: u32 = 0xff80;
}
