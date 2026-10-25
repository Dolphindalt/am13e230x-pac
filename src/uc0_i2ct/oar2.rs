#[doc = "Register `OAR2` reader"]
pub type R = crate::R<Oar2Spec>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<Oar2Spec>;
#[doc = "Field `OAR2` reader - Own Address 2This field specifies the alternate OAR2 address."]
pub type Oar2R = crate::FieldReader;
#[doc = "Field `OAR2` writer - Own Address 2This field specifies the alternate OAR2 address."]
pub type Oar2W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Own Address 2 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oar2en {
    #[doc = "0: The alternate address is disabled."]
    Disable = 0,
    #[doc = "1: Enables the use of the alternate address in the OAR2 field."]
    Enable = 1,
}
impl From<Oar2en> for bool {
    #[inline(always)]
    fn from(variant: Oar2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAR2EN` reader - Own Address 2 Enable"]
pub type Oar2enR = crate::BitReader<Oar2en>;
impl Oar2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oar2en {
        match self.bits {
            false => Oar2en::Disable,
            true => Oar2en::Enable,
        }
    }
    #[doc = "The alternate address is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Oar2en::Disable
    }
    #[doc = "Enables the use of the alternate address in the OAR2 field."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Oar2en::Enable
    }
}
#[doc = "Field `OAR2EN` writer - Own Address 2 Enable"]
pub type Oar2enW<'a, REG> = crate::BitWriter<'a, REG, Oar2en>;
impl<'a, REG> Oar2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The alternate address is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Oar2en::Disable)
    }
    #[doc = "Enables the use of the alternate address in the OAR2 field."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Oar2en::Enable)
    }
}
#[doc = "Field `OAR2_MASK` reader - Own Address 2 Mask: This field specifies bits A6 through A0 of the Target address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
pub type Oar2MaskR = crate::FieldReader;
#[doc = "Field `OAR2_MASK` writer - Own Address 2 Mask: This field specifies bits A6 through A0 of the Target address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
pub type Oar2MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:6 - Own Address 2This field specifies the alternate OAR2 address."]
    #[inline(always)]
    pub fn oar2(&self) -> Oar2R {
        Oar2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Own Address 2 Enable"]
    #[inline(always)]
    pub fn oar2en(&self) -> Oar2enR {
        Oar2enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Own Address 2 Mask: This field specifies bits A6 through A0 of the Target address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
    #[inline(always)]
    pub fn oar2_mask(&self) -> Oar2MaskR {
        Oar2MaskR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR2")
            .field("oar2", &self.oar2())
            .field("oar2en", &self.oar2en())
            .field("oar2_mask", &self.oar2_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Own Address 2This field specifies the alternate OAR2 address."]
    #[inline(always)]
    pub fn oar2(&mut self) -> Oar2W<'_, Oar2Spec> {
        Oar2W::new(self, 0)
    }
    #[doc = "Bit 7 - Own Address 2 Enable"]
    #[inline(always)]
    pub fn oar2en(&mut self) -> Oar2enW<'_, Oar2Spec> {
        Oar2enW::new(self, 7)
    }
    #[doc = "Bits 16:22 - Own Address 2 Mask: This field specifies bits A6 through A0 of the Target address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
    #[inline(always)]
    pub fn oar2_mask(&mut self) -> Oar2MaskW<'_, Oar2Spec> {
        Oar2MaskW::new(self, 16)
    }
}
#[doc = "Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar2Spec;
impl crate::RegisterSpec for Oar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for Oar2Spec {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for Oar2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for Oar2Spec {}
