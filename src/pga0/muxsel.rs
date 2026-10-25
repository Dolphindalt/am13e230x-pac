#[doc = "Register `MUXSEL` reader"]
pub type R = crate::R<MuxselSpec>;
#[doc = "Register `MUXSEL` writer"]
pub type W = crate::W<MuxselSpec>;
#[doc = "Field `PMUXSEL` reader - Positive Input Mux Select"]
pub type PmuxselR = crate::FieldReader;
#[doc = "Field `PMUXSEL` writer - Positive Input Mux Select"]
pub type PmuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `NMUXSEL` reader - Negative Input Mux Select"]
pub type NmuxselR = crate::FieldReader;
#[doc = "Field `NMUXSEL` writer - Negative Input Mux Select"]
pub type NmuxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `MMUXSEL` reader - M Mux Select"]
pub type MmuxselR = crate::FieldReader;
#[doc = "Field `MMUXSEL` writer - M Mux Select"]
pub type MmuxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `PGA_FBONPIN` reader - Connect PGAOUT to PGA_INM"]
pub type PgaFbonpinR = crate::BitReader;
#[doc = "Field `PGA_FBONPIN` writer - Connect PGAOUT to PGA_INM"]
pub type PgaFbonpinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGA_CHOP` reader - Chopper Mode Control"]
pub type PgaChopR = crate::FieldReader;
#[doc = "Field `PGA_CHOP` writer - Chopper Mode Control"]
pub type PgaChopW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - Positive Input Mux Select"]
    #[inline(always)]
    pub fn pmuxsel(&self) -> PmuxselR {
        PmuxselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Negative Input Mux Select"]
    #[inline(always)]
    pub fn nmuxsel(&self) -> NmuxselR {
        NmuxselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - M Mux Select"]
    #[inline(always)]
    pub fn mmuxsel(&self) -> MmuxselR {
        MmuxselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Connect PGAOUT to PGA_INM"]
    #[inline(always)]
    pub fn pga_fbonpin(&self) -> PgaFbonpinR {
        PgaFbonpinR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Chopper Mode Control"]
    #[inline(always)]
    pub fn pga_chop(&self) -> PgaChopR {
        PgaChopR::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSEL")
            .field("pmuxsel", &self.pmuxsel())
            .field("nmuxsel", &self.nmuxsel())
            .field("mmuxsel", &self.mmuxsel())
            .field("pga_fbonpin", &self.pga_fbonpin())
            .field("pga_chop", &self.pga_chop())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Positive Input Mux Select"]
    #[inline(always)]
    pub fn pmuxsel(&mut self) -> PmuxselW<'_, MuxselSpec> {
        PmuxselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Negative Input Mux Select"]
    #[inline(always)]
    pub fn nmuxsel(&mut self) -> NmuxselW<'_, MuxselSpec> {
        NmuxselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - M Mux Select"]
    #[inline(always)]
    pub fn mmuxsel(&mut self) -> MmuxselW<'_, MuxselSpec> {
        MmuxselW::new(self, 8)
    }
    #[doc = "Bit 12 - Connect PGAOUT to PGA_INM"]
    #[inline(always)]
    pub fn pga_fbonpin(&mut self) -> PgaFbonpinW<'_, MuxselSpec> {
        PgaFbonpinW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Chopper Mode Control"]
    #[inline(always)]
    pub fn pga_chop(&mut self) -> PgaChopW<'_, MuxselSpec> {
        PgaChopW::new(self, 13)
    }
}
#[doc = "Mux Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxselSpec;
impl crate::RegisterSpec for MuxselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`muxsel::R`](R) reader structure"]
impl crate::Readable for MuxselSpec {}
#[doc = "`write(|w| ..)` method takes [`muxsel::W`](W) writer structure"]
impl crate::Writable for MuxselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MUXSEL to value 0"]
impl crate::Resettable for MuxselSpec {}
