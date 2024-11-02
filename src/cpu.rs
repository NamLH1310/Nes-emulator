const LOOKUP: [Instruction; 256] = [
    Instruction { name: "BRK", operate: Operate::BRK, addr_mode: AddrMode::IMM, cycles: 7 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "ASL", operate: Operate::ASL, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "PHP", operate: Operate::PHP, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "ASL", operate: Operate::ASL, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "ASL", operate: Operate::ASL, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BPL", operate: Operate::BPL, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "ASL", operate: Operate::ASL, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "CLC", operate: Operate::CLC, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ORA", operate: Operate::ORA, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "ASL", operate: Operate::ASL, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "JSR", operate: Operate::JSR, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "BIT", operate: Operate::BIT, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "ROL", operate: Operate::ROL, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "PLP", operate: Operate::PLP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "ROL", operate: Operate::ROL, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "BIT", operate: Operate::BIT, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "ROL", operate: Operate::ROL, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BMI", operate: Operate::BMI, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "ROL", operate: Operate::ROL, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "SEC", operate: Operate::SEC, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "AND", operate: Operate::AND, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "ROL", operate: Operate::ROL, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "RTI", operate: Operate::RTI, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "LSR", operate: Operate::LSR, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "PHA", operate: Operate::PHA, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "LSR", operate: Operate::LSR, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "JMP", operate: Operate::JMP, addr_mode: AddrMode::ABS, cycles: 3 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "LSR", operate: Operate::LSR, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BVC", operate: Operate::BVC, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "LSR", operate: Operate::LSR, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "CLI", operate: Operate::CLI, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "EOR", operate: Operate::EOR, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "LSR", operate: Operate::LSR, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "RTS", operate: Operate::RTS, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "ROR", operate: Operate::ROR, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "PLA", operate: Operate::PLA, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "ROR", operate: Operate::ROR, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "JMP", operate: Operate::JMP, addr_mode: AddrMode::IND, cycles: 5 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "ROR", operate: Operate::ROR, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BVS", operate: Operate::BVS, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "ROR", operate: Operate::ROR, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "SEI", operate: Operate::SEI, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "ADC", operate: Operate::ADC, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "ROR", operate: Operate::ROR, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "STY", operate: Operate::STY, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "STX", operate: Operate::STX, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "DEY", operate: Operate::DEY, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "TXA", operate: Operate::TXA, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "STY", operate: Operate::STY, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "STX", operate: Operate::STX, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "BCC", operate: Operate::BCC, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::IZY, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "STY", operate: Operate::STY, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "STX", operate: Operate::STX, addr_mode: AddrMode::ZPY, cycles: 4 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "TYA", operate: Operate::TYA, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::ABY, cycles: 5 },
    Instruction { name: "TXS", operate: Operate::TXS, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "STA", operate: Operate::STA, addr_mode: AddrMode::ABX, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "LDY", operate: Operate::LDY, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "LDX", operate: Operate::LDX, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "LDY", operate: Operate::LDY, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "LDX", operate: Operate::LDX, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 3 },
    Instruction { name: "TAY", operate: Operate::TAY, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "TAX", operate: Operate::TAX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "LDY", operate: Operate::LDY, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "LDX", operate: Operate::LDX, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "BCS", operate: Operate::BCS, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "LDY", operate: Operate::LDY, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "LDX", operate: Operate::LDX, addr_mode: AddrMode::ZPY, cycles: 4 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "CLV", operate: Operate::CLV, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "TSX", operate: Operate::TSX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "LDY", operate: Operate::LDY, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "LDA", operate: Operate::LDA, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "LDX", operate: Operate::LDX, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "CPY", operate: Operate::CPY, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "CPY", operate: Operate::CPY, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "DEC", operate: Operate::DEC, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "INY", operate: Operate::INY, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "DEX", operate: Operate::DEX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "CPY", operate: Operate::CPY, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "DEC", operate: Operate::DEC, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BNE", operate: Operate::BNE, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "DEC", operate: Operate::DEC, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "CLD", operate: Operate::CLD, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "NOP", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "CMP", operate: Operate::CMP, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "DEC", operate: Operate::DEC, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "CPX", operate: Operate::CPX, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::IZX, cycles: 6 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "CPX", operate: Operate::CPX, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::ZP0, cycles: 3 },
    Instruction { name: "INC", operate: Operate::INC, addr_mode: AddrMode::ZP0, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 5 },
    Instruction { name: "INX", operate: Operate::INX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::IMM, cycles: 2 },
    Instruction { name: "NOP", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::SBC, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "CPX", operate: Operate::CPX, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::ABS, cycles: 4 },
    Instruction { name: "INC", operate: Operate::INC, addr_mode: AddrMode::ABS, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "BEQ", operate: Operate::BEQ, addr_mode: AddrMode::REL, cycles: 2 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::IZY, cycles: 5 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 8 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::ZPX, cycles: 4 },
    Instruction { name: "INC", operate: Operate::INC, addr_mode: AddrMode::ZPX, cycles: 6 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 6 },
    Instruction { name: "SED", operate: Operate::SED, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::ABY, cycles: 4 },
    Instruction { name: "NOP", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 2 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
    Instruction { name: "???", operate: Operate::NOP, addr_mode: AddrMode::IMP, cycles: 4 },
    Instruction { name: "SBC", operate: Operate::SBC, addr_mode: AddrMode::ABX, cycles: 4 },
    Instruction { name: "INC", operate: Operate::INC, addr_mode: AddrMode::ABX, cycles: 7 },
    Instruction { name: "???", operate: Operate::XXX, addr_mode: AddrMode::IMP, cycles: 7 },
];

#[derive(Default)]
pub(crate)  struct CPU<'a> {
    a: u8, // Accumulator register
    x: u8, // X register
    y: u8, // Y register
    stkp: u16, // Stack pointer
    pc: u16, // Program counter
    status: u8, // Status register

    fetched: u8, // Represents the working input value to the ALU
    temp: u16,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8, // Count how many cycles the instruction has remaining
    clock_count: u32, // A global accumulation of the number of clocks

    bus: Option<&'a mut Bus<'a>>,
}

impl<'a> CPU<'a> {
    pub fn new() -> CPU<'a> {
        CPU::default()
    }

    fn write(&mut self, addr: u16, data: u8) {
        match self.bus {
            Some(ref mut bus) => bus.write(addr, data),
            None => {}
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match self.bus {
            Some(ref bus) => bus.read(addr, false),
            None => 0
        }
    }

    fn fetch(&mut self) -> u8 {
        match LOOKUP[self.opcode as usize].addr_mode {
            AddrMode::IMP => {}
            _ => self.fetched = self.read(self.addr_abs),
        }

        self.fetched
    }

    fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            self.cycles = LOOKUP[self.opcode as usize].cycles;

            let additional_cycle = self.exec_addr_mode();

            let additional_cycle = additional_cycle & self.exec_operate();

            self.cycles += additional_cycle;
            self.set_flag(Flag::U, true);
        }

        self.clock_count += 1;
        self.cycles -= 1;
    }
}

enum AddrMode {
    ABS,
    ABX,
    ABY,
    IMM,
    IMP,
    IND,
    IZX,
    IZY,
    REL,
    ZP0,
    ZPX,
    ZPY,
}

enum Operate {
    BRK,
    ORA,
    ASL,
    PHP,
    BPL,
    CLC,
    JSR,
    AND,
    ROL,
    PLP,
    BIT,
    BMI,
    SEC,
    RTI,
    EOR,
    LSR,
    PHA,
    JMP,
    BVC,
    CLI,
    RTS,
    ADC,
    ROR,
    PLA,
    BVS,
    SEI,
    STA,
    STY,
    STX,
    DEY,
    TXA,
    BCC,
    TYA,
    TXS,
    LDY,
    LDA,
    LDX,
    TAY,
    TAX,
    BCS,
    CLV,
    TSX,
    CPY,
    CMP,
    DEC,
    INY,
    DEX,
    BNE,
    CLD,
    NOP,
    CPX,
    SBC,
    INC,
    INX,
    BEQ,
    SED,
    XXX,
}

impl CPU<'_> {
    fn exec_addr_mode(&mut self) -> u8 {
        use crate::cpu::AddrMode::*;

        let opcode = self.opcode as usize;

        match LOOKUP[opcode].addr_mode {
            ABS => self.ABS(),
            ABX => self.ABX(),
            ABY => self.ABY(),
            IMM => self.IMM(),
            IMP => self.IMP(),
            IND => self.IND(),
            IZX => self.IZX(),
            IZY => self.IZY(),
            REL => self.REL(),
            ZP0 => self.ZP0(),
            ZPX => self.ZPX(),
            ZPY => self.ZPY(),
        }
    }

    fn exec_operate(&mut self) -> u8 {
        use crate::cpu::Operate::*;

        let opcode = self.opcode as usize;

        match LOOKUP[opcode].operate {
            BRK => self.BRK(),
            ORA => self.ORA(),
            ASL => self.ASL(),
            PHP => self.PHP(),
            BPL => self.BPL(),
            CLC => self.CLC(),
            JSR => self.JSR(),
            AND => self.AND(),
            ROL => self.ROL(),
            PLP => self.PLP(),
            BIT => self.BIT(),
            BMI => self.BMI(),
            SEC => self.SEC(),
            RTI => self.RTI(),
            EOR => self.EOR(),
            LSR => self.LSR(),
            PHA => self.PHA(),
            JMP => self.JMP(),
            BVC => self.BVC(),
            CLI => self.CLI(),
            RTS => self.RTS(),
            ADC => self.ADC(),
            ROR => self.ROR(),
            PLA => self.PLA(),
            BVS => self.BVS(),
            SEI => self.SEI(),
            STA => self.STA(),
            STY => self.STY(),
            STX => self.STX(),
            DEY => self.DEY(),
            TXA => self.TXA(),
            BCC => self.BCC(),
            TYA => self.TYA(),
            TXS => self.TXS(),
            LDY => self.LDY(),
            LDA => self.LDA(),
            LDX => self.LDX(),
            TAY => self.TAY(),
            TAX => self.TAX(),
            BCS => self.BCS(),
            CLV => self.CLV(),
            TSX => self.TSX(),
            CPY => self.CPY(),
            CMP => self.CMP(),
            DEC => self.DEC(),
            INY => self.INY(),
            DEX => self.DEX(),
            BNE => self.BNE(),
            CLD => self.CLD(),
            NOP => self.NOP(),
            CPX => self.CPX(),
            SBC => self.SBC(),
            INC => self.INC(),
            INX => self.INX(),
            BEQ => self.BEQ(),
            SED => self.SED(),
            XXX => self.XXX(),
        }
    }
}

impl CPU<'_> {
    fn get_flag(&self, flag: Flag) -> u8 {
        match self.status & flag.bits() {
            0 => 0,
            _ => 1
        }
    }

    fn set_flag(&mut self, flag: Flag, v: bool) {
        if v {
            // set flag
            self.status |= flag.bits();
        } else {
            // unset flag
            self.status &= !flag.bits();
        }
    }

    fn irq(&mut self) {
        if self.get_flag(Flag::I) == 0 {
            self.write(0x0100 + self.stkp, ((self.pc >> 8) & 0x00ff) as u8);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp, (self.pc & 0x00ff) as u8);
            self.stkp -= 1;

            self.set_flag(Flag::B, false);
            self.set_flag(Flag::U, true);
            self.set_flag(Flag::I, true);
            self.write(0x0100 + self.stkp, self.status);
            self.stkp -= 1;

            self.addr_abs = 0xfffe;
            let lo = self.read(self.addr_abs) as u16;
            let hi = self.read(self.addr_abs + 1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }
}

// Opcode
impl CPU<'_> {
    fn branch_if(&mut self, condition: bool) -> u8 {
        if condition {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xff00 != self.pc & 0xff00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn ADC(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.a as u16) + (self.fetched as u16) + (self.get_flag(Flag::C) as u16);

        self.set_flag(Flag::C, self.temp > 255);
        self.set_flag(Flag::Z, (self.temp & 0x00ff) == 0);
        self.set_flag(Flag::V, ((!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ self.temp)) & 0x0080) != 0);
        self.set_flag(Flag::N, self.temp & 0x80 != 0);
        self.a = (self.temp & 0x00ff) as u8;

        1
    }

    fn AND(&mut self) -> u8 {
        self.fetch();
        self.a &= self.fetched;
        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, self.a & 0x80 != 0);
        1
    }

    fn ASL(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.fetched as u16) << 1;
        self.set_flag(Flag::C, self.temp & 0xff00 > 0);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x00);
        self.set_flag(Flag::N, self.temp & 0x80 != 0);
        match LOOKUP[self.opcode as usize].addr_mode {
            AddrMode::IMP => self.a = (self.temp & 0x00ff) as u8,
            _ => self.write(self.addr_abs, (self.temp & 0x00ff) as u8)
        }
        0
    }

    fn BCC(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::C) == 0)
    }

    fn BCS(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::C) == 1)
    }

    fn BEQ(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::Z) == 1)
    }

    fn BIT(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.a & self.fetched) as u16;
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x00);
        self.set_flag(Flag::N, self.fetched & (1 << 7) != 0);
        self.set_flag(Flag::V, self.fetched & (1 << 6) != 0);
        0
    }

    fn BMI(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::N) == 1)
    }

    fn BNE(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::Z) == 0)
    }

    fn BPL(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::N) == 0)
    }

    fn BRK(&mut self) -> u8 {
        self.pc += 1;
        self.set_flag(Flag::I, true);
        self.write(0x0100 + self.stkp, ((self.pc >> 8) & 0x00ff) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp, (self.pc & 0x00ff) as u8);
        self.stkp -= 1;

        self.set_flag(Flag::B, true);
        self.write(0x0100 + self.stkp, self.status);
        self.stkp -= 1;
        self.set_flag(Flag::B, false);

        self.pc = ((self.read(0xffff) as u16) << 8) | self.read(0xfffe) as u16;

        0
    }

    fn BVC(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::V) == 0)
    }

    fn BVS(&mut self) -> u8 {
        self.branch_if(self.get_flag(Flag::V) == 1)
    }

    fn CLC(&mut self) -> u8 {
        self.set_flag(Flag::C, false);
        0
    }

    fn CLD(&mut self) -> u8 {
        self.set_flag(Flag::D, false);
        0
    }

    fn CLI(&mut self) -> u8 {
        self.set_flag(Flag::I, false);
        0
    }

    fn CLV(&mut self) -> u8 {
        self.set_flag(Flag::V, false);
        0
    }
    fn CMP(&mut self) -> u8 {
        self.fetch();
        self.temp = self.a as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.a >= self.fetched);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x0000);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        1
    }

    fn CPX(&mut self) -> u8 {
        self.fetch();
        self.temp = self.x as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.x >= self.fetched);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x0000);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        0
    }

    fn CPY(&mut self) -> u8 {
        self.fetch();
        self.temp = self.y as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.y >= self.fetched);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x0000);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        0
    }

    fn DEC(&mut self) -> u8 {
        self.fetch();
        self.temp = self.fetched as u16 - 1;
        self.write(self.addr_abs, (self.temp & 0x00ff) as u8);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x0000);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        0
    }

    fn DEX(&mut self) -> u8 {
        self.x -= 1;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x & 0x80 != 0);
        0
    }

    fn DEY(&mut self) -> u8 {
        self.y -= 1;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.y & 0x80 != 0);
        0
    }

    fn EOR(&mut self) -> u8 {
        self.fetch();
        self.a ^= self.fetched;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, self.a & 0x80 != 0);
        1
    }

    fn INC(&mut self) -> u8 {
        self.fetch();
        self.temp = self.fetched as u16 + 1;
        self.write(self.addr_abs, (self.temp & 0x00ff) as u8);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0x0000);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        0
    }

    fn INX(&mut self) -> u8 {
        self.x += 1;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x & 0x80 != 0);
        0
    }

    fn INY(&mut self) -> u8 {
        self.y += 1;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.x & 0x80 != 0);
        0
    }

    fn JMP(&mut self) -> u8 {
        self.pc = self.addr_abs;
        0
    }

    fn JSR(&mut self) -> u8 { 0 }
    fn LDA(&mut self) -> u8 { 0 }
    fn LDX(&mut self) -> u8 { 0 }
    fn LDY(&mut self) -> u8 { 0 }
    fn LSR(&mut self) -> u8 { 0 }
    fn NOP(&mut self) -> u8 { 0 }
    fn ORA(&mut self) -> u8 { 0 }
    fn PHA(&mut self) -> u8 { 0 }
    fn PHP(&mut self) -> u8 { 0 }
    fn PLA(&mut self) -> u8 { 0 }
    fn PLP(&mut self) -> u8 { 0 }
    fn ROL(&mut self) -> u8 { 0 }
    fn ROR(&mut self) -> u8 { 0 }
    fn RTI(&mut self) -> u8 { 0 }
    fn RTS(&mut self) -> u8 { 0 }

    fn SBC(&mut self) -> u8 {
        self.fetch();
        let value = self.fetched as u16 ^ 0x00ff;
        self.temp = self.a as u16 + value + self.get_flag(Flag::C) as u16;
        self.set_flag(Flag::C, self.temp & 0xff00 != 0);
        self.set_flag(Flag::Z, self.temp & 0x00ff == 0);
        self.set_flag(Flag::V, ((self.temp ^ self.a as u16) & (self.temp ^ value) & 0x0080) != 0);
        self.set_flag(Flag::N, self.temp & 0x0080 != 0);
        self.a = (self.temp & 0x00ff) as u8;
        1
    }

    fn SEC(&mut self) -> u8 { 0 }
    fn SED(&mut self) -> u8 { 0 }
    fn SEI(&mut self) -> u8 { 0 }
    fn STA(&mut self) -> u8 { 0 }
    fn STX(&mut self) -> u8 { 0 }
    fn STY(&mut self) -> u8 { 0 }
    fn TAX(&mut self) -> u8 { 0 }
    fn TAY(&mut self) -> u8 { 0 }
    fn TSX(&mut self) -> u8 { 0 }
    fn TXA(&mut self) -> u8 { 0 }
    fn TXS(&mut self) -> u8 { 0 }
    fn TYA(&mut self) -> u8 { 0 }
    fn XXX(&mut self) -> u8 { 0 }
}

// Addressing mode
impl CPU<'_> {
    fn IMP(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    fn IMM(&mut self) -> u8 {
        self.addr_abs = self.a as u16;
        0
    }

    fn ZP0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }

    fn ZPX(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }

    fn ZPY(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }

    fn REL(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if self.addr_rel & 0x80 != 0 {
            self.addr_rel |= 0xff00;
        }
        0
    }

    fn ABS(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        0
    }


    fn ABX(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = ((hi) << 8) | lo;
        self.addr_abs += self.x as u16;

        if self.addr_abs & 0xff00 != hi << 8 {
            1
        } else {
            0
        }
    }
    fn ABY(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = ((hi) << 8) | lo;
        self.addr_abs += self.y as u16;

        if self.addr_abs & 0xff00 != hi << 8 {
            1
        } else {
            0
        }
    }
    fn IND(&mut self) -> u8 {
        let lo_ptr = self.read(self.pc) as u16;
        self.pc += 1;
        let hi_ptr = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr = (hi_ptr << 8) | lo_ptr;

        self.addr_abs = if lo_ptr == 0x00ff {
            // Page boundary bug
            ((self.read(ptr & 0xff00) as u16) << 8) | (self.read(ptr) as u16)
        } else {
            ((self.read(ptr + 1) as u16) << 8) | (self.read(ptr) as u16)
        };

        0
    }

    fn IZX(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc += 1;
        let lo = self.read(t + self.x as u16) as u16 & 0x00ff;
        let hi = self.read(t + 1 + self.x as u16) as u16 & 0x00ff;
        self.addr_abs = (hi << 8) | lo;
        0
    }

    fn IZY(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = self.read(t & 0x00ff) as u16;
        let hi = self.read((t + 1) & 0x00ff) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if self.addr_abs & 0xff00 != hi << 8 {
            1
        } else {
            0
        }
    }
}


use bitflags::bitflags;

bitflags! {
    struct Flag: u8 {
        const C = 1 << 0; // Carry bit
        const Z = 1 << 1; // Zero
        const I = 1 << 2; // Disable interrupts
        const D = 1 << 3; // Decimal mode
        const B = 1 << 4; // Break
        const U = 1 << 5; // Unused
        const V = 1 << 6; // Overflow
        const N = 1 << 7; // Negative
    }
}


pub(crate) struct Bus<'a> {
    cpu: Option<&'a mut CPU<'a>>,
    ram: Vec<u8>,
}

impl<'a> Bus<'a> {
    pub fn new() -> Bus<'a> {
        Bus {
            cpu: None,
            ram: vec![0; 64 * 1024],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    fn read(&self, addr: u16, read_only: bool) -> u8 {
        match addr {
            0x0000..=0xffff => self.ram[addr as usize],
            _ => 0
        }
    }
}

struct Instruction
{
    name: &'static str,
    operate: Operate,
    addr_mode: AddrMode,
    cycles: u8,
}

pub(crate) fn init() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    unsafe {
        let ptr_cpu = &mut cpu as *mut CPU;
        let ptr_bus = &mut bus as *mut Bus;
        bus.cpu = ptr_cpu.as_mut();
        cpu.bus = ptr_bus.as_mut();
    }
}