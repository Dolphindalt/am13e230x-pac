#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `ENABLE` reader - Setting this bit enables the module."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Setting this bit enables the module."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "General call response enable Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gencall {
    #[doc = "0: Do not respond to a general call"]
    Disable = 0,
    #[doc = "1: Respond to a general call"]
    Enable = 1,
}
impl From<Gencall> for bool {
    #[inline(always)]
    fn from(variant: Gencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCALL` reader - General call response enable Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
pub type GencallR = crate::BitReader<Gencall>;
impl GencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gencall {
        match self.bits {
            false => Gencall::Disable,
            true => Gencall::Enable,
        }
    }
    #[doc = "Do not respond to a general call"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gencall::Disable
    }
    #[doc = "Respond to a general call"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gencall::Enable
    }
}
#[doc = "Field `GENCALL` writer - General call response enable Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
pub type GencallW<'a, REG> = crate::BitWriter<'a, REG, Gencall>;
impl<'a, REG> GencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not respond to a general call"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gencall::Disable)
    }
    #[doc = "Respond to a general call"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gencall::Enable)
    }
}
#[doc = "Tx Empty Interrupt on TREQ\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxemptyOnTreq {
    #[doc = "0: When 0, RIS:TXEMPTY will be set when only the Target TX FIFO is empty. This allows the TXEMPTY interrupt to be used to indicate that the I2C bus is being clock stretched and that Target TX data is required."]
    Disable = 0,
    #[doc = "1: When 1, RIS:TXEMPTY will be set when the Target State Machine is in the TX_WAIT state which occurs when the TX FIFO is empty AND the I2C transaction is clock stretched waiting for the FIFO to receive data."]
    Enable = 1,
}
impl From<TxemptyOnTreq> for bool {
    #[inline(always)]
    fn from(variant: TxemptyOnTreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY_ON_TREQ` reader - Tx Empty Interrupt on TREQ"]
pub type TxemptyOnTreqR = crate::BitReader<TxemptyOnTreq>;
impl TxemptyOnTreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxemptyOnTreq {
        match self.bits {
            false => TxemptyOnTreq::Disable,
            true => TxemptyOnTreq::Enable,
        }
    }
    #[doc = "When 0, RIS:TXEMPTY will be set when only the Target TX FIFO is empty. This allows the TXEMPTY interrupt to be used to indicate that the I2C bus is being clock stretched and that Target TX data is required."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxemptyOnTreq::Disable
    }
    #[doc = "When 1, RIS:TXEMPTY will be set when the Target State Machine is in the TX_WAIT state which occurs when the TX FIFO is empty AND the I2C transaction is clock stretched waiting for the FIFO to receive data."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxemptyOnTreq::Enable
    }
}
#[doc = "Field `TXEMPTY_ON_TREQ` writer - Tx Empty Interrupt on TREQ"]
pub type TxemptyOnTreqW<'a, REG> = crate::BitWriter<'a, REG, TxemptyOnTreq>;
impl<'a, REG> TxemptyOnTreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When 0, RIS:TXEMPTY will be set when only the Target TX FIFO is empty. This allows the TXEMPTY interrupt to be used to indicate that the I2C bus is being clock stretched and that Target TX data is required."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TxemptyOnTreq::Disable)
    }
    #[doc = "When 1, RIS:TXEMPTY will be set when the Target State Machine is in the TX_WAIT state which occurs when the TX FIFO is empty AND the I2C transaction is clock stretched waiting for the FIFO to receive data."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TxemptyOnTreq::Enable)
    }
}
#[doc = "Tx Trigger when Target FSM is in Tx Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxtrigTxmode {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When 1, RIS:TXTRG will be set when the Target TX FIFO has reached the trigger level AND the Target State Machine is in the TXMODE as defined in the SSR register. When cleared RIS:TXTRG will be set when the Target TX FIFO is at or above the trigger level. This setting can be used to hold off the TX DMA until a transaction starts. This allows the DMA to be configured when the I2C is idle but have it wait till the transaction starts to load the Target TX FIFO, so it can load from a memory buffer that might be changing over time."]
    Enable = 1,
}
impl From<TxtrigTxmode> for bool {
    #[inline(always)]
    fn from(variant: TxtrigTxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTRIG_TXMODE` reader - Tx Trigger when Target FSM is in Tx Mode"]
pub type TxtrigTxmodeR = crate::BitReader<TxtrigTxmode>;
impl TxtrigTxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxtrigTxmode {
        match self.bits {
            false => TxtrigTxmode::Disable,
            true => TxtrigTxmode::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxtrigTxmode::Disable
    }
    #[doc = "When 1, RIS:TXTRG will be set when the Target TX FIFO has reached the trigger level AND the Target State Machine is in the TXMODE as defined in the SSR register. When cleared RIS:TXTRG will be set when the Target TX FIFO is at or above the trigger level. This setting can be used to hold off the TX DMA until a transaction starts. This allows the DMA to be configured when the I2C is idle but have it wait till the transaction starts to load the Target TX FIFO, so it can load from a memory buffer that might be changing over time."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxtrigTxmode::Enable
    }
}
#[doc = "Field `TXTRIG_TXMODE` writer - Tx Trigger when Target FSM is in Tx Mode"]
pub type TxtrigTxmodeW<'a, REG> = crate::BitWriter<'a, REG, TxtrigTxmode>;
impl<'a, REG> TxtrigTxmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TxtrigTxmode::Disable)
    }
    #[doc = "When 1, RIS:TXTRG will be set when the Target TX FIFO has reached the trigger level AND the Target State Machine is in the TXMODE as defined in the SSR register. When cleared RIS:TXTRG will be set when the Target TX FIFO is at or above the trigger level. This setting can be used to hold off the TX DMA until a transaction starts. This allows the DMA to be configured when the I2C is idle but have it wait till the transaction starts to load the Target TX FIFO, so it can load from a memory buffer that might be changing over time."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TxtrigTxmode::Enable)
    }
}
#[doc = "Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Target State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxwaitStaleTxfifo {
    #[doc = "0: When 0, the TX FIFO empty signal to the Target State Machine indicates that the TX FIFO is empty."]
    Disable = 0,
    #[doc = "1: When 1, the TX FIFO empty signal to the Target State Machine will indicate that the TX FIFO is empty or that the TX FIFO data is stale. The TX FIFO data is determined to be stale when there is data in the TX FIFO when the Target State Machine leaves the TXMODE as defined in the SSR register. This can occur is a Stop or timeout occur when there are bytes left in the TX FIFO."]
    Enable = 1,
}
impl From<TxwaitStaleTxfifo> for bool {
    #[inline(always)]
    fn from(variant: TxwaitStaleTxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXWAIT_STALE_TXFIFO` reader - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Target State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
pub type TxwaitStaleTxfifoR = crate::BitReader<TxwaitStaleTxfifo>;
impl TxwaitStaleTxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxwaitStaleTxfifo {
        match self.bits {
            false => TxwaitStaleTxfifo::Disable,
            true => TxwaitStaleTxfifo::Enable,
        }
    }
    #[doc = "When 0, the TX FIFO empty signal to the Target State Machine indicates that the TX FIFO is empty."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxwaitStaleTxfifo::Disable
    }
    #[doc = "When 1, the TX FIFO empty signal to the Target State Machine will indicate that the TX FIFO is empty or that the TX FIFO data is stale. The TX FIFO data is determined to be stale when there is data in the TX FIFO when the Target State Machine leaves the TXMODE as defined in the SSR register. This can occur is a Stop or timeout occur when there are bytes left in the TX FIFO."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxwaitStaleTxfifo::Enable
    }
}
#[doc = "Field `TXWAIT_STALE_TXFIFO` writer - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Target State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
pub type TxwaitStaleTxfifoW<'a, REG> = crate::BitWriter<'a, REG, TxwaitStaleTxfifo>;
impl<'a, REG> TxwaitStaleTxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When 0, the TX FIFO empty signal to the Target State Machine indicates that the TX FIFO is empty."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TxwaitStaleTxfifo::Disable)
    }
    #[doc = "When 1, the TX FIFO empty signal to the Target State Machine will indicate that the TX FIFO is empty or that the TX FIFO data is stale. The TX FIFO data is determined to be stale when there is data in the TX FIFO when the Target State Machine leaves the TXMODE as defined in the SSR register. This can occur is a Stop or timeout occur when there are bytes left in the TX FIFO."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TxwaitStaleTxfifo::Enable)
    }
}
#[doc = "Rx full interrupt generated on RREQ condition as indicated in SSR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfullOnRreq {
    #[doc = "0: When 0, RIS:SRXFULL will be set when only the Target RX FIFO is full. This allows the SRXFULL interrupt to be used to indicate that the I2C bus is being clock stretched and that the FW must either read the RX FIFO or ACK/NACK the current Rx byte."]
    Disable = 0,
    #[doc = "1: When 1, RIS:SRXFULL will be set when the Target State Machine is in the RX_WAIT or RX_ACK_WAIT states which occurs when the I2C transaction is clock stretched because the RX FIFO is full or the ACKOEN has been set and the state machine is waiting for FW to ACK/NACK the current byte."]
    Enable = 1,
}
impl From<RxfullOnRreq> for bool {
    #[inline(always)]
    fn from(variant: RxfullOnRreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL_ON_RREQ` reader - Rx full interrupt generated on RREQ condition as indicated in SSR"]
pub type RxfullOnRreqR = crate::BitReader<RxfullOnRreq>;
impl RxfullOnRreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxfullOnRreq {
        match self.bits {
            false => RxfullOnRreq::Disable,
            true => RxfullOnRreq::Enable,
        }
    }
    #[doc = "When 0, RIS:SRXFULL will be set when only the Target RX FIFO is full. This allows the SRXFULL interrupt to be used to indicate that the I2C bus is being clock stretched and that the FW must either read the RX FIFO or ACK/NACK the current Rx byte."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RxfullOnRreq::Disable
    }
    #[doc = "When 1, RIS:SRXFULL will be set when the Target State Machine is in the RX_WAIT or RX_ACK_WAIT states which occurs when the I2C transaction is clock stretched because the RX FIFO is full or the ACKOEN has been set and the state machine is waiting for FW to ACK/NACK the current byte."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RxfullOnRreq::Enable
    }
}
#[doc = "Field `RXFULL_ON_RREQ` writer - Rx full interrupt generated on RREQ condition as indicated in SSR"]
pub type RxfullOnRreqW<'a, REG> = crate::BitWriter<'a, REG, RxfullOnRreq>;
impl<'a, REG> RxfullOnRreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When 0, RIS:SRXFULL will be set when only the Target RX FIFO is full. This allows the SRXFULL interrupt to be used to indicate that the I2C bus is being clock stretched and that the FW must either read the RX FIFO or ACK/NACK the current Rx byte."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RxfullOnRreq::Disable)
    }
    #[doc = "When 1, RIS:SRXFULL will be set when the Target State Machine is in the RX_WAIT or RX_ACK_WAIT states which occurs when the I2C transaction is clock stretched because the RX FIFO is full or the ACKOEN has been set and the state machine is waiting for FW to ACK/NACK the current byte."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RxfullOnRreq::Enable)
    }
}
#[doc = "Enable Default Host Address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnDefhostadr {
    #[doc = "0: When this bit is 0, the default host address is not matched NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    Disable = 0,
    #[doc = "1: When this bit is 1, default host address of 7h000_1000 is always matched by the Target address match logic."]
    Enable = 1,
}
impl From<EnDefhostadr> for bool {
    #[inline(always)]
    fn from(variant: EnDefhostadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_DEFHOSTADR` reader - Enable Default Host Address"]
pub type EnDefhostadrR = crate::BitReader<EnDefhostadr>;
impl EnDefhostadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnDefhostadr {
        match self.bits {
            false => EnDefhostadr::Disable,
            true => EnDefhostadr::Enable,
        }
    }
    #[doc = "When this bit is 0, the default host address is not matched NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnDefhostadr::Disable
    }
    #[doc = "When this bit is 1, default host address of 7h000_1000 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnDefhostadr::Enable
    }
}
#[doc = "Field `EN_DEFHOSTADR` writer - Enable Default Host Address"]
pub type EnDefhostadrW<'a, REG> = crate::BitWriter<'a, REG, EnDefhostadr>;
impl<'a, REG> EnDefhostadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is 0, the default host address is not matched NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnDefhostadr::Disable)
    }
    #[doc = "When this bit is 1, default host address of 7h000_1000 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnDefhostadr::Enable)
    }
}
#[doc = "Enable Alert Response Address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnAlrespadr {
    #[doc = "0: When this bit is 0, the alert response address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    Disable = 0,
    #[doc = "1: When this bit is 1, alert response address of 7h000_1100 is always matched by the Target address match logic."]
    Enable = 1,
}
impl From<EnAlrespadr> for bool {
    #[inline(always)]
    fn from(variant: EnAlrespadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_ALRESPADR` reader - Enable Alert Response Address"]
pub type EnAlrespadrR = crate::BitReader<EnAlrespadr>;
impl EnAlrespadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnAlrespadr {
        match self.bits {
            false => EnAlrespadr::Disable,
            true => EnAlrespadr::Enable,
        }
    }
    #[doc = "When this bit is 0, the alert response address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnAlrespadr::Disable
    }
    #[doc = "When this bit is 1, alert response address of 7h000_1100 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnAlrespadr::Enable
    }
}
#[doc = "Field `EN_ALRESPADR` writer - Enable Alert Response Address"]
pub type EnAlrespadrW<'a, REG> = crate::BitWriter<'a, REG, EnAlrespadr>;
impl<'a, REG> EnAlrespadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is 0, the alert response address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnAlrespadr::Disable)
    }
    #[doc = "When this bit is 1, alert response address of 7h000_1100 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnAlrespadr::Enable)
    }
}
#[doc = "Enable Default device address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnDefdevadr {
    #[doc = "0: When this bit is 0, the default device address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2."]
    Disable = 0,
    #[doc = "1: When this bit is 1, default device address of 7h110_0001 is always matched by the Target address match logic."]
    Enable = 1,
}
impl From<EnDefdevadr> for bool {
    #[inline(always)]
    fn from(variant: EnDefdevadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_DEFDEVADR` reader - Enable Default device address"]
pub type EnDefdevadrR = crate::BitReader<EnDefdevadr>;
impl EnDefdevadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnDefdevadr {
        match self.bits {
            false => EnDefdevadr::Disable,
            true => EnDefdevadr::Enable,
        }
    }
    #[doc = "When this bit is 0, the default device address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnDefdevadr::Disable
    }
    #[doc = "When this bit is 1, default device address of 7h110_0001 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnDefdevadr::Enable
    }
}
#[doc = "Field `EN_DEFDEVADR` writer - Enable Default device address"]
pub type EnDefdevadrW<'a, REG> = crate::BitWriter<'a, REG, EnDefdevadr>;
impl<'a, REG> EnDefdevadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is 0, the default device address is not matched. NOTE: it may still be matched if programmed inside SOAR/SOAR2."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnDefdevadr::Disable)
    }
    #[doc = "When this bit is 1, default device address of 7h110_0001 is always matched by the Target address match logic."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnDefdevadr::Enable)
    }
}
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
#[doc = "Clock Stretch Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkstretch {
    #[doc = "0: clock stretching is disabled"]
    Disable = 0,
    #[doc = "1: clock stretching is enabled"]
    Enable = 1,
}
impl From<Clkstretch> for bool {
    #[inline(always)]
    fn from(variant: Clkstretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTRETCH` reader - Clock Stretch Enable"]
pub type ClkstretchR = crate::BitReader<Clkstretch>;
impl ClkstretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkstretch {
        match self.bits {
            false => Clkstretch::Disable,
            true => Clkstretch::Enable,
        }
    }
    #[doc = "clock stretching is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clkstretch::Disable
    }
    #[doc = "clock stretching is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clkstretch::Enable
    }
}
#[doc = "Field `CLKSTRETCH` writer - Clock Stretch Enable"]
pub type ClkstretchW<'a, REG> = crate::BitWriter<'a, REG, Clkstretch>;
impl<'a, REG> ClkstretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock stretching is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Disable)
    }
    #[doc = "clock stretching is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Enable)
    }
}
#[doc = "Target Wakeup Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuen {
    #[doc = "0: When 0, the Target is not allowed to clock stretch on START detection"]
    Disable = 0,
    #[doc = "1: When 1, the Target is allowed to clock stretch on START detection and wait for faster clock to be abvailable. This allows clean wake up support for I2C in low power mode use cases"]
    Enable = 1,
}
impl From<Wuen> for bool {
    #[inline(always)]
    fn from(variant: Wuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUEN` reader - Target Wakeup Enable"]
pub type WuenR = crate::BitReader<Wuen>;
impl WuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuen {
        match self.bits {
            false => Wuen::Disable,
            true => Wuen::Enable,
        }
    }
    #[doc = "When 0, the Target is not allowed to clock stretch on START detection"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wuen::Disable
    }
    #[doc = "When 1, the Target is allowed to clock stretch on START detection and wait for faster clock to be abvailable. This allows clean wake up support for I2C in low power mode use cases"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wuen::Enable
    }
}
#[doc = "Field `WUEN` writer - Target Wakeup Enable"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG, Wuen>;
impl<'a, REG> WuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When 0, the Target is not allowed to clock stretch on START detection"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::Disable)
    }
    #[doc = "When 1, the Target is allowed to clock stretch on START detection and wait for faster clock to be abvailable. This allows clean wake up support for I2C in low power mode use cases"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit enables the module."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General call response enable Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        GencallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx Empty Interrupt on TREQ"]
    #[inline(always)]
    pub fn txempty_on_treq(&self) -> TxemptyOnTreqR {
        TxemptyOnTreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx Trigger when Target FSM is in Tx Mode"]
    #[inline(always)]
    pub fn txtrig_txmode(&self) -> TxtrigTxmodeR {
        TxtrigTxmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Target State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
    #[inline(always)]
    pub fn txwait_stale_txfifo(&self) -> TxwaitStaleTxfifoR {
        TxwaitStaleTxfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx full interrupt generated on RREQ condition as indicated in SSR"]
    #[inline(always)]
    pub fn rxfull_on_rreq(&self) -> RxfullOnRreqR {
        RxfullOnRreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Default Host Address"]
    #[inline(always)]
    pub fn en_defhostadr(&self) -> EnDefhostadrR {
        EnDefhostadrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Alert Response Address"]
    #[inline(always)]
    pub fn en_alrespadr(&self) -> EnAlrespadrR {
        EnAlrespadrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Default device address"]
    #[inline(always)]
    pub fn en_defdevadr(&self) -> EnDefdevadrR {
        EnDefdevadrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn clkstretch(&self) -> ClkstretchR {
        ClkstretchR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Target Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("enable", &self.enable())
            .field("gencall", &self.gencall())
            .field("clkstretch", &self.clkstretch())
            .field("txempty_on_treq", &self.txempty_on_treq())
            .field("txtrig_txmode", &self.txtrig_txmode())
            .field("txwait_stale_txfifo", &self.txwait_stale_txfifo())
            .field("rxfull_on_rreq", &self.rxfull_on_rreq())
            .field("en_defhostadr", &self.en_defhostadr())
            .field("en_alrespadr", &self.en_alrespadr())
            .field("en_defdevadr", &self.en_defdevadr())
            .field("wuen", &self.wuen())
            .field("suspend", &self.suspend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit enables the module."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - General call response enable Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
    #[inline(always)]
    pub fn gencall(&mut self) -> GencallW<'_, CtrSpec> {
        GencallW::new(self, 1)
    }
    #[doc = "Bit 3 - Tx Empty Interrupt on TREQ"]
    #[inline(always)]
    pub fn txempty_on_treq(&mut self) -> TxemptyOnTreqW<'_, CtrSpec> {
        TxemptyOnTreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx Trigger when Target FSM is in Tx Mode"]
    #[inline(always)]
    pub fn txtrig_txmode(&mut self) -> TxtrigTxmodeW<'_, CtrSpec> {
        TxtrigTxmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Target State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
    #[inline(always)]
    pub fn txwait_stale_txfifo(&mut self) -> TxwaitStaleTxfifoW<'_, CtrSpec> {
        TxwaitStaleTxfifoW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx full interrupt generated on RREQ condition as indicated in SSR"]
    #[inline(always)]
    pub fn rxfull_on_rreq(&mut self) -> RxfullOnRreqW<'_, CtrSpec> {
        RxfullOnRreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Default Host Address"]
    #[inline(always)]
    pub fn en_defhostadr(&mut self) -> EnDefhostadrW<'_, CtrSpec> {
        EnDefhostadrW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Alert Response Address"]
    #[inline(always)]
    pub fn en_alrespadr(&mut self) -> EnAlrespadrW<'_, CtrSpec> {
        EnAlrespadrW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Default device address"]
    #[inline(always)]
    pub fn en_defdevadr(&mut self) -> EnDefdevadrW<'_, CtrSpec> {
        EnDefdevadrW::new(self, 9)
    }
    #[doc = "Bit 11 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, CtrSpec> {
        SuspendW::new(self, 11)
    }
    #[doc = "Bit 20 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn clkstretch(&mut self) -> ClkstretchW<'_, CtrSpec> {
        ClkstretchW::new(self, 20)
    }
    #[doc = "Bit 21 - Target Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WuenW<'_, CtrSpec> {
        WuenW::new(self, 21)
    }
}
#[doc = "I2C Target Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTR to value 0x0030_0404"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0x0030_0404;
}
