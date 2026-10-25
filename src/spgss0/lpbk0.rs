#[doc = "Register `LPBK0` reader"]
pub type R = crate::R<Lpbk0Spec>;
#[doc = "Register `LPBK0` writer"]
pub type W = crate::W<Lpbk0Spec>;
#[doc = "Loopback enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Loopback enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - Loopback enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
#[doc = "Select SPG controller for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Controller {
    #[doc = "0: SPG commander select for loopback."]
    Spg1 = 0,
    #[doc = "1: SPG commander select for loopback."]
    Spg2 = 1,
    #[doc = "2: SPG commander select for loopback."]
    Spg3 = 2,
    #[doc = "3: SPG commander select for loopback."]
    Spg4 = 3,
    #[doc = "4: SPG commander select for loopback."]
    Spg5 = 4,
    #[doc = "5: SPG commander select for loopback."]
    Spg6 = 5,
    #[doc = "6: SPG commander select for loopback."]
    Spg7 = 6,
    #[doc = "7: SPG commander select for loopback."]
    Spg8 = 7,
    #[doc = "8: SPG commander select for loopback."]
    Spg9 = 8,
    #[doc = "9: SPG commander select for loopback."]
    Spg10 = 9,
    #[doc = "10: SPG commander select for loopback."]
    Spg11 = 10,
    #[doc = "11: SPG commander select for loopback."]
    Spg12 = 11,
    #[doc = "12: SPG commander select for loopback."]
    Spg13 = 12,
    #[doc = "13: SPG commander select for loopback."]
    Spg14 = 13,
    #[doc = "14: SPG commander select for loopback."]
    Spg15 = 14,
    #[doc = "15: SPG commander select for loopback."]
    Spg16 = 15,
}
impl From<Controller> for u8 {
    #[inline(always)]
    fn from(variant: Controller) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Controller {
    type Ux = u8;
}
impl crate::IsEnum for Controller {}
#[doc = "Field `CONTROLLER` reader - Select SPG controller for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
pub type ControllerR = crate::FieldReader<Controller>;
impl ControllerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Controller {
        match self.bits {
            0 => Controller::Spg1,
            1 => Controller::Spg2,
            2 => Controller::Spg3,
            3 => Controller::Spg4,
            4 => Controller::Spg5,
            5 => Controller::Spg6,
            6 => Controller::Spg7,
            7 => Controller::Spg8,
            8 => Controller::Spg9,
            9 => Controller::Spg10,
            10 => Controller::Spg11,
            11 => Controller::Spg12,
            12 => Controller::Spg13,
            13 => Controller::Spg14,
            14 => Controller::Spg15,
            15 => Controller::Spg16,
            _ => unreachable!(),
        }
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg1(&self) -> bool {
        *self == Controller::Spg1
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg2(&self) -> bool {
        *self == Controller::Spg2
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg3(&self) -> bool {
        *self == Controller::Spg3
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg4(&self) -> bool {
        *self == Controller::Spg4
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg5(&self) -> bool {
        *self == Controller::Spg5
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg6(&self) -> bool {
        *self == Controller::Spg6
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg7(&self) -> bool {
        *self == Controller::Spg7
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg8(&self) -> bool {
        *self == Controller::Spg8
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg9(&self) -> bool {
        *self == Controller::Spg9
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg10(&self) -> bool {
        *self == Controller::Spg10
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg11(&self) -> bool {
        *self == Controller::Spg11
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg12(&self) -> bool {
        *self == Controller::Spg12
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg13(&self) -> bool {
        *self == Controller::Spg13
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg14(&self) -> bool {
        *self == Controller::Spg14
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg15(&self) -> bool {
        *self == Controller::Spg15
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg16(&self) -> bool {
        *self == Controller::Spg16
    }
}
#[doc = "Field `CONTROLLER` writer - Select SPG controller for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
pub type ControllerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Controller, crate::Safe>;
impl<'a, REG> ControllerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg1(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg1)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg2(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg2)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg3(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg3)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg4(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg4)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg5(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg5)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg6(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg6)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg7(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg7)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg8(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg8)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg9(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg9)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg10(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg10)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg11(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg11)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg12(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg12)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg13(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg13)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg14(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg14)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg15(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg15)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg16(self) -> &'a mut crate::W<REG> {
        self.variant(Controller::Spg16)
    }
}
#[doc = "Select SPG peripheral for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peripheral {
    #[doc = "0: SPG responder select for loopback."]
    Spg1 = 0,
    #[doc = "1: SPG responder select for loopback."]
    Spg2 = 1,
    #[doc = "2: SPG responder select for loopback."]
    Spg3 = 2,
    #[doc = "3: SPG responder select for loopback."]
    Spg4 = 3,
    #[doc = "4: SPG responder select for loopback."]
    Spg5 = 4,
    #[doc = "5: SPG responder select for loopback."]
    Spg6 = 5,
    #[doc = "6: SPG responder select for loopback."]
    Spg7 = 6,
    #[doc = "7: SPG responder select for loopback."]
    Spg8 = 7,
    #[doc = "8: SPG responder select for loopback."]
    Spg9 = 8,
    #[doc = "9: SPG responder select for loopback."]
    Spg10 = 9,
    #[doc = "10: SPG responder select for loopback."]
    Spg11 = 10,
    #[doc = "11: SPG responder select for loopback."]
    Spg12 = 11,
    #[doc = "12: SPG responder select for loopback."]
    Spg13 = 12,
    #[doc = "13: SPG responder select for loopback."]
    Spg14 = 13,
    #[doc = "14: SPG responder select for loopback."]
    Spg15 = 14,
    #[doc = "15: SPG responder select for loopback."]
    Spg16 = 15,
}
impl From<Peripheral> for u8 {
    #[inline(always)]
    fn from(variant: Peripheral) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peripheral {
    type Ux = u8;
}
impl crate::IsEnum for Peripheral {}
#[doc = "Field `PERIPHERAL` reader - Select SPG peripheral for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
pub type PeripheralR = crate::FieldReader<Peripheral>;
impl PeripheralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peripheral {
        match self.bits {
            0 => Peripheral::Spg1,
            1 => Peripheral::Spg2,
            2 => Peripheral::Spg3,
            3 => Peripheral::Spg4,
            4 => Peripheral::Spg5,
            5 => Peripheral::Spg6,
            6 => Peripheral::Spg7,
            7 => Peripheral::Spg8,
            8 => Peripheral::Spg9,
            9 => Peripheral::Spg10,
            10 => Peripheral::Spg11,
            11 => Peripheral::Spg12,
            12 => Peripheral::Spg13,
            13 => Peripheral::Spg14,
            14 => Peripheral::Spg15,
            15 => Peripheral::Spg16,
            _ => unreachable!(),
        }
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg1(&self) -> bool {
        *self == Peripheral::Spg1
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg2(&self) -> bool {
        *self == Peripheral::Spg2
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg3(&self) -> bool {
        *self == Peripheral::Spg3
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg4(&self) -> bool {
        *self == Peripheral::Spg4
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg5(&self) -> bool {
        *self == Peripheral::Spg5
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg6(&self) -> bool {
        *self == Peripheral::Spg6
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg7(&self) -> bool {
        *self == Peripheral::Spg7
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg8(&self) -> bool {
        *self == Peripheral::Spg8
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg9(&self) -> bool {
        *self == Peripheral::Spg9
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg10(&self) -> bool {
        *self == Peripheral::Spg10
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg11(&self) -> bool {
        *self == Peripheral::Spg11
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg12(&self) -> bool {
        *self == Peripheral::Spg12
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg13(&self) -> bool {
        *self == Peripheral::Spg13
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg14(&self) -> bool {
        *self == Peripheral::Spg14
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg15(&self) -> bool {
        *self == Peripheral::Spg15
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg16(&self) -> bool {
        *self == Peripheral::Spg16
    }
}
#[doc = "Field `PERIPHERAL` writer - Select SPG peripheral for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
pub type PeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 4, Peripheral, crate::Safe>;
impl<'a, REG> PeripheralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg1(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg1)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg2(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg2)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg3(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg3)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg4(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg4)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg5(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg5)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg6(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg6)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg7(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg7)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg8(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg8)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg9(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg9)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg10(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg10)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg11(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg11)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg12(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg12)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg13(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg13)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg14(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg14)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg15(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg15)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg16(self) -> &'a mut crate::W<REG> {
        self.variant(Peripheral::Spg16)
    }
}
impl R {
    #[doc = "Bit 0 - Loopback enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Select SPG controller for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
    #[inline(always)]
    pub fn controller(&self) -> ControllerR {
        ControllerR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select SPG peripheral for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
    #[inline(always)]
    pub fn peripheral(&self) -> PeripheralR {
        PeripheralR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPBK0")
            .field("controller", &self.controller())
            .field("en", &self.en())
            .field("peripheral", &self.peripheral())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Loopback enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Lpbk0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select SPG controller for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
    #[inline(always)]
    pub fn controller(&mut self) -> ControllerW<'_, Lpbk0Spec> {
        ControllerW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Select SPG peripheral for loopback. Signals invovled in loopback based on interface chosen 1. SPI : CS, SCLK, SDI, SDO 2. I2C : SCL, SDA 3. UART : TX, RX"]
    #[inline(always)]
    pub fn peripheral(&mut self) -> PeripheralW<'_, Lpbk0Spec> {
        PeripheralW::new(self, 12)
    }
}
#[doc = "Loopback between SPG instances User has to ensure that the SPGs looped back have the same underlying IP instantiated; i.e. both UART, both SPI, both I2C, etc.,\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbk0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbk0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpbk0Spec;
impl crate::RegisterSpec for Lpbk0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbk0::R`](R) reader structure"]
impl crate::Readable for Lpbk0Spec {}
#[doc = "`write(|w| ..)` method takes [`lpbk0::W`](W) writer structure"]
impl crate::Writable for Lpbk0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LPBK0 to value 0"]
impl crate::Resettable for Lpbk0Spec {}
