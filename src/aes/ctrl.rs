#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Output Ready. If 1b, this read-only status bit indicates that an AES output block is available for the CPU to retrieve.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputRdy {
    #[doc = "0: Not Ready"]
    Notready = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<OutputRdy> for bool {
    #[inline(always)]
    fn from(variant: OutputRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_RDY` reader - Output Ready. If 1b, this read-only status bit indicates that an AES output block is available for the CPU to retrieve."]
pub type OutputRdyR = crate::BitReader<OutputRdy>;
impl OutputRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutputRdy {
        match self.bits {
            false => OutputRdy::Notready,
            true => OutputRdy::Ready,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == OutputRdy::Notready
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OutputRdy::Ready
    }
}
#[doc = "Ready for input. If 1b, this read-only status bit indicates that the 16-byte input buffer is empty, and the CPU is permitted to write the next block of data. After reset, this bit is 0. After writing a context, this bit will become 1b.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputRdy {
    #[doc = "0: Not Ready"]
    Notempty = 0,
    #[doc = "1: Ready"]
    Empty = 1,
}
impl From<InputRdy> for bool {
    #[inline(always)]
    fn from(variant: InputRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT_RDY` reader - Ready for input. If 1b, this read-only status bit indicates that the 16-byte input buffer is empty, and the CPU is permitted to write the next block of data. After reset, this bit is 0. After writing a context, this bit will become 1b."]
pub type InputRdyR = crate::BitReader<InputRdy>;
impl InputRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputRdy {
        match self.bits {
            false => InputRdy::Notempty,
            true => InputRdy::Empty,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == InputRdy::Notempty
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == InputRdy::Empty
    }
}
#[doc = "Direction. If set to 1b an encrypt operation is performed. If set to 0b a decrypt operation is performed. Note: This bit must be written with a 1b when CBC-MAC is selected.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Decryption"]
    Decrypt = 0,
    #[doc = "1: Encryption"]
    Encrypt = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction. If set to 1b an encrypt operation is performed. If set to 0b a decrypt operation is performed. Note: This bit must be written with a 1b when CBC-MAC is selected."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Decrypt,
            true => Dir::Encrypt,
        }
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == Dir::Decrypt
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == Dir::Encrypt
    }
}
#[doc = "Field `DIR` writer - Direction. If set to 1b an encrypt operation is performed. If set to 0b a decrypt operation is performed. Note: This bit must be written with a 1b when CBC-MAC is selected."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Decrypt)
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Encrypt)
    }
}
#[doc = "Specifies the encryption strength / key width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keysize {
    #[doc = "1: 128-bit key"]
    K128 = 1,
    #[doc = "3: 256-bit key"]
    K256 = 3,
}
impl From<Keysize> for u8 {
    #[inline(always)]
    fn from(variant: Keysize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keysize {
    type Ux = u8;
}
impl crate::IsEnum for Keysize {}
#[doc = "Field `KEYSIZE` reader - Specifies the encryption strength / key width"]
pub type KeysizeR = crate::FieldReader<Keysize>;
impl KeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keysize> {
        match self.bits {
            1 => Some(Keysize::K128),
            3 => Some(Keysize::K256),
            _ => None,
        }
    }
    #[doc = "128-bit key"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == Keysize::K128
    }
    #[doc = "256-bit key"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Keysize::K256
    }
}
#[doc = "Field `KEYSIZE` writer - Specifies the encryption strength / key width"]
pub type KeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Keysize, crate::Safe>;
impl<'a, REG> KeysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit key"]
    #[inline(always)]
    pub fn k128(self) -> &'a mut crate::W<REG> {
        self.variant(Keysize::K128)
    }
    #[doc = "256-bit key"]
    #[inline(always)]
    pub fn k256(self) -> &'a mut crate::W<REG> {
        self.variant(Keysize::K256)
    }
}
#[doc = "If set to 1b, cipher-block-chaining (CBC) mode is selected.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbc {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Cbc> for bool {
    #[inline(always)]
    fn from(variant: Cbc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBC` reader - If set to 1b, cipher-block-chaining (CBC) mode is selected."]
pub type CbcR = crate::BitReader<Cbc>;
impl CbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbc {
        match self.bits {
            false => Cbc::Disable,
            true => Cbc::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cbc::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cbc::Enable
    }
}
#[doc = "Field `CBC` writer - If set to 1b, cipher-block-chaining (CBC) mode is selected."]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG, Cbc>;
impl<'a, REG> CbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cbc::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cbc::Enable)
    }
}
#[doc = "If set to 1b, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctr {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Ctr> for bool {
    #[inline(always)]
    fn from(variant: Ctr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR` reader - If set to 1b, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CtrR = crate::BitReader<Ctr>;
impl CtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctr {
        match self.bits {
            false => Ctr::Disable,
            true => Ctr::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctr::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctr::Enable
    }
}
#[doc = "Field `CTR` writer - If set to 1b, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CtrW<'a, REG> = crate::BitWriter<'a, REG, Ctr>;
impl<'a, REG> CtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctr::Enable)
    }
}
#[doc = "When the CTR bit is set, specifies the counter width for AES-CTR mode. When the CFB bit is set, specifies the CFB mode feedback width:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrWidth {
    #[doc = "0: 32-bit counter (CFB-128 mode when CFB is selected)"]
    Ctr32 = 0,
    #[doc = "1: 64-bit counter"]
    Ctr64 = 1,
    #[doc = "2: 96-bit counter"]
    Ctr96 = 2,
    #[doc = "3: 128-bit counter"]
    Ctr128 = 3,
}
impl From<CtrWidth> for u8 {
    #[inline(always)]
    fn from(variant: CtrWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrWidth {
    type Ux = u8;
}
impl crate::IsEnum for CtrWidth {}
#[doc = "Field `CTR_WIDTH` reader - When the CTR bit is set, specifies the counter width for AES-CTR mode. When the CFB bit is set, specifies the CFB mode feedback width:"]
pub type CtrWidthR = crate::FieldReader<CtrWidth>;
impl CtrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrWidth {
        match self.bits {
            0 => CtrWidth::Ctr32,
            1 => CtrWidth::Ctr64,
            2 => CtrWidth::Ctr96,
            3 => CtrWidth::Ctr128,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit counter (CFB-128 mode when CFB is selected)"]
    #[inline(always)]
    pub fn is_ctr32(&self) -> bool {
        *self == CtrWidth::Ctr32
    }
    #[doc = "64-bit counter"]
    #[inline(always)]
    pub fn is_ctr64(&self) -> bool {
        *self == CtrWidth::Ctr64
    }
    #[doc = "96-bit counter"]
    #[inline(always)]
    pub fn is_ctr96(&self) -> bool {
        *self == CtrWidth::Ctr96
    }
    #[doc = "128-bit counter"]
    #[inline(always)]
    pub fn is_ctr128(&self) -> bool {
        *self == CtrWidth::Ctr128
    }
}
#[doc = "Field `CTR_WIDTH` writer - When the CTR bit is set, specifies the counter width for AES-CTR mode. When the CFB bit is set, specifies the CFB mode feedback width:"]
pub type CtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtrWidth, crate::Safe>;
impl<'a, REG> CtrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit counter (CFB-128 mode when CFB is selected)"]
    #[inline(always)]
    pub fn ctr32(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::Ctr32)
    }
    #[doc = "64-bit counter"]
    #[inline(always)]
    pub fn ctr64(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::Ctr64)
    }
    #[doc = "96-bit counter"]
    #[inline(always)]
    pub fn ctr96(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::Ctr96)
    }
    #[doc = "128-bit counter"]
    #[inline(always)]
    pub fn ctr128(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::Ctr128)
    }
}
#[doc = "When the CFB bit is set, specifies the CFB mode feedback width:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icm {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Icm> for bool {
    #[inline(always)]
    fn from(variant: Icm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICM` reader - When the CFB bit is set, specifies the CFB mode feedback width:"]
pub type IcmR = crate::BitReader<Icm>;
impl IcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icm {
        match self.bits {
            false => Icm::Disable,
            true => Icm::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Icm::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Icm::Enable
    }
}
#[doc = "Field `ICM` writer - When the CFB bit is set, specifies the CFB mode feedback width:"]
pub type IcmW<'a, REG> = crate::BitWriter<'a, REG, Icm>;
impl<'a, REG> IcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Icm::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Icm::Enable)
    }
}
#[doc = "If set to 1b, AES cipher feedback mode CFB is selected. Use the ctr_width field to specify the feedback width.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfb {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Cfb> for bool {
    #[inline(always)]
    fn from(variant: Cfb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFB` reader - If set to 1b, AES cipher feedback mode CFB is selected. Use the ctr_width field to specify the feedback width."]
pub type CfbR = crate::BitReader<Cfb>;
impl CfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfb {
        match self.bits {
            false => Cfb::Disable,
            true => Cfb::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cfb::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cfb::Enable
    }
}
#[doc = "Field `CFB` writer - If set to 1b, AES cipher feedback mode CFB is selected. Use the ctr_width field to specify the feedback width."]
pub type CfbW<'a, REG> = crate::BitWriter<'a, REG, Cfb>;
impl<'a, REG> CfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cfb::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cfb::Enable)
    }
}
#[doc = "If set to 1b, AES-CBC MAC is selected, the Direction bit must be set to 1 for this mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbcmac {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Cbcmac> for bool {
    #[inline(always)]
    fn from(variant: Cbcmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBCMAC` reader - If set to 1b, AES-CBC MAC is selected, the Direction bit must be set to 1 for this mode."]
pub type CbcmacR = crate::BitReader<Cbcmac>;
impl CbcmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbcmac {
        match self.bits {
            false => Cbcmac::Disable,
            true => Cbcmac::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cbcmac::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cbcmac::Enable
    }
}
#[doc = "Field `CBCMAC` writer - If set to 1b, AES-CBC MAC is selected, the Direction bit must be set to 1 for this mode."]
pub type CbcmacW<'a, REG> = crate::BitWriter<'a, REG, Cbcmac>;
impl<'a, REG> CbcmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cbcmac::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cbcmac::Enable)
    }
}
#[doc = "If not set to 00b, AES-GCM mode is selected, this is a combined mode, using the Galois field multiplier GF(2128) for authentication and AES-CTR mode for encryption, the bits specify the GCM mode: 01b = GHASH with H loaded and Y0-encrypted forced to zero 10b = GHASH with H loaded and Y0-encrypted calculated internally 11b = Autonomous GHASH (both H and Y0-encrypted calculated internally) Note: Besides GCM, the CTR mode bits must also be set to 1b to enable GCM with AES-CTR; if the CTR bit is not set a GHASH (authentication) only operation is performed. A GHASH only operation is only allowed if the GCM mode is set to '01b' and the direction bit is set to '0b'. Other modes may not be selected in combination with GCM. Table 14 below shows the valid combinations for the GCM and CTR mode bits, all other options are invalid and must not be selected.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gcm {
    #[doc = "1: GHASH with H loaded and Y0-encrypted forced to 0."]
    ForceZero = 1,
    #[doc = "2: GHASH with H loaded and Y0-encrypted calculated internally"]
    LoadHashKey = 2,
    #[doc = "3: Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    Autonomous = 3,
}
impl From<Gcm> for u8 {
    #[inline(always)]
    fn from(variant: Gcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gcm {
    type Ux = u8;
}
impl crate::IsEnum for Gcm {}
#[doc = "Field `GCM` reader - If not set to 00b, AES-GCM mode is selected, this is a combined mode, using the Galois field multiplier GF(2128) for authentication and AES-CTR mode for encryption, the bits specify the GCM mode: 01b = GHASH with H loaded and Y0-encrypted forced to zero 10b = GHASH with H loaded and Y0-encrypted calculated internally 11b = Autonomous GHASH (both H and Y0-encrypted calculated internally) Note: Besides GCM, the CTR mode bits must also be set to 1b to enable GCM with AES-CTR; if the CTR bit is not set a GHASH (authentication) only operation is performed. A GHASH only operation is only allowed if the GCM mode is set to '01b' and the direction bit is set to '0b'. Other modes may not be selected in combination with GCM. Table 14 below shows the valid combinations for the GCM and CTR mode bits, all other options are invalid and must not be selected."]
pub type GcmR = crate::FieldReader<Gcm>;
impl GcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gcm> {
        match self.bits {
            1 => Some(Gcm::ForceZero),
            2 => Some(Gcm::LoadHashKey),
            3 => Some(Gcm::Autonomous),
            _ => None,
        }
    }
    #[doc = "GHASH with H loaded and Y0-encrypted forced to 0."]
    #[inline(always)]
    pub fn is_force_zero(&self) -> bool {
        *self == Gcm::ForceZero
    }
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    #[inline(always)]
    pub fn is_load_hash_key(&self) -> bool {
        *self == Gcm::LoadHashKey
    }
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn is_autonomous(&self) -> bool {
        *self == Gcm::Autonomous
    }
}
#[doc = "Field `GCM` writer - If not set to 00b, AES-GCM mode is selected, this is a combined mode, using the Galois field multiplier GF(2128) for authentication and AES-CTR mode for encryption, the bits specify the GCM mode: 01b = GHASH with H loaded and Y0-encrypted forced to zero 10b = GHASH with H loaded and Y0-encrypted calculated internally 11b = Autonomous GHASH (both H and Y0-encrypted calculated internally) Note: Besides GCM, the CTR mode bits must also be set to 1b to enable GCM with AES-CTR; if the CTR bit is not set a GHASH (authentication) only operation is performed. A GHASH only operation is only allowed if the GCM mode is set to '01b' and the direction bit is set to '0b'. Other modes may not be selected in combination with GCM. Table 14 below shows the valid combinations for the GCM and CTR mode bits, all other options are invalid and must not be selected."]
pub type GcmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gcm, crate::Safe>;
impl<'a, REG> GcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GHASH with H loaded and Y0-encrypted forced to 0."]
    #[inline(always)]
    pub fn force_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Gcm::ForceZero)
    }
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    #[inline(always)]
    pub fn load_hash_key(self) -> &'a mut crate::W<REG> {
        self.variant(Gcm::LoadHashKey)
    }
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn autonomous(self) -> &'a mut crate::W<REG> {
        self.variant(Gcm::Autonomous)
    }
}
#[doc = "If set to 1b, AES-CCM is selected, this is a combined mode, using AES for both authentication and encryption. In addition to the CCM bit, the CTR mode bit must be set such that AES-CTR is enabled. Other combinations with CCM are invalid.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccm {
    #[doc = "0: Disable CBC mode"]
    Disable = 0,
    #[doc = "1: Select CBC mode"]
    Enable = 1,
}
impl From<Ccm> for bool {
    #[inline(always)]
    fn from(variant: Ccm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCM` reader - If set to 1b, AES-CCM is selected, this is a combined mode, using AES for both authentication and encryption. In addition to the CCM bit, the CTR mode bit must be set such that AES-CTR is enabled. Other combinations with CCM are invalid."]
pub type CcmR = crate::BitReader<Ccm>;
impl CcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccm {
        match self.bits {
            false => Ccm::Disable,
            true => Ccm::Enable,
        }
    }
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ccm::Disable
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ccm::Enable
    }
}
#[doc = "Field `CCM` writer - If set to 1b, AES-CCM is selected, this is a combined mode, using AES for both authentication and encryption. In addition to the CCM bit, the CTR mode bit must be set such that AES-CTR is enabled. Other combinations with CCM are invalid."]
pub type CcmW<'a, REG> = crate::BitWriter<'a, REG, Ccm>;
impl<'a, REG> CcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CBC mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ccm::Disable)
    }
    #[doc = "Select CBC mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ccm::Enable)
    }
}
#[doc = "Field `CCML` reader - Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CcmlR = crate::FieldReader;
#[doc = "Field `CCML` writer - Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CcmlW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CCMM` reader - Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-39 always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmmR = crate::FieldReader;
#[doc = "Field `CCMM` writer - Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-39 always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmmW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `OFB_GCM_CCM_CONT` reader - This bit has a dual use, depending on the selection of CCM/GCM, see bits \\[18:16\\]. If CCM/GCM is not selected: If this bit is set to 1b, full block AES output feedback mode (OFB-128) is selected. If CCM/GCM is selected: Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
pub type OfbGcmCcmContR = crate::BitReader;
#[doc = "Field `OFB_GCM_CCM_CONT` writer - This bit has a dual use, depending on the selection of CCM/GCM, see bits \\[18:16\\]. If CCM/GCM is not selected: If this bit is set to 1b, full block AES output feedback mode (OFB-128) is selected. If CCM/GCM is selected: Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
pub type OfbGcmCcmContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_DIGEST` reader - Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to 1b to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Note: Interruption can only be done on full block (128 bits) boundaries. The minimum number of remaining bytes to resume and finalize the operation, must be greater than or equal to 1."]
pub type GetDigestR = crate::BitReader;
#[doc = "Field `GET_DIGEST` writer - Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to 1b to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Note: Interruption can only be done on full block (128 bits) boundaries. The minimum number of remaining bytes to resume and finalize the operation, must be greater than or equal to 1."]
pub type GetDigestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCM_CONT` reader - Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
pub type GcmContR = crate::BitReader;
#[doc = "Field `GCM_CONT` writer - Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
pub type GcmContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAVE_CNTXT` reader - This bit is used to indicate that an authentication TAG or result IV needs to be stored as a result context. If this bit is set, context output DMA and/or interrupt will be asserted if the operation is finished, and related signals are enabled. Typically, this value must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine will hold its full context until the TAG and/or IV registers are read. Only after reading the TAG or IV, a new DMA request for a new (input) context will be asserted. If this bit is not set, the engine will assert the context input DMA request signal directly after starting to process the last block with the current context."]
pub type SaveCntxtR = crate::BitReader;
#[doc = "Field `SAVE_CNTXT` writer - This bit is used to indicate that an authentication TAG or result IV needs to be stored as a result context. If this bit is set, context output DMA and/or interrupt will be asserted if the operation is finished, and related signals are enabled. Typically, this value must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine will hold its full context until the TAG and/or IV registers are read. Only after reading the TAG or IV, a new DMA request for a new (input) context will be asserted. If this bit is not set, the engine will assert the context input DMA request signal directly after starting to process the last block with the current context."]
pub type SaveCntxtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "If 1b, this read-only status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SavedCntxtRdy {
    #[doc = "0: Not ready"]
    Notready = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<SavedCntxtRdy> for bool {
    #[inline(always)]
    fn from(variant: SavedCntxtRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAVED_CNTXT_RDY` reader - If 1b, this read-only status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
pub type SavedCntxtRdyR = crate::BitReader<SavedCntxtRdy>;
impl SavedCntxtRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SavedCntxtRdy {
        match self.bits {
            false => SavedCntxtRdy::Notready,
            true => SavedCntxtRdy::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == SavedCntxtRdy::Notready
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SavedCntxtRdy::Ready
    }
}
#[doc = "If 1b, this read-only status bit indicates that the context data registers can be overwritten, and the CPU is permitted to write the next context.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CntxtRdy {
    #[doc = "0: Not ready"]
    Notready = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<CntxtRdy> for bool {
    #[inline(always)]
    fn from(variant: CntxtRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTXT_RDY` reader - If 1b, this read-only status bit indicates that the context data registers can be overwritten, and the CPU is permitted to write the next context."]
pub type CntxtRdyR = crate::BitReader<CntxtRdy>;
impl CntxtRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CntxtRdy {
        match self.bits {
            false => CntxtRdy::Notready,
            true => CntxtRdy::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == CntxtRdy::Notready
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CntxtRdy::Ready
    }
}
impl R {
    #[doc = "Bit 0 - Output Ready. If 1b, this read-only status bit indicates that an AES output block is available for the CPU to retrieve."]
    #[inline(always)]
    pub fn output_rdy(&self) -> OutputRdyR {
        OutputRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ready for input. If 1b, this read-only status bit indicates that the 16-byte input buffer is empty, and the CPU is permitted to write the next block of data. After reset, this bit is 0. After writing a context, this bit will become 1b."]
    #[inline(always)]
    pub fn input_rdy(&self) -> InputRdyR {
        InputRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction. If set to 1b an encrypt operation is performed. If set to 0b a decrypt operation is performed. Note: This bit must be written with a 1b when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Specifies the encryption strength / key width"]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - If set to 1b, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If set to 1b, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - When the CTR bit is set, specifies the counter width for AES-CTR mode. When the CFB bit is set, specifies the CFB mode feedback width:"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CtrWidthR {
        CtrWidthR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - When the CFB bit is set, specifies the CFB mode feedback width:"]
    #[inline(always)]
    pub fn icm(&self) -> IcmR {
        IcmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set to 1b, AES cipher feedback mode CFB is selected. Use the ctr_width field to specify the feedback width."]
    #[inline(always)]
    pub fn cfb(&self) -> CfbR {
        CfbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - If set to 1b, AES-CBC MAC is selected, the Direction bit must be set to 1 for this mode."]
    #[inline(always)]
    pub fn cbcmac(&self) -> CbcmacR {
        CbcmacR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - If not set to 00b, AES-GCM mode is selected, this is a combined mode, using the Galois field multiplier GF(2128) for authentication and AES-CTR mode for encryption, the bits specify the GCM mode: 01b = GHASH with H loaded and Y0-encrypted forced to zero 10b = GHASH with H loaded and Y0-encrypted calculated internally 11b = Autonomous GHASH (both H and Y0-encrypted calculated internally) Note: Besides GCM, the CTR mode bits must also be set to 1b to enable GCM with AES-CTR; if the CTR bit is not set a GHASH (authentication) only operation is performed. A GHASH only operation is only allowed if the GCM mode is set to '01b' and the direction bit is set to '0b'. Other modes may not be selected in combination with GCM. Table 14 below shows the valid combinations for the GCM and CTR mode bits, all other options are invalid and must not be selected."]
    #[inline(always)]
    pub fn gcm(&self) -> GcmR {
        GcmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - If set to 1b, AES-CCM is selected, this is a combined mode, using AES for both authentication and encryption. In addition to the CCM bit, the CTR mode bit must be set such that AES-CTR is enabled. Other combinations with CCM are invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CcmR {
        CcmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccml(&self) -> CcmlR {
        CcmlR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-39 always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccmm(&self) -> CcmmR {
        CcmmR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 26 - This bit has a dual use, depending on the selection of CCM/GCM, see bits \\[18:16\\]. If CCM/GCM is not selected: If this bit is set to 1b, full block AES output feedback mode (OFB-128) is selected. If CCM/GCM is selected: Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn ofb_gcm_ccm_cont(&self) -> OfbGcmCcmContR {
        OfbGcmCcmContR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to 1b to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Note: Interruption can only be done on full block (128 bits) boundaries. The minimum number of remaining bytes to resume and finalize the operation, must be greater than or equal to 1."]
    #[inline(always)]
    pub fn get_digest(&self) -> GetDigestR {
        GetDigestR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn gcm_cont(&self) -> GcmContR {
        GcmContR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to indicate that an authentication TAG or result IV needs to be stored as a result context. If this bit is set, context output DMA and/or interrupt will be asserted if the operation is finished, and related signals are enabled. Typically, this value must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine will hold its full context until the TAG and/or IV registers are read. Only after reading the TAG or IV, a new DMA request for a new (input) context will be asserted. If this bit is not set, the engine will assert the context input DMA request signal directly after starting to process the last block with the current context."]
    #[inline(always)]
    pub fn save_cntxt(&self) -> SaveCntxtR {
        SaveCntxtR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If 1b, this read-only status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
    #[inline(always)]
    pub fn saved_cntxt_rdy(&self) -> SavedCntxtRdyR {
        SavedCntxtRdyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If 1b, this read-only status bit indicates that the context data registers can be overwritten, and the CPU is permitted to write the next context."]
    #[inline(always)]
    pub fn cntxt_rdy(&self) -> CntxtRdyR {
        CntxtRdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("output_rdy", &self.output_rdy())
            .field("input_rdy", &self.input_rdy())
            .field("dir", &self.dir())
            .field("keysize", &self.keysize())
            .field("cbc", &self.cbc())
            .field("ctr", &self.ctr())
            .field("ctr_width", &self.ctr_width())
            .field("icm", &self.icm())
            .field("cfb", &self.cfb())
            .field("cbcmac", &self.cbcmac())
            .field("gcm", &self.gcm())
            .field("ccm", &self.ccm())
            .field("ccml", &self.ccml())
            .field("ccmm", &self.ccmm())
            .field("ofb_gcm_ccm_cont", &self.ofb_gcm_ccm_cont())
            .field("get_digest", &self.get_digest())
            .field("gcm_cont", &self.gcm_cont())
            .field("save_cntxt", &self.save_cntxt())
            .field("saved_cntxt_rdy", &self.saved_cntxt_rdy())
            .field("cntxt_rdy", &self.cntxt_rdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Direction. If set to 1b an encrypt operation is performed. If set to 0b a decrypt operation is performed. Note: This bit must be written with a 1b when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CtrlSpec> {
        DirW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Specifies the encryption strength / key width"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KeysizeW<'_, CtrlSpec> {
        KeysizeW::new(self, 3)
    }
    #[doc = "Bit 5 - If set to 1b, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, CtrlSpec> {
        CbcW::new(self, 5)
    }
    #[doc = "Bit 6 - If set to 1b, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<'_, CtrlSpec> {
        CtrW::new(self, 6)
    }
    #[doc = "Bits 7:8 - When the CTR bit is set, specifies the counter width for AES-CTR mode. When the CFB bit is set, specifies the CFB mode feedback width:"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CtrWidthW<'_, CtrlSpec> {
        CtrWidthW::new(self, 7)
    }
    #[doc = "Bit 9 - When the CFB bit is set, specifies the CFB mode feedback width:"]
    #[inline(always)]
    pub fn icm(&mut self) -> IcmW<'_, CtrlSpec> {
        IcmW::new(self, 9)
    }
    #[doc = "Bit 10 - If set to 1b, AES cipher feedback mode CFB is selected. Use the ctr_width field to specify the feedback width."]
    #[inline(always)]
    pub fn cfb(&mut self) -> CfbW<'_, CtrlSpec> {
        CfbW::new(self, 10)
    }
    #[doc = "Bit 15 - If set to 1b, AES-CBC MAC is selected, the Direction bit must be set to 1 for this mode."]
    #[inline(always)]
    pub fn cbcmac(&mut self) -> CbcmacW<'_, CtrlSpec> {
        CbcmacW::new(self, 15)
    }
    #[doc = "Bits 16:17 - If not set to 00b, AES-GCM mode is selected, this is a combined mode, using the Galois field multiplier GF(2128) for authentication and AES-CTR mode for encryption, the bits specify the GCM mode: 01b = GHASH with H loaded and Y0-encrypted forced to zero 10b = GHASH with H loaded and Y0-encrypted calculated internally 11b = Autonomous GHASH (both H and Y0-encrypted calculated internally) Note: Besides GCM, the CTR mode bits must also be set to 1b to enable GCM with AES-CTR; if the CTR bit is not set a GHASH (authentication) only operation is performed. A GHASH only operation is only allowed if the GCM mode is set to '01b' and the direction bit is set to '0b'. Other modes may not be selected in combination with GCM. Table 14 below shows the valid combinations for the GCM and CTR mode bits, all other options are invalid and must not be selected."]
    #[inline(always)]
    pub fn gcm(&mut self) -> GcmW<'_, CtrlSpec> {
        GcmW::new(self, 16)
    }
    #[doc = "Bit 18 - If set to 1b, AES-CCM is selected, this is a combined mode, using AES for both authentication and encryption. In addition to the CCM bit, the CTR mode bit must be set such that AES-CTR is enabled. Other combinations with CCM are invalid."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CcmW<'_, CtrlSpec> {
        CcmW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccml(&mut self) -> CcmlW<'_, CtrlSpec> {
        CcmlW::new(self, 19)
    }
    #[doc = "Bits 22:24 - Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-39 always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccmm(&mut self) -> CcmmW<'_, CtrlSpec> {
        CcmmW::new(self, 22)
    }
    #[doc = "Bit 26 - This bit has a dual use, depending on the selection of CCM/GCM, see bits \\[18:16\\]. If CCM/GCM is not selected: If this bit is set to 1b, full block AES output feedback mode (OFB-128) is selected. If CCM/GCM is selected: Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn ofb_gcm_ccm_cont(&mut self) -> OfbGcmCcmContW<'_, CtrlSpec> {
        OfbGcmCcmContW::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to 1b to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Note: Interruption can only be done on full block (128 bits) boundaries. The minimum number of remaining bytes to resume and finalize the operation, must be greater than or equal to 1."]
    #[inline(always)]
    pub fn get_digest(&mut self) -> GetDigestW<'_, CtrlSpec> {
        GetDigestW::new(self, 27)
    }
    #[doc = "Bit 28 - Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to 1b together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn gcm_cont(&mut self) -> GcmContW<'_, CtrlSpec> {
        GcmContW::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to indicate that an authentication TAG or result IV needs to be stored as a result context. If this bit is set, context output DMA and/or interrupt will be asserted if the operation is finished, and related signals are enabled. Typically, this value must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine will hold its full context until the TAG and/or IV registers are read. Only after reading the TAG or IV, a new DMA request for a new (input) context will be asserted. If this bit is not set, the engine will assert the context input DMA request signal directly after starting to process the last block with the current context."]
    #[inline(always)]
    pub fn save_cntxt(&mut self) -> SaveCntxtW<'_, CtrlSpec> {
        SaveCntxtW::new(self, 29)
    }
}
#[doc = "Input/Output Buffer Control and Mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRL to value 0x8000_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
