#[doc = "Register `SUB0CFG` reader"]
pub type R = crate::R<Sub0cfgSpec>;
#[doc = "Register `SUB0CFG` writer"]
pub type W = crate::W<Sub0cfgSpec>;
#[doc = "Field `ENABLE` reader - This bit is used to enable subscriber 0 event."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - This bit is used to enable subscriber 0 event."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "These bits configure the output policy for subscriber 0 event.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outpolicy {
    #[doc = "0: Selected DIO pins are set"]
    SetPins = 0,
    #[doc = "1: Selected DIO pins are cleared"]
    ClearPins = 1,
    #[doc = "2: Selected DIO pins are toggled"]
    TogglePins = 2,
}
impl From<Outpolicy> for u8 {
    #[inline(always)]
    fn from(variant: Outpolicy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outpolicy {
    type Ux = u8;
}
impl crate::IsEnum for Outpolicy {}
#[doc = "Field `OUTPOLICY` reader - These bits configure the output policy for subscriber 0 event."]
pub type OutpolicyR = crate::FieldReader<Outpolicy>;
impl OutpolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outpolicy> {
        match self.bits {
            0 => Some(Outpolicy::SetPins),
            1 => Some(Outpolicy::ClearPins),
            2 => Some(Outpolicy::TogglePins),
            _ => None,
        }
    }
    #[doc = "Selected DIO pins are set"]
    #[inline(always)]
    pub fn is_set_pins(&self) -> bool {
        *self == Outpolicy::SetPins
    }
    #[doc = "Selected DIO pins are cleared"]
    #[inline(always)]
    pub fn is_clear_pins(&self) -> bool {
        *self == Outpolicy::ClearPins
    }
    #[doc = "Selected DIO pins are toggled"]
    #[inline(always)]
    pub fn is_toggle_pins(&self) -> bool {
        *self == Outpolicy::TogglePins
    }
}
#[doc = "Field `OUTPOLICY` writer - These bits configure the output policy for subscriber 0 event."]
pub type OutpolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Outpolicy, crate::Safe>;
impl<'a, REG> OutpolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selected DIO pins are set"]
    #[inline(always)]
    pub fn set_pins(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::SetPins)
    }
    #[doc = "Selected DIO pins are cleared"]
    #[inline(always)]
    pub fn clear_pins(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::ClearPins)
    }
    #[doc = "Selected DIO pins are toggled"]
    #[inline(always)]
    pub fn toggle_pins(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::TogglePins)
    }
}
#[doc = "Field `INDEX` reader - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable subscriber 0 event."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 0 event."]
    #[inline(always)]
    pub fn outpolicy(&self) -> OutpolicyR {
        OutpolicyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUB0CFG")
            .field("enable", &self.enable())
            .field("outpolicy", &self.outpolicy())
            .field("index", &self.index())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable subscriber 0 event."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Sub0cfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 0 event."]
    #[inline(always)]
    pub fn outpolicy(&mut self) -> OutpolicyW<'_, Sub0cfgSpec> {
        OutpolicyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, Sub0cfgSpec> {
        IndexW::new(self, 16)
    }
}
#[doc = "Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sub0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub0cfgSpec;
impl crate::RegisterSpec for Sub0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub0cfg::R`](R) reader structure"]
impl crate::Readable for Sub0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sub0cfg::W`](W) writer structure"]
impl crate::Writable for Sub0cfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SUB0CFG to value 0"]
impl crate::Resettable for Sub0cfgSpec {}
