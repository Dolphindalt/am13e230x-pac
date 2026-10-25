#[doc = "Register `CMDTYPE` reader"]
pub type R = crate::R<CmdtypeSpec>;
#[doc = "Register `CMDTYPE` writer"]
pub type W = crate::W<CmdtypeSpec>;
#[doc = "Command type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    #[doc = "0: No Operation"]
    Noop = 0,
    #[doc = "1: Program"]
    Program = 1,
    #[doc = "2: Erase"]
    Erase = 2,
    #[doc = "3: ReadVerify - Perform a standalone ReadVerify operation."]
    Readverify_SecretEnum = 3,
    #[doc = "4: Mode Change - Perform a mode change only, no other operation."]
    Modechange = 4,
    #[doc = "5: Clear Status - Clear status bits in FW_SMSTAT only."]
    Clearstatus = 5,
    #[doc = "6: Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    Blankverify = 6,
}
impl From<Command> for u8 {
    #[inline(always)]
    fn from(variant: Command) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Command {
    type Ux = u8;
}
impl crate::IsEnum for Command {}
#[doc = "Field `COMMAND` reader - Command type"]
pub type CommandR = crate::FieldReader<Command>;
impl CommandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Command> {
        match self.bits {
            0 => Some(Command::Noop),
            1 => Some(Command::Program),
            2 => Some(Command::Erase),
            3 => Some(Command::Readverify_SecretEnum),
            4 => Some(Command::Modechange),
            5 => Some(Command::Clearstatus),
            6 => Some(Command::Blankverify),
            _ => None,
        }
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == Command::Noop
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == Command::Program
    }
    #[doc = "Erase"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == Command::Erase
    }
    #[doc = "ReadVerify - Perform a standalone ReadVerify operation."]
    #[inline(always)]
    pub fn is_readverify__secret_enum(&self) -> bool {
        *self == Command::Readverify_SecretEnum
    }
    #[doc = "Mode Change - Perform a mode change only, no other operation."]
    #[inline(always)]
    pub fn is_modechange(&self) -> bool {
        *self == Command::Modechange
    }
    #[doc = "Clear Status - Clear status bits in FW_SMSTAT only."]
    #[inline(always)]
    pub fn is_clearstatus(&self) -> bool {
        *self == Command::Clearstatus
    }
    #[doc = "Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    #[inline(always)]
    pub fn is_blankverify(&self) -> bool {
        *self == Command::Blankverify
    }
}
#[doc = "Field `COMMAND` writer - Command type"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 3, Command>;
impl<'a, REG> CommandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Noop)
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Program)
    }
    #[doc = "Erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Erase)
    }
    #[doc = "ReadVerify - Perform a standalone ReadVerify operation."]
    #[inline(always)]
    pub fn readverify__secret_enum(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Readverify_SecretEnum)
    }
    #[doc = "Mode Change - Perform a mode change only, no other operation."]
    #[inline(always)]
    pub fn modechange(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Modechange)
    }
    #[doc = "Clear Status - Clear status bits in FW_SMSTAT only."]
    #[inline(always)]
    pub fn clearstatus(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Clearstatus)
    }
    #[doc = "Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    #[inline(always)]
    pub fn blankverify(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Blankverify)
    }
}
#[doc = "Command size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: Operate on 1 flash word"]
    Oneword = 0,
    #[doc = "1: Operate on 2 flash words"]
    Twoword = 1,
    #[doc = "2: Operate on 4 flash words"]
    Fourword = 2,
    #[doc = "3: Operate on 8 flash words"]
    Eightword = 3,
    #[doc = "4: Operate on a flash sector"]
    Sector = 4,
    #[doc = "5: Operate on an entire flash bank"]
    Bank = 5,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Command size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::Oneword),
            1 => Some(Size::Twoword),
            2 => Some(Size::Fourword),
            3 => Some(Size::Eightword),
            4 => Some(Size::Sector),
            5 => Some(Size::Bank),
            _ => None,
        }
    }
    #[doc = "Operate on 1 flash word"]
    #[inline(always)]
    pub fn is_oneword(&self) -> bool {
        *self == Size::Oneword
    }
    #[doc = "Operate on 2 flash words"]
    #[inline(always)]
    pub fn is_twoword(&self) -> bool {
        *self == Size::Twoword
    }
    #[doc = "Operate on 4 flash words"]
    #[inline(always)]
    pub fn is_fourword(&self) -> bool {
        *self == Size::Fourword
    }
    #[doc = "Operate on 8 flash words"]
    #[inline(always)]
    pub fn is_eightword(&self) -> bool {
        *self == Size::Eightword
    }
    #[doc = "Operate on a flash sector"]
    #[inline(always)]
    pub fn is_sector(&self) -> bool {
        *self == Size::Sector
    }
    #[doc = "Operate on an entire flash bank"]
    #[inline(always)]
    pub fn is_bank(&self) -> bool {
        *self == Size::Bank
    }
}
#[doc = "Field `SIZE` writer - Command size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Operate on 1 flash word"]
    #[inline(always)]
    pub fn oneword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Oneword)
    }
    #[doc = "Operate on 2 flash words"]
    #[inline(always)]
    pub fn twoword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Twoword)
    }
    #[doc = "Operate on 4 flash words"]
    #[inline(always)]
    pub fn fourword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Fourword)
    }
    #[doc = "Operate on 8 flash words"]
    #[inline(always)]
    pub fn eightword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Eightword)
    }
    #[doc = "Operate on a flash sector"]
    #[inline(always)]
    pub fn sector(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Sector)
    }
    #[doc = "Operate on an entire flash bank"]
    #[inline(always)]
    pub fn bank(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Bank)
    }
}
impl R {
    #[doc = "Bits 0:2 - Command type"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Command size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDTYPE")
            .field("command", &self.command())
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Command type"]
    #[inline(always)]
    pub fn command(&mut self) -> CommandW<'_, CmdtypeSpec> {
        CommandW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Command size"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, CmdtypeSpec> {
        SizeW::new(self, 4)
    }
}
#[doc = "Command Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdtypeSpec;
impl crate::RegisterSpec for CmdtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdtype::R`](R) reader structure"]
impl crate::Readable for CmdtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdtype::W`](W) writer structure"]
impl crate::Writable for CmdtypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDTYPE to value 0"]
impl crate::Resettable for CmdtypeSpec {}
