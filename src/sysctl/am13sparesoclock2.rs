#[doc = "Register `AM13SPARESOCLOCK2` reader"]
pub type R = crate::R<Am13sparesoclock2Spec>;
#[doc = "Register `AM13SPARESOCLOCK2` writer"]
pub type W = crate::W<Am13sparesoclock2Spec>;
#[doc = "SRAM3 static mux select between CBUS and SBUS\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram3StaticMuxSel {
    #[doc = "0: SBUS path is selected"]
    Sbus = 0,
    #[doc = "1: CBUS path is selected"]
    Cbus = 1,
}
impl From<Sram3StaticMuxSel> for bool {
    #[inline(always)]
    fn from(variant: Sram3StaticMuxSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM3_STATIC_MUX_SEL` reader - SRAM3 static mux select between CBUS and SBUS"]
pub type Sram3StaticMuxSelR = crate::BitReader<Sram3StaticMuxSel>;
impl Sram3StaticMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram3StaticMuxSel {
        match self.bits {
            false => Sram3StaticMuxSel::Sbus,
            true => Sram3StaticMuxSel::Cbus,
        }
    }
    #[doc = "SBUS path is selected"]
    #[inline(always)]
    pub fn is_sbus(&self) -> bool {
        *self == Sram3StaticMuxSel::Sbus
    }
    #[doc = "CBUS path is selected"]
    #[inline(always)]
    pub fn is_cbus(&self) -> bool {
        *self == Sram3StaticMuxSel::Cbus
    }
}
#[doc = "Field `SRAM3_STATIC_MUX_SEL` writer - SRAM3 static mux select between CBUS and SBUS"]
pub type Sram3StaticMuxSelW<'a, REG> = crate::BitWriter<'a, REG, Sram3StaticMuxSel>;
impl<'a, REG> Sram3StaticMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SBUS path is selected"]
    #[inline(always)]
    pub fn sbus(self) -> &'a mut crate::W<REG> {
        self.variant(Sram3StaticMuxSel::Sbus)
    }
    #[doc = "CBUS path is selected"]
    #[inline(always)]
    pub fn cbus(self) -> &'a mut crate::W<REG> {
        self.variant(Sram3StaticMuxSel::Cbus)
    }
}
#[doc = "Field `SPARE` reader - Spare SOC LOCK Register 2"]
pub type SpareR = crate::FieldReader<u32>;
#[doc = "Field `SPARE` writer - Spare SOC LOCK Register 2"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32, crate::Safe>;
impl R {
    #[doc = "Bit 0 - SRAM3 static mux select between CBUS and SBUS"]
    #[inline(always)]
    pub fn sram3_static_mux_sel(&self) -> Sram3StaticMuxSelR {
        Sram3StaticMuxSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Spare SOC LOCK Register 2"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AM13SPARESOCLOCK2")
            .field("spare", &self.spare())
            .field("sram3_static_mux_sel", &self.sram3_static_mux_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SRAM3 static mux select between CBUS and SBUS"]
    #[inline(always)]
    pub fn sram3_static_mux_sel(&mut self) -> Sram3StaticMuxSelW<'_, Am13sparesoclock2Spec> {
        Sram3StaticMuxSelW::new(self, 0)
    }
    #[doc = "Bits 1:31 - Spare SOC LOCK Register 2"]
    #[inline(always)]
    pub fn spare(&mut self) -> SpareW<'_, Am13sparesoclock2Spec> {
        SpareW::new(self, 1)
    }
}
#[doc = "AM13 Spare SOC LOCK Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`am13sparesoclock2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`am13sparesoclock2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Am13sparesoclock2Spec;
impl crate::RegisterSpec for Am13sparesoclock2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`am13sparesoclock2::R`](R) reader structure"]
impl crate::Readable for Am13sparesoclock2Spec {}
#[doc = "`write(|w| ..)` method takes [`am13sparesoclock2::W`](W) writer structure"]
impl crate::Writable for Am13sparesoclock2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AM13SPARESOCLOCK2 to value 0x01"]
impl crate::Resettable for Am13sparesoclock2Spec {
    const RESET_VALUE: u32 = 0x01;
}
