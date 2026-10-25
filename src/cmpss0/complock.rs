#[doc = "Register `COMPLOCK` reader"]
pub type R = crate::R<ComplockSpec>;
#[doc = "Register `COMPLOCK` writer"]
pub type W = crate::W<ComplockSpec>;
#[doc = "Field `COMPCTL` reader - COMPCTL Lock"]
pub type CompctlR = crate::BitReader;
#[doc = "Field `COMPCTL` writer - COMPCTL Lock"]
pub type CompctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPHYSCTL` reader - COMPHYSCTL Lock"]
pub type ComphysctlR = crate::BitReader;
#[doc = "Field `COMPHYSCTL` writer - COMPHYSCTL Lock"]
pub type ComphysctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCTL` reader - DACCTL Lock"]
pub type DacctlR = crate::BitReader;
#[doc = "Field `DACCTL` writer - DACCTL Lock"]
pub type DacctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRIP` reader - CTRIP Lock"]
pub type CtripR = crate::BitReader;
#[doc = "Field `CTRIP` writer - CTRIP Lock"]
pub type CtripW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMPCTL Lock"]
    #[inline(always)]
    pub fn compctl(&self) -> CompctlR {
        CompctlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMPHYSCTL Lock"]
    #[inline(always)]
    pub fn comphysctl(&self) -> ComphysctlR {
        ComphysctlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DACCTL Lock"]
    #[inline(always)]
    pub fn dacctl(&self) -> DacctlR {
        DacctlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTRIP Lock"]
    #[inline(always)]
    pub fn ctrip(&self) -> CtripR {
        CtripR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPLOCK")
            .field("compctl", &self.compctl())
            .field("comphysctl", &self.comphysctl())
            .field("dacctl", &self.dacctl())
            .field("ctrip", &self.ctrip())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - COMPCTL Lock"]
    #[inline(always)]
    pub fn compctl(&mut self) -> CompctlW<'_, ComplockSpec> {
        CompctlW::new(self, 0)
    }
    #[doc = "Bit 1 - COMPHYSCTL Lock"]
    #[inline(always)]
    pub fn comphysctl(&mut self) -> ComphysctlW<'_, ComplockSpec> {
        ComphysctlW::new(self, 1)
    }
    #[doc = "Bit 2 - DACCTL Lock"]
    #[inline(always)]
    pub fn dacctl(&mut self) -> DacctlW<'_, ComplockSpec> {
        DacctlW::new(self, 2)
    }
    #[doc = "Bit 3 - CTRIP Lock"]
    #[inline(always)]
    pub fn ctrip(&mut self) -> CtripW<'_, ComplockSpec> {
        CtripW::new(self, 3)
    }
}
#[doc = "CMPSS Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`complock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`complock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComplockSpec;
impl crate::RegisterSpec for ComplockSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`complock::R`](R) reader structure"]
impl crate::Readable for ComplockSpec {}
#[doc = "`write(|w| ..)` method takes [`complock::W`](W) writer structure"]
impl crate::Writable for ComplockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPLOCK to value 0"]
impl crate::Resettable for ComplockSpec {}
