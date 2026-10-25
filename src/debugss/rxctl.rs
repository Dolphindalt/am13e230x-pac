#[doc = "Register `RXCTL` reader"]
pub type R = crate::R<RxctlSpec>;
#[doc = "Register `RXCTL` writer"]
pub type W = crate::W<RxctlSpec>;
#[doc = "Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive {
    #[doc = "0: RXD empty"]
    Empty = 0,
    #[doc = "1: RXD full"]
    Full = 1,
}
impl From<Receive> for bool {
    #[inline(always)]
    fn from(variant: Receive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE` reader - Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field."]
pub type ReceiveR = crate::BitReader<Receive>;
impl ReceiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive {
        match self.bits {
            false => Receive::Empty,
            true => Receive::Full,
        }
    }
    #[doc = "RXD empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Receive::Empty
    }
    #[doc = "RXD full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Receive::Full
    }
}
#[doc = "Field `RECEIVE_FLAGS` reader - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
pub type ReceiveFlagsR = crate::FieldReader;
#[doc = "Field `RECEIVE_FLAGS` writer - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
pub type ReceiveFlagsW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field."]
    #[inline(always)]
    pub fn receive(&self) -> ReceiveR {
        ReceiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn receive_flags(&self) -> ReceiveFlagsR {
        ReceiveFlagsR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCTL")
            .field("receive", &self.receive())
            .field("receive_flags", &self.receive_flags())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:7 - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn receive_flags(&mut self) -> ReceiveFlagsW<'_, RxctlSpec> {
        ReceiveFlagsW::new(self, 1)
    }
}
#[doc = "Receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctlSpec;
impl crate::RegisterSpec for RxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctl::R`](R) reader structure"]
impl crate::Readable for RxctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxctl::W`](W) writer structure"]
impl crate::Writable for RxctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RXCTL to value 0"]
impl crate::Resettable for RxctlSpec {}
