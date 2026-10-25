#[doc = "Register `ETSEL` reader"]
pub type R = crate::R<EtselSpec>;
#[doc = "Register `ETSEL` writer"]
pub type W = crate::W<EtselSpec>;
#[doc = "Field `ET1_SEL` reader - Event trigger1 selection"]
pub type Et1SelR = crate::FieldReader;
#[doc = "Field `ET1_SEL` writer - Event trigger1 selection"]
pub type Et1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `ET2_SEL` reader - Event trigger2 selection"]
pub type Et2SelR = crate::FieldReader;
#[doc = "Field `ET2_SEL` writer - Event trigger2 selection"]
pub type Et2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - Event trigger1 selection"]
    #[inline(always)]
    pub fn et1_sel(&self) -> Et1SelR {
        Et1SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Event trigger2 selection"]
    #[inline(always)]
    pub fn et2_sel(&self) -> Et2SelR {
        Et2SelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETSEL")
            .field("et1_sel", &self.et1_sel())
            .field("et2_sel", &self.et2_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Event trigger1 selection"]
    #[inline(always)]
    pub fn et1_sel(&mut self) -> Et1SelW<'_, EtselSpec> {
        Et1SelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Event trigger2 selection"]
    #[inline(always)]
    pub fn et2_sel(&mut self) -> Et2SelW<'_, EtselSpec> {
        Et2SelW::new(self, 8)
    }
}
#[doc = "Event trigger selection\n\nYou can [`read`](crate::Reg::read) this register and get [`etsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtselSpec;
impl crate::RegisterSpec for EtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etsel::R`](R) reader structure"]
impl crate::Readable for EtselSpec {}
#[doc = "`write(|w| ..)` method takes [`etsel::W`](W) writer structure"]
impl crate::Writable for EtselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ETSEL to value 0"]
impl crate::Resettable for EtselSpec {}
