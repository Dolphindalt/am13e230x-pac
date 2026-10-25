#[doc = "Register `CANTX` reader"]
pub type R = crate::R<CantxSpec>;
#[doc = "Register `CANTX` writer"]
pub type W = crate::W<CantxSpec>;
#[doc = "GPIO channel Select 0: Select the P-Channel for the F update 1: Select the G-Channel for the F update\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gsel {
    #[doc = "0: Select the P-Channel for the F update"]
    Pchan = 0,
    #[doc = "1: Select the G-Channel for the F update"]
    Gchan = 1,
}
impl From<Gsel> for bool {
    #[inline(always)]
    fn from(variant: Gsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSEL` writer - GPIO channel Select 0: Select the P-Channel for the F update 1: Select the G-Channel for the F update"]
pub type GselW<'a, REG> = crate::BitWriter<'a, REG, Gsel>;
impl<'a, REG> GselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select the P-Channel for the F update"]
    #[inline(always)]
    pub fn pchan(self) -> &'a mut crate::W<REG> {
        self.variant(Gsel::Pchan)
    }
    #[doc = "Select the G-Channel for the F update"]
    #[inline(always)]
    pub fn gchan(self) -> &'a mut crate::W<REG> {
        self.variant(Gsel::Gchan)
    }
}
#[doc = "Field `LOCK` writer - Sets lock bit"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOADDR` writer - IO Address. This is the address that corresponds to the SOC address\\[27:2\\] of the module IP instance specific IO signal in the Full Write subregion of the pinmux subregion."]
pub type IoaddrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32, crate::Safe>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANTX").finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO channel Select 0: Select the P-Channel for the F update 1: Select the G-Channel for the F update"]
    #[inline(always)]
    pub fn gsel(&mut self) -> GselW<'_, CantxSpec> {
        GselW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets lock bit"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CantxSpec> {
        LockW::new(self, 1)
    }
    #[doc = "Bits 2:27 - IO Address. This is the address that corresponds to the SOC address\\[27:2\\] of the module IP instance specific IO signal in the Full Write subregion of the pinmux subregion."]
    #[inline(always)]
    pub fn ioaddr(&mut self) -> IoaddrW<'_, CantxSpec> {
        IoaddrW::new(self, 2)
    }
}
#[doc = "FUPDATE version of CANTX\n\nYou can [`read`](crate::Reg::read) this register and get [`cantx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cantx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CantxSpec;
impl crate::RegisterSpec for CantxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cantx::R`](R) reader structure"]
impl crate::Readable for CantxSpec {}
#[doc = "`write(|w| ..)` method takes [`cantx::W`](W) writer structure"]
impl crate::Writable for CantxSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CANTX to value 0"]
impl crate::Resettable for CantxSpec {}
