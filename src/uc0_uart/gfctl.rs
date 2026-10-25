#[doc = "Register `GFCTL` reader"]
pub type R = crate::R<GfctlSpec>;
#[doc = "Register `GFCTL` writer"]
pub type W = crate::W<GfctlSpec>;
#[doc = "Glitch Suppression Pulse Width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dgfsel {
    #[doc = "0: Bypass GF"]
    Disabled = 0,
}
impl From<Dgfsel> for u8 {
    #[inline(always)]
    fn from(variant: Dgfsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dgfsel {
    type Ux = u8;
}
impl crate::IsEnum for Dgfsel {}
#[doc = "Field `DGFSEL` reader - Glitch Suppression Pulse Width"]
pub type DgfselR = crate::FieldReader<Dgfsel>;
impl DgfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dgfsel> {
        match self.bits {
            0 => Some(Dgfsel::Disabled),
            _ => None,
        }
    }
    #[doc = "Bypass GF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dgfsel::Disabled
    }
}
#[doc = "Field `DGFSEL` writer - Glitch Suppression Pulse Width"]
pub type DgfselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Dgfsel, crate::Safe>;
impl<'a, REG> DgfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass GF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn dgfsel(&self) -> DgfselR {
        DgfselR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GFCTL")
            .field("dgfsel", &self.dgfsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn dgfsel(&mut self) -> DgfselW<'_, GfctlSpec> {
        DgfselW::new(self, 0)
    }
}
#[doc = "Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfctlSpec;
impl crate::RegisterSpec for GfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfctl::R`](R) reader structure"]
impl crate::Readable for GfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gfctl::W`](W) writer structure"]
impl crate::Writable for GfctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GFCTL to value 0"]
impl crate::Resettable for GfctlSpec {}
