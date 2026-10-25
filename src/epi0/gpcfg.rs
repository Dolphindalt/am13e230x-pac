#[doc = "Register `GPCFG` reader"]
pub type R = crate::R<GpcfgSpec>;
#[doc = "Register `GPCFG` writer"]
pub type W = crate::W<GpcfgSpec>;
#[doc = "Size of Data Bus\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsize {
    #[doc = "0: 8 bits wide (EPI0S0 to EPI0S7)"]
    GpcfgBusSize8 = 0,
    #[doc = "1: 16 bits wide (EPI0S0 to EPI0S15)"]
    GpcfgBusSize16 = 1,
    #[doc = "2: 24 bits wide (EPI0S0 to EPI0S23)"]
    GpcfgBusSize24 = 2,
    #[doc = "3: 32 bits wide (EPI0S0 to EPI0S31)This size may not be used with an EPI clock. This value is normally used for acquisition input and actuator control as well as other general-purpose uses that require 32 bits per direction."]
    GpcfgBusSize32 = 3,
}
impl From<Dsize> for u8 {
    #[inline(always)]
    fn from(variant: Dsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsize {
    type Ux = u8;
}
impl crate::IsEnum for Dsize {}
#[doc = "Field `DSIZE` reader - Size of Data Bus"]
pub type DsizeR = crate::FieldReader<Dsize>;
impl DsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsize {
        match self.bits {
            0 => Dsize::GpcfgBusSize8,
            1 => Dsize::GpcfgBusSize16,
            2 => Dsize::GpcfgBusSize24,
            3 => Dsize::GpcfgBusSize32,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bits wide (EPI0S0 to EPI0S7)"]
    #[inline(always)]
    pub fn is_gpcfg_bus_size_8(&self) -> bool {
        *self == Dsize::GpcfgBusSize8
    }
    #[doc = "16 bits wide (EPI0S0 to EPI0S15)"]
    #[inline(always)]
    pub fn is_gpcfg_bus_size_16(&self) -> bool {
        *self == Dsize::GpcfgBusSize16
    }
    #[doc = "24 bits wide (EPI0S0 to EPI0S23)"]
    #[inline(always)]
    pub fn is_gpcfg_bus_size_24(&self) -> bool {
        *self == Dsize::GpcfgBusSize24
    }
    #[doc = "32 bits wide (EPI0S0 to EPI0S31)This size may not be used with an EPI clock. This value is normally used for acquisition input and actuator control as well as other general-purpose uses that require 32 bits per direction."]
    #[inline(always)]
    pub fn is_gpcfg_bus_size_32(&self) -> bool {
        *self == Dsize::GpcfgBusSize32
    }
}
#[doc = "Field `DSIZE` writer - Size of Data Bus"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsize, crate::Safe>;
impl<'a, REG> DsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits wide (EPI0S0 to EPI0S7)"]
    #[inline(always)]
    pub fn gpcfg_bus_size_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::GpcfgBusSize8)
    }
    #[doc = "16 bits wide (EPI0S0 to EPI0S15)"]
    #[inline(always)]
    pub fn gpcfg_bus_size_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::GpcfgBusSize16)
    }
    #[doc = "24 bits wide (EPI0S0 to EPI0S23)"]
    #[inline(always)]
    pub fn gpcfg_bus_size_24(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::GpcfgBusSize24)
    }
    #[doc = "32 bits wide (EPI0S0 to EPI0S31)This size may not be used with an EPI clock. This value is normally used for acquisition input and actuator control as well as other general-purpose uses that require 32 bits per direction."]
    #[inline(always)]
    pub fn gpcfg_bus_size_32(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::GpcfgBusSize32)
    }
}
#[doc = "Address Bus Size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Asize {
    #[doc = "0: No address"]
    GpcfgAddrSizeNone = 0,
    #[doc = "1: Up to 4 bits wide."]
    GpcfgAddrSize4 = 1,
    #[doc = "2: Up to 12 bits wide. This size cannot be used with 24-bit data."]
    GpcfgAddrSize12 = 2,
    #[doc = "3: Up to 20 bits wide. This size cannot be used with data sizes other than 8."]
    GpcfgAddrSize20 = 3,
}
impl From<Asize> for u8 {
    #[inline(always)]
    fn from(variant: Asize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Asize {
    type Ux = u8;
}
impl crate::IsEnum for Asize {}
#[doc = "Field `ASIZE` reader - Address Bus Size"]
pub type AsizeR = crate::FieldReader<Asize>;
impl AsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asize {
        match self.bits {
            0 => Asize::GpcfgAddrSizeNone,
            1 => Asize::GpcfgAddrSize4,
            2 => Asize::GpcfgAddrSize12,
            3 => Asize::GpcfgAddrSize20,
            _ => unreachable!(),
        }
    }
    #[doc = "No address"]
    #[inline(always)]
    pub fn is_gpcfg_addr_size_none(&self) -> bool {
        *self == Asize::GpcfgAddrSizeNone
    }
    #[doc = "Up to 4 bits wide."]
    #[inline(always)]
    pub fn is_gpcfg_addr_size_4(&self) -> bool {
        *self == Asize::GpcfgAddrSize4
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data."]
    #[inline(always)]
    pub fn is_gpcfg_addr_size_12(&self) -> bool {
        *self == Asize::GpcfgAddrSize12
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8."]
    #[inline(always)]
    pub fn is_gpcfg_addr_size_20(&self) -> bool {
        *self == Asize::GpcfgAddrSize20
    }
}
#[doc = "Field `ASIZE` writer - Address Bus Size"]
pub type AsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Asize, crate::Safe>;
impl<'a, REG> AsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No address"]
    #[inline(always)]
    pub fn gpcfg_addr_size_none(self) -> &'a mut crate::W<REG> {
        self.variant(Asize::GpcfgAddrSizeNone)
    }
    #[doc = "Up to 4 bits wide."]
    #[inline(always)]
    pub fn gpcfg_addr_size_4(self) -> &'a mut crate::W<REG> {
        self.variant(Asize::GpcfgAddrSize4)
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data."]
    #[inline(always)]
    pub fn gpcfg_addr_size_12(self) -> &'a mut crate::W<REG> {
        self.variant(Asize::GpcfgAddrSize12)
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8."]
    #[inline(always)]
    pub fn gpcfg_addr_size_20(self) -> &'a mut crate::W<REG> {
        self.variant(Asize::GpcfgAddrSize20)
    }
}
#[doc = "2-Cycle Reads\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rd2cyc {
    #[doc = "0: Data is captured on the EPI clock cycle with READ strobe asserted."]
    One = 0,
    #[doc = "1: Reads are two EPI clock cycles, with address on one EPI clock cycle (with the RD strobe asserted) and data captured on the following EPI clock cycle (with the RD strobe de-asserted). The next address (if any) is in the cycle following."]
    Two = 1,
}
impl From<Rd2cyc> for bool {
    #[inline(always)]
    fn from(variant: Rd2cyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD2CYC` reader - 2-Cycle Reads"]
pub type Rd2cycR = crate::BitReader<Rd2cyc>;
impl Rd2cycR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rd2cyc {
        match self.bits {
            false => Rd2cyc::One,
            true => Rd2cyc::Two,
        }
    }
    #[doc = "Data is captured on the EPI clock cycle with READ strobe asserted."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Rd2cyc::One
    }
    #[doc = "Reads are two EPI clock cycles, with address on one EPI clock cycle (with the RD strobe asserted) and data captured on the following EPI clock cycle (with the RD strobe de-asserted). The next address (if any) is in the cycle following."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Rd2cyc::Two
    }
}
#[doc = "Field `RD2CYC` writer - 2-Cycle Reads"]
pub type Rd2cycW<'a, REG> = crate::BitWriter<'a, REG, Rd2cyc>;
impl<'a, REG> Rd2cycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is captured on the EPI clock cycle with READ strobe asserted."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Rd2cyc::One)
    }
    #[doc = "Reads are two EPI clock cycles, with address on one EPI clock cycle (with the RD strobe asserted) and data captured on the following EPI clock cycle (with the RD strobe de-asserted). The next address (if any) is in the cycle following."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Rd2cyc::Two)
    }
}
#[doc = "2-Cycle Writes\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wr2cyc {
    #[doc = "0: Data is output on the same EPI clock cycle as the address."]
    One = 0,
    #[doc = "1: Writes are two EPI clock cycles long, with address on one EPI clock cycle (with the WR strobe asserted) and data written on the following EPI clock cycle (with WR strobe de-asserted). The next address (if any) is in the cycle following."]
    Two = 1,
}
impl From<Wr2cyc> for bool {
    #[inline(always)]
    fn from(variant: Wr2cyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR2CYC` reader - 2-Cycle Writes"]
pub type Wr2cycR = crate::BitReader<Wr2cyc>;
impl Wr2cycR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wr2cyc {
        match self.bits {
            false => Wr2cyc::One,
            true => Wr2cyc::Two,
        }
    }
    #[doc = "Data is output on the same EPI clock cycle as the address."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Wr2cyc::One
    }
    #[doc = "Writes are two EPI clock cycles long, with address on one EPI clock cycle (with the WR strobe asserted) and data written on the following EPI clock cycle (with WR strobe de-asserted). The next address (if any) is in the cycle following."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Wr2cyc::Two
    }
}
#[doc = "Field `WR2CYC` writer - 2-Cycle Writes"]
pub type Wr2cycW<'a, REG> = crate::BitWriter<'a, REG, Wr2cyc>;
impl<'a, REG> Wr2cycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is output on the same EPI clock cycle as the address."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Wr2cyc::One)
    }
    #[doc = "Writes are two EPI clock cycles long, with address on one EPI clock cycle (with the WR strobe asserted) and data written on the following EPI clock cycle (with WR strobe de-asserted). The next address (if any) is in the cycle following."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Wr2cyc::Two)
    }
}
#[doc = "Field `FRMCNT` reader - Frame Count"]
pub type FrmcntR = crate::FieldReader;
#[doc = "Field `FRMCNT` writer - Frame Count"]
pub type FrmcntW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "50/50 Frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frm50 {
    #[doc = "0: The FRAME signal is output as a single pulse, and then held low for the count."]
    GpcfgFramePulse = 0,
    #[doc = "1: The FRAME signal is output as 50/50 duty cycle using count (see FRMCNT)."]
    GpcfgFrame50_50 = 1,
}
impl From<Frm50> for bool {
    #[inline(always)]
    fn from(variant: Frm50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRM50` reader - 50/50 Frame"]
pub type Frm50R = crate::BitReader<Frm50>;
impl Frm50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frm50 {
        match self.bits {
            false => Frm50::GpcfgFramePulse,
            true => Frm50::GpcfgFrame50_50,
        }
    }
    #[doc = "The FRAME signal is output as a single pulse, and then held low for the count."]
    #[inline(always)]
    pub fn is_gpcfg_frame_pulse(&self) -> bool {
        *self == Frm50::GpcfgFramePulse
    }
    #[doc = "The FRAME signal is output as 50/50 duty cycle using count (see FRMCNT)."]
    #[inline(always)]
    pub fn is_gpcfg_frame_50_50(&self) -> bool {
        *self == Frm50::GpcfgFrame50_50
    }
}
#[doc = "Field `FRM50` writer - 50/50 Frame"]
pub type Frm50W<'a, REG> = crate::BitWriter<'a, REG, Frm50>;
impl<'a, REG> Frm50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The FRAME signal is output as a single pulse, and then held low for the count."]
    #[inline(always)]
    pub fn gpcfg_frame_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Frm50::GpcfgFramePulse)
    }
    #[doc = "The FRAME signal is output as 50/50 duty cycle using count (see FRMCNT)."]
    #[inline(always)]
    pub fn gpcfg_frame_50_50(self) -> &'a mut crate::W<REG> {
        self.variant(Frm50::GpcfgFrame50_50)
    }
}
#[doc = "Ready Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdyen {
    #[doc = "0: The external peripheral does not drive an iRDY signal and is assumed to be ready always."]
    NotUsed = 0,
    #[doc = "1: The external peripheral drives an iRDY signal into pin EPI0S27."]
    Enable = 1,
}
impl From<Rdyen> for bool {
    #[inline(always)]
    fn from(variant: Rdyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDYEN` reader - Ready Enable"]
pub type RdyenR = crate::BitReader<Rdyen>;
impl RdyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdyen {
        match self.bits {
            false => Rdyen::NotUsed,
            true => Rdyen::Enable,
        }
    }
    #[doc = "The external peripheral does not drive an iRDY signal and is assumed to be ready always."]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Rdyen::NotUsed
    }
    #[doc = "The external peripheral drives an iRDY signal into pin EPI0S27."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rdyen::Enable
    }
}
#[doc = "Field `RDYEN` writer - Ready Enable"]
pub type RdyenW<'a, REG> = crate::BitWriter<'a, REG, Rdyen>;
impl<'a, REG> RdyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The external peripheral does not drive an iRDY signal and is assumed to be ready always."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyen::NotUsed)
    }
    #[doc = "The external peripheral drives an iRDY signal into pin EPI0S27."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyen::Enable)
    }
}
#[doc = "Clock Gated\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgate {
    #[doc = "0: The EPI clock is free running."]
    FreeRun = 0,
    #[doc = "1: The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    ForceLow = 1,
}
impl From<Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGATE` reader - Clock Gated"]
pub type ClkgateR = crate::BitReader<Clkgate>;
impl ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgate {
        match self.bits {
            false => Clkgate::FreeRun,
            true => Clkgate::ForceLow,
        }
    }
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == Clkgate::FreeRun
    }
    #[doc = "The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Clkgate::ForceLow
    }
}
#[doc = "Field `CLKGATE` writer - Clock Gated"]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Clkgate>;
impl<'a, REG> ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgate::FreeRun)
    }
    #[doc = "The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgate::ForceLow)
    }
}
#[doc = "Clock Pin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpin {
    #[doc = "0: No clock output."]
    Disable = 0,
    #[doc = "1: EPI0S31 functions as the EPI clock output."]
    Enable = 1,
}
impl From<Clkpin> for bool {
    #[inline(always)]
    fn from(variant: Clkpin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPIN` reader - Clock Pin"]
pub type ClkpinR = crate::BitReader<Clkpin>;
impl ClkpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpin {
        match self.bits {
            false => Clkpin::Disable,
            true => Clkpin::Enable,
        }
    }
    #[doc = "No clock output."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clkpin::Disable
    }
    #[doc = "EPI0S31 functions as the EPI clock output."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clkpin::Enable
    }
}
#[doc = "Field `CLKPIN` writer - Clock Pin"]
pub type ClkpinW<'a, REG> = crate::BitWriter<'a, REG, Clkpin>;
impl<'a, REG> ClkpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clock output."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpin::Disable)
    }
    #[doc = "EPI0S31 functions as the EPI clock output."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpin::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn asize(&self) -> AsizeR {
        AsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 18 - 2-Cycle Reads"]
    #[inline(always)]
    pub fn rd2cyc(&self) -> Rd2cycR {
        Rd2cycR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn wr2cyc(&self) -> Wr2cycR {
        Wr2cycR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FrmcntR {
        FrmcntR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn frm50(&self) -> Frm50R {
        Frm50R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&self) -> RdyenR {
        RdyenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn clkpin(&self) -> ClkpinR {
        ClkpinR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPCFG")
            .field("dsize", &self.dsize())
            .field("asize", &self.asize())
            .field("rd2cyc", &self.rd2cyc())
            .field("wr2cyc", &self.wr2cyc())
            .field("frmcnt", &self.frmcnt())
            .field("frm50", &self.frm50())
            .field("rdyen", &self.rdyen())
            .field("clkgate", &self.clkgate())
            .field("clkpin", &self.clkpin())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DsizeW<'_, GpcfgSpec> {
        DsizeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn asize(&mut self) -> AsizeW<'_, GpcfgSpec> {
        AsizeW::new(self, 4)
    }
    #[doc = "Bit 18 - 2-Cycle Reads"]
    #[inline(always)]
    pub fn rd2cyc(&mut self) -> Rd2cycW<'_, GpcfgSpec> {
        Rd2cycW::new(self, 18)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn wr2cyc(&mut self) -> Wr2cycW<'_, GpcfgSpec> {
        Wr2cycW::new(self, 19)
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FrmcntW<'_, GpcfgSpec> {
        FrmcntW::new(self, 22)
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn frm50(&mut self) -> Frm50W<'_, GpcfgSpec> {
        Frm50W::new(self, 26)
    }
    #[doc = "Bit 28 - Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&mut self) -> RdyenW<'_, GpcfgSpec> {
        RdyenW::new(self, 28)
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> ClkgateW<'_, GpcfgSpec> {
        ClkgateW::new(self, 30)
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn clkpin(&mut self) -> ClkpinW<'_, GpcfgSpec> {
        ClkpinW::new(self, 31)
    }
}
#[doc = "EPI General-Purpose Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpcfgSpec;
impl crate::RegisterSpec for GpcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpcfg::R`](R) reader structure"]
impl crate::Readable for GpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gpcfg::W`](W) writer structure"]
impl crate::Writable for GpcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GPCFG to value 0"]
impl crate::Resettable for GpcfgSpec {}
