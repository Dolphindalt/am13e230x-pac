#[doc = "Register `IPMODE` reader"]
pub type R = crate::R<IpmodeSpec>;
#[doc = "Register `IPMODE` writer"]
pub type W = crate::W<IpmodeSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Select {
    #[doc = "0: Default functionality is UART"]
    Uart = 0,
    #[doc = "1: SPG in SPI Mode"]
    Spi = 1,
    #[doc = "2: SPG in I2C Controller Mode"]
    I2cController = 2,
    #[doc = "3: SPG in I2C Peripheral Mode"]
    I2cPeripheral = 3,
}
impl From<Select> for u8 {
    #[inline(always)]
    fn from(variant: Select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Select {
    type Ux = u8;
}
impl crate::IsEnum for Select {}
#[doc = "Field `SELECT` reader - Free run control"]
pub type SelectR = crate::FieldReader<Select>;
impl SelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Select {
        match self.bits {
            0 => Select::Uart,
            1 => Select::Spi,
            2 => Select::I2cController,
            3 => Select::I2cPeripheral,
            _ => unreachable!(),
        }
    }
    #[doc = "Default functionality is UART"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Select::Uart
    }
    #[doc = "SPG in SPI Mode"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Select::Spi
    }
    #[doc = "SPG in I2C Controller Mode"]
    #[inline(always)]
    pub fn is_i2c_controller(&self) -> bool {
        *self == Select::I2cController
    }
    #[doc = "SPG in I2C Peripheral Mode"]
    #[inline(always)]
    pub fn is_i2c_peripheral(&self) -> bool {
        *self == Select::I2cPeripheral
    }
}
#[doc = "Field `SELECT` writer - Free run control"]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 2, Select, crate::Safe>;
impl<'a, REG> SelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default functionality is UART"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Uart)
    }
    #[doc = "SPG in SPI Mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Spi)
    }
    #[doc = "SPG in I2C Controller Mode"]
    #[inline(always)]
    pub fn i2c_controller(self) -> &'a mut crate::W<REG> {
        self.variant(Select::I2cController)
    }
    #[doc = "SPG in I2C Peripheral Mode"]
    #[inline(always)]
    pub fn i2c_peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(Select::I2cPeripheral)
    }
}
impl R {
    #[doc = "Bits 0:1 - Free run control"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPMODE")
            .field("select", &self.select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Free run control"]
    #[inline(always)]
    pub fn select(&mut self) -> SelectW<'_, IpmodeSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "Mode Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpmodeSpec;
impl crate::RegisterSpec for IpmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipmode::R`](R) reader structure"]
impl crate::Readable for IpmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`ipmode::W`](W) writer structure"]
impl crate::Writable for IpmodeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IPMODE to value 0"]
impl crate::Resettable for IpmodeSpec {}
