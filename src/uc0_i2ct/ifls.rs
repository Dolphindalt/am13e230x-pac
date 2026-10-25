#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "1: TX FIFO <= 3/4 empty"]
    Lvl3_4 = 1,
    #[doc = "2: TX FIFO <= 1/2 empty (default)"]
    Lvl1_2 = 2,
    #[doc = "3: TX FIFO <= 1/4 empty"]
    Lvl1_4 = 3,
    #[doc = "4: Opposite of full"]
    LvlNotFull = 4,
    #[doc = "5: TX FIFO is empty"]
    LvlEmpty = 5,
    #[doc = "6: TX FIFO <= 1"]
    LvlAlmostEmpty = 6,
    #[doc = "7: TX_FIFO >= (MAX_FIFO_LEN -1)"]
    LvlAlmostFull = 7,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            1 => Some(Txiflsel::Lvl3_4),
            2 => Some(Txiflsel::Lvl1_2),
            3 => Some(Txiflsel::Lvl1_4),
            4 => Some(Txiflsel::LvlNotFull),
            5 => Some(Txiflsel::LvlEmpty),
            6 => Some(Txiflsel::LvlAlmostEmpty),
            7 => Some(Txiflsel::LvlAlmostFull),
            _ => None,
        }
    }
    #[doc = "TX FIFO <= 3/4 empty"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Txiflsel::Lvl3_4
    }
    #[doc = "TX FIFO <= 1/2 empty (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Txiflsel::Lvl1_2
    }
    #[doc = "TX FIFO <= 1/4 empty"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Txiflsel::Lvl1_4
    }
    #[doc = "Opposite of full"]
    #[inline(always)]
    pub fn is_lvl_not_full(&self) -> bool {
        *self == Txiflsel::LvlNotFull
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == Txiflsel::LvlEmpty
    }
    #[doc = "TX FIFO <= 1"]
    #[inline(always)]
    pub fn is_lvl_almost_empty(&self) -> bool {
        *self == Txiflsel::LvlAlmostEmpty
    }
    #[doc = "TX_FIFO >= (MAX_FIFO_LEN -1)"]
    #[inline(always)]
    pub fn is_lvl_almost_full(&self) -> bool {
        *self == Txiflsel::LvlAlmostFull
    }
}
#[doc = "Field `TXIFLSEL` writer - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel, crate::Safe>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX FIFO <= 3/4 empty"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl3_4)
    }
    #[doc = "TX FIFO <= 1/2 empty (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_2)
    }
    #[doc = "TX FIFO <= 1/4 empty"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_4)
    }
    #[doc = "Opposite of full"]
    #[inline(always)]
    pub fn lvl_not_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlNotFull)
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlEmpty)
    }
    #[doc = "TX FIFO <= 1"]
    #[inline(always)]
    pub fn lvl_almost_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlAlmostEmpty)
    }
    #[doc = "TX_FIFO >= (MAX_FIFO_LEN -1)"]
    #[inline(always)]
    pub fn lvl_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlAlmostFull)
    }
}
#[doc = "RX FIFO Flush\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txclr {
    #[doc = "0: Disable FIFO clear"]
    Disable = 0,
    #[doc = "1: Enable FIFO Clear"]
    Enable = 1,
}
impl From<Txclr> for bool {
    #[inline(always)]
    fn from(variant: Txclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCLR` reader - RX FIFO Flush"]
pub type TxclrR = crate::BitReader<Txclr>;
impl TxclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txclr {
        match self.bits {
            false => Txclr::Disable,
            true => Txclr::Enable,
        }
    }
    #[doc = "Disable FIFO clear"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txclr::Disable
    }
    #[doc = "Enable FIFO Clear"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txclr::Enable
    }
}
#[doc = "Field `TXCLR` writer - RX FIFO Flush"]
pub type TxclrW<'a, REG> = crate::BitWriter<'a, REG, Txclr>;
impl<'a, REG> TxclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable FIFO clear"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txclr::Disable)
    }
    #[doc = "Enable FIFO Clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txclr::Enable)
    }
}
#[doc = "FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "1: RX FIFO >= 1/4 full"]
    Lvl1_4 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    Lvl1_2 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    Lvl3_4 = 3,
    #[doc = "4: Opposite of empty"]
    LvlNotEmpty = 4,
    #[doc = "5: RX FIFO is full"]
    LvlFull = 5,
    #[doc = "6: RX_FIFO >= (MAX_FIFO_LEN -1)"]
    LvlAlmostFull = 6,
    #[doc = "7: RX_FIFO <= 1"]
    LvlAlmostEmpty = 7,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            1 => Some(Rxiflsel::Lvl1_4),
            2 => Some(Rxiflsel::Lvl1_2),
            3 => Some(Rxiflsel::Lvl3_4),
            4 => Some(Rxiflsel::LvlNotEmpty),
            5 => Some(Rxiflsel::LvlFull),
            6 => Some(Rxiflsel::LvlAlmostFull),
            7 => Some(Rxiflsel::LvlAlmostEmpty),
            _ => None,
        }
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Rxiflsel::Lvl1_4
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Rxiflsel::Lvl1_2
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Rxiflsel::Lvl3_4
    }
    #[doc = "Opposite of empty"]
    #[inline(always)]
    pub fn is_lvl_not_empty(&self) -> bool {
        *self == Rxiflsel::LvlNotEmpty
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == Rxiflsel::LvlFull
    }
    #[doc = "RX_FIFO >= (MAX_FIFO_LEN -1)"]
    #[inline(always)]
    pub fn is_lvl_almost_full(&self) -> bool {
        *self == Rxiflsel::LvlAlmostFull
    }
    #[doc = "RX_FIFO <= 1"]
    #[inline(always)]
    pub fn is_lvl_almost_empty(&self) -> bool {
        *self == Rxiflsel::LvlAlmostEmpty
    }
}
#[doc = "Field `RXIFLSEL` writer - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel, crate::Safe>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_4)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_2)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl3_4)
    }
    #[doc = "Opposite of empty"]
    #[inline(always)]
    pub fn lvl_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlNotEmpty)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlFull)
    }
    #[doc = "RX_FIFO >= (MAX_FIFO_LEN -1)"]
    #[inline(always)]
    pub fn lvl_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlAlmostFull)
    }
    #[doc = "RX_FIFO <= 1"]
    #[inline(always)]
    pub fn lvl_almost_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlAlmostEmpty)
    }
}
#[doc = "RX FIFO CLEAR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxclr {
    #[doc = "0: Disable FIFO clear"]
    Disable = 0,
    #[doc = "1: Enable FIFO Clear"]
    Enable = 1,
}
impl From<Rxclr> for bool {
    #[inline(always)]
    fn from(variant: Rxclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXCLR` reader - RX FIFO CLEAR"]
pub type RxclrR = crate::BitReader<Rxclr>;
impl RxclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxclr {
        match self.bits {
            false => Rxclr::Disable,
            true => Rxclr::Enable,
        }
    }
    #[doc = "Disable FIFO clear"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxclr::Disable
    }
    #[doc = "Enable FIFO Clear"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxclr::Enable
    }
}
#[doc = "Field `RXCLR` writer - RX FIFO CLEAR"]
pub type RxclrW<'a, REG> = crate::BitWriter<'a, REG, Rxclr>;
impl<'a, REG> RxclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable FIFO clear"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxclr::Disable)
    }
    #[doc = "Enable FIFO Clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxclr::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RX FIFO Flush"]
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - RX FIFO CLEAR"]
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFLS")
            .field("txiflsel", &self.txiflsel())
            .field("rxiflsel", &self.rxiflsel())
            .field("txclr", &self.txclr())
            .field("rxclr", &self.rxclr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bit 3 - RX FIFO Flush"]
    #[inline(always)]
    pub fn txclr(&mut self) -> TxclrW<'_, IflsSpec> {
        TxclrW::new(self, 3)
    }
    #[doc = "Bits 4:6 - FIFO Level Select for generating events (interrupt/dma). Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, IflsSpec> {
        RxiflselW::new(self, 4)
    }
    #[doc = "Bit 7 - RX FIFO CLEAR"]
    #[inline(always)]
    pub fn rxclr(&mut self) -> RxclrW<'_, IflsSpec> {
        RxclrW::new(self, 7)
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IFLS to value 0x22"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x22;
}
