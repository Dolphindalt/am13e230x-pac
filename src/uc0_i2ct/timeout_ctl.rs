#[doc = "Register `TIMEOUT_CTL` reader"]
pub type R = crate::R<TimeoutCtlSpec>;
#[doc = "Register `TIMEOUT_CTL` writer"]
pub type W = crate::W<TimeoutCtlSpec>;
#[doc = "Field `TCNTLA` reader - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
pub type TcntlaR = crate::FieldReader;
#[doc = "Field `TCNTLA` writer - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
pub type TcntlaW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Timeout Counter A Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcntaen {
    #[doc = "0: Disable Timeout Counter B"]
    Disable = 0,
    #[doc = "1: Enable Timeout Counter B"]
    Enable = 1,
}
impl From<Tcntaen> for bool {
    #[inline(always)]
    fn from(variant: Tcntaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCNTAEN` reader - Timeout Counter A Enable"]
pub type TcntaenR = crate::BitReader<Tcntaen>;
impl TcntaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcntaen {
        match self.bits {
            false => Tcntaen::Disable,
            true => Tcntaen::Enable,
        }
    }
    #[doc = "Disable Timeout Counter B"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tcntaen::Disable
    }
    #[doc = "Enable Timeout Counter B"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tcntaen::Enable
    }
}
#[doc = "Field `TCNTAEN` writer - Timeout Counter A Enable"]
pub type TcntaenW<'a, REG> = crate::BitWriter<'a, REG, Tcntaen>;
impl<'a, REG> TcntaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Timeout Counter B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcntaen::Disable)
    }
    #[doc = "Enable Timeout Counter B"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcntaen::Enable)
    }
}
#[doc = "Field `TCNTLB` reader - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
pub type TcntlbR = crate::FieldReader;
#[doc = "Field `TCNTLB` writer - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
pub type TcntlbW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Timeout Counter B Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcntben {
    #[doc = "0: Disable Timeout Counter B"]
    Disable = 0,
    #[doc = "1: Enable Timeout Counter B"]
    Enable = 1,
}
impl From<Tcntben> for bool {
    #[inline(always)]
    fn from(variant: Tcntben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCNTBEN` reader - Timeout Counter B Enable"]
pub type TcntbenR = crate::BitReader<Tcntben>;
impl TcntbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcntben {
        match self.bits {
            false => Tcntben::Disable,
            true => Tcntben::Enable,
        }
    }
    #[doc = "Disable Timeout Counter B"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tcntben::Disable
    }
    #[doc = "Enable Timeout Counter B"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tcntben::Enable
    }
}
#[doc = "Field `TCNTBEN` writer - Timeout Counter B Enable"]
pub type TcntbenW<'a, REG> = crate::BitWriter<'a, REG, Tcntben>;
impl<'a, REG> TcntbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Timeout Counter B"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcntben::Disable)
    }
    #[doc = "Enable Timeout Counter B"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcntben::Enable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
    #[inline(always)]
    pub fn tcntla(&self) -> TcntlaR {
        TcntlaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Timeout Counter A Enable"]
    #[inline(always)]
    pub fn tcntaen(&self) -> TcntaenR {
        TcntaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
    #[inline(always)]
    pub fn tcntlb(&self) -> TcntlbR {
        TcntlbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Timeout Counter B Enable"]
    #[inline(always)]
    pub fn tcntben(&self) -> TcntbenR {
        TcntbenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT_CTL")
            .field("tcntla", &self.tcntla())
            .field("tcntaen", &self.tcntaen())
            .field("tcntlb", &self.tcntlb())
            .field("tcntben", &self.tcntben())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
    #[inline(always)]
    pub fn tcntla(&mut self) -> TcntlaW<'_, TimeoutCtlSpec> {
        TcntlaW::new(self, 0)
    }
    #[doc = "Bit 15 - Timeout Counter A Enable"]
    #[inline(always)]
    pub fn tcntaen(&mut self) -> TcntaenW<'_, TimeoutCtlSpec> {
        TcntaenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
    #[inline(always)]
    pub fn tcntlb(&mut self) -> TcntlbW<'_, TimeoutCtlSpec> {
        TcntlbW::new(self, 16)
    }
    #[doc = "Bit 31 - Timeout Counter B Enable"]
    #[inline(always)]
    pub fn tcntben(&mut self) -> TcntbenW<'_, TimeoutCtlSpec> {
        TcntbenW::new(self, 31)
    }
}
#[doc = "I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCtlSpec;
impl crate::RegisterSpec for TimeoutCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_ctl::R`](R) reader structure"]
impl crate::Readable for TimeoutCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctl::W`](W) writer structure"]
impl crate::Writable for TimeoutCtlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMEOUT_CTL to value 0x0002_0002"]
impl crate::Resettable for TimeoutCtlSpec {
    const RESET_VALUE: u32 = 0x0002_0002;
}
