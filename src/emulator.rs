use std::fmt;

const REGISITERS_COUNT: usize = 8;

#[derive(Clone)]
enum Register{
    AL,
    AH,
    CL,
    CH,
    DL,
    DH,
    BL,
    BH,
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AL  => write!(f, "AL"),
            Self::AH  => write!(f, "AH"),
            Self::CL  => write!(f, "CL"),
            Self::CH  => write!(f, "CH"),
            Self::DL  => write!(f, "DL"),
            Self::DH  => write!(f, "DH"),
            Self::BL  => write!(f, "BL"),
            Self::BH  => write!(f, "BH"),
            Self::EAX => write!(f, "EAX"),
            Self::ECX => write!(f, "ECX"),
            Self::EDX => write!(f, "EDX"),
            Self::EBX => write!(f, "EBX"),
            Self::ESP => write!(f, "ESP"),
            Self::EBP => write!(f, "EBP"),
            Self::ESI => write!(f, "ESI"),
            Self::EDI => write!(f, "EDI"),
        }
    }
}

impl Register {
    fn remap(&self) -> Self {
        match self {
            Self::AL | Self::AH | Self::EAX => Self::EAX,
            Self::CL | Self::CH | Self::ECX => Self::ECX,
            Self::DL | Self::DH | Self::EDX => Self::EDX,
            Self::BL | Self::BH | Self::EBX => Self::EBX,
            Self::ESP => Self::ESP,
            Self::EBP => Self::EBP,
            Self::ESI => Self::ESI,
            Self::EDI => Self::EDI,
        }
    }
}

pub struct Emulator {
    registers: [u32; REGISITERS_COUNT],
    eflags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Emulator");
        writeln!(f, "<Registers>");
        writeln!(f, "  EAX 0x{:08X}", self.registers[Register::EAX as usize]);
        writeln!(f, "  ECX 0x{:08X}", self.registers[Register::ECX as usize]);
        writeln!(f, "  ECX 0x{:08X}", self.registers[Register::ECX as usize]);
        writeln!(f, "  EDX 0x{:08X}", self.registers[Register::EDX as usize]);
        writeln!(f, "  EBX 0x{:08X}", self.registers[Register::EBX as usize]);
        writeln!(f, "  ESP 0x{:08X}", self.registers[Register::ESP as usize]);
        writeln!(f, "  EBP 0x{:08X}", self.registers[Register::EBP as usize]);
        writeln!(f, "  ESI 0x{:08X}", self.registers[Register::ESI as usize]);
        writeln!(f, "  EDI 0x{:08X}", self.registers[Register::EDI as usize]);
        writeln!(f, "<EFlags>");
        let cf   = if (self.eflags & 0x0000_0001) != 0 {"CF"}else{""};
        let pf   = if (self.eflags & 0x0000_0004) != 0 {"PF"}else{""};
        let af   = if (self.eflags & 0x0000_0010) != 0 {"AF"}else{""};
        let zf   = if (self.eflags & 0x0000_0040) != 0 {"ZF"}else{""};
        let sf   = if (self.eflags & 0x0000_0080) != 0 {"SF"}else{""};
        let tf   = if (self.eflags & 0x0000_0100) != 0 {"TF"}else{""};
        let intf = if (self.eflags & 0x0000_0200) != 0 {"IF"}else{""};
        let df   = if (self.eflags & 0x0000_0400) != 0 {"DF"}else{""};
        let of   = if (self.eflags & 0x0000_0800) != 0 {"OF"}else{""};
        let iopl = if (self.eflags & 0x0000_3000) != 0 {"IOPL"}else{""};
        let nl   = if (self.eflags & 0x0000_4000) != 0 {"NL"}else{""};
        let md   = if (self.eflags & 0x0000_8000) != 0 {"MD"}else{""};
        let rf   = if (self.eflags & 0x0001_0000) != 0 {"RF"}else{""};
        let vm   = if (self.eflags & 0x0002_0000) != 0 {"VM"}else{""};
        let ac   = if (self.eflags & 0x0004_0000) != 0 {"AC"}else{""};
        let vif  = if (self.eflags & 0x0008_0000) != 0 {"VIF"}else{""};
        let vip  = if (self.eflags & 0x0010_0000) != 0 {"VIP"}else{""};
        let id   = if (self.eflags & 0x0020_0000) != 0 {"ID"}else{""};
        let ai   = if (self.eflags & 0x8000_0000) != 0 {"AI"}else{""};
        let mut flags = vec![cf, pf, af, zf, sf, tf, intf, df, of, iopl, nl, md, rf, vm, ac, vif, vip, id, ai];
        flags.retain(|&x| x != "");
        writeln!(f, "  0x{:08X} [{}]", self.eflags, flags.join(" "));
        writeln!(f, "<EIP>");
        writeln!(f, "  0x{:08X}", self.eip);
        writeln!(f, "<Memory>")
    }
}
impl Emulator {
    pub fn new(ram_size: usize, eip: u32, esp: u32) -> Self {
        let mut regs= [0u32; REGISITERS_COUNT];
        regs[Register::ESP as usize] = esp;
        Emulator {
            registers: regs,
            eflags: 0u32,
            memory: Vec::<u8>::with_capacity(ram_size),
            eip: eip,
        }
    }
    pub fn read_binary(&mut self, binary: &[u8]){
        for b in binary.iter() {
            self.memory.push(*b);
        }
    }
    pub fn set_register8(&mut self, index: Register, value: u8){
        let reg = index.remap();
        if let Some(reg) = self.registers.get_mut(reg.clone() as usize){
            let rem = *reg & 0xffffff00;
            *reg = rem | (value as u32);
        } else {
            eprintln!("Error: failed set_register8, set value={value} to {index} register");
        }
    }
    pub fn get_register8(&self, index: Register) -> Result<u8, String> {
        if let Some(reg) = self.registers.get(index.clone() as usize) {
            Ok(*reg as u8)
        } else {
            Err(format!("Error: failed get_register8, specified {index} register"))
        }
    }
    pub fn set_register32(&mut self, index: Register, value: u32){
        if let Some(reg) = self.registers.get_mut(index.clone() as usize){
            *reg = value;
        } else {
            eprintln!("Error: failed set_register32, set value={value} to {index} register");
        }
    }
    pub fn get_register32(&self, index: Register) -> Result<u32, String> {
        if let Some(reg) = self.registers.get(index.clone() as usize) {
            Ok(*reg)
        } else {
            Err(format!("Error: failed get_register8, specified {index} register"))
        }
    }
}

#[cfg(test)]
mod emulator_tests{
    use super::*;
    #[test]
    fn test_read_binary(){
        let mut emu = Emulator::new(1024*1024, 0x7c00, 0x7c00);
        let mut binary = [0u8; 1024];
        for (i, b) in binary.iter_mut().enumerate() {
            *b = i as u8;
        }
        emu.read_binary(&binary);
        for i in 0..binary.len() {
            assert_eq!(binary.get(i), emu.memory.get(i));
        }
    }
    #[test]
    fn test_set_get_register(){
        let mut emu = Emulator::new(1024, 0x7c00, 0x7c00);
        assert_eq!(Ok(0u8),    emu.get_register8(Register::AL));
        assert_eq!(Ok(0u8),    emu.get_register8(Register::AH));

        emu.set_register8(Register::AL, 8u8);
        assert_eq!(Ok(8u8),    emu.get_register8(Register::AL));
        emu.set_register8(Register::AH, 8u8);
        assert_eq!(Ok(8u8),    emu.get_register8(Register::AH));

        assert_eq!(Ok(136u32), emu.get_register32(Register::EAX));

    }
}
