pub trait DsiHostCtrlIo {
    type Error;

    fn write(&mut self, command: DsiWriteCommand) -> Result<(), Self::Error>;
    fn read(&mut self, command: DsiReadCommand, buf: &mut [u8]) -> Result<(), Self::Error>;
}

#[repr(u8)]
#[derive(Debug)]
pub enum DsiReadCommand {
    DcsShort { arg: u8 } = 0x06,

    // xx x101 0-7 arguments
    GenericShortP0 = 0x04,
    GenericShortP1 { arg0: u8 } = 0x14,
    GenericShortP2 { arg0: u8, arg1: u8 } = 0x24,
}

impl DsiReadCommand {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum DsiWriteCommand<'i> {
    DcsShortP0 { arg: u8 } = 0x5,
    DcsShortP1 { arg: u8, data: u8 } = 0x15,
    DcsLongWrite { arg: u8, data: &'i [u8] } = 0x39,

    GenericShortP0 = 0x03,
    GenericShortP1 = 0x13,
    GenericShortP2 = 0x23,
    GenericLongWrite { arg: u8, data: &'i [u8] } = 0x29,

    SetMaximumReturnPacketSize(u16) = 0x37,
}

impl<'i> DsiWriteCommand<'i> {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
