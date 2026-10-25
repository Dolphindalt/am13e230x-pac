#[doc = "Register `GCTL` reader"]
pub type R = crate::R<GctlSpec>;
#[doc = "Register `GCTL` writer"]
pub type W = crate::W<GctlSpec>;
#[doc = "Enables shadow to active load of bufferred registers and register fields.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shdwlden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Shdwlden> for bool {
    #[inline(always)]
    fn from(variant: Shdwlden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHDWLDEN` reader - Enables shadow to active load of bufferred registers and register fields."]
pub type ShdwldenR = crate::BitReader<Shdwlden>;
impl ShdwldenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shdwlden {
        match self.bits {
            false => Shdwlden::Disable,
            true => Shdwlden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Shdwlden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Shdwlden::Enable
    }
}
#[doc = "Field `SHDWLDEN` writer - Enables shadow to active load of bufferred registers and register fields."]
pub type ShdwldenW<'a, REG> = crate::BitWriter<'a, REG, Shdwlden>;
impl<'a, REG> ShdwldenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Shdwlden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Shdwlden::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables shadow to active load of bufferred registers and register fields."]
    #[inline(always)]
    pub fn shdwlden(&self) -> ShdwldenR {
        ShdwldenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCTL")
            .field("shdwlden", &self.shdwlden())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadow to active load of bufferred registers and register fields."]
    #[inline(always)]
    pub fn shdwlden(&mut self) -> ShdwldenW<'_, GctlSpec> {
        ShdwldenW::new(self, 0)
    }
}
#[doc = "Global control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctlSpec;
impl crate::RegisterSpec for GctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctl::R`](R) reader structure"]
impl crate::Readable for GctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctl::W`](W) writer structure"]
impl crate::Writable for GctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GCTL to value 0"]
impl crate::Resettable for GctlSpec {}
