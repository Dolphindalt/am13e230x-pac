#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "UART Send Break (for LIN Protocol) 1. Break condition is sent on the line for as long as this bit is set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk {
    #[doc = "0: Normal operation"]
    Disable = 0,
    #[doc = "1: A low level is continually output on the UARTxTXD signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods)."]
    Enable = 1,
}
impl From<Brk> for bool {
    #[inline(always)]
    fn from(variant: Brk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK` reader - UART Send Break (for LIN Protocol) 1. Break condition is sent on the line for as long as this bit is set"]
pub type BrkR = crate::BitReader<Brk>;
impl BrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brk {
        match self.bits {
            false => Brk::Disable,
            true => Brk::Enable,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Brk::Disable
    }
    #[doc = "A low level is continually output on the UARTxTXD signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Brk::Enable
    }
}
#[doc = "Field `BRK` writer - UART Send Break (for LIN Protocol) 1. Break condition is sent on the line for as long as this bit is set"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG, Brk>;
impl<'a, REG> BrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::Disable)
    }
    #[doc = "A low level is continually output on the UARTxTXD signal, after completing transmission of the current character. For the proper execution of the break command, software must set this bit for at least two frames (character periods)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::Enable)
    }
}
#[doc = "UART Parity Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "0: Parity is disabled and no parity bit is added to the data frame."]
    Disable = 0,
    #[doc = "1: Parity checking and generation is enabled."]
    Enable = 1,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - UART Parity Enable"]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            false => Pen::Disable,
            true => Pen::Enable,
        }
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pen::Disable
    }
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pen::Enable
    }
}
#[doc = "Field `PEN` writer - UART Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity is disabled and no parity bit is added to the data frame."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Disable)
    }
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Enable)
    }
}
#[doc = "UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eps {
    #[doc = "0: Odd parity is performed, which checks for an odd number of 1s."]
    Odd = 0,
    #[doc = "1: Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits."]
    Even = 1,
}
impl From<Eps> for bool {
    #[inline(always)]
    fn from(variant: Eps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
pub type EpsR = crate::BitReader<Eps>;
impl EpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eps {
        match self.bits {
            false => Eps::Odd,
            true => Eps::Even,
        }
    }
    #[doc = "Odd parity is performed, which checks for an odd number of 1s."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Eps::Odd
    }
    #[doc = "Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Eps::Even
    }
}
#[doc = "Field `EPS` writer - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG, Eps>;
impl<'a, REG> EpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd parity is performed, which checks for an odd number of 1s."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Odd)
    }
    #[doc = "Even parity generation and checking is performed during transmission and reception, which checks for an even number of 1s in data and parity bits."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Even)
    }
}
#[doc = "UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stp2 {
    #[doc = "0: One stop bit is transmitted at the end of a frame."]
    Disable = 0,
    #[doc = "1: Two stop bits are transmitted at the end of a frame. The receive logic checks for two stop bits being received and provide Frame Error if either is invalid."]
    Enable = 1,
}
impl From<Stp2> for bool {
    #[inline(always)]
    fn from(variant: Stp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STP2` reader - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
pub type Stp2R = crate::BitReader<Stp2>;
impl Stp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stp2 {
        match self.bits {
            false => Stp2::Disable,
            true => Stp2::Enable,
        }
    }
    #[doc = "One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stp2::Disable
    }
    #[doc = "Two stop bits are transmitted at the end of a frame. The receive logic checks for two stop bits being received and provide Frame Error if either is invalid."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stp2::Enable
    }
}
#[doc = "Field `STP2` writer - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG, Stp2>;
impl<'a, REG> Stp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit is transmitted at the end of a frame."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::Disable)
    }
    #[doc = "Two stop bits are transmitted at the end of a frame. The receive logic checks for two stop bits being received and provide Frame Error if either is invalid."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::Enable)
    }
}
#[doc = "UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "0: 5 bits (default)"]
    Databit5 = 0,
    #[doc = "1: 6 bits"]
    Databit6 = 1,
    #[doc = "2: 7 bits"]
    Databit7 = 2,
    #[doc = "3: 8 bits"]
    Databit8 = 3,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
#[doc = "Field `WLEN` reader - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wlen {
        match self.bits {
            0 => Wlen::Databit5,
            1 => Wlen::Databit6,
            2 => Wlen::Databit7,
            3 => Wlen::Databit8,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn is_databit5(&self) -> bool {
        *self == Wlen::Databit5
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_databit6(&self) -> bool {
        *self == Wlen::Databit6
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_databit7(&self) -> bool {
        *self == Wlen::Databit7
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_databit8(&self) -> bool {
        *self == Wlen::Databit8
    }
}
#[doc = "Field `WLEN` writer - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn databit5(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Databit5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn databit6(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Databit6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn databit7(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Databit7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn databit8(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Databit8)
    }
}
#[doc = "UART Stick Parity Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sps {
    #[doc = "0: Disable Stick Parity"]
    Disable = 0,
    #[doc = "1: Enable Stick Parity"]
    Enable = 1,
}
impl From<Sps> for bool {
    #[inline(always)]
    fn from(variant: Sps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPS` reader - UART Stick Parity Select"]
pub type SpsR = crate::BitReader<Sps>;
impl SpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sps {
        match self.bits {
            false => Sps::Disable,
            true => Sps::Enable,
        }
    }
    #[doc = "Disable Stick Parity"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sps::Disable
    }
    #[doc = "Enable Stick Parity"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sps::Enable
    }
}
#[doc = "Field `SPS` writer - UART Stick Parity Select"]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG, Sps>;
impl<'a, REG> SpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Stick Parity"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Disable)
    }
    #[doc = "Enable Stick Parity"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Enable)
    }
}
#[doc = "UART send IDLE pattern. When this bit is set, SENDIDLE period of 11 bit times will be sent on the TX line. Read STAT.SENDIDLE bit to readback current status of SENDIDLE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sendidle {
    #[doc = "0: Normal operation"]
    Disable = 0,
    #[doc = "1: Send Idle Pattern"]
    Enable = 1,
}
impl From<Sendidle> for bool {
    #[inline(always)]
    fn from(variant: Sendidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENDIDLE` reader - UART send IDLE pattern. When this bit is set, SENDIDLE period of 11 bit times will be sent on the TX line. Read STAT.SENDIDLE bit to readback current status of SENDIDLE"]
pub type SendidleR = crate::BitReader<Sendidle>;
impl SendidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sendidle {
        match self.bits {
            false => Sendidle::Disable,
            true => Sendidle::Enable,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sendidle::Disable
    }
    #[doc = "Send Idle Pattern"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sendidle::Enable
    }
}
#[doc = "Field `SENDIDLE` writer - UART send IDLE pattern. When this bit is set, SENDIDLE period of 11 bit times will be sent on the TX line. Read STAT.SENDIDLE bit to readback current status of SENDIDLE"]
pub type SendidleW<'a, REG> = crate::BitWriter<'a, REG, Sendidle>;
impl<'a, REG> SendidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sendidle::Disable)
    }
    #[doc = "Send Idle Pattern"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sendidle::Enable)
    }
}
#[doc = "Field `EXTDIR_SETUP` reader - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
pub type ExtdirSetupR = crate::FieldReader;
#[doc = "Field `EXTDIR_SETUP` writer - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
pub type ExtdirSetupW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `EXTDIR_HOLD` reader - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
pub type ExtdirHoldR = crate::FieldReader;
#[doc = "Field `EXTDIR_HOLD` writer - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
pub type ExtdirHoldW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Suspend external communication\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend {
    #[doc = "0: Functional mode resumed"]
    Disable = 0,
    #[doc = "1: External communication suspended"]
    Enable = 1,
}
impl From<Suspend> for bool {
    #[inline(always)]
    fn from(variant: Suspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND` reader - Suspend external communication"]
pub type SuspendR = crate::BitReader<Suspend>;
impl SuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend {
        match self.bits {
            false => Suspend::Disable,
            true => Suspend::Enable,
        }
    }
    #[doc = "Functional mode resumed"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Suspend::Disable
    }
    #[doc = "External communication suspended"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Suspend::Enable
    }
}
#[doc = "Field `SUSPEND` writer - Suspend external communication"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG, Suspend>;
impl<'a, REG> SuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Functional mode resumed"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::Disable)
    }
    #[doc = "External communication suspended"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - UART Send Break (for LIN Protocol) 1. Break condition is sent on the line for as long as this bit is set"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART send IDLE pattern. When this bit is set, SENDIDLE period of 11 bit times will be sent on the TX line. Read STAT.SENDIDLE bit to readback current status of SENDIDLE"]
    #[inline(always)]
    pub fn sendidle(&self) -> SendidleR {
        SendidleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
    #[inline(always)]
    pub fn extdir_setup(&self) -> ExtdirSetupR {
        ExtdirSetupR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
    #[inline(always)]
    pub fn extdir_hold(&self) -> ExtdirHoldR {
        ExtdirHoldR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCRH")
            .field("brk", &self.brk())
            .field("pen", &self.pen())
            .field("eps", &self.eps())
            .field("stp2", &self.stp2())
            .field("wlen", &self.wlen())
            .field("sps", &self.sps())
            .field("sendidle", &self.sendidle())
            .field("extdir_setup", &self.extdir_setup())
            .field("extdir_hold", &self.extdir_hold())
            .field("suspend", &self.suspend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - UART Send Break (for LIN Protocol) 1. Break condition is sent on the line for as long as this bit is set"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, LcrhSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrhSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - UART Even Parity Select This bit has no effect when parity is disabled by the PEN bit. For 9-Bit UART Mode transmissions, this bit controls the address byte and data byte indication (9th bit). 0 = The transferred byte is a data byte 1 = The transferred byte is an address byte"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrhSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select When in 7816 smart card mode (the SMART bit is set in the UARTCTL register), the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn stp2(&mut self) -> Stp2W<'_, LcrhSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bits 4:5 - UART Word Length The bits indicate the number of data bits transmitted or received in a frame as follows:"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<'_, LcrhSpec> {
        WlenW::new(self, 4)
    }
    #[doc = "Bit 6 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<'_, LcrhSpec> {
        SpsW::new(self, 6)
    }
    #[doc = "Bit 7 - UART send IDLE pattern. When this bit is set, SENDIDLE period of 11 bit times will be sent on the TX line. Read STAT.SENDIDLE bit to readback current status of SENDIDLE"]
    #[inline(always)]
    pub fn sendidle(&mut self) -> SendidleW<'_, LcrhSpec> {
        SendidleW::new(self, 7)
    }
    #[doc = "Bits 16:20 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be set before the START bit is send"]
    #[inline(always)]
    pub fn extdir_setup(&mut self) -> ExtdirSetupW<'_, LcrhSpec> {
        ExtdirSetupW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Defines the number of UARTclk ticks the signal to control the external driver for the RS485 will be reset after the beginning of the stop bit. (If 2 STOP bits are enabled the beginning of the 2nd STOP bit.)"]
    #[inline(always)]
    pub fn extdir_hold(&mut self) -> ExtdirHoldW<'_, LcrhSpec> {
        ExtdirHoldW::new(self, 21)
    }
    #[doc = "Bit 26 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, LcrhSpec> {
        SuspendW::new(self, 26)
    }
}
#[doc = "UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {}
