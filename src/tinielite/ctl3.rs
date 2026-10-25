#[doc = "Register `CTL3` reader"]
pub type R = crate::R<Ctl3Spec>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<Ctl3Spec>;
#[doc = "Field `IFMAPCIRSTART` reader - Circular buffer input feature map start address"]
pub type IfmapcirstartR = crate::FieldReader<u16>;
#[doc = "Field `IFMAPCIRSTART` writer - Circular buffer input feature map start address"]
pub type IfmapcirstartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IFMAPCIREND` reader - Circular buffer input feature map end address"]
pub type IfmapcirendR = crate::FieldReader<u16>;
#[doc = "Field `IFMAPCIREND` writer - Circular buffer input feature map end address"]
pub type IfmapcirendW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Circular buffer input feature map start address"]
    #[inline(always)]
    pub fn ifmapcirstart(&self) -> IfmapcirstartR {
        IfmapcirstartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Circular buffer input feature map end address"]
    #[inline(always)]
    pub fn ifmapcirend(&self) -> IfmapcirendR {
        IfmapcirendR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL3")
            .field("ifmapcirstart", &self.ifmapcirstart())
            .field("ifmapcirend", &self.ifmapcirend())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Circular buffer input feature map start address"]
    #[inline(always)]
    pub fn ifmapcirstart(&mut self) -> IfmapcirstartW<'_, Ctl3Spec> {
        IfmapcirstartW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Circular buffer input feature map end address"]
    #[inline(always)]
    pub fn ifmapcirend(&mut self) -> IfmapcirendW<'_, Ctl3Spec> {
        IfmapcirendW::new(self, 16)
    }
}
#[doc = "CTL3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3Spec;
impl crate::RegisterSpec for Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for Ctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for Ctl3Spec {}
