#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DmactlSpec>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DmactlSpec>;
#[doc = "DMA request. Software-controlled DMA start. DMAREQ is reset automatically.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmareq {
    #[doc = "0: Default read value"]
    Idle = 0,
    #[doc = "1: DMA transfer request (start DMA)"]
    Request = 1,
}
impl From<Dmareq> for bool {
    #[inline(always)]
    fn from(variant: Dmareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREQ` reader - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
pub type DmareqR = crate::BitReader<Dmareq>;
impl DmareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmareq {
        match self.bits {
            false => Dmareq::Idle,
            true => Dmareq::Request,
        }
    }
    #[doc = "Default read value"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Dmareq::Idle
    }
    #[doc = "DMA transfer request (start DMA)"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == Dmareq::Request
    }
}
#[doc = "Field `DMAREQ` writer - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
pub type DmareqW<'a, REG> = crate::BitWriter<'a, REG, Dmareq>;
impl<'a, REG> DmareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default read value"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Dmareq::Idle)
    }
    #[doc = "DMA transfer request (start DMA)"]
    #[inline(always)]
    pub fn request(self) -> &'a mut crate::W<REG> {
        self.variant(Dmareq::Request)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA channel disabled"]
    Disable = 0,
    #[doc = "1: DMA channel enabled"]
    Enable = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disable,
            true => Dmaen::Enable,
        }
    }
    #[doc = "DMA channel disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
    #[doc = "DMA channel enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA channel disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
    #[doc = "DMA channel enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
}
#[doc = "Automatic DMA channel enable on DMASA, DMADA, DMASZ register write. If channel is configured as SW trigger (DMATCTL=0), the AUTOEN will set the DMAEN and DMAREQ. If channel is configured as HW trigger (DMACTL!=0), the AUTOEN will only set the DMAEN.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaautoen {
    #[doc = "0: No automatic DMA enable"]
    Disable = 0,
    #[doc = "1: Automatic DMA enable on DMASA register write."]
    Dmasa = 1,
    #[doc = "2: Automatic DMA enable on DMADA register write."]
    Dmada = 2,
    #[doc = "3: Automatic DMA enable on DMASZ register write."]
    Dmasz = 3,
}
impl From<Dmaautoen> for u8 {
    #[inline(always)]
    fn from(variant: Dmaautoen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaautoen {
    type Ux = u8;
}
impl crate::IsEnum for Dmaautoen {}
#[doc = "Field `DMAAUTOEN` reader - Automatic DMA channel enable on DMASA, DMADA, DMASZ register write. If channel is configured as SW trigger (DMATCTL=0), the AUTOEN will set the DMAEN and DMAREQ. If channel is configured as HW trigger (DMACTL!=0), the AUTOEN will only set the DMAEN."]
pub type DmaautoenR = crate::FieldReader<Dmaautoen>;
impl DmaautoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaautoen {
        match self.bits {
            0 => Dmaautoen::Disable,
            1 => Dmaautoen::Dmasa,
            2 => Dmaautoen::Dmada,
            3 => Dmaautoen::Dmasz,
            _ => unreachable!(),
        }
    }
    #[doc = "No automatic DMA enable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaautoen::Disable
    }
    #[doc = "Automatic DMA enable on DMASA register write."]
    #[inline(always)]
    pub fn is_dmasa(&self) -> bool {
        *self == Dmaautoen::Dmasa
    }
    #[doc = "Automatic DMA enable on DMADA register write."]
    #[inline(always)]
    pub fn is_dmada(&self) -> bool {
        *self == Dmaautoen::Dmada
    }
    #[doc = "Automatic DMA enable on DMASZ register write."]
    #[inline(always)]
    pub fn is_dmasz(&self) -> bool {
        *self == Dmaautoen::Dmasz
    }
}
#[doc = "Field `DMAAUTOEN` writer - Automatic DMA channel enable on DMASA, DMADA, DMASZ register write. If channel is configured as SW trigger (DMATCTL=0), the AUTOEN will set the DMAEN and DMAREQ. If channel is configured as HW trigger (DMACTL!=0), the AUTOEN will only set the DMAEN."]
pub type DmaautoenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaautoen, crate::Safe>;
impl<'a, REG> DmaautoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No automatic DMA enable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaautoen::Disable)
    }
    #[doc = "Automatic DMA enable on DMASA register write."]
    #[inline(always)]
    pub fn dmasa(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaautoen::Dmasa)
    }
    #[doc = "Automatic DMA enable on DMADA register write."]
    #[inline(always)]
    pub fn dmada(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaautoen::Dmada)
    }
    #[doc = "Automatic DMA enable on DMASZ register write."]
    #[inline(always)]
    pub fn dmasz(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaautoen::Dmasz)
    }
}
#[doc = "Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmapreirq {
    #[doc = "0: Pre-IRQ event disabled."]
    PreirqDisable = 0,
    #[doc = "1: Issure Pre-IRQ event when DMASZ=1"]
    Preirq1 = 1,
    #[doc = "2: Issure Pre-IRQ event when DMASZ=2"]
    Preirq2 = 2,
    #[doc = "3: Issure Pre-IRQ event when DMASZ=4"]
    Preirq4 = 3,
    #[doc = "4: Issure Pre-IRQ event when DMASZ=8"]
    Preirq8 = 4,
    #[doc = "5: Issure Pre-IRQ event when DMASZ=32"]
    Preirq32 = 5,
    #[doc = "6: Issure Pre-IRQ event when DMASZ=64"]
    Preirq64 = 6,
    #[doc = "7: Issure Pre-IRQ event when DMASZ reached the half size point of the original transfer size"]
    PreirqHalf = 7,
}
impl From<Dmapreirq> for u8 {
    #[inline(always)]
    fn from(variant: Dmapreirq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmapreirq {
    type Ux = u8;
}
impl crate::IsEnum for Dmapreirq {}
#[doc = "Field `DMAPREIRQ` reader - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
pub type DmapreirqR = crate::FieldReader<Dmapreirq>;
impl DmapreirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmapreirq {
        match self.bits {
            0 => Dmapreirq::PreirqDisable,
            1 => Dmapreirq::Preirq1,
            2 => Dmapreirq::Preirq2,
            3 => Dmapreirq::Preirq4,
            4 => Dmapreirq::Preirq8,
            5 => Dmapreirq::Preirq32,
            6 => Dmapreirq::Preirq64,
            7 => Dmapreirq::PreirqHalf,
            _ => unreachable!(),
        }
    }
    #[doc = "Pre-IRQ event disabled."]
    #[inline(always)]
    pub fn is_preirq_disable(&self) -> bool {
        *self == Dmapreirq::PreirqDisable
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=1"]
    #[inline(always)]
    pub fn is_preirq_1(&self) -> bool {
        *self == Dmapreirq::Preirq1
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=2"]
    #[inline(always)]
    pub fn is_preirq_2(&self) -> bool {
        *self == Dmapreirq::Preirq2
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=4"]
    #[inline(always)]
    pub fn is_preirq_4(&self) -> bool {
        *self == Dmapreirq::Preirq4
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=8"]
    #[inline(always)]
    pub fn is_preirq_8(&self) -> bool {
        *self == Dmapreirq::Preirq8
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=32"]
    #[inline(always)]
    pub fn is_preirq_32(&self) -> bool {
        *self == Dmapreirq::Preirq32
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=64"]
    #[inline(always)]
    pub fn is_preirq_64(&self) -> bool {
        *self == Dmapreirq::Preirq64
    }
    #[doc = "Issure Pre-IRQ event when DMASZ reached the half size point of the original transfer size"]
    #[inline(always)]
    pub fn is_preirq_half(&self) -> bool {
        *self == Dmapreirq::PreirqHalf
    }
}
#[doc = "Field `DMAPREIRQ` writer - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
pub type DmapreirqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dmapreirq, crate::Safe>;
impl<'a, REG> DmapreirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pre-IRQ event disabled."]
    #[inline(always)]
    pub fn preirq_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::PreirqDisable)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=1"]
    #[inline(always)]
    pub fn preirq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq1)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=2"]
    #[inline(always)]
    pub fn preirq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq2)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=4"]
    #[inline(always)]
    pub fn preirq_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq4)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=8"]
    #[inline(always)]
    pub fn preirq_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq8)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=32"]
    #[inline(always)]
    pub fn preirq_32(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq32)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ=64"]
    #[inline(always)]
    pub fn preirq_64(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::Preirq64)
    }
    #[doc = "Issure Pre-IRQ event when DMASZ reached the half size point of the original transfer size"]
    #[inline(always)]
    pub fn preirq_half(self) -> &'a mut crate::W<REG> {
        self.variant(Dmapreirq::PreirqHalf)
    }
}
#[doc = "DMA source width. This bit selects the source data width as a byte, half word, word, long word or long-long word.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasrcwdth {
    #[doc = "0: Source data width is BYTE (8-bit)"]
    Byte = 0,
    #[doc = "1: Source data width is HALF-WORD (16-bit)"]
    Half = 1,
    #[doc = "2: Source data width is WORD (32-bit)"]
    Word = 2,
    #[doc = "3: Source data width is LONG-WORD (64-bit)"]
    Long = 3,
    #[doc = "4: Source data width is LONG-LONG-WORD (128-bit)"]
    Longlong = 4,
}
impl From<Dmasrcwdth> for u8 {
    #[inline(always)]
    fn from(variant: Dmasrcwdth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmasrcwdth {
    type Ux = u8;
}
impl crate::IsEnum for Dmasrcwdth {}
#[doc = "Field `DMASRCWDTH` reader - DMA source width. This bit selects the source data width as a byte, half word, word, long word or long-long word."]
pub type DmasrcwdthR = crate::FieldReader<Dmasrcwdth>;
impl DmasrcwdthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmasrcwdth> {
        match self.bits {
            0 => Some(Dmasrcwdth::Byte),
            1 => Some(Dmasrcwdth::Half),
            2 => Some(Dmasrcwdth::Word),
            3 => Some(Dmasrcwdth::Long),
            4 => Some(Dmasrcwdth::Longlong),
            _ => None,
        }
    }
    #[doc = "Source data width is BYTE (8-bit)"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dmasrcwdth::Byte
    }
    #[doc = "Source data width is HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Dmasrcwdth::Half
    }
    #[doc = "Source data width is WORD (32-bit)"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dmasrcwdth::Word
    }
    #[doc = "Source data width is LONG-WORD (64-bit)"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == Dmasrcwdth::Long
    }
    #[doc = "Source data width is LONG-LONG-WORD (128-bit)"]
    #[inline(always)]
    pub fn is_longlong(&self) -> bool {
        *self == Dmasrcwdth::Longlong
    }
}
#[doc = "Field `DMASRCWDTH` writer - DMA source width. This bit selects the source data width as a byte, half word, word, long word or long-long word."]
pub type DmasrcwdthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dmasrcwdth>;
impl<'a, REG> DmasrcwdthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source data width is BYTE (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcwdth::Byte)
    }
    #[doc = "Source data width is HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcwdth::Half)
    }
    #[doc = "Source data width is WORD (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcwdth::Word)
    }
    #[doc = "Source data width is LONG-WORD (64-bit)"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcwdth::Long)
    }
    #[doc = "Source data width is LONG-LONG-WORD (128-bit)"]
    #[inline(always)]
    pub fn longlong(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcwdth::Longlong)
    }
}
#[doc = "DMA destination width. This bit selects the destination as a byte, half word, word, long word or long-long word.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadstwdth {
    #[doc = "0: Destination data width is BYTE (8-bit)"]
    Byte = 0,
    #[doc = "1: Destination data width is HALF-WORD (16-bit)"]
    Half = 1,
    #[doc = "2: Destination data width is WORD (32-bit)"]
    Word = 2,
    #[doc = "3: Destination data width is LONG-WORD (64-bit)"]
    Long = 3,
    #[doc = "4: Source data width is LONG-LONG-WORD (128-bit)"]
    Longlong = 4,
}
impl From<Dmadstwdth> for u8 {
    #[inline(always)]
    fn from(variant: Dmadstwdth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmadstwdth {
    type Ux = u8;
}
impl crate::IsEnum for Dmadstwdth {}
#[doc = "Field `DMADSTWDTH` reader - DMA destination width. This bit selects the destination as a byte, half word, word, long word or long-long word."]
pub type DmadstwdthR = crate::FieldReader<Dmadstwdth>;
impl DmadstwdthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmadstwdth> {
        match self.bits {
            0 => Some(Dmadstwdth::Byte),
            1 => Some(Dmadstwdth::Half),
            2 => Some(Dmadstwdth::Word),
            3 => Some(Dmadstwdth::Long),
            4 => Some(Dmadstwdth::Longlong),
            _ => None,
        }
    }
    #[doc = "Destination data width is BYTE (8-bit)"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dmadstwdth::Byte
    }
    #[doc = "Destination data width is HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Dmadstwdth::Half
    }
    #[doc = "Destination data width is WORD (32-bit)"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dmadstwdth::Word
    }
    #[doc = "Destination data width is LONG-WORD (64-bit)"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == Dmadstwdth::Long
    }
    #[doc = "Source data width is LONG-LONG-WORD (128-bit)"]
    #[inline(always)]
    pub fn is_longlong(&self) -> bool {
        *self == Dmadstwdth::Longlong
    }
}
#[doc = "Field `DMADSTWDTH` writer - DMA destination width. This bit selects the destination as a byte, half word, word, long word or long-long word."]
pub type DmadstwdthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dmadstwdth>;
impl<'a, REG> DmadstwdthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Destination data width is BYTE (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstwdth::Byte)
    }
    #[doc = "Destination data width is HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstwdth::Half)
    }
    #[doc = "Destination data width is WORD (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstwdth::Word)
    }
    #[doc = "Destination data width is LONG-WORD (64-bit)"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstwdth::Long)
    }
    #[doc = "Source data width is LONG-LONG-WORD (128-bit)"]
    #[inline(always)]
    pub fn longlong(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstwdth::Longlong)
    }
}
#[doc = "DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasrcincr {
    #[doc = "0: Address is unchanged (+0)"]
    Unchanged = 0,
    #[doc = "2: Decremented by 1 (-1 * DMASRCWDTH)"]
    Decrement = 2,
    #[doc = "3: Incremented by 1 (+1 * DMASRCWDTH)"]
    Increment = 3,
    #[doc = "8: Stride size 2 (+2 * DMASRCWDTH)"]
    Stride2 = 8,
    #[doc = "9: Stride size 3 (+3 * DMASRCWDTH)"]
    Stride3 = 9,
    #[doc = "10: Stride size 4 (+4 * DMASRCWDTH)"]
    Stride4 = 10,
    #[doc = "11: Stride size 5 (+5 * DMASRCWDTH)"]
    Stride5 = 11,
    #[doc = "12: Stride size 6 (+6 * DMASRCWDTH)"]
    Stride6 = 12,
    #[doc = "13: Stride size 7 (+7 * DMASRCWDTH)"]
    Stride7 = 13,
    #[doc = "14: Stride size 8 (+8 * DMASRCWDTH)"]
    Stride8 = 14,
    #[doc = "15: Stride size 9 (+9 * DMASRCWDTH)"]
    Stride9 = 15,
}
impl From<Dmasrcincr> for u8 {
    #[inline(always)]
    fn from(variant: Dmasrcincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmasrcincr {
    type Ux = u8;
}
impl crate::IsEnum for Dmasrcincr {}
#[doc = "Field `DMASRCINCR` reader - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
pub type DmasrcincrR = crate::FieldReader<Dmasrcincr>;
impl DmasrcincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmasrcincr> {
        match self.bits {
            0 => Some(Dmasrcincr::Unchanged),
            2 => Some(Dmasrcincr::Decrement),
            3 => Some(Dmasrcincr::Increment),
            8 => Some(Dmasrcincr::Stride2),
            9 => Some(Dmasrcincr::Stride3),
            10 => Some(Dmasrcincr::Stride4),
            11 => Some(Dmasrcincr::Stride5),
            12 => Some(Dmasrcincr::Stride6),
            13 => Some(Dmasrcincr::Stride7),
            14 => Some(Dmasrcincr::Stride8),
            15 => Some(Dmasrcincr::Stride9),
            _ => None,
        }
    }
    #[doc = "Address is unchanged (+0)"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == Dmasrcincr::Unchanged
    }
    #[doc = "Decremented by 1 (-1 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == Dmasrcincr::Decrement
    }
    #[doc = "Incremented by 1 (+1 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == Dmasrcincr::Increment
    }
    #[doc = "Stride size 2 (+2 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_2(&self) -> bool {
        *self == Dmasrcincr::Stride2
    }
    #[doc = "Stride size 3 (+3 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_3(&self) -> bool {
        *self == Dmasrcincr::Stride3
    }
    #[doc = "Stride size 4 (+4 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_4(&self) -> bool {
        *self == Dmasrcincr::Stride4
    }
    #[doc = "Stride size 5 (+5 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_5(&self) -> bool {
        *self == Dmasrcincr::Stride5
    }
    #[doc = "Stride size 6 (+6 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_6(&self) -> bool {
        *self == Dmasrcincr::Stride6
    }
    #[doc = "Stride size 7 (+7 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_7(&self) -> bool {
        *self == Dmasrcincr::Stride7
    }
    #[doc = "Stride size 8 (+8 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_8(&self) -> bool {
        *self == Dmasrcincr::Stride8
    }
    #[doc = "Stride size 9 (+9 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn is_stride_9(&self) -> bool {
        *self == Dmasrcincr::Stride9
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
pub type DmasrcincrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dmasrcincr>;
impl<'a, REG> DmasrcincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address is unchanged (+0)"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Unchanged)
    }
    #[doc = "Decremented by 1 (-1 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Decrement)
    }
    #[doc = "Incremented by 1 (+1 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Increment)
    }
    #[doc = "Stride size 2 (+2 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride2)
    }
    #[doc = "Stride size 3 (+3 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride3)
    }
    #[doc = "Stride size 4 (+4 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride4)
    }
    #[doc = "Stride size 5 (+5 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride5)
    }
    #[doc = "Stride size 6 (+6 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride6)
    }
    #[doc = "Stride size 7 (+7 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride7)
    }
    #[doc = "Stride size 8 (+8 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride8)
    }
    #[doc = "Stride size 9 (+9 * DMASRCWDTH)"]
    #[inline(always)]
    pub fn stride_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Stride9)
    }
}
#[doc = "DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadstincr {
    #[doc = "0: Address is unchanged (+0)"]
    Unchanged = 0,
    #[doc = "2: Decremented by 1 (-1 * DMADSTWDTH)"]
    Decrement = 2,
    #[doc = "3: Incremented by 1 (+1 * DMADSTWDTH)"]
    Increment = 3,
    #[doc = "8: Stride size 2 (+2 * DMADSTWDTH)"]
    Stride2 = 8,
    #[doc = "9: Stride size 3 (+3 * DMADSTWDTH)"]
    Stride3 = 9,
    #[doc = "10: Stride size 4 (+4 * DMADSTWDTH)"]
    Stride4 = 10,
    #[doc = "11: Stride size 5 (+5 * DMADSTWDTH)"]
    Stride5 = 11,
    #[doc = "12: Stride size 6 (+6 * DMADSTWDTH)"]
    Stride6 = 12,
    #[doc = "13: Stride size 7 (+7 * DMADSTWDTH)"]
    Stride7 = 13,
    #[doc = "14: Stride size 8 (+8 * DMADSTWDTH)"]
    Stride8 = 14,
    #[doc = "15: Stride size 9 (+9 * DMADSTWDTH)"]
    Stride9 = 15,
}
impl From<Dmadstincr> for u8 {
    #[inline(always)]
    fn from(variant: Dmadstincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmadstincr {
    type Ux = u8;
}
impl crate::IsEnum for Dmadstincr {}
#[doc = "Field `DMADSTINCR` reader - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
pub type DmadstincrR = crate::FieldReader<Dmadstincr>;
impl DmadstincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmadstincr> {
        match self.bits {
            0 => Some(Dmadstincr::Unchanged),
            2 => Some(Dmadstincr::Decrement),
            3 => Some(Dmadstincr::Increment),
            8 => Some(Dmadstincr::Stride2),
            9 => Some(Dmadstincr::Stride3),
            10 => Some(Dmadstincr::Stride4),
            11 => Some(Dmadstincr::Stride5),
            12 => Some(Dmadstincr::Stride6),
            13 => Some(Dmadstincr::Stride7),
            14 => Some(Dmadstincr::Stride8),
            15 => Some(Dmadstincr::Stride9),
            _ => None,
        }
    }
    #[doc = "Address is unchanged (+0)"]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == Dmadstincr::Unchanged
    }
    #[doc = "Decremented by 1 (-1 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == Dmadstincr::Decrement
    }
    #[doc = "Incremented by 1 (+1 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == Dmadstincr::Increment
    }
    #[doc = "Stride size 2 (+2 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_2(&self) -> bool {
        *self == Dmadstincr::Stride2
    }
    #[doc = "Stride size 3 (+3 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_3(&self) -> bool {
        *self == Dmadstincr::Stride3
    }
    #[doc = "Stride size 4 (+4 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_4(&self) -> bool {
        *self == Dmadstincr::Stride4
    }
    #[doc = "Stride size 5 (+5 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_5(&self) -> bool {
        *self == Dmadstincr::Stride5
    }
    #[doc = "Stride size 6 (+6 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_6(&self) -> bool {
        *self == Dmadstincr::Stride6
    }
    #[doc = "Stride size 7 (+7 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_7(&self) -> bool {
        *self == Dmadstincr::Stride7
    }
    #[doc = "Stride size 8 (+8 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_8(&self) -> bool {
        *self == Dmadstincr::Stride8
    }
    #[doc = "Stride size 9 (+9 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn is_stride_9(&self) -> bool {
        *self == Dmadstincr::Stride9
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
pub type DmadstincrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dmadstincr>;
impl<'a, REG> DmadstincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address is unchanged (+0)"]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Unchanged)
    }
    #[doc = "Decremented by 1 (-1 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Decrement)
    }
    #[doc = "Incremented by 1 (+1 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Increment)
    }
    #[doc = "Stride size 2 (+2 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride2)
    }
    #[doc = "Stride size 3 (+3 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride3)
    }
    #[doc = "Stride size 4 (+4 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride4)
    }
    #[doc = "Stride size 5 (+5 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride5)
    }
    #[doc = "Stride size 6 (+6 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride6)
    }
    #[doc = "Stride size 7 (+7 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride7)
    }
    #[doc = "Stride size 8 (+8 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride8)
    }
    #[doc = "Stride size 9 (+9 * DMADSTWDTH)"]
    #[inline(always)]
    pub fn stride_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Stride9)
    }
}
#[doc = "DMA Extended Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaem {
    #[doc = "0: Normal mode is related to transfers from SRC to DST"]
    Normal = 0,
    #[doc = "1: Gather mode will read a data from an address table located at SA, and the data is transfered to the DA"]
    Gathermode = 1,
    #[doc = "2: Fill mode will copy the SA register content as data to DA"]
    Fillmode = 2,
    #[doc = "3: Table mode will read an address and data value from SA and write the data to address"]
    Tablemode = 3,
}
impl From<Dmaem> for u8 {
    #[inline(always)]
    fn from(variant: Dmaem) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaem {
    type Ux = u8;
}
impl crate::IsEnum for Dmaem {}
#[doc = "Field `DMAEM` reader - DMA Extended Mode"]
pub type DmaemR = crate::FieldReader<Dmaem>;
impl DmaemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaem {
        match self.bits {
            0 => Dmaem::Normal,
            1 => Dmaem::Gathermode,
            2 => Dmaem::Fillmode,
            3 => Dmaem::Tablemode,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode is related to transfers from SRC to DST"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Dmaem::Normal
    }
    #[doc = "Gather mode will read a data from an address table located at SA, and the data is transfered to the DA"]
    #[inline(always)]
    pub fn is_gathermode(&self) -> bool {
        *self == Dmaem::Gathermode
    }
    #[doc = "Fill mode will copy the SA register content as data to DA"]
    #[inline(always)]
    pub fn is_fillmode(&self) -> bool {
        *self == Dmaem::Fillmode
    }
    #[doc = "Table mode will read an address and data value from SA and write the data to address"]
    #[inline(always)]
    pub fn is_tablemode(&self) -> bool {
        *self == Dmaem::Tablemode
    }
}
#[doc = "Field `DMAEM` writer - DMA Extended Mode"]
pub type DmaemW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaem, crate::Safe>;
impl<'a, REG> DmaemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode is related to transfers from SRC to DST"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaem::Normal)
    }
    #[doc = "Gather mode will read a data from an address table located at SA, and the data is transfered to the DA"]
    #[inline(always)]
    pub fn gathermode(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaem::Gathermode)
    }
    #[doc = "Fill mode will copy the SA register content as data to DA"]
    #[inline(always)]
    pub fn fillmode(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaem::Fillmode)
    }
    #[doc = "Table mode will read an address and data value from SA and write the data to address"]
    #[inline(always)]
    pub fn tablemode(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaem::Tablemode)
    }
}
#[doc = "DMA transfer mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmatm {
    #[doc = "0: Single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated and the DMAEN is cleared."]
    Single = 0,
    #[doc = "1: Block transfer. Each trigger transfers the complete block defined in DMASZ. After the transfer is complete an event can be generated and the DMAEN is cleared."]
    Block = 1,
    #[doc = "2: Repeated single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    Rptsngl = 2,
    #[doc = "3: Repeated block transfer. Each trigger transfers the complete block defined in DMASZ. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    Rptblck = 3,
}
impl From<Dmatm> for u8 {
    #[inline(always)]
    fn from(variant: Dmatm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmatm {
    type Ux = u8;
}
impl crate::IsEnum for Dmatm {}
#[doc = "Field `DMATM` reader - DMA transfer mode"]
pub type DmatmR = crate::FieldReader<Dmatm>;
impl DmatmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatm {
        match self.bits {
            0 => Dmatm::Single,
            1 => Dmatm::Block,
            2 => Dmatm::Rptsngl,
            3 => Dmatm::Rptblck,
            _ => unreachable!(),
        }
    }
    #[doc = "Single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated and the DMAEN is cleared."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Dmatm::Single
    }
    #[doc = "Block transfer. Each trigger transfers the complete block defined in DMASZ. After the transfer is complete an event can be generated and the DMAEN is cleared."]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Dmatm::Block
    }
    #[doc = "Repeated single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    #[inline(always)]
    pub fn is_rptsngl(&self) -> bool {
        *self == Dmatm::Rptsngl
    }
    #[doc = "Repeated block transfer. Each trigger transfers the complete block defined in DMASZ. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    #[inline(always)]
    pub fn is_rptblck(&self) -> bool {
        *self == Dmatm::Rptblck
    }
}
#[doc = "Field `DMATM` writer - DMA transfer mode"]
pub type DmatmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmatm, crate::Safe>;
impl<'a, REG> DmatmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated and the DMAEN is cleared."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatm::Single)
    }
    #[doc = "Block transfer. Each trigger transfers the complete block defined in DMASZ. After the transfer is complete an event can be generated and the DMAEN is cleared."]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatm::Block)
    }
    #[doc = "Repeated single transfer. Each transfers requires a new trigger. When the DMASZ counts down to zero an event can be generated. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    #[inline(always)]
    pub fn rptsngl(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatm::Rptsngl)
    }
    #[doc = "Repeated block transfer. Each trigger transfers the complete block defined in DMASZ. After the last transfer the DMASA, DMADA, DAMSZ registers are restored to its initial value and the DMAEN stays enabled."]
    #[inline(always)]
    pub fn rptblck(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatm::Rptblck)
    }
}
impl R {
    #[doc = "Bit 0 - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Automatic DMA channel enable on DMASA, DMADA, DMASZ register write. If channel is configured as SW trigger (DMATCTL=0), the AUTOEN will set the DMAEN and DMAREQ. If channel is configured as HW trigger (DMACTL!=0), the AUTOEN will only set the DMAEN."]
    #[inline(always)]
    pub fn dmaautoen(&self) -> DmaautoenR {
        DmaautoenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
    #[inline(always)]
    pub fn dmapreirq(&self) -> DmapreirqR {
        DmapreirqR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DMA source width. This bit selects the source data width as a byte, half word, word, long word or long-long word."]
    #[inline(always)]
    pub fn dmasrcwdth(&self) -> DmasrcwdthR {
        DmasrcwdthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DMA destination width. This bit selects the destination as a byte, half word, word, long word or long-long word."]
    #[inline(always)]
    pub fn dmadstwdth(&self) -> DmadstwdthR {
        DmadstwdthR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DmasrcincrR {
        DmasrcincrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DmadstincrR {
        DmadstincrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - DMA Extended Mode"]
    #[inline(always)]
    pub fn dmaem(&self) -> DmaemR {
        DmaemR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DMA transfer mode"]
    #[inline(always)]
    pub fn dmatm(&self) -> DmatmR {
        DmatmR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTL")
            .field("dmareq", &self.dmareq())
            .field("dmaen", &self.dmaen())
            .field("dmasrcwdth", &self.dmasrcwdth())
            .field("dmadstwdth", &self.dmadstwdth())
            .field("dmasrcincr", &self.dmasrcincr())
            .field("dmadstincr", &self.dmadstincr())
            .field("dmatm", &self.dmatm())
            .field("dmaem", &self.dmaem())
            .field("dmapreirq", &self.dmapreirq())
            .field("dmaautoen", &self.dmaautoen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
    #[inline(always)]
    pub fn dmareq(&mut self) -> DmareqW<'_, DmactlSpec> {
        DmareqW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DmactlSpec> {
        DmaenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Automatic DMA channel enable on DMASA, DMADA, DMASZ register write. If channel is configured as SW trigger (DMATCTL=0), the AUTOEN will set the DMAEN and DMAREQ. If channel is configured as HW trigger (DMACTL!=0), the AUTOEN will only set the DMAEN."]
    #[inline(always)]
    pub fn dmaautoen(&mut self) -> DmaautoenW<'_, DmactlSpec> {
        DmaautoenW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
    #[inline(always)]
    pub fn dmapreirq(&mut self) -> DmapreirqW<'_, DmactlSpec> {
        DmapreirqW::new(self, 4)
    }
    #[doc = "Bits 8:10 - DMA source width. This bit selects the source data width as a byte, half word, word, long word or long-long word."]
    #[inline(always)]
    pub fn dmasrcwdth(&mut self) -> DmasrcwdthW<'_, DmactlSpec> {
        DmasrcwdthW::new(self, 8)
    }
    #[doc = "Bits 12:14 - DMA destination width. This bit selects the destination as a byte, half word, word, long word or long-long word."]
    #[inline(always)]
    pub fn dmadstwdth(&mut self) -> DmadstwdthW<'_, DmactlSpec> {
        DmadstwdthW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
    #[inline(always)]
    pub fn dmasrcincr(&mut self) -> DmasrcincrW<'_, DmactlSpec> {
        DmasrcincrW::new(self, 16)
    }
    #[doc = "Bits 20:23 - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
    #[inline(always)]
    pub fn dmadstincr(&mut self) -> DmadstincrW<'_, DmactlSpec> {
        DmadstincrW::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA Extended Mode"]
    #[inline(always)]
    pub fn dmaem(&mut self) -> DmaemW<'_, DmactlSpec> {
        DmaemW::new(self, 24)
    }
    #[doc = "Bits 28:29 - DMA transfer mode"]
    #[inline(always)]
    pub fn dmatm(&mut self) -> DmatmW<'_, DmactlSpec> {
        DmatmW::new(self, 28)
    }
}
#[doc = "DMA Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactlSpec;
impl crate::RegisterSpec for DmactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DmactlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DmactlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DmactlSpec {}
