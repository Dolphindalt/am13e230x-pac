#[doc = "Register `PC` reader"]
pub type R = crate::R<PcSpec>;
#[doc = "Register `PC` writer"]
pub type W = crate::W<PcSpec>;
#[doc = "Field `VALUE` reader - Program count value."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Program count value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Program count value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PC").field("value", &self.value()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Program count value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, PcSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Program count register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcSpec;
impl crate::RegisterSpec for PcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PcSpec {}
#[doc = "`write(|w| ..)` method takes [`pc::W`](W) writer structure"]
impl crate::Writable for PcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PcSpec {}
