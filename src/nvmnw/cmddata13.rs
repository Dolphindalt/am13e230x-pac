#[doc = "Register `CMDDATA13` reader"]
pub type R = crate::R<Cmddata13Spec>;
#[doc = "Register `CMDDATA13` writer"]
pub type W = crate::W<Cmddata13Spec>;
#[doc = "Field `VAL` reader - A 32-bit data value is placed in this field."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - A 32-bit data value is placed in this field."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDDATA13")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Cmddata13Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmddata13Spec;
impl crate::RegisterSpec for Cmddata13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata13::R`](R) reader structure"]
impl crate::Readable for Cmddata13Spec {}
#[doc = "`write(|w| ..)` method takes [`cmddata13::W`](W) writer structure"]
impl crate::Writable for Cmddata13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDDATA13 to value 0"]
impl crate::Resettable for Cmddata13Spec {}
