#[doc = "Register `GLDCTL` reader"]
pub type R = crate::R<GldctlSpec>;
#[doc = "Register `GLDCTL` writer"]
pub type W = crate::W<GldctlSpec>;
#[doc = "Field `GLD` reader - Global load enable"]
pub type GldR = crate::BitReader;
#[doc = "Field `GLD` writer - Global load enable"]
pub type GldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSHTMODE` reader - Global load one-shot enable"]
pub type OshtmodeR = crate::BitReader;
#[doc = "Field `OSHTMODE` writer - Global load one-shot enable"]
pub type OshtmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLDMODE` reader - Select global load event"]
pub type GldmodeR = crate::FieldReader;
#[doc = "Field `GLDMODE` writer - Select global load event"]
pub type GldmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Global load enable"]
    #[inline(always)]
    pub fn gld(&self) -> GldR {
        GldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global load one-shot enable"]
    #[inline(always)]
    pub fn oshtmode(&self) -> OshtmodeR {
        OshtmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select global load event"]
    #[inline(always)]
    pub fn gldmode(&self) -> GldmodeR {
        GldmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLDCTL")
            .field("gld", &self.gld())
            .field("oshtmode", &self.oshtmode())
            .field("gldmode", &self.gldmode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Global load enable"]
    #[inline(always)]
    pub fn gld(&mut self) -> GldW<'_, GldctlSpec> {
        GldW::new(self, 0)
    }
    #[doc = "Bit 1 - Global load one-shot enable"]
    #[inline(always)]
    pub fn oshtmode(&mut self) -> OshtmodeW<'_, GldctlSpec> {
        OshtmodeW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Select global load event"]
    #[inline(always)]
    pub fn gldmode(&mut self) -> GldmodeW<'_, GldctlSpec> {
        GldmodeW::new(self, 4)
    }
}
#[doc = "Global load control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GldctlSpec;
impl crate::RegisterSpec for GldctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gldctl::R`](R) reader structure"]
impl crate::Readable for GldctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gldctl::W`](W) writer structure"]
impl crate::Writable for GldctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GLDCTL to value 0"]
impl crate::Resettable for GldctlSpec {}
