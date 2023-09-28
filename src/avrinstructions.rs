#[derive(Debug, Clone)]
struct TwoRegisters {
    Rr: u8,
    Rd: u8,
}

#[derive(Debug, Clone)]
struct OneRegister {
    Rd: u8,
}

#[derive(Debug, Clone)]
struct OneRegisterDisplaceMent {
    Rd: u8,
    q: u8
}

#[derive(Debug, Clone)]
struct OneRegisterConstantAddress {
    Rd: u8,
    A: u8
}

#[derive(Debug, Clone)]
struct OneRegisterConstantValue {
    Rd: u8,
    K: u8
}


#[derive(Debug, Clone)]
struct Constant7Bit {
    K: u8
}

#[derive(Debug, Clone)]
struct Constant12Bit {
    K: u16
}

#[derive(Debug, Clone)]
struct OneRegisterConstantValue16Bit {
    Rd: u8,
    K: u16
}


#[derive(Debug, Clone)]
struct BitAddressInSREG {
    s: u8
}


#[derive(Debug, Clone)]
struct RegisterIdAndBitNumber {
    A: u8,
    b: u8
}

#[derive(Debug, Clone)]
struct ConstantAndBitNumber {
    K: u8,
    b: u8
}

#[derive(Debug, Clone)]
pub enum AvrInstructions {
    ADD(TwoRegisters),
    SBC(TwoRegisters),
    CPC(TwoRegisters),
    LSL(OneRegister),
    MOVW(OneRegister), // only r16:r31
    NOP,
    ADC(TwoRegisters),
    SUB(TwoRegisters),
    CPSE(TwoRegisters),
    CP(TwoRegisters),
    ROL(OneRegister),
    AND(TwoRegisters),
    OR(TwoRegisters),
    EOR(TwoRegisters),
    TST(OneRegister),
    CLR(OneRegister),
    MOV(TwoRegisters),
    CPI(OneRegisterConstantValue),
    SBCI(OneRegisterConstantValue),
    SUBI(OneRegisterConstantValue),
    ORI(OneRegisterConstantValue),
    SBR(OneRegisterConstantValue),
    ANDI(OneRegisterConstantValue),
    CBR(OneRegisterConstantValue),
    ADIW(OneRegisterConstantValue), // only r24:r31
    SBIW(OneRegisterConstantValue), // only r24:r31 
    COM(OneRegister),
    NEG(OneRegister),
    INC(OneRegister),
    DEC(OneRegister),
    IJMP,
    ICALL,
    RET,
    RETI,
    SBIC(RegisterIdAndBitNumber),
    SBIS(RegisterIdAndBitNumber),
    SBI(RegisterIdAndBitNumber),
    CBI(RegisterIdAndBitNumber),
    LSR(OneRegister),
    ROR(OneRegister),
    ASR(OneRegister),
    SWAP(OneRegister),
    SET,
    BCLR(BitAddressInSREG),
    SEC,
    CLC,
    SEN,
    CLN,
    SEZ,
    CLZ,
    SEI,
    CLI,
    SES,
    CLS,
    SEV,
    CLV,
    CLT,
    SEH,
    CLH,
    LD(OneRegister),
    LDS(OneRegisterConstantValue16Bit),
    ST(OneRegister),
    STS(OneRegisterConstantValue16Bit),
    LPM(OneRegister),
    SPM,
    PUSH(OneRegister),
    POP(OneRegister),
    SLEEP,
    WDR,
    BREAK,
    IN(OneRegisterConstantAddress),
    OUT(OneRegisterConstantAddress),
    LDD(OneRegisterDisplaceMent),
    STD(OneRegisterDisplaceMent),
    RJMP(Constant12Bit),
    RCALL(Constant12Bit),
    SER(OneRegister),
    LDI(OneRegisterConstantValue),
    SBRC(RegisterIdAndBitNumber),
    SBRS(RegisterIdAndBitNumber),
    BRBS(ConstantAndBitNumber),
    BRBC(ConstantAndBitNumber),
    BREQ(Constant7Bit),
    BRNE(Constant7Bit),
    BRCS(Constant7Bit),
    BRCC(Constant7Bit),
    BRSH(Constant7Bit),
    BRLO(Constant7Bit),
    BRMI(Constant7Bit),
    BRPL(Constant7Bit),
    BRGE(Constant7Bit),
    BRLT(Constant7Bit),
    BRHS(Constant7Bit),
    BRHC(Constant7Bit),
    BRTS(Constant7Bit),
    BRTC(Constant7Bit),
    BRVS(Constant7Bit),
    BRVC(Constant7Bit),
    BRIE(Constant7Bit),
    BRID(Constant7Bit),
    BST(RegisterIdAndBitNumber),
    BLD(RegisterIdAndBitNumber),
}


#[derive(Debug, Default, Clone)]
pub struct AvrInstructionSignals {
    pub rd: u8,
    pub rr: u8,
    pub K: u8,
    pub A: u8,
    pub s: u8,
    pub k_16: u16,
    pub k: u8,
    pub q: u8,
    pub b: u8,
}