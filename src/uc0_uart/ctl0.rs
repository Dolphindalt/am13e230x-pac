#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `ENABLE` reader - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "UART Loop Back Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbe {
    #[doc = "0: Normal operation."]
    Disable = 0,
    #[doc = "1: The UARTxTX path is fed through the UARTxRX path internally."]
    Enable = 1,
}
impl From<Lbe> for bool {
    #[inline(always)]
    fn from(variant: Lbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBE` reader - UART Loop Back Enable"]
pub type LbeR = crate::BitReader<Lbe>;
impl LbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbe {
        match self.bits {
            false => Lbe::Disable,
            true => Lbe::Enable,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbe::Disable
    }
    #[doc = "The UARTxTX path is fed through the UARTxRX path internally."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbe::Enable
    }
}
#[doc = "Field `LBE` writer - UART Loop Back Enable"]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG, Lbe>;
impl<'a, REG> LbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Disable)
    }
    #[doc = "The UARTxTX path is fed through the UARTxRX path internally."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Enable)
    }
}
#[doc = "UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxe {
    #[doc = "0: The receive section of the UART is disabled."]
    Disable = 0,
    #[doc = "1: The receive section of the UART is enabled."]
    Enable = 1,
}
impl From<Rxe> for bool {
    #[inline(always)]
    fn from(variant: Rxe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXE` reader - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
pub type RxeR = crate::BitReader<Rxe>;
impl RxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxe {
        match self.bits {
            false => Rxe::Disable,
            true => Rxe::Enable,
        }
    }
    #[doc = "The receive section of the UART is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxe::Disable
    }
    #[doc = "The receive section of the UART is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxe::Enable
    }
}
#[doc = "Field `RXE` writer - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG, Rxe>;
impl<'a, REG> RxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive section of the UART is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Disable)
    }
    #[doc = "The receive section of the UART is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Enable)
    }
}
#[doc = "UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "0: The transmit section of the UART is disabled. The UARTxTXD pin of the UART can be controlled by the TXD_CTL bit when enabled."]
    Disable = 0,
    #[doc = "1: The transmit section of the UART is enabled."]
    Enable = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::Disable,
            true => Txe::Enable,
        }
    }
    #[doc = "The transmit section of the UART is disabled. The UARTxTXD pin of the UART can be controlled by the TXD_CTL bit when enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txe::Disable
    }
    #[doc = "The transmit section of the UART is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txe::Enable
    }
}
#[doc = "Field `TXE` writer - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG, Txe>;
impl<'a, REG> TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit section of the UART is disabled. The UARTxTXD pin of the UART can be controlled by the TXD_CTL bit when enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Disable)
    }
    #[doc = "The transmit section of the UART is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Enable)
    }
}
#[doc = "TXD Pin Control Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdOutEn {
    #[doc = "0: TXD pin can not be controlled by TXD_OUT"]
    Disable = 0,
    #[doc = "1: TXD pin can be controlled by TXD_OUT"]
    Enable = 1,
}
impl From<TxdOutEn> for bool {
    #[inline(always)]
    fn from(variant: TxdOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXD_OUT_EN` reader - TXD Pin Control Enable"]
pub type TxdOutEnR = crate::BitReader<TxdOutEn>;
impl TxdOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxdOutEn {
        match self.bits {
            false => TxdOutEn::Disable,
            true => TxdOutEn::Enable,
        }
    }
    #[doc = "TXD pin can not be controlled by TXD_OUT"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxdOutEn::Disable
    }
    #[doc = "TXD pin can be controlled by TXD_OUT"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxdOutEn::Enable
    }
}
#[doc = "Field `TXD_OUT_EN` writer - TXD Pin Control Enable"]
pub type TxdOutEnW<'a, REG> = crate::BitWriter<'a, REG, TxdOutEn>;
impl<'a, REG> TxdOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXD pin can not be controlled by TXD_OUT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TxdOutEn::Disable)
    }
    #[doc = "TXD pin can be controlled by TXD_OUT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TxdOutEn::Enable)
    }
}
#[doc = "TXD Pin Control Value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdOut {
    #[doc = "0: TXD pin is low"]
    Low = 0,
    #[doc = "1: TXD pin is high"]
    High = 1,
}
impl From<TxdOut> for bool {
    #[inline(always)]
    fn from(variant: TxdOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXD_OUT` reader - TXD Pin Control Value"]
pub type TxdOutR = crate::BitReader<TxdOut>;
impl TxdOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxdOut {
        match self.bits {
            false => TxdOut::Low,
            true => TxdOut::High,
        }
    }
    #[doc = "TXD pin is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TxdOut::Low
    }
    #[doc = "TXD pin is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TxdOut::High
    }
}
#[doc = "Field `TXD_OUT` writer - TXD Pin Control Value"]
pub type TxdOutW<'a, REG> = crate::BitWriter<'a, REG, TxdOut>;
impl<'a, REG> TxdOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXD pin is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TxdOut::Low)
    }
    #[doc = "TXD pin is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TxdOut::High)
    }
}
#[doc = "Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Normal operation"]
    Uart = 0,
    #[doc = "1: RS485 mode: UART needs to be IDLE with receiving data for the in EXTDIR_HOLD set time. EXTDIR_SETUP defines the time the RTS line is set to high before sending. When the buffer is empty the RTS line is set low again. A transmit will be delayed as long the UART is receiving data."]
    Rs485 = 1,
    #[doc = "2: The UART operates in IDLE Line Mode"]
    Idleline = 2,
    #[doc = "3: The UART operates in 9 Bit Address mode"]
    Addr9bit = 3,
    #[doc = "4: ISO7816 Smart Card Support The application must ensure that it sets 8-bit word length (WLEN set to 3h) and even parity (PEN set to 1, EPS set to 1, SPS set to 0) in UARTLCRH when using ISO7816 mode. The value of the STP2 bit in UARTLCRH is ignored and the number of stop bits is forced to 2."]
    Smart = 4,
    #[doc = "5: DALI Mode:"]
    Dali = 5,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Uart),
            1 => Some(Mode::Rs485),
            2 => Some(Mode::Idleline),
            3 => Some(Mode::Addr9bit),
            4 => Some(Mode::Smart),
            5 => Some(Mode::Dali),
            _ => None,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Mode::Uart
    }
    #[doc = "RS485 mode: UART needs to be IDLE with receiving data for the in EXTDIR_HOLD set time. EXTDIR_SETUP defines the time the RTS line is set to high before sending. When the buffer is empty the RTS line is set low again. A transmit will be delayed as long the UART is receiving data."]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == Mode::Rs485
    }
    #[doc = "The UART operates in IDLE Line Mode"]
    #[inline(always)]
    pub fn is_idleline(&self) -> bool {
        *self == Mode::Idleline
    }
    #[doc = "The UART operates in 9 Bit Address mode"]
    #[inline(always)]
    pub fn is_addr9bit(&self) -> bool {
        *self == Mode::Addr9bit
    }
    #[doc = "ISO7816 Smart Card Support The application must ensure that it sets 8-bit word length (WLEN set to 3h) and even parity (PEN set to 1, EPS set to 1, SPS set to 0) in UARTLCRH when using ISO7816 mode. The value of the STP2 bit in UARTLCRH is ignored and the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn is_smart(&self) -> bool {
        *self == Mode::Smart
    }
    #[doc = "DALI Mode:"]
    #[inline(always)]
    pub fn is_dali(&self) -> bool {
        *self == Mode::Dali
    }
}
#[doc = "Field `MODE` writer - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Uart)
    }
    #[doc = "RS485 mode: UART needs to be IDLE with receiving data for the in EXTDIR_HOLD set time. EXTDIR_SETUP defines the time the RTS line is set to high before sending. When the buffer is empty the RTS line is set low again. A transmit will be delayed as long the UART is receiving data."]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Rs485)
    }
    #[doc = "The UART operates in IDLE Line Mode"]
    #[inline(always)]
    pub fn idleline(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Idleline)
    }
    #[doc = "The UART operates in 9 Bit Address mode"]
    #[inline(always)]
    pub fn addr9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Addr9bit)
    }
    #[doc = "ISO7816 Smart Card Support The application must ensure that it sets 8-bit word length (WLEN set to 3h) and even parity (PEN set to 1, EPS set to 1, SPS set to 0) in UARTLCRH when using ISO7816 mode. The value of the STP2 bit in UARTLCRH is ignored and the number of stop bits is forced to 2."]
    #[inline(always)]
    pub fn smart(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Smart)
    }
    #[doc = "DALI Mode:"]
    #[inline(always)]
    pub fn dali(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dali)
    }
}
#[doc = "Field `RTS` reader - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or buffer status. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or buffer status. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable hardware controlled Request to Send\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "0: RTS hardware flow control is disabled."]
    Disable = 0,
    #[doc = "1: RTS hardware flow control is enabled. Data is only requested (by asserting UARTxRTS) when the receive FIFO/buffer has available entries."]
    Enable = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - Enable hardware controlled Request to Send"]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::Disable,
            true => Rtsen::Enable,
        }
    }
    #[doc = "RTS hardware flow control is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtsen::Disable
    }
    #[doc = "RTS hardware flow control is enabled. Data is only requested (by asserting UARTxRTS) when the receive FIFO/buffer has available entries."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtsen::Enable
    }
}
#[doc = "Field `RTSEN` writer - Enable hardware controlled Request to Send"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Disable)
    }
    #[doc = "RTS hardware flow control is enabled. Data is only requested (by asserting UARTxRTS) when the receive FIFO/buffer has available entries."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Enable)
    }
}
#[doc = "Enable Clear To Send\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: CTS hardware flow control is disabled."]
    Disable = 0,
    #[doc = "1: CTS hardware flow control is enabled. Data is only transmitted when the UARTxCTS signal is asserted."]
    Enable = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - Enable Clear To Send"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disable,
            true => Ctsen::Enable,
        }
    }
    #[doc = "CTS hardware flow control is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsen::Disable
    }
    #[doc = "CTS hardware flow control is enabled. Data is only transmitted when the UARTxCTS signal is asserted."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsen::Enable
    }
}
#[doc = "Field `CTSEN` writer - Enable Clear To Send"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disable)
    }
    #[doc = "CTS hardware flow control is enabled. Data is only transmitted when the UARTxCTS signal is asserted."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enable)
    }
}
#[doc = "High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hse {
    #[doc = "0: 16x oversampling."]
    Ovs16 = 0,
    #[doc = "1: 8x oversampling."]
    Ovs8 = 1,
    #[doc = "2: 3x oversampling. IrDA, Manchester and DALI not supported when 3x oversampling is enabled."]
    Ovs3 = 2,
}
impl From<Hse> for u8 {
    #[inline(always)]
    fn from(variant: Hse) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hse {
    type Ux = u8;
}
impl crate::IsEnum for Hse {}
#[doc = "Field `HSE` reader - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
pub type HseR = crate::FieldReader<Hse>;
impl HseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hse> {
        match self.bits {
            0 => Some(Hse::Ovs16),
            1 => Some(Hse::Ovs8),
            2 => Some(Hse::Ovs3),
            _ => None,
        }
    }
    #[doc = "16x oversampling."]
    #[inline(always)]
    pub fn is_ovs16(&self) -> bool {
        *self == Hse::Ovs16
    }
    #[doc = "8x oversampling."]
    #[inline(always)]
    pub fn is_ovs8(&self) -> bool {
        *self == Hse::Ovs8
    }
    #[doc = "3x oversampling. IrDA, Manchester and DALI not supported when 3x oversampling is enabled."]
    #[inline(always)]
    pub fn is_ovs3(&self) -> bool {
        *self == Hse::Ovs3
    }
}
#[doc = "Field `HSE` writer - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
pub type HseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hse, crate::Safe>;
impl<'a, REG> HseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x oversampling."]
    #[inline(always)]
    pub fn ovs16(self) -> &'a mut crate::W<REG> {
        self.variant(Hse::Ovs16)
    }
    #[doc = "8x oversampling."]
    #[inline(always)]
    pub fn ovs8(self) -> &'a mut crate::W<REG> {
        self.variant(Hse::Ovs8)
    }
    #[doc = "3x oversampling. IrDA, Manchester and DALI not supported when 3x oversampling is enabled."]
    #[inline(always)]
    pub fn ovs3(self) -> &'a mut crate::W<REG> {
        self.variant(Hse::Ovs3)
    }
}
#[doc = "Majority Voting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Majvote {
    #[doc = "0: Majority voting is disabled"]
    Disable = 0,
    #[doc = "1: Majority voting is enabled"]
    Enable = 1,
}
impl From<Majvote> for bool {
    #[inline(always)]
    fn from(variant: Majvote) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAJVOTE` reader - Majority Voting"]
pub type MajvoteR = crate::BitReader<Majvote>;
impl MajvoteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Majvote {
        match self.bits {
            false => Majvote::Disable,
            true => Majvote::Enable,
        }
    }
    #[doc = "Majority voting is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Majvote::Disable
    }
    #[doc = "Majority voting is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Majvote::Enable
    }
}
#[doc = "Field `MAJVOTE` writer - Majority Voting"]
pub type MajvoteW<'a, REG> = crate::BitWriter<'a, REG, Majvote>;
impl<'a, REG> MajvoteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Majority voting is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Majvote::Disable)
    }
    #[doc = "Majority voting is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Majvote::Enable)
    }
}
#[doc = "MSB First\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbfirst {
    #[doc = "0: Least significant bit is sent first in the protocol packet"]
    Disable = 0,
    #[doc = "1: Most significant bit is sent first in the protocol packet"]
    Enable = 1,
}
impl From<Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - MSB First"]
pub type MsbfirstR = crate::BitReader<Msbfirst>;
impl MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbfirst {
        match self.bits {
            false => Msbfirst::Disable,
            true => Msbfirst::Enable,
        }
    }
    #[doc = "Least significant bit is sent first in the protocol packet"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Msbfirst::Disable
    }
    #[doc = "Most significant bit is sent first in the protocol packet"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Msbfirst::Enable
    }
}
#[doc = "Field `MSBFIRST` writer - MSB First"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Msbfirst>;
impl<'a, REG> MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Least significant bit is sent first in the protocol packet"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::Disable)
    }
    #[doc = "Most significant bit is sent first in the protocol packet"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXD Pin Control Enable"]
    #[inline(always)]
    pub fn txd_out_en(&self) -> TxdOutEnR {
        TxdOutEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXD Pin Control Value"]
    #[inline(always)]
    pub fn txd_out(&self) -> TxdOutR {
        TxdOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or buffer status. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable hardware controlled Request to Send"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Clear To Send"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 18 - Majority Voting"]
    #[inline(always)]
    pub fn majvote(&self) -> MajvoteR {
        MajvoteR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSB First"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL0")
            .field("enable", &self.enable())
            .field("hse", &self.hse())
            .field("lbe", &self.lbe())
            .field("rxe", &self.rxe())
            .field("txe", &self.txe())
            .field("rts", &self.rts())
            .field("rtsen", &self.rtsen())
            .field("ctsen", &self.ctsen())
            .field("mode", &self.mode())
            .field("txd_out", &self.txd_out())
            .field("txd_out_en", &self.txd_out_en())
            .field("majvote", &self.majvote())
            .field("msbfirst", &self.msbfirst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - UART Module Enable. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. If the ENABLE bit is not set, all registers can still be accessed and updated. It is recommended to setup and change the UART operation mode with having the ENABLE bit cleared to avoid unpredictable behavior during the setup or update. If disabled the UART module will not send or receive any data and the logic is held in reset state."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Ctl0Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LbeW<'_, Ctl0Spec> {
        LbeW::new(self, 2)
    }
    #[doc = "Bit 3 - UART Receive Enable If the UART is disabled in the middle of a receive, it completes the current character before stopping. #b#NOTE:#/b# To enable reception, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, Ctl0Spec> {
        RxeW::new(self, 3)
    }
    #[doc = "Bit 4 - UART Transmit Enable If the UART is disabled in the middle of a transmission, it completes the current character before stopping. #b#NOTE:#/b# To enable transmission, the UARTEN bit must be set."]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, Ctl0Spec> {
        TxeW::new(self, 4)
    }
    #[doc = "Bit 5 - TXD Pin Control Enable"]
    #[inline(always)]
    pub fn txd_out_en(&mut self) -> TxdOutEnW<'_, Ctl0Spec> {
        TxdOutEnW::new(self, 5)
    }
    #[doc = "Bit 6 - TXD Pin Control Value"]
    #[inline(always)]
    pub fn txd_out(&mut self) -> TxdOutW<'_, Ctl0Spec> {
        TxdOutW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Set the communication mode and protocol used. (Not defined settings uses the default setting: 0)"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Ctl0Spec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bit 12 - Request to Send If RTSEN is set the RTS output signals is controlled by the hardware logic using the FIFO fill level or buffer status. If RTSEN is cleared the RTS output is controlled by the RTS bit. The bit is the complement of the UART request to send, RTS modem status output."]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, Ctl0Spec> {
        RtsW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable hardware controlled Request to Send"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, Ctl0Spec> {
        RtsenW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Clear To Send"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Ctl0Spec> {
        CtsenW::new(self, 14)
    }
    #[doc = "Bits 15:16 - High-Speed Bit Oversampling Enable #b#NOTE:#/b# The bit oversampling influences the UART baud-rate configuration. The state of this bit has no effect on clock generation in ISO7816 smart card mode (the SMART bit is set)."]
    #[inline(always)]
    pub fn hse(&mut self) -> HseW<'_, Ctl0Spec> {
        HseW::new(self, 15)
    }
    #[doc = "Bit 18 - Majority Voting"]
    #[inline(always)]
    pub fn majvote(&mut self) -> MajvoteW<'_, Ctl0Spec> {
        MajvoteW::new(self, 18)
    }
    #[doc = "Bit 19 - MSB First"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<'_, Ctl0Spec> {
        MsbfirstW::new(self, 19)
    }
}
#[doc = "UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTL0 to value 0x38"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x38;
}
