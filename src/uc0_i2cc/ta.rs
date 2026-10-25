#[doc = "Register `TA` reader"]
pub type R = crate::R<TaSpec>;
#[doc = "Register `TA` writer"]
pub type W = crate::W<TaSpec>;
#[doc = "Receive/Send The DIR bit specifies if the next operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: in transmit mode."]
    Transmit = 0,
    #[doc = "1: is in receive mode."]
    Receive = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Receive/Send The DIR bit specifies if the next operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Transmit,
            true => Dir::Receive,
        }
    }
    #[doc = "in transmit mode."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == Dir::Transmit
    }
    #[doc = "is in receive mode."]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Dir::Receive
    }
}
#[doc = "Field `DIR` writer - Receive/Send The DIR bit specifies if the next operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "in transmit mode."]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Transmit)
    }
    #[doc = "is in receive mode."]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Receive)
    }
}
#[doc = "Field `ADDR` reader - Target Address"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Target Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Addressing Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: 7-bit addressing mode"]
    Mode7 = 0,
    #[doc = "1: 10-bit addressing mode"]
    Mode10 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Addressing Mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode7,
            true => Mode::Mode10,
        }
    }
    #[doc = "7-bit addressing mode"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Mode::Mode7
    }
    #[doc = "10-bit addressing mode"]
    #[inline(always)]
    pub fn is_mode10(&self) -> bool {
        *self == Mode::Mode10
    }
}
#[doc = "Field `MODE` writer - Addressing Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit addressing mode"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode7)
    }
    #[doc = "10-bit addressing mode"]
    #[inline(always)]
    pub fn mode10(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode10)
    }
}
impl R {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Target Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Addressing Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TA")
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, TaSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Target Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, TaSpec> {
        AddrW::new(self, 1)
    }
    #[doc = "Bit 15 - Addressing Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, TaSpec> {
        ModeW::new(self, 15)
    }
}
#[doc = "Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaSpec;
impl crate::RegisterSpec for TaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ta::R`](R) reader structure"]
impl crate::Readable for TaSpec {}
#[doc = "`write(|w| ..)` method takes [`ta::W`](W) writer structure"]
impl crate::Writable for TaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TA to value 0"]
impl crate::Resettable for TaSpec {}
