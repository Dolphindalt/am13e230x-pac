#[doc = "Register `COMPSTS` reader"]
pub type R = crate::R<CompstsSpec>;
#[doc = "Register `COMPSTS` writer"]
pub type W = crate::W<CompstsSpec>;
#[doc = "Field `COMPHSTS` reader - High Comparator Status"]
pub type ComphstsR = crate::BitReader;
#[doc = "Field `COMPHLATCH` reader - High Comparator Latched Status"]
pub type ComphlatchR = crate::BitReader;
#[doc = "Field `COMPLSTS` reader - Low Comparator Status"]
pub type ComplstsR = crate::BitReader;
#[doc = "Field `COMPLLATCH` reader - Low Comparator Latched Status"]
pub type CompllatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - High Comparator Status"]
    #[inline(always)]
    pub fn comphsts(&self) -> ComphstsR {
        ComphstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Comparator Latched Status"]
    #[inline(always)]
    pub fn comphlatch(&self) -> ComphlatchR {
        ComphlatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Comparator Status"]
    #[inline(always)]
    pub fn complsts(&self) -> ComplstsR {
        ComplstsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Comparator Latched Status"]
    #[inline(always)]
    pub fn compllatch(&self) -> CompllatchR {
        CompllatchR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPSTS")
            .field("comphsts", &self.comphsts())
            .field("comphlatch", &self.comphlatch())
            .field("complsts", &self.complsts())
            .field("compllatch", &self.compllatch())
            .finish()
    }
}
impl W {}
#[doc = "CMPSS Comparator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompstsSpec;
impl crate::RegisterSpec for CompstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compsts::R`](R) reader structure"]
impl crate::Readable for CompstsSpec {}
#[doc = "`write(|w| ..)` method takes [`compsts::W`](W) writer structure"]
impl crate::Writable for CompstsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPSTS to value 0"]
impl crate::Resettable for CompstsSpec {}
