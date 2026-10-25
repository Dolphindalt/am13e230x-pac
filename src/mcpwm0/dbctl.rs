#[doc = "Register `DBCTL` reader"]
pub type R = crate::R<DbctlSpec>;
#[doc = "Register `DBCTL` writer"]
pub type W = crate::W<DbctlSpec>;
#[doc = "Field `OUT_MODE` reader - Dead Band Output Mode Control"]
pub type OutModeR = crate::FieldReader;
#[doc = "Field `OUT_MODE` writer - Dead Band Output Mode Control"]
pub type OutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `POLSEL` reader - Polarity Select Control"]
pub type PolselR = crate::FieldReader;
#[doc = "Field `POLSEL` writer - Polarity Select Control"]
pub type PolselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `IN_MODE` reader - Dead Band Input Select Mode Control"]
pub type InModeR = crate::FieldReader;
#[doc = "Field `IN_MODE` writer - Dead Band Input Select Mode Control"]
pub type InModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `OUTSWAP` reader - Dead Band Output Swap Control"]
pub type OutswapR = crate::FieldReader;
#[doc = "Field `OUTSWAP` writer - Dead Band Output Swap Control"]
pub type OutswapW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `DEDB_MODE` reader - Dead Band Dual-Edge B Mode Control"]
pub type DedbModeR = crate::BitReader;
#[doc = "Field `DEDB_MODE` writer - Dead Band Dual-Edge B Mode Control"]
pub type DedbModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOADFEDMODE` reader - Shadow to active load of DBFED"]
pub type LoadfedmodeR = crate::FieldReader;
#[doc = "Field `LOADFEDMODE` writer - Shadow to active load of DBFED"]
pub type LoadfedmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `LOADREDMODE` reader - Shadow to active load of DBRED"]
pub type LoadredmodeR = crate::FieldReader;
#[doc = "Field `LOADREDMODE` writer - Shadow to active load of DBRED"]
pub type LoadredmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Dead Band Output Mode Control"]
    #[inline(always)]
    pub fn out_mode(&self) -> OutModeR {
        OutModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Polarity Select Control"]
    #[inline(always)]
    pub fn polsel(&self) -> PolselR {
        PolselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Dead Band Input Select Mode Control"]
    #[inline(always)]
    pub fn in_mode(&self) -> InModeR {
        InModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Dead Band Output Swap Control"]
    #[inline(always)]
    pub fn outswap(&self) -> OutswapR {
        OutswapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Dead Band Dual-Edge B Mode Control"]
    #[inline(always)]
    pub fn dedb_mode(&self) -> DedbModeR {
        DedbModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Shadow to active load of DBFED"]
    #[inline(always)]
    pub fn loadfedmode(&self) -> LoadfedmodeR {
        LoadfedmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Shadow to active load of DBRED"]
    #[inline(always)]
    pub fn loadredmode(&self) -> LoadredmodeR {
        LoadredmodeR::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBCTL")
            .field("out_mode", &self.out_mode())
            .field("polsel", &self.polsel())
            .field("in_mode", &self.in_mode())
            .field("outswap", &self.outswap())
            .field("dedb_mode", &self.dedb_mode())
            .field("loadfedmode", &self.loadfedmode())
            .field("loadredmode", &self.loadredmode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Dead Band Output Mode Control"]
    #[inline(always)]
    pub fn out_mode(&mut self) -> OutModeW<'_, DbctlSpec> {
        OutModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Polarity Select Control"]
    #[inline(always)]
    pub fn polsel(&mut self) -> PolselW<'_, DbctlSpec> {
        PolselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Dead Band Input Select Mode Control"]
    #[inline(always)]
    pub fn in_mode(&mut self) -> InModeW<'_, DbctlSpec> {
        InModeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Dead Band Output Swap Control"]
    #[inline(always)]
    pub fn outswap(&mut self) -> OutswapW<'_, DbctlSpec> {
        OutswapW::new(self, 6)
    }
    #[doc = "Bit 8 - Dead Band Dual-Edge B Mode Control"]
    #[inline(always)]
    pub fn dedb_mode(&mut self) -> DedbModeW<'_, DbctlSpec> {
        DedbModeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Shadow to active load of DBFED"]
    #[inline(always)]
    pub fn loadfedmode(&mut self) -> LoadfedmodeW<'_, DbctlSpec> {
        LoadfedmodeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Shadow to active load of DBRED"]
    #[inline(always)]
    pub fn loadredmode(&mut self) -> LoadredmodeW<'_, DbctlSpec> {
        LoadredmodeW::new(self, 18)
    }
}
#[doc = "Dead band control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbctlSpec;
impl crate::RegisterSpec for DbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbctl::R`](R) reader structure"]
impl crate::Readable for DbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbctl::W`](W) writer structure"]
impl crate::Writable for DbctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBCTL to value 0"]
impl crate::Resettable for DbctlSpec {}
