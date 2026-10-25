#[doc = "Register `CMDDATA11` reader"]
pub type R = crate::R<Cmddata11Spec>;
#[doc = "Register `CMDDATA11` writer"]
pub type W = crate::W<Cmddata11Spec>;
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
        f.debug_struct("CMDDATA11")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Cmddata11Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmddata11Spec;
impl crate::RegisterSpec for Cmddata11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata11::R`](R) reader structure"]
impl crate::Readable for Cmddata11Spec {}
#[doc = "`write(|w| ..)` method takes [`cmddata11::W`](W) writer structure"]
impl crate::Writable for Cmddata11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDDATA11 to value 0"]
impl crate::Resettable for Cmddata11Spec {}
