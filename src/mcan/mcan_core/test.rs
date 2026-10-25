#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbck {
    #[doc = "0: Reset value, Loop Back Mode is disabled"]
    Disable = 0,
    #[doc = "1: Loop Back Mode is enabled"]
    Enable = 1,
}
impl From<Lbck> for bool {
    #[inline(always)]
    fn from(variant: Lbck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCK` reader - Loop Back Mode"]
pub type LbckR = crate::BitReader<Lbck>;
impl LbckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbck {
        match self.bits {
            false => Lbck::Disable,
            true => Lbck::Enable,
        }
    }
    #[doc = "Reset value, Loop Back Mode is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbck::Disable
    }
    #[doc = "Loop Back Mode is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbck::Enable
    }
}
#[doc = "Field `LBCK` writer - Loop Back Mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG, Lbck>;
impl<'a, REG> LbckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset value, Loop Back Mode is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbck::Disable)
    }
    #[doc = "Loop Back Mode is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbck::Enable)
    }
}
#[doc = "Field `TX` reader - Control of Transmit Pin"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - Control of Transmit Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Receive Pin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: The CAN bus is dominant (CAN RX pin = '0')"]
    Dominant = 0,
    #[doc = "1: The CAN bus is recessive (CAN RX pin = '1')"]
    Recessive = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Receive Pin"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::Dominant,
            true => Rx::Recessive,
        }
    }
    #[doc = "The CAN bus is dominant (CAN RX pin = '0')"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == Rx::Dominant
    }
    #[doc = "The CAN bus is recessive (CAN RX pin = '1')"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == Rx::Recessive
    }
}
impl R {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<'_, TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, TestSpec> {
        TxW::new(self, 5)
    }
}
#[doc = "MCAN Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
