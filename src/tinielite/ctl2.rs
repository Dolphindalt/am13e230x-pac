#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `CLLOW` reader - Lower clamp value"]
pub type CllowR = crate::FieldReader<u16>;
#[doc = "Field `CLLOW` writer - Lower clamp value"]
pub type CllowW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CLHIGH` reader - Upper clamp value"]
pub type ClhighR = crate::FieldReader<u16>;
#[doc = "Field `CLHIGH` writer - Upper clamp value"]
pub type ClhighW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Lower clamp value"]
    #[inline(always)]
    pub fn cllow(&self) -> CllowR {
        CllowR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Upper clamp value"]
    #[inline(always)]
    pub fn clhigh(&self) -> ClhighR {
        ClhighR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL2")
            .field("cllow", &self.cllow())
            .field("clhigh", &self.clhigh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Lower clamp value"]
    #[inline(always)]
    pub fn cllow(&mut self) -> CllowW<'_, Ctl2Spec> {
        CllowW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Upper clamp value"]
    #[inline(always)]
    pub fn clhigh(&mut self) -> ClhighW<'_, Ctl2Spec> {
        ClhighW::new(self, 16)
    }
}
#[doc = "CTL2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {}
