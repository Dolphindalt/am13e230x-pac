#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `IFMAPSTARTHI` reader - Start address (Higher 16 bits) of input feature map data."]
pub type IfmapstarthiR = crate::FieldReader<u16>;
#[doc = "Field `IFMAPSTARTHI` writer - Start address (Higher 16 bits) of input feature map data."]
pub type IfmapstarthiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFMAPSTARTHI` reader - Start address (Higher 16 bits) of output feature map data."]
pub type OfmapstarthiR = crate::FieldReader<u16>;
#[doc = "Field `OFMAPSTARTHI` writer - Start address (Higher 16 bits) of output feature map data."]
pub type OfmapstarthiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start address (Higher 16 bits) of input feature map data."]
    #[inline(always)]
    pub fn ifmapstarthi(&self) -> IfmapstarthiR {
        IfmapstarthiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address (Higher 16 bits) of output feature map data."]
    #[inline(always)]
    pub fn ofmapstarthi(&self) -> OfmapstarthiR {
        OfmapstarthiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("ifmapstarthi", &self.ifmapstarthi())
            .field("ofmapstarthi", &self.ofmapstarthi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address (Higher 16 bits) of input feature map data."]
    #[inline(always)]
    pub fn ifmapstarthi(&mut self) -> IfmapstarthiW<'_, Ctl1Spec> {
        IfmapstarthiW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address (Higher 16 bits) of output feature map data."]
    #[inline(always)]
    pub fn ofmapstarthi(&mut self) -> OfmapstarthiW<'_, Ctl1Spec> {
        OfmapstarthiW::new(self, 16)
    }
}
#[doc = "CTL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
