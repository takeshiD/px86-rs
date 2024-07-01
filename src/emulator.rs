const REGISITERS_COUNT: usize = 8;
enum Register{
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    AL = EAX,
    CL = ECX,
    DL = EDX,
    BL = EBX,
    AH = AL + 4,
    CH = CL + 4,
    DH = DL + 4,
    BH = BL + 4,
}

pub struct Emulator {
    registers: [u32; REGISITERS_COUNT],
    eflags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    fn new(size: usize, eip: u32, esp: u32) -> Self {
        Emulator {
            registers: [0u32; REGISITERS_COUNT],
            eflags: 0u32,
            memory: Vec::with_capacity(size),
            eip: eip,
        }
    }
}
