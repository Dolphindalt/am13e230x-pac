#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Start Transfer\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrmStart {
    #[doc = "0: In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    Disable = 0,
    #[doc = "1: The Controller is able to transmit or receive data. See field decoding in Table: MCTR Field decoding."]
    Enable = 1,
}
impl From<FrmStart> for bool {
    #[inline(always)]
    fn from(variant: FrmStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRM_START` reader - Start Transfer"]
pub type FrmStartR = crate::BitReader<FrmStart>;
impl FrmStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrmStart {
        match self.bits {
            false => FrmStart::Disable,
            true => FrmStart::Enable,
        }
    }
    #[doc = "In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FrmStart::Disable
    }
    #[doc = "The Controller is able to transmit or receive data. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FrmStart::Enable
    }
}
#[doc = "Field `FRM_START` writer - Start Transfer"]
pub type FrmStartW<'a, REG> = crate::BitWriter<'a, REG, FrmStart>;
impl<'a, REG> FrmStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FrmStart::Disable)
    }
    #[doc = "The Controller is able to transmit or receive data. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FrmStart::Enable)
    }
}
#[doc = "Generate START\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: The controller does not generate the START condition."]
    Disable = 0,
    #[doc = "1: The controller generates the START or repeated START condition. See field decoding in Table: MCTR Field decoding."]
    Enable = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Generate START"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::Disable,
            true => Start::Enable,
        }
    }
    #[doc = "The controller does not generate the START condition."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Start::Disable
    }
    #[doc = "The controller generates the START or repeated START condition. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Start::Enable
    }
}
#[doc = "Field `START` writer - Generate START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The controller does not generate the START condition."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Disable)
    }
    #[doc = "The controller generates the START or repeated START condition. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Enable)
    }
}
#[doc = "Generate STOP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: The controller does not generate the STOP condition."]
    Disable = 0,
    #[doc = "1: The controller generates the STOP condition. See field decoding in Table: MCTR Field decoding."]
    Enable = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Generate STOP"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::Disable,
            true => Stop::Enable,
        }
    }
    #[doc = "The controller does not generate the STOP condition."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stop::Disable
    }
    #[doc = "The controller generates the STOP condition. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stop::Enable
    }
}
#[doc = "Field `STOP` writer - Generate STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The controller does not generate the STOP condition."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Disable)
    }
    #[doc = "The controller generates the STOP condition. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Enable)
    }
}
#[doc = "Data Acknowledge Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "0: The last received data byte of a transaction is not acknowledged automatically ."]
    Disable = 0,
    #[doc = "1: The last received data byte of a transaction is acknowledged automatically ."]
    Enable = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - Data Acknowledge Enable"]
pub type AckR = crate::BitReader<Ack>;
impl AckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack {
        match self.bits {
            false => Ack::Disable,
            true => Ack::Enable,
        }
    }
    #[doc = "The last received data byte of a transaction is not acknowledged automatically ."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ack::Disable
    }
    #[doc = "The last received data byte of a transaction is acknowledged automatically ."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ack::Enable
    }
}
#[doc = "Field `ACK` writer - Data Acknowledge Enable"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The last received data byte of a transaction is not acknowledged automatically ."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Disable)
    }
    #[doc = "The last received data byte of a transaction is acknowledged automatically ."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Enable)
    }
}
#[doc = "ACK overrride Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackoen {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    Enable = 1,
}
impl From<Ackoen> for bool {
    #[inline(always)]
    fn from(variant: Ackoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKOEN` reader - ACK overrride Enable"]
pub type AckoenR = crate::BitReader<Ackoen>;
impl AckoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackoen {
        match self.bits {
            false => Ackoen::Disable,
            true => Ackoen::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ackoen::Disable
    }
    #[doc = "When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ackoen::Enable
    }
}
#[doc = "Field `ACKOEN` writer - ACK overrride Enable"]
pub type AckoenW<'a, REG> = crate::BitWriter<'a, REG, Ackoen>;
impl<'a, REG> AckoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoen::Disable)
    }
    #[doc = "When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ackoen::Enable)
    }
}
#[doc = "Read on TX Empty\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdOnTxempty {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    Enable = 1,
}
impl From<RdOnTxempty> for bool {
    #[inline(always)]
    fn from(variant: RdOnTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_ON_TXEMPTY` reader - Read on TX Empty"]
pub type RdOnTxemptyR = crate::BitReader<RdOnTxempty>;
impl RdOnTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdOnTxempty {
        match self.bits {
            false => RdOnTxempty::Disable,
            true => RdOnTxempty::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RdOnTxempty::Disable
    }
    #[doc = "When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RdOnTxempty::Enable
    }
}
#[doc = "Field `RD_ON_TXEMPTY` writer - Read on TX Empty"]
pub type RdOnTxemptyW<'a, REG> = crate::BitWriter<'a, REG, RdOnTxempty>;
impl<'a, REG> RdOnTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RdOnTxempty::Disable)
    }
    #[doc = "When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RdOnTxempty::Enable)
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
#[doc = "Field `BLEN` reader - I2C transaction length"]
pub type BlenR = crate::FieldReader<u16>;
#[doc = "Field `BLEN` writer - I2C transaction length"]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Start Transfer"]
    #[inline(always)]
    pub fn frm_start(&self) -> FrmStartR {
        FrmStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACK overrride Enable"]
    #[inline(always)]
    pub fn ackoen(&self) -> AckoenR {
        AckoenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn rd_on_txempty(&self) -> RdOnTxemptyR {
        RdOnTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:27 - I2C transaction length"]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("frm_start", &self.frm_start())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("ack", &self.ack())
            .field("blen", &self.blen())
            .field("ackoen", &self.ackoen())
            .field("rd_on_txempty", &self.rd_on_txempty())
            .field("suspend", &self.suspend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start Transfer"]
    #[inline(always)]
    pub fn frm_start(&mut self) -> FrmStartW<'_, CtrSpec> {
        FrmStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtrSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CtrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, CtrSpec> {
        AckW::new(self, 3)
    }
    #[doc = "Bit 4 - ACK overrride Enable"]
    #[inline(always)]
    pub fn ackoen(&mut self) -> AckoenW<'_, CtrSpec> {
        AckoenW::new(self, 4)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn rd_on_txempty(&mut self) -> RdOnTxemptyW<'_, CtrSpec> {
        RdOnTxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, CtrSpec> {
        SuspendW::new(self, 6)
    }
    #[doc = "Bits 16:27 - I2C transaction length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BlenW<'_, CtrSpec> {
        BlenW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CtrSpec {}
