#[doc = "Register `TZCTL` reader"]
pub type R = crate::R<TzctlSpec>;
#[doc = "Register `TZCTL` writer"]
pub type W = crate::W<TzctlSpec>;
#[doc = "Field `TZA` reader - TZ1 to TZ8 Trip Action on PWMA"]
pub type TzaR = crate::FieldReader;
#[doc = "Field `TZA` writer - TZ1 to TZ8 Trip Action on PWMA"]
pub type TzaW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `TZB` reader - TZ1 to TZ8 Trip Action on PWMB"]
pub type TzbR = crate::FieldReader;
#[doc = "Field `TZB` writer - TZ1 to TZ8 Trip Action on PWMB"]
pub type TzbW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `CBCPULSE` reader - Clear Pulse for CBC Trip Latch"]
pub type CbcpulseR = crate::FieldReader;
#[doc = "Field `CBCPULSE` writer - Clear Pulse for CBC Trip Latch"]
pub type CbcpulseW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - TZ1 to TZ8 Trip Action on PWMA"]
    #[inline(always)]
    pub fn tza(&self) -> TzaR {
        TzaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TZ1 to TZ8 Trip Action on PWMB"]
    #[inline(always)]
    pub fn tzb(&self) -> TzbR {
        TzbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clear Pulse for CBC Trip Latch"]
    #[inline(always)]
    pub fn cbcpulse(&self) -> CbcpulseR {
        CbcpulseR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZCTL")
            .field("tza", &self.tza())
            .field("tzb", &self.tzb())
            .field("cbcpulse", &self.cbcpulse())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - TZ1 to TZ8 Trip Action on PWMA"]
    #[inline(always)]
    pub fn tza(&mut self) -> TzaW<'_, TzctlSpec> {
        TzaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - TZ1 to TZ8 Trip Action on PWMB"]
    #[inline(always)]
    pub fn tzb(&mut self) -> TzbW<'_, TzctlSpec> {
        TzbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Clear Pulse for CBC Trip Latch"]
    #[inline(always)]
    pub fn cbcpulse(&mut self) -> CbcpulseW<'_, TzctlSpec> {
        CbcpulseW::new(self, 4)
    }
}
#[doc = "Trip Zone control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzctlSpec;
impl crate::RegisterSpec for TzctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzctl::R`](R) reader structure"]
impl crate::Readable for TzctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzctl::W`](W) writer structure"]
impl crate::Writable for TzctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZCTL to value 0x10"]
impl crate::Resettable for TzctlSpec {
    const RESET_VALUE: u32 = 0x10;
}
