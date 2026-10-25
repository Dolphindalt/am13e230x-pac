#[doc = "Register `RADDR0` reader"]
pub type R = crate::R<Raddr0Spec>;
#[doc = "Register `RADDR0` writer"]
pub type W = crate::W<Raddr0Spec>;
#[doc = "Field `ADDR` reader - Current Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Current Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Current Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADDR0")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Raddr0Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "EPI Read Address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Raddr0Spec;
impl crate::RegisterSpec for Raddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raddr0::R`](R) reader structure"]
impl crate::Readable for Raddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`raddr0::W`](W) writer structure"]
impl crate::Writable for Raddr0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RADDR0 to value 0"]
impl crate::Resettable for Raddr0Spec {}
