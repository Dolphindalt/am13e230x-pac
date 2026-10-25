#[doc = "Register `QCAPCTL` reader"]
pub type R = crate::R<QcapctlSpec>;
#[doc = "Register `QCAPCTL` writer"]
pub type W = crate::W<QcapctlSpec>;
#[doc = "Unit position event prescaler\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upps {
    #[doc = "0: UPEVNT = QCLK/1"]
    Qclk1 = 0,
    #[doc = "1: UPEVNT = QCLK/2"]
    Qclk2 = 1,
    #[doc = "2: UPEVNT = QCLK/4"]
    Qclk4 = 2,
    #[doc = "3: UPEVNT = QCLK/8"]
    Qclk8 = 3,
    #[doc = "4: UPEVNT = QCLK/16"]
    Qclk16 = 4,
    #[doc = "5: UPEVNT = QCLK/32"]
    Qclk32 = 5,
    #[doc = "6: UPEVNT = QCLK/64"]
    Qclk64 = 6,
    #[doc = "7: UPEVNT = QCLK/128"]
    Qclk128 = 7,
    #[doc = "8: UPEVNT = QCLK/256"]
    Qclk256 = 8,
    #[doc = "9: UPEVNT = QCLK/512"]
    Qclk512 = 9,
    #[doc = "10: UPEVNT = QCLK/1024"]
    Qclk1024 = 10,
    #[doc = "11: UPEVNT = QCLK/2048"]
    Qclk2048 = 11,
    #[doc = "12: Reserved"]
    QclkRsvd0 = 12,
    #[doc = "13: Reserved"]
    QclkRsvd1 = 13,
    #[doc = "14: Reserved"]
    QclkRsvd2 = 14,
    #[doc = "15: Reserved"]
    QclkRsvd3 = 15,
}
impl From<Upps> for u8 {
    #[inline(always)]
    fn from(variant: Upps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upps {
    type Ux = u8;
}
impl crate::IsEnum for Upps {}
#[doc = "Field `UPPS` reader - Unit position event prescaler"]
pub type UppsR = crate::FieldReader<Upps>;
impl UppsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upps {
        match self.bits {
            0 => Upps::Qclk1,
            1 => Upps::Qclk2,
            2 => Upps::Qclk4,
            3 => Upps::Qclk8,
            4 => Upps::Qclk16,
            5 => Upps::Qclk32,
            6 => Upps::Qclk64,
            7 => Upps::Qclk128,
            8 => Upps::Qclk256,
            9 => Upps::Qclk512,
            10 => Upps::Qclk1024,
            11 => Upps::Qclk2048,
            12 => Upps::QclkRsvd0,
            13 => Upps::QclkRsvd1,
            14 => Upps::QclkRsvd2,
            15 => Upps::QclkRsvd3,
            _ => unreachable!(),
        }
    }
    #[doc = "UPEVNT = QCLK/1"]
    #[inline(always)]
    pub fn is_qclk1(&self) -> bool {
        *self == Upps::Qclk1
    }
    #[doc = "UPEVNT = QCLK/2"]
    #[inline(always)]
    pub fn is_qclk2(&self) -> bool {
        *self == Upps::Qclk2
    }
    #[doc = "UPEVNT = QCLK/4"]
    #[inline(always)]
    pub fn is_qclk4(&self) -> bool {
        *self == Upps::Qclk4
    }
    #[doc = "UPEVNT = QCLK/8"]
    #[inline(always)]
    pub fn is_qclk8(&self) -> bool {
        *self == Upps::Qclk8
    }
    #[doc = "UPEVNT = QCLK/16"]
    #[inline(always)]
    pub fn is_qclk16(&self) -> bool {
        *self == Upps::Qclk16
    }
    #[doc = "UPEVNT = QCLK/32"]
    #[inline(always)]
    pub fn is_qclk32(&self) -> bool {
        *self == Upps::Qclk32
    }
    #[doc = "UPEVNT = QCLK/64"]
    #[inline(always)]
    pub fn is_qclk64(&self) -> bool {
        *self == Upps::Qclk64
    }
    #[doc = "UPEVNT = QCLK/128"]
    #[inline(always)]
    pub fn is_qclk128(&self) -> bool {
        *self == Upps::Qclk128
    }
    #[doc = "UPEVNT = QCLK/256"]
    #[inline(always)]
    pub fn is_qclk256(&self) -> bool {
        *self == Upps::Qclk256
    }
    #[doc = "UPEVNT = QCLK/512"]
    #[inline(always)]
    pub fn is_qclk512(&self) -> bool {
        *self == Upps::Qclk512
    }
    #[doc = "UPEVNT = QCLK/1024"]
    #[inline(always)]
    pub fn is_qclk1024(&self) -> bool {
        *self == Upps::Qclk1024
    }
    #[doc = "UPEVNT = QCLK/2048"]
    #[inline(always)]
    pub fn is_qclk2048(&self) -> bool {
        *self == Upps::Qclk2048
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_qclk_rsvd0(&self) -> bool {
        *self == Upps::QclkRsvd0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_qclk_rsvd1(&self) -> bool {
        *self == Upps::QclkRsvd1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_qclk_rsvd2(&self) -> bool {
        *self == Upps::QclkRsvd2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_qclk_rsvd3(&self) -> bool {
        *self == Upps::QclkRsvd3
    }
}
#[doc = "Field `UPPS` writer - Unit position event prescaler"]
pub type UppsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Upps, crate::Safe>;
impl<'a, REG> UppsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UPEVNT = QCLK/1"]
    #[inline(always)]
    pub fn qclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk1)
    }
    #[doc = "UPEVNT = QCLK/2"]
    #[inline(always)]
    pub fn qclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk2)
    }
    #[doc = "UPEVNT = QCLK/4"]
    #[inline(always)]
    pub fn qclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk4)
    }
    #[doc = "UPEVNT = QCLK/8"]
    #[inline(always)]
    pub fn qclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk8)
    }
    #[doc = "UPEVNT = QCLK/16"]
    #[inline(always)]
    pub fn qclk16(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk16)
    }
    #[doc = "UPEVNT = QCLK/32"]
    #[inline(always)]
    pub fn qclk32(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk32)
    }
    #[doc = "UPEVNT = QCLK/64"]
    #[inline(always)]
    pub fn qclk64(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk64)
    }
    #[doc = "UPEVNT = QCLK/128"]
    #[inline(always)]
    pub fn qclk128(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk128)
    }
    #[doc = "UPEVNT = QCLK/256"]
    #[inline(always)]
    pub fn qclk256(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk256)
    }
    #[doc = "UPEVNT = QCLK/512"]
    #[inline(always)]
    pub fn qclk512(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk512)
    }
    #[doc = "UPEVNT = QCLK/1024"]
    #[inline(always)]
    pub fn qclk1024(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk1024)
    }
    #[doc = "UPEVNT = QCLK/2048"]
    #[inline(always)]
    pub fn qclk2048(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Qclk2048)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn qclk_rsvd0(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::QclkRsvd0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn qclk_rsvd1(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::QclkRsvd1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn qclk_rsvd2(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::QclkRsvd2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn qclk_rsvd3(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::QclkRsvd3)
    }
}
#[doc = "eQEP capture timer clock prescaler\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccps {
    #[doc = "0: CAPCLK = SYSCLKOUT/1"]
    Sysclkout1 = 0,
    #[doc = "1: CAPCLK = SYSCLKOUT/2"]
    Sysclkout2 = 1,
    #[doc = "2: CAPCLK = SYSCLKOUT/4"]
    Sysclkout4 = 2,
    #[doc = "3: CAPCLK = SYSCLKOUT/8"]
    Sysclkout8 = 3,
    #[doc = "4: CAPCLK = SYSCLKOUT/16"]
    Sysclkout16 = 4,
    #[doc = "5: CAPCLK = SYSCLKOUT/32"]
    Sysclkout32 = 5,
    #[doc = "6: CAPCLK = SYSCLKOUT/64"]
    Sysclkout64 = 6,
    #[doc = "7: CAPCLK = SYSCLKOUT/128"]
    Sysclkout128 = 7,
}
impl From<Ccps> for u8 {
    #[inline(always)]
    fn from(variant: Ccps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccps {
    type Ux = u8;
}
impl crate::IsEnum for Ccps {}
#[doc = "Field `CCPS` reader - eQEP capture timer clock prescaler"]
pub type CcpsR = crate::FieldReader<Ccps>;
impl CcpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccps {
        match self.bits {
            0 => Ccps::Sysclkout1,
            1 => Ccps::Sysclkout2,
            2 => Ccps::Sysclkout4,
            3 => Ccps::Sysclkout8,
            4 => Ccps::Sysclkout16,
            5 => Ccps::Sysclkout32,
            6 => Ccps::Sysclkout64,
            7 => Ccps::Sysclkout128,
            _ => unreachable!(),
        }
    }
    #[doc = "CAPCLK = SYSCLKOUT/1"]
    #[inline(always)]
    pub fn is_sysclkout1(&self) -> bool {
        *self == Ccps::Sysclkout1
    }
    #[doc = "CAPCLK = SYSCLKOUT/2"]
    #[inline(always)]
    pub fn is_sysclkout2(&self) -> bool {
        *self == Ccps::Sysclkout2
    }
    #[doc = "CAPCLK = SYSCLKOUT/4"]
    #[inline(always)]
    pub fn is_sysclkout4(&self) -> bool {
        *self == Ccps::Sysclkout4
    }
    #[doc = "CAPCLK = SYSCLKOUT/8"]
    #[inline(always)]
    pub fn is_sysclkout8(&self) -> bool {
        *self == Ccps::Sysclkout8
    }
    #[doc = "CAPCLK = SYSCLKOUT/16"]
    #[inline(always)]
    pub fn is_sysclkout16(&self) -> bool {
        *self == Ccps::Sysclkout16
    }
    #[doc = "CAPCLK = SYSCLKOUT/32"]
    #[inline(always)]
    pub fn is_sysclkout32(&self) -> bool {
        *self == Ccps::Sysclkout32
    }
    #[doc = "CAPCLK = SYSCLKOUT/64"]
    #[inline(always)]
    pub fn is_sysclkout64(&self) -> bool {
        *self == Ccps::Sysclkout64
    }
    #[doc = "CAPCLK = SYSCLKOUT/128"]
    #[inline(always)]
    pub fn is_sysclkout128(&self) -> bool {
        *self == Ccps::Sysclkout128
    }
}
#[doc = "Field `CCPS` writer - eQEP capture timer clock prescaler"]
pub type CcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccps, crate::Safe>;
impl<'a, REG> CcpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAPCLK = SYSCLKOUT/1"]
    #[inline(always)]
    pub fn sysclkout1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout1)
    }
    #[doc = "CAPCLK = SYSCLKOUT/2"]
    #[inline(always)]
    pub fn sysclkout2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout2)
    }
    #[doc = "CAPCLK = SYSCLKOUT/4"]
    #[inline(always)]
    pub fn sysclkout4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout4)
    }
    #[doc = "CAPCLK = SYSCLKOUT/8"]
    #[inline(always)]
    pub fn sysclkout8(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout8)
    }
    #[doc = "CAPCLK = SYSCLKOUT/16"]
    #[inline(always)]
    pub fn sysclkout16(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout16)
    }
    #[doc = "CAPCLK = SYSCLKOUT/32"]
    #[inline(always)]
    pub fn sysclkout32(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout32)
    }
    #[doc = "CAPCLK = SYSCLKOUT/64"]
    #[inline(always)]
    pub fn sysclkout64(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout64)
    }
    #[doc = "CAPCLK = SYSCLKOUT/128"]
    #[inline(always)]
    pub fn sysclkout128(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Sysclkout128)
    }
}
#[doc = "Enable eQEP capture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen {
    #[doc = "0: eQEP capture unit is disabled"]
    Disable = 0,
    #[doc = "1: eQEP capture unit is enabled"]
    Enable = 1,
}
impl From<Cen> for bool {
    #[inline(always)]
    fn from(variant: Cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Enable eQEP capture"]
pub type CenR = crate::BitReader<Cen>;
impl CenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen {
        match self.bits {
            false => Cen::Disable,
            true => Cen::Enable,
        }
    }
    #[doc = "eQEP capture unit is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cen::Disable
    }
    #[doc = "eQEP capture unit is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cen::Enable
    }
}
#[doc = "Field `CEN` writer - Enable eQEP capture"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG, Cen>;
impl<'a, REG> CenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "eQEP capture unit is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Disable)
    }
    #[doc = "eQEP capture unit is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Unit position event prescaler"]
    #[inline(always)]
    pub fn upps(&self) -> UppsR {
        UppsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - eQEP capture timer clock prescaler"]
    #[inline(always)]
    pub fn ccps(&self) -> CcpsR {
        CcpsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable eQEP capture"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QCAPCTL")
            .field("upps", &self.upps())
            .field("ccps", &self.ccps())
            .field("cen", &self.cen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Unit position event prescaler"]
    #[inline(always)]
    pub fn upps(&mut self) -> UppsW<'_, QcapctlSpec> {
        UppsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - eQEP capture timer clock prescaler"]
    #[inline(always)]
    pub fn ccps(&mut self) -> CcpsW<'_, QcapctlSpec> {
        CcpsW::new(self, 4)
    }
    #[doc = "Bit 15 - Enable eQEP capture"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, QcapctlSpec> {
        CenW::new(self, 15)
    }
}
#[doc = "Qaudrature Capture Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qcapctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcapctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcapctlSpec;
impl crate::RegisterSpec for QcapctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qcapctl::R`](R) reader structure"]
impl crate::Readable for QcapctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qcapctl::W`](W) writer structure"]
impl crate::Writable for QcapctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QCAPCTL to value 0"]
impl crate::Resettable for QcapctlSpec {}
