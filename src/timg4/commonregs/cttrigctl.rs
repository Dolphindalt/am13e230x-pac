#[doc = "Register `CTTRIGCTL` reader"]
pub type R = crate::R<CttrigctlSpec>;
#[doc = "Register `CTTRIGCTL` writer"]
pub type W = crate::W<CttrigctlSpec>;
#[doc = "Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cten {
    #[doc = "0: Cross trigger generation disabled."]
    Disabled = 0,
    #[doc = "1: Cross trigger generation enabled"]
    Enable = 1,
}
impl From<Cten> for bool {
    #[inline(always)]
    fn from(variant: Cten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEN` reader - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
pub type CtenR = crate::BitReader<Cten>;
impl CtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cten {
        match self.bits {
            false => Cten::Disabled,
            true => Cten::Enable,
        }
    }
    #[doc = "Cross trigger generation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cten::Disabled
    }
    #[doc = "Cross trigger generation enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cten::Enable
    }
}
#[doc = "Field `CTEN` writer - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
pub type CtenW<'a, REG> = crate::BitWriter<'a, REG, Cten>;
impl<'a, REG> CtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cross trigger generation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cten::Disabled)
    }
    #[doc = "Cross trigger generation enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cten::Enable)
    }
}
#[doc = "Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evtcten {
    #[doc = "0: Cross trigger generation disabled."]
    Disabled = 0,
    #[doc = "1: Cross trigger generation enabled"]
    Enable = 1,
}
impl From<Evtcten> for bool {
    #[inline(always)]
    fn from(variant: Evtcten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTCTEN` reader - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers."]
pub type EvtctenR = crate::BitReader<Evtcten>;
impl EvtctenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evtcten {
        match self.bits {
            false => Evtcten::Disabled,
            true => Evtcten::Enable,
        }
    }
    #[doc = "Cross trigger generation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Evtcten::Disabled
    }
    #[doc = "Cross trigger generation enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Evtcten::Enable
    }
}
#[doc = "Field `EVTCTEN` writer - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers."]
pub type EvtctenW<'a, REG> = crate::BitWriter<'a, REG, Evtcten>;
impl<'a, REG> EvtctenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cross trigger generation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcten::Disabled)
    }
    #[doc = "Cross trigger generation enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcten::Enable)
    }
}
#[doc = "Used to Select the subscriber port that should be used for input cross trigger.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evtcttrigsel {
    #[doc = "0: Use FSUB0 as cross trigger source."]
    Fsub0 = 0,
    #[doc = "1: Use FSUB1 as cross trigger source."]
    Fsub1 = 1,
    #[doc = "2: Use Zero event as cross trigger source."]
    Z = 2,
    #[doc = "3: Use Load event as cross trigger source."]
    L = 3,
    #[doc = "4: Use CCD0 event as cross trigger source."]
    Ccd0 = 4,
    #[doc = "5: Use CCD1 event as cross trigger source."]
    Ccd1 = 5,
    #[doc = "6: Use CCD2 event as cross trigger source."]
    Ccd2 = 6,
    #[doc = "7: Use CCD3 event as cross trigger source."]
    Ccd3 = 7,
    #[doc = "8: Use CCU0 event as cross trigger source."]
    Ccu0 = 8,
    #[doc = "9: Use CCU1 event as cross trigger source."]
    Ccu1 = 9,
    #[doc = "10: Use CCU2 event as cross trigger source."]
    Ccu2 = 10,
    #[doc = "11: Use CCU3 event as cross trigger source."]
    Ccu3 = 11,
}
impl From<Evtcttrigsel> for u8 {
    #[inline(always)]
    fn from(variant: Evtcttrigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evtcttrigsel {
    type Ux = u8;
}
impl crate::IsEnum for Evtcttrigsel {}
#[doc = "Field `EVTCTTRIGSEL` reader - Used to Select the subscriber port that should be used for input cross trigger."]
pub type EvtcttrigselR = crate::FieldReader<Evtcttrigsel>;
impl EvtcttrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evtcttrigsel> {
        match self.bits {
            0 => Some(Evtcttrigsel::Fsub0),
            1 => Some(Evtcttrigsel::Fsub1),
            2 => Some(Evtcttrigsel::Z),
            3 => Some(Evtcttrigsel::L),
            4 => Some(Evtcttrigsel::Ccd0),
            5 => Some(Evtcttrigsel::Ccd1),
            6 => Some(Evtcttrigsel::Ccd2),
            7 => Some(Evtcttrigsel::Ccd3),
            8 => Some(Evtcttrigsel::Ccu0),
            9 => Some(Evtcttrigsel::Ccu1),
            10 => Some(Evtcttrigsel::Ccu2),
            11 => Some(Evtcttrigsel::Ccu3),
            _ => None,
        }
    }
    #[doc = "Use FSUB0 as cross trigger source."]
    #[inline(always)]
    pub fn is_fsub0(&self) -> bool {
        *self == Evtcttrigsel::Fsub0
    }
    #[doc = "Use FSUB1 as cross trigger source."]
    #[inline(always)]
    pub fn is_fsub1(&self) -> bool {
        *self == Evtcttrigsel::Fsub1
    }
    #[doc = "Use Zero event as cross trigger source."]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == Evtcttrigsel::Z
    }
    #[doc = "Use Load event as cross trigger source."]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == Evtcttrigsel::L
    }
    #[doc = "Use CCD0 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccd0(&self) -> bool {
        *self == Evtcttrigsel::Ccd0
    }
    #[doc = "Use CCD1 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccd1(&self) -> bool {
        *self == Evtcttrigsel::Ccd1
    }
    #[doc = "Use CCD2 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccd2(&self) -> bool {
        *self == Evtcttrigsel::Ccd2
    }
    #[doc = "Use CCD3 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccd3(&self) -> bool {
        *self == Evtcttrigsel::Ccd3
    }
    #[doc = "Use CCU0 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccu0(&self) -> bool {
        *self == Evtcttrigsel::Ccu0
    }
    #[doc = "Use CCU1 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccu1(&self) -> bool {
        *self == Evtcttrigsel::Ccu1
    }
    #[doc = "Use CCU2 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccu2(&self) -> bool {
        *self == Evtcttrigsel::Ccu2
    }
    #[doc = "Use CCU3 event as cross trigger source."]
    #[inline(always)]
    pub fn is_ccu3(&self) -> bool {
        *self == Evtcttrigsel::Ccu3
    }
}
#[doc = "Field `EVTCTTRIGSEL` writer - Used to Select the subscriber port that should be used for input cross trigger."]
pub type EvtcttrigselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Evtcttrigsel, crate::Safe>;
impl<'a, REG> EvtcttrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use FSUB0 as cross trigger source."]
    #[inline(always)]
    pub fn fsub0(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Fsub0)
    }
    #[doc = "Use FSUB1 as cross trigger source."]
    #[inline(always)]
    pub fn fsub1(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Fsub1)
    }
    #[doc = "Use Zero event as cross trigger source."]
    #[inline(always)]
    pub fn z(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Z)
    }
    #[doc = "Use Load event as cross trigger source."]
    #[inline(always)]
    pub fn l(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::L)
    }
    #[doc = "Use CCD0 event as cross trigger source."]
    #[inline(always)]
    pub fn ccd0(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccd0)
    }
    #[doc = "Use CCD1 event as cross trigger source."]
    #[inline(always)]
    pub fn ccd1(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccd1)
    }
    #[doc = "Use CCD2 event as cross trigger source."]
    #[inline(always)]
    pub fn ccd2(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccd2)
    }
    #[doc = "Use CCD3 event as cross trigger source."]
    #[inline(always)]
    pub fn ccd3(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccd3)
    }
    #[doc = "Use CCU0 event as cross trigger source."]
    #[inline(always)]
    pub fn ccu0(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccu0)
    }
    #[doc = "Use CCU1 event as cross trigger source."]
    #[inline(always)]
    pub fn ccu1(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccu1)
    }
    #[doc = "Use CCU2 event as cross trigger source."]
    #[inline(always)]
    pub fn ccu2(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccu2)
    }
    #[doc = "Use CCU3 event as cross trigger source."]
    #[inline(always)]
    pub fn ccu3(self) -> &'a mut crate::W<REG> {
        self.variant(Evtcttrigsel::Ccu3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
    #[inline(always)]
    pub fn cten(&self) -> CtenR {
        CtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers."]
    #[inline(always)]
    pub fn evtcten(&self) -> EvtctenR {
        EvtctenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Used to Select the subscriber port that should be used for input cross trigger."]
    #[inline(always)]
    pub fn evtcttrigsel(&self) -> EvtcttrigselR {
        EvtcttrigselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTTRIGCTL")
            .field("cten", &self.cten())
            .field("evtcten", &self.evtcten())
            .field("evtcttrigsel", &self.evtcttrigsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
    #[inline(always)]
    pub fn cten(&mut self) -> CtenW<'_, CttrigctlSpec> {
        CtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers."]
    #[inline(always)]
    pub fn evtcten(&mut self) -> EvtctenW<'_, CttrigctlSpec> {
        EvtctenW::new(self, 1)
    }
    #[doc = "Bits 16:19 - Used to Select the subscriber port that should be used for input cross trigger."]
    #[inline(always)]
    pub fn evtcttrigsel(&mut self) -> EvtcttrigselW<'_, CttrigctlSpec> {
        EvtcttrigselW::new(self, 16)
    }
}
#[doc = "Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cttrigctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrigctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CttrigctlSpec;
impl crate::RegisterSpec for CttrigctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cttrigctl::R`](R) reader structure"]
impl crate::Readable for CttrigctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cttrigctl::W`](W) writer structure"]
impl crate::Writable for CttrigctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTTRIGCTL to value 0"]
impl crate::Resettable for CttrigctlSpec {}
