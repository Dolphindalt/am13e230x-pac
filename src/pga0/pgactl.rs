#[doc = "Register `PGACTL` reader"]
pub type R = crate::R<PgactlSpec>;
#[doc = "Register `PGACTL` writer"]
pub type W = crate::W<PgactlSpec>;
#[doc = "Field `PGAEN` reader - PGA Enable"]
pub type PgaenR = crate::BitReader;
#[doc = "Field `PGAEN` writer - PGA Enable"]
pub type PgaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT_RES_SEL` reader - Filter Resistor Select"]
pub type FiltResSelR = crate::FieldReader;
#[doc = "Field `FILT_RES_SEL` writer - Filter Resistor Select"]
pub type FiltResSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `GAIN` reader - PGA gain setting"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - PGA gain setting"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `PGA_OUTENABLE` reader - PGA Output Enable"]
pub type PgaOutenableR = crate::BitReader;
#[doc = "Field `PGA_OUTENABLE` writer - PGA Output Enable"]
pub type PgaOutenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGA_OUTEN_INTGAIN` reader - PGA Internal Gain Output Enable"]
pub type PgaOutenIntgainR = crate::BitReader;
#[doc = "Field `PGA_OUTEN_INTGAIN` writer - PGA Internal Gain Output Enable"]
pub type PgaOutenIntgainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOP_EXTCTRL` reader - External Chop Signal Control"]
pub type ChopExtctrlR = crate::BitReader;
#[doc = "Field `CHOP_EXTCTRL` writer - External Chop Signal Control"]
pub type ChopExtctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PGA Enable"]
    #[inline(always)]
    pub fn pgaen(&self) -> PgaenR {
        PgaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Filter Resistor Select"]
    #[inline(always)]
    pub fn filt_res_sel(&self) -> FiltResSelR {
        FiltResSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 5:7 - PGA gain setting"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - PGA Output Enable"]
    #[inline(always)]
    pub fn pga_outenable(&self) -> PgaOutenableR {
        PgaOutenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PGA Internal Gain Output Enable"]
    #[inline(always)]
    pub fn pga_outen_intgain(&self) -> PgaOutenIntgainR {
        PgaOutenIntgainR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - External Chop Signal Control"]
    #[inline(always)]
    pub fn chop_extctrl(&self) -> ChopExtctrlR {
        ChopExtctrlR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGACTL")
            .field("pgaen", &self.pgaen())
            .field("filt_res_sel", &self.filt_res_sel())
            .field("gain", &self.gain())
            .field("pga_outenable", &self.pga_outenable())
            .field("pga_outen_intgain", &self.pga_outen_intgain())
            .field("chop_extctrl", &self.chop_extctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PGA Enable"]
    #[inline(always)]
    pub fn pgaen(&mut self) -> PgaenW<'_, PgactlSpec> {
        PgaenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Filter Resistor Select"]
    #[inline(always)]
    pub fn filt_res_sel(&mut self) -> FiltResSelW<'_, PgactlSpec> {
        FiltResSelW::new(self, 1)
    }
    #[doc = "Bits 5:7 - PGA gain setting"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, PgactlSpec> {
        GainW::new(self, 5)
    }
    #[doc = "Bit 8 - PGA Output Enable"]
    #[inline(always)]
    pub fn pga_outenable(&mut self) -> PgaOutenableW<'_, PgactlSpec> {
        PgaOutenableW::new(self, 8)
    }
    #[doc = "Bit 9 - PGA Internal Gain Output Enable"]
    #[inline(always)]
    pub fn pga_outen_intgain(&mut self) -> PgaOutenIntgainW<'_, PgactlSpec> {
        PgaOutenIntgainW::new(self, 9)
    }
    #[doc = "Bit 15 - External Chop Signal Control"]
    #[inline(always)]
    pub fn chop_extctrl(&mut self) -> ChopExtctrlW<'_, PgactlSpec> {
        ChopExtctrlW::new(self, 15)
    }
}
#[doc = "PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgactlSpec;
impl crate::RegisterSpec for PgactlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pgactl::R`](R) reader structure"]
impl crate::Readable for PgactlSpec {}
#[doc = "`write(|w| ..)` method takes [`pgactl::W`](W) writer structure"]
impl crate::Writable for PgactlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PGACTL to value 0"]
impl crate::Resettable for PgactlSpec {}
