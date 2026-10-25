#[doc = "Register `WWDTCTL0` reader"]
pub type R = crate::R<Wwdtctl0Spec>;
#[doc = "Register `WWDTCTL0` writer"]
pub type W = crate::W<Wwdtctl0Spec>;
#[doc = "Field `CLKDIV` reader - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Timer Period of the WWDT. These bits select the total watchdog timer count.\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Per {
    #[doc = "0: Total timer count is 2^25"]
    En25 = 0,
    #[doc = "1: Total timer count is 2^21"]
    En21 = 1,
    #[doc = "2: Total timer count is 2^18"]
    En18 = 2,
    #[doc = "3: Total timer count is 2^15"]
    En15 = 3,
    #[doc = "4: Total timer count is 2^12 (default)"]
    En12 = 4,
    #[doc = "5: Total timer count is 2^10"]
    En10 = 5,
    #[doc = "6: Total timer count is 2^8"]
    En8 = 6,
    #[doc = "7: Total timer count is 2^6"]
    En6 = 7,
}
impl From<Per> for u8 {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Per {
    type Ux = u8;
}
impl crate::IsEnum for Per {}
#[doc = "Field `PER` reader - Timer Period of the WWDT. These bits select the total watchdog timer count."]
pub type PerR = crate::FieldReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            0 => Per::En25,
            1 => Per::En21,
            2 => Per::En18,
            3 => Per::En15,
            4 => Per::En12,
            5 => Per::En10,
            6 => Per::En8,
            7 => Per::En6,
            _ => unreachable!(),
        }
    }
    #[doc = "Total timer count is 2^25"]
    #[inline(always)]
    pub fn is_en_25(&self) -> bool {
        *self == Per::En25
    }
    #[doc = "Total timer count is 2^21"]
    #[inline(always)]
    pub fn is_en_21(&self) -> bool {
        *self == Per::En21
    }
    #[doc = "Total timer count is 2^18"]
    #[inline(always)]
    pub fn is_en_18(&self) -> bool {
        *self == Per::En18
    }
    #[doc = "Total timer count is 2^15"]
    #[inline(always)]
    pub fn is_en_15(&self) -> bool {
        *self == Per::En15
    }
    #[doc = "Total timer count is 2^12 (default)"]
    #[inline(always)]
    pub fn is_en_12(&self) -> bool {
        *self == Per::En12
    }
    #[doc = "Total timer count is 2^10"]
    #[inline(always)]
    pub fn is_en_10(&self) -> bool {
        *self == Per::En10
    }
    #[doc = "Total timer count is 2^8"]
    #[inline(always)]
    pub fn is_en_8(&self) -> bool {
        *self == Per::En8
    }
    #[doc = "Total timer count is 2^6"]
    #[inline(always)]
    pub fn is_en_6(&self) -> bool {
        *self == Per::En6
    }
}
#[doc = "Field `PER` writer - Timer Period of the WWDT. These bits select the total watchdog timer count."]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Per, crate::Safe>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Total timer count is 2^25"]
    #[inline(always)]
    pub fn en_25(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En25)
    }
    #[doc = "Total timer count is 2^21"]
    #[inline(always)]
    pub fn en_21(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En21)
    }
    #[doc = "Total timer count is 2^18"]
    #[inline(always)]
    pub fn en_18(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En18)
    }
    #[doc = "Total timer count is 2^15"]
    #[inline(always)]
    pub fn en_15(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En15)
    }
    #[doc = "Total timer count is 2^12 (default)"]
    #[inline(always)]
    pub fn en_12(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En12)
    }
    #[doc = "Total timer count is 2^10"]
    #[inline(always)]
    pub fn en_10(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En10)
    }
    #[doc = "Total timer count is 2^8"]
    #[inline(always)]
    pub fn en_8(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En8)
    }
    #[doc = "Total timer count is 2^6"]
    #[inline(always)]
    pub fn en_6(self) -> &'a mut crate::W<REG> {
        self.variant(Per::En6)
    }
}
#[doc = "Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Window0 {
    #[doc = "0: 0% (No closed Window)"]
    Size0 = 0,
    #[doc = "1: 12.50% of the total timer period is closed window"]
    Size12 = 1,
    #[doc = "2: 18.75% of the total timer period is closed window"]
    Size18 = 2,
    #[doc = "3: 25% of the total timer period is closed window"]
    Size25 = 3,
    #[doc = "4: 50% of the total timer period is closed window"]
    Size50 = 4,
    #[doc = "5: 75% of the total timer period is closed window"]
    Size75 = 5,
    #[doc = "6: 81.25% of the total timer period is closed window"]
    Size81 = 6,
    #[doc = "7: 87.50% of the total timer period is closed window"]
    Size87 = 7,
}
impl From<Window0> for u8 {
    #[inline(always)]
    fn from(variant: Window0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Window0 {
    type Ux = u8;
}
impl crate::IsEnum for Window0 {}
#[doc = "Field `WINDOW0` reader - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Window0R = crate::FieldReader<Window0>;
impl Window0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Window0 {
        match self.bits {
            0 => Window0::Size0,
            1 => Window0::Size12,
            2 => Window0::Size18,
            3 => Window0::Size25,
            4 => Window0::Size50,
            5 => Window0::Size75,
            6 => Window0::Size81,
            7 => Window0::Size87,
            _ => unreachable!(),
        }
    }
    #[doc = "0% (No closed Window)"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        *self == Window0::Size0
    }
    #[doc = "12.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_12(&self) -> bool {
        *self == Window0::Size12
    }
    #[doc = "18.75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_18(&self) -> bool {
        *self == Window0::Size18
    }
    #[doc = "25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_25(&self) -> bool {
        *self == Window0::Size25
    }
    #[doc = "50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_50(&self) -> bool {
        *self == Window0::Size50
    }
    #[doc = "75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_75(&self) -> bool {
        *self == Window0::Size75
    }
    #[doc = "81.25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_81(&self) -> bool {
        *self == Window0::Size81
    }
    #[doc = "87.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_87(&self) -> bool {
        *self == Window0::Size87
    }
}
#[doc = "Field `WINDOW0` writer - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Window0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Window0, crate::Safe>;
impl<'a, REG> Window0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0% (No closed Window)"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size0)
    }
    #[doc = "12.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_12(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size12)
    }
    #[doc = "18.75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_18(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size18)
    }
    #[doc = "25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_25(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size25)
    }
    #[doc = "50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_50(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size50)
    }
    #[doc = "75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_75(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size75)
    }
    #[doc = "81.25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_81(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size81)
    }
    #[doc = "87.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_87(self) -> &'a mut crate::W<REG> {
        self.variant(Window0::Size87)
    }
}
#[doc = "Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Window1 {
    #[doc = "0: 0% (No closed Window)"]
    Size0 = 0,
    #[doc = "1: 12.50% of the total timer period is closed window"]
    Size12 = 1,
    #[doc = "2: 18.75% of the total timer period is closed window"]
    Size18 = 2,
    #[doc = "3: 25% of the total timer period is closed window"]
    Size25 = 3,
    #[doc = "4: 50% of the total timer period is closed window"]
    Size50 = 4,
    #[doc = "5: 75% of the total timer period is closed window"]
    Size75 = 5,
    #[doc = "6: 81.25% of the total timer period is closed window"]
    Size81 = 6,
    #[doc = "7: 87.50% of the total timer period is closed window"]
    Size87 = 7,
}
impl From<Window1> for u8 {
    #[inline(always)]
    fn from(variant: Window1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Window1 {
    type Ux = u8;
}
impl crate::IsEnum for Window1 {}
#[doc = "Field `WINDOW1` reader - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Window1R = crate::FieldReader<Window1>;
impl Window1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Window1 {
        match self.bits {
            0 => Window1::Size0,
            1 => Window1::Size12,
            2 => Window1::Size18,
            3 => Window1::Size25,
            4 => Window1::Size50,
            5 => Window1::Size75,
            6 => Window1::Size81,
            7 => Window1::Size87,
            _ => unreachable!(),
        }
    }
    #[doc = "0% (No closed Window)"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        *self == Window1::Size0
    }
    #[doc = "12.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_12(&self) -> bool {
        *self == Window1::Size12
    }
    #[doc = "18.75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_18(&self) -> bool {
        *self == Window1::Size18
    }
    #[doc = "25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_25(&self) -> bool {
        *self == Window1::Size25
    }
    #[doc = "50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_50(&self) -> bool {
        *self == Window1::Size50
    }
    #[doc = "75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_75(&self) -> bool {
        *self == Window1::Size75
    }
    #[doc = "81.25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_81(&self) -> bool {
        *self == Window1::Size81
    }
    #[doc = "87.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn is_size_87(&self) -> bool {
        *self == Window1::Size87
    }
}
#[doc = "Field `WINDOW1` writer - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Window1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Window1, crate::Safe>;
impl<'a, REG> Window1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0% (No closed Window)"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size0)
    }
    #[doc = "12.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_12(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size12)
    }
    #[doc = "18.75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_18(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size18)
    }
    #[doc = "25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_25(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size25)
    }
    #[doc = "50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_50(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size50)
    }
    #[doc = "75% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_75(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size75)
    }
    #[doc = "81.25% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_81(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size81)
    }
    #[doc = "87.50% of the total timer period is closed window"]
    #[inline(always)]
    pub fn size_87(self) -> &'a mut crate::W<REG> {
        self.variant(Window1::Size87)
    }
}
#[doc = "Window Watchdog Timer Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Window Watchdog Timer Mode. The WWDT will generate a error signal to the ESM when following conditions occur: - Timer Expiration (Timeout) - Reset WWDT during the active window closed period - Keyword violation"]
    Window = 0,
    #[doc = "1: Interval Timer Mode. The WWDT acts as an interval timer. It generates an interrupt on timeout."]
    Interval = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Window Watchdog Timer Mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Window,
            true => Mode::Interval,
        }
    }
    #[doc = "Window Watchdog Timer Mode. The WWDT will generate a error signal to the ESM when following conditions occur: - Timer Expiration (Timeout) - Reset WWDT during the active window closed period - Keyword violation"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == Mode::Window
    }
    #[doc = "Interval Timer Mode. The WWDT acts as an interval timer. It generates an interrupt on timeout."]
    #[inline(always)]
    pub fn is_interval(&self) -> bool {
        *self == Mode::Interval
    }
}
#[doc = "Field `MODE` writer - Window Watchdog Timer Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window Watchdog Timer Mode. The WWDT will generate a error signal to the ESM when following conditions occur: - Timer Expiration (Timeout) - Reset WWDT during the active window closed period - Keyword violation"]
    #[inline(always)]
    pub fn window(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Window)
    }
    #[doc = "Interval Timer Mode. The WWDT acts as an interval timer. It generates an interrupt on timeout."]
    #[inline(always)]
    pub fn interval(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Interval)
    }
}
#[doc = "Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stism {
    #[doc = "0: The WWDT continues to function in Sleep mode."]
    Cont = 0,
    #[doc = "1: The WWDT stops in Sleep mode and resumes where it was stopped after wakeup."]
    Stop = 1,
}
impl From<Stism> for bool {
    #[inline(always)]
    fn from(variant: Stism) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STISM` reader - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
pub type StismR = crate::BitReader<Stism>;
impl StismR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stism {
        match self.bits {
            false => Stism::Cont,
            true => Stism::Stop,
        }
    }
    #[doc = "The WWDT continues to function in Sleep mode."]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Stism::Cont
    }
    #[doc = "The WWDT stops in Sleep mode and resumes where it was stopped after wakeup."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stism::Stop
    }
}
#[doc = "Field `STISM` writer - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
pub type StismW<'a, REG> = crate::BitWriter<'a, REG, Stism>;
impl<'a, REG> StismW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WWDT continues to function in Sleep mode."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Stism::Cont)
    }
    #[doc = "The WWDT stops in Sleep mode and resumes where it was stopped after wakeup."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stism::Stop)
    }
}
#[doc = "Write key: the write only takes effect if KEY is 0xC9\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "201: KEY to allow write access to this register"]
    Unlock = 201,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0xC9"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "KEY to allow write access to this register"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Timer Period of the WWDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn window0(&self) -> Window0R {
        Window0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn window1(&self) -> Window1R {
        Window1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Window Watchdog Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
    #[inline(always)]
    pub fn stism(&self) -> StismR {
        StismR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDTCTL0")
            .field("per", &self.per())
            .field("window0", &self.window0())
            .field("mode", &self.mode())
            .field("stism", &self.stism())
            .field("clkdiv", &self.clkdiv())
            .field("window1", &self.window1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, Wwdtctl0Spec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timer Period of the WWDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, Wwdtctl0Spec> {
        PerW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn window0(&mut self) -> Window0W<'_, Wwdtctl0Spec> {
        Window0W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn window1(&mut self) -> Window1W<'_, Wwdtctl0Spec> {
        Window1W::new(self, 12)
    }
    #[doc = "Bit 16 - Window Watchdog Timer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Wwdtctl0Spec> {
        ModeW::new(self, 16)
    }
    #[doc = "Bit 17 - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
    #[inline(always)]
    pub fn stism(&mut self) -> StismW<'_, Wwdtctl0Spec> {
        StismW::new(self, 17)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0xC9"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Wwdtctl0Spec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdtctl0Spec;
impl crate::RegisterSpec for Wwdtctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtctl0::R`](R) reader structure"]
impl crate::Readable for Wwdtctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`wwdtctl0::W`](W) writer structure"]
impl crate::Writable for Wwdtctl0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WWDTCTL0 to value 0x43"]
impl crate::Resettable for Wwdtctl0Spec {
    const RESET_VALUE: u32 = 0x43;
}
