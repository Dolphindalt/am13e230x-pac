#[doc = "Register `XTALCR2` reader"]
pub type R = crate::R<Xtalcr2Spec>;
#[doc = "Register `XTALCR2` writer"]
pub type W = crate::W<Xtalcr2Spec>;
#[doc = "Field `XIF` reader - XI Initial value deposited before XOSC start. Polarity selection to initialise XI/X1 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
pub type XifR = crate::BitReader;
#[doc = "Field `XIF` writer - XI Initial value deposited before XOSC start. Polarity selection to initialise XI/X1 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
pub type XifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOF` reader - XO Initial value deposited before XOSC start. Polarity selection to initialise XO/X2 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
pub type XofR = crate::BitReader;
#[doc = "Field `XOF` writer - XO Initial value deposited before XOSC start. Polarity selection to initialise XO/X2 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
pub type XofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - XOSC pads initialisation enable. Configures XTAL oscillator pad initilisation. This register has effect only when XOSC is OFF (no SE , no XTAL mode). If this register is set during XOSC off state (XOSCOFF=1 & SE=0); then upon change of these controls this bit gets reset and rearmed"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - XOSC pads initialisation enable. Configures XTAL oscillator pad initilisation. This register has effect only when XOSC is OFF (no SE , no XTAL mode). If this register is set during XOSC off state (XOSCOFF=1 & SE=0); then upon change of these controls this bit gets reset and rearmed"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FKEEPXI` reader - This field when written 0xface allows to hold the force value on XI as programmed on XIF. 0xface: Force on XI is continued as per XIF value regardless of XOSC ON/OFF state only in XTAL mode. In Single ended mode this field has no impact. Any other value: Force on XI is removed with enabling of XOSC as per FEN function."]
pub type FkeepxiR = crate::FieldReader<u16>;
#[doc = "Field `FKEEPXI` writer - This field when written 0xface allows to hold the force value on XI as programmed on XIF. 0xface: Force on XI is continued as per XIF value regardless of XOSC ON/OFF state only in XTAL mode. In Single ended mode this field has no impact. Any other value: Force on XI is removed with enabling of XOSC as per FEN function."]
pub type FkeepxiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bit 0 - XI Initial value deposited before XOSC start. Polarity selection to initialise XI/X1 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
    #[inline(always)]
    pub fn xif(&self) -> XifR {
        XifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XO Initial value deposited before XOSC start. Polarity selection to initialise XO/X2 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
    #[inline(always)]
    pub fn xof(&self) -> XofR {
        XofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC pads initialisation enable. Configures XTAL oscillator pad initilisation. This register has effect only when XOSC is OFF (no SE , no XTAL mode). If this register is set during XOSC off state (XOSCOFF=1 & SE=0); then upon change of these controls this bit gets reset and rearmed"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This field when written 0xface allows to hold the force value on XI as programmed on XIF. 0xface: Force on XI is continued as per XIF value regardless of XOSC ON/OFF state only in XTAL mode. In Single ended mode this field has no impact. Any other value: Force on XI is removed with enabling of XOSC as per FEN function."]
    #[inline(always)]
    pub fn fkeepxi(&self) -> FkeepxiR {
        FkeepxiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTALCR2")
            .field("fen", &self.fen())
            .field("xof", &self.xof())
            .field("fkeepxi", &self.fkeepxi())
            .field("xif", &self.xif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XI Initial value deposited before XOSC start. Polarity selection to initialise XI/X1 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
    #[inline(always)]
    pub fn xif(&mut self) -> XifW<'_, Xtalcr2Spec> {
        XifW::new(self, 0)
    }
    #[doc = "Bit 1 - XO Initial value deposited before XOSC start. Polarity selection to initialise XO/X2 pad of the XOSC before start-up. This value shall be deposited on the pad before XOSC started (XOSCOFF=1). If FEN=0 or XOSC is in XTAL or SE mode; then this value will not be applied to the pad."]
    #[inline(always)]
    pub fn xof(&mut self) -> XofW<'_, Xtalcr2Spec> {
        XofW::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC pads initialisation enable. Configures XTAL oscillator pad initilisation. This register has effect only when XOSC is OFF (no SE , no XTAL mode). If this register is set during XOSC off state (XOSCOFF=1 & SE=0); then upon change of these controls this bit gets reset and rearmed"]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<'_, Xtalcr2Spec> {
        FenW::new(self, 2)
    }
    #[doc = "Bits 16:31 - This field when written 0xface allows to hold the force value on XI as programmed on XIF. 0xface: Force on XI is continued as per XIF value regardless of XOSC ON/OFF state only in XTAL mode. In Single ended mode this field has no impact. Any other value: Force on XI is removed with enabling of XOSC as per FEN function."]
    #[inline(always)]
    pub fn fkeepxi(&mut self) -> FkeepxiW<'_, Xtalcr2Spec> {
        FkeepxiW::new(self, 16)
    }
}
#[doc = "XTAL Control Register for pad init\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtalcr2Spec;
impl crate::RegisterSpec for Xtalcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalcr2::R`](R) reader structure"]
impl crate::Readable for Xtalcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`xtalcr2::W`](W) writer structure"]
impl crate::Writable for Xtalcr2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets XTALCR2 to value 0x03"]
impl crate::Resettable for Xtalcr2Spec {
    const RESET_VALUE: u32 = 0x03;
}
