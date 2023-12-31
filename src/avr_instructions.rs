#[derive(Debug, Copy, Clone)]
pub struct TwoRegisters {
    pub Rr: u8,
    pub Rd: u8,
}


#[derive(Debug, Copy, Clone)]
pub struct TwoRegistersPair {
    pub Rd_lower: u8,
    pub Rr_lower: u8,
}



#[derive(Debug, Copy, Clone)]
pub struct OneRegister {
    pub Rd: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct OneRegisterDisplaceMent {
    pub Rd: u8,
    pub q: u8
}

#[derive(Debug, Copy, Clone)]
pub struct OneRegisterConstantAddress {
    pub Rd: u8,
    pub A: u8
}

#[derive(Debug, Copy, Clone)]
pub struct OneRegisterConstantValue {
    pub Rd: u8,
    pub K: u8
}


#[derive(Debug, Copy, Clone)]
pub struct Constant7Bit {
    pub K: u8
}

#[derive(Debug, Copy, Clone)]
pub struct Constant12Bit {
    pub K: u16
}

#[derive(Debug, Copy, Clone)]
pub struct OneRegisterConstantValue16Bit {
    pub Rd: u8,
    pub K: u16
}


#[derive(Debug, Copy, Clone)]
pub struct BitAddressInSREG {
    pub s: u8
}


#[derive(Debug, Copy, Clone)]
pub struct RegisterIdAndBitNumber {
    pub R: u8,
    pub b: u8
}

#[derive(Debug, Copy, Clone)]
pub struct ConstantAndBitNumber {
    pub K: u8,
    pub b: u8
}


#[derive(Debug, Clone, Copy)]
pub enum AvrInstructions {
    ADD(TwoRegisters),
    SBC(TwoRegisters),
    CPC(TwoRegisters),
    LSL(OneRegister),
    MOVW(TwoRegistersPair), // register pair
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
