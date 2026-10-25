#[doc = "Register `CTL4` reader"]
pub type R = crate::R<Ctl4Spec>;
#[doc = "Register `CTL4` writer"]
pub type W = crate::W<Ctl4Spec>;
#[doc = "Field `OFMAPCIRSTART` reader - Circular buffer output feature map start address."]
pub type OfmapcirstartR = crate::FieldReader<u16>;
#[doc = "Field `OFMAPCIRSTART` writer - Circular buffer output feature map start address."]
pub type OfmapcirstartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFMAPCIREND` reader - Circular buffer output feature map end address."]
pub type OfmapcirendR = crate::FieldReader<u16>;
#[doc = "Field `OFMAPCIREND` writer - Circular buffer output feature map end address."]
pub type OfmapcirendW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Circular buffer output feature map start address."]
    #[inline(always)]
    pub fn ofmapcirstart(&self) -> OfmapcirstartR {
        OfmapcirstartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Circular buffer output feature map end address."]
    #[inline(always)]
    pub fn ofmapcirend(&self) -> OfmapcirendR {
        OfmapcirendR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL4")
            .field("ofmapcirstart", &self.ofmapcirstart())
            .field("ofmapcirend", &self.ofmapcirend())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Circular buffer output feature map start address."]
    #[inline(always)]
    pub fn ofmapcirstart(&mut self) -> OfmapcirstartW<'_, Ctl4Spec> {
        OfmapcirstartW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Circular buffer output feature map end address."]
    #[inline(always)]
    pub fn ofmapcirend(&mut self) -> OfmapcirendW<'_, Ctl4Spec> {
        OfmapcirendW::new(self, 16)
    }
}
#[doc = "CTL4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl4Spec;
impl crate::RegisterSpec for Ctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl4::R`](R) reader structure"]
impl crate::Readable for Ctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl4::W`](W) writer structure"]
impl crate::Writable for Ctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL4 to value 0"]
impl crate::Resettable for Ctl4Spec {}
