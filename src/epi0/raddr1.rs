#[doc = "Register `RADDR1` reader"]
pub type R = crate::R<Raddr1Spec>;
#[doc = "Register `RADDR1` writer"]
pub type W = crate::W<Raddr1Spec>;
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
        f.debug_struct("RADDR1")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Raddr1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "EPI Read Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`raddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Raddr1Spec;
impl crate::RegisterSpec for Raddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raddr1::R`](R) reader structure"]
impl crate::Readable for Raddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`raddr1::W`](W) writer structure"]
impl crate::Writable for Raddr1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RADDR1 to value 0"]
impl crate::Resettable for Raddr1Spec {}
