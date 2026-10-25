#[doc = "Register `ACKCTL` reader"]
pub type R = crate::R<AckctlSpec>;
#[doc = "Register `ACKCTL` writer"]
pub type W = crate::W<AckctlSpec>;
#[doc = "I2C Target ACK Override Enable Read SR.ACKOEN to check current status of this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackoen {
    #[doc = "0: A response in not provided."]
    Disable = 0,
    #[doc = "1: An ACK or NACK is sent according to the value written to the ACKOVAL bit."]
    Enable = 1,
}
impl From<Ackoen> for bool {
    #[inline(always)]
    fn from(variant: Ackoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN` reader - I2C Target ACK Override Enable Read SR.ACKOEN to check current status of this bit"]
pub type AckoenR = crate::BitReader<Ackoen>;
impl AckoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackoen {
        match self.bits {
            false => Ackoen::Disable,
            true => Ackoen::Enable,
        }
    }
    #[doc = "A response in not provided."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ackoen::Disable
    }
    #[doc = "An ACK or NACK is sent according to the value written to the ACKOVAL bit."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ackoen::Enable
    }
}
#[doc = "Field `ACKOEN` writer - I2C Target ACK Override Enable Read SR.ACKOEN to check current status of this bit"]
pub type AckoenW<'a, REG> = crate::BitWriter<'a, REG, Ackoen>;
impl<'a, REG> AckoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A response in not provided."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoen::Disable)
    }
    #[doc = "An ACK or NACK is sent according to the value written to the ACKOVAL bit."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoen::Enable)
    }
}
#[doc = "ACK Override Value Note: Override control is not applicable to Address frame. Bytes following address frame can be acknowledged using ACKOVAL.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackoval {
    #[doc = "0: An ACK is sent indicating valid data or command."]
    Disable = 0,
    #[doc = "1: A NACK is sent indicating invalid data or command."]
    Enable = 1,
}
impl From<Ackoval> for bool {
    #[inline(always)]
    fn from(variant: Ackoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOVAL` reader - ACK Override Value Note: Override control is not applicable to Address frame. Bytes following address frame can be acknowledged using ACKOVAL."]
pub type AckovalR = crate::BitReader<Ackoval>;
impl AckovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackoval {
        match self.bits {
            false => Ackoval::Disable,
            true => Ackoval::Enable,
        }
    }
    #[doc = "An ACK is sent indicating valid data or command."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ackoval::Disable
    }
    #[doc = "A NACK is sent indicating invalid data or command."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ackoval::Enable
    }
}
#[doc = "Field `ACKOVAL` writer - ACK Override Value Note: Override control is not applicable to Address frame. Bytes following address frame can be acknowledged using ACKOVAL."]
pub type AckovalW<'a, REG> = crate::BitWriter<'a, REG, Ackoval>;
impl<'a, REG> AckovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An ACK is sent indicating valid data or command."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoval::Disable)
    }
    #[doc = "A NACK is sent indicating invalid data or command."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoval::Enable)
    }
}
#[doc = "When set this bit will automatically enable target override acknowlede following a Start Condition. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckoenOnStart {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When set this bit will automatically turn on the Target ACKOEN field following a Start Condition."]
    Enable = 1,
}
impl From<AckoenOnStart> for bool {
    #[inline(always)]
    fn from(variant: AckoenOnStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN_ON_START` reader - When set this bit will automatically enable target override acknowlede following a Start Condition. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnStartR = crate::BitReader<AckoenOnStart>;
impl AckoenOnStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckoenOnStart {
        match self.bits {
            false => AckoenOnStart::Disable,
            true => AckoenOnStart::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AckoenOnStart::Disable
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following a Start Condition."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AckoenOnStart::Enable
    }
}
#[doc = "Field `ACKOEN_ON_START` writer - When set this bit will automatically enable target override acknowlede following a Start Condition. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnStartW<'a, REG> = crate::BitWriter<'a, REG, AckoenOnStart>;
impl<'a, REG> AckoenOnStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnStart::Disable)
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following a Start Condition."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnStart::Enable)
    }
}
#[doc = "When set this bit will automatically turn on the Target acknowledge override following a ACK/NACK of the byte received just prior to the PEC byte. However, setting ACKCTL.ACKOEN bit will not automatically be ACKed/NACKed by the State Machine and firmware must perform this function by writing Target.ACKCTL register. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckoenOnPecnext {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing Target_SACKCTL."]
    Enable = 1,
}
impl From<AckoenOnPecnext> for bool {
    #[inline(always)]
    fn from(variant: AckoenOnPecnext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN_ON_PECNEXT` reader - When set this bit will automatically turn on the Target acknowledge override following a ACK/NACK of the byte received just prior to the PEC byte. However, setting ACKCTL.ACKOEN bit will not automatically be ACKed/NACKed by the State Machine and firmware must perform this function by writing Target.ACKCTL register. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnPecnextR = crate::BitReader<AckoenOnPecnext>;
impl AckoenOnPecnextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckoenOnPecnext {
        match self.bits {
            false => AckoenOnPecnext::Disable,
            true => AckoenOnPecnext::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AckoenOnPecnext::Disable
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing Target_SACKCTL."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AckoenOnPecnext::Enable
    }
}
#[doc = "Field `ACKOEN_ON_PECNEXT` writer - When set this bit will automatically turn on the Target acknowledge override following a ACK/NACK of the byte received just prior to the PEC byte. However, setting ACKCTL.ACKOEN bit will not automatically be ACKed/NACKed by the State Machine and firmware must perform this function by writing Target.ACKCTL register. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnPecnextW<'a, REG> = crate::BitWriter<'a, REG, AckoenOnPecnext>;
impl<'a, REG> AckoenOnPecnextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnPecnext::Disable)
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing Target_SACKCTL."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnPecnext::Enable)
    }
}
#[doc = "When set, this bit will automatically turn on target acknowledge enable field following the ACK/NACK of the received PEC byte. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckoenOnPecdone {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the received PEC byte."]
    Enable = 1,
}
impl From<AckoenOnPecdone> for bool {
    #[inline(always)]
    fn from(variant: AckoenOnPecdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN_ON_PECDONE` reader - When set, this bit will automatically turn on target acknowledge enable field following the ACK/NACK of the received PEC byte. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnPecdoneR = crate::BitReader<AckoenOnPecdone>;
impl AckoenOnPecdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckoenOnPecdone {
        match self.bits {
            false => AckoenOnPecdone::Disable,
            true => AckoenOnPecdone::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AckoenOnPecdone::Disable
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the received PEC byte."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AckoenOnPecdone::Enable
    }
}
#[doc = "Field `ACKOEN_ON_PECDONE` writer - When set, this bit will automatically turn on target acknowledge enable field following the ACK/NACK of the received PEC byte. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
pub type AckoenOnPecdoneW<'a, REG> = crate::BitWriter<'a, REG, AckoenOnPecdone>;
impl<'a, REG> AckoenOnPecdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnPecdone::Disable)
    }
    #[doc = "When set this bit will automatically turn on the Target ACKOEN field following the ACK/NACK of the received PEC byte."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckoenOnPecdone::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Target ACK Override Enable Read SR.ACKOEN to check current status of this bit"]
    #[inline(always)]
    pub fn ackoen(&self) -> AckoenR {
        AckoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACK Override Value Note: Override control is not applicable to Address frame. Bytes following address frame can be acknowledged using ACKOVAL."]
    #[inline(always)]
    pub fn ackoval(&self) -> AckovalR {
        AckovalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set this bit will automatically enable target override acknowlede following a Start Condition. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_start(&self) -> AckoenOnStartR {
        AckoenOnStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set this bit will automatically turn on the Target acknowledge override following a ACK/NACK of the byte received just prior to the PEC byte. However, setting ACKCTL.ACKOEN bit will not automatically be ACKed/NACKed by the State Machine and firmware must perform this function by writing Target.ACKCTL register. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_pecnext(&self) -> AckoenOnPecnextR {
        AckoenOnPecnextR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, this bit will automatically turn on target acknowledge enable field following the ACK/NACK of the received PEC byte. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_pecdone(&self) -> AckoenOnPecdoneR {
        AckoenOnPecdoneR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACKCTL")
            .field("ackoen", &self.ackoen())
            .field("ackoval", &self.ackoval())
            .field("ackoen_on_start", &self.ackoen_on_start())
            .field("ackoen_on_pecnext", &self.ackoen_on_pecnext())
            .field("ackoen_on_pecdone", &self.ackoen_on_pecdone())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2C Target ACK Override Enable Read SR.ACKOEN to check current status of this bit"]
    #[inline(always)]
    pub fn ackoen(&mut self) -> AckoenW<'_, AckctlSpec> {
        AckoenW::new(self, 0)
    }
    #[doc = "Bit 1 - ACK Override Value Note: Override control is not applicable to Address frame. Bytes following address frame can be acknowledged using ACKOVAL."]
    #[inline(always)]
    pub fn ackoval(&mut self) -> AckovalW<'_, AckctlSpec> {
        AckovalW::new(self, 1)
    }
    #[doc = "Bit 2 - When set this bit will automatically enable target override acknowlede following a Start Condition. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_start(&mut self) -> AckoenOnStartW<'_, AckctlSpec> {
        AckoenOnStartW::new(self, 2)
    }
    #[doc = "Bit 3 - When set this bit will automatically turn on the Target acknowledge override following a ACK/NACK of the byte received just prior to the PEC byte. However, setting ACKCTL.ACKOEN bit will not automatically be ACKed/NACKed by the State Machine and firmware must perform this function by writing Target.ACKCTL register. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_pecnext(&mut self) -> AckoenOnPecnextW<'_, AckctlSpec> {
        AckoenOnPecnextW::new(self, 3)
    }
    #[doc = "Bit 4 - When set, this bit will automatically turn on target acknowledge enable field following the ACK/NACK of the received PEC byte. Status bit ACKOEN will reflect a value of '1' when automatic override enable is used."]
    #[inline(always)]
    pub fn ackoen_on_pecdone(&mut self) -> AckoenOnPecdoneW<'_, AckctlSpec> {
        AckoenOnPecdoneW::new(self, 4)
    }
}
#[doc = "I2C Target ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ackctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ackctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AckctlSpec;
impl crate::RegisterSpec for AckctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ackctl::R`](R) reader structure"]
impl crate::Readable for AckctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ackctl::W`](W) writer structure"]
impl crate::Writable for AckctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ACKCTL to value 0"]
impl crate::Resettable for AckctlSpec {}
