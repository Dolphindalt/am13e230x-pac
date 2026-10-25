#[doc = "Register `CMDDATA5` reader"]
pub type R = crate::R<Cmddata5Spec>;
#[doc = "Register `CMDDATA5` writer"]
pub type W = crate::W<Cmddata5Spec>;
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
        f.debug_struct("CMDDATA5")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Cmddata5Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmddata5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmddata5Spec;
impl crate::RegisterSpec for Cmddata5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata5::R`](R) reader structure"]
impl crate::Readable for Cmddata5Spec {}
#[doc = "`write(|w| ..)` method takes [`cmddata5::W`](W) writer structure"]
impl crate::Writable for Cmddata5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDDATA5 to value 0"]
impl crate::Resettable for Cmddata5Spec {}
