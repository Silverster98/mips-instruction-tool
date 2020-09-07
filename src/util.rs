
pub mod c2i {

    // R type

    fn add(args: [u32; 6]) -> String {
        format!("add ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn addu(args: [u32; 6]) -> String {
        format!("addu ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn sub(args: [u32; 6]) -> String {
        format!("sub ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn subu(args: [u32; 6]) -> String {
        format!("subu ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn slt(args: [u32; 6]) -> String {
        format!("slt ${}, ${}, ${}", args[2], args[1], args[0])
    }

    fn sltu(args: [u32; 6]) -> String {
        format!("sltu ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn sll(args: [u32; 6]) -> String {
        format!("sll ${}, ${}, ${}", args[2], args[1], args[3])
    }

    fn sllv(args: [u32; 6]) -> String {
        format!("sllv ${}, ${}, ${}", args[2], args[1], args[0])
    }

    fn sra(args: [u32; 6]) -> String {
        format!("sra ${}, ${}, ${}", args[2], args[1], args[3])
    }

    fn srav(args: [u32; 6]) -> String {
        format!("srav ${}, ${}, ${}", args[2], args[1], args[0])
    }

    fn srlv(args: [u32; 6]) -> String {
        format!("srlv ${}, ${}, ${}", args[2], args[1], args[0])
    }

    fn srl(args: [u32; 6]) -> String {
        format!("srl ${}, ${}, ${}", args[2], args[1], args[3])
    }

    fn div(args: [u32; 6]) -> String {
        format!("div ${}, ${}", args[0], args[1])
    }

    fn divu(args: [u32; 6]) -> String {
        format!("divu ${}, ${}", args[0], args[1])
    }

    fn mult(args: [u32; 6]) -> String {
        format!("mult ${}, ${}", args[0], args[1])
    }

    fn multu(args: [u32; 6]) -> String {
        format!("multu ${}, ${}", args[0], args[1])
    }

    fn and(args: [u32; 6]) -> String {
        format!("and ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn nor(args: [u32; 6]) -> String {
        format!("nor ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn or(args: [u32; 6]) -> String {
        format!("or ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn xor(args: [u32; 6]) -> String {
        format!("xor ${}, ${}, ${}", args[2], args[0], args[1])
    }

    fn jr(args: [u32; 6]) -> String {
        format!("jr ${}", args[0])
    }

    fn jalr(args: [u32; 6]) -> String {
        format!("jalr ${}, ${}", args[2], args[0])
    }

    fn _break(_args: [u32; 6]) -> String {
        String::from("break")
    }
    
    fn syscall(_args: [u32; 6]) -> String {
        String::from("syscall")
    }

    fn mfhi(args: [u32; 6]) -> String {
        format!("mfhi ${}", args[2])
    }

    fn mflo(args: [u32; 6]) -> String {
        format!("mflo ${}", args[2])
    }

    fn mthi(args: [u32; 6]) -> String {
        format!("mthi ${}", args[0])
    }

    fn mtlo(args: [u32; 6]) -> String {
        format!("mtlo ${}", args[0])
    }

    // I type

    fn addi(args: [u32; 6]) -> String {
        format!("addi ${}, ${}, {:04x}", args[1], args[0], args[4])
    }

    fn addiu(args: [u32; 6]) -> String {
        format!("addiu ${}, ${}, {:04x}", args[1], args[0], args[4])
    }

    fn slti(args: [u32; 6]) -> String {
        format!("slti ${}, ${}, ${:04x}", args[1], args[0], args[4])
    }

    fn sltiu(args: [u32; 6]) -> String {
        format!("sltiu ${}, ${}, ${:04x}", args[1], args[0], args[4])
    }

    fn andi(args: [u32; 6]) -> String {
        format!("andi ${}, ${}, ${:04x}", args[1], args[0], args[4])
    }

    fn lui(args: [u32; 6]) -> String {
        format!("lui ${}, 0x{:04x}", args[1], args[4])
    }

    fn ori(args: [u32; 6]) -> String {
        format!("ori ${}, ${}, 0x{:04x}", args[1], args[0], args[4])
    }

    fn xori(args: [u32; 6]) -> String {
        format!("xori ${}, ${}, ${:04x}", args[1], args[0], args[4])
    }

    fn beq(args: [u32; 6]) -> String {
        format!("beq ${}, ${}, ${:04x}", args[0], args[1], args[4])
    }

    fn bne(args: [u32; 6]) -> String {
        format!("bne ${}, ${}, ${:04x}", args[0], args[1], args[4])
    }

    fn bgez_bltz_bgezal_bltzal(args: [u32; 6]) -> String {
        if args[1] == 1 {
            format!("bgez ${}, ${:04x}", args[0], args[4])
        } else if args[1] == 0 {
            format!("bltz ${}, ${:04x}", args[0], args[4])
        } else if args[1] == 17 {
            format!("bgezal ${}, ${:04x}", args[0], args[4])
        } else if args[1] == 16 {
            format!("bltzal ${}, ${:04x}", args[0], args[4])
        } else {
            String::from("")
        }
    }

    fn bgtz(args: [u32; 6]) -> String {
        format!("bgtz ${}, ${:04x}", args[0], args[4])
    }

    fn blez(args: [u32; 6]) -> String {
        format!("blez ${}, ${:04x}", args[0], args[4])
    }

    fn lb(args: [u32; 6]) -> String {
        format!("lb ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn lbu(args: [u32; 6]) -> String {
        format!("lbu ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn lh(args: [u32; 6]) -> String {
        format!("lh ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn lhu(args: [u32; 6]) -> String {
        format!("lhu ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn lw(args: [u32; 6]) -> String {
        format!("lw ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn sb(args: [u32; 6]) -> String {
        format!("sb ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn sh(args: [u32; 6]) -> String {
        format!("sh ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    fn sw(args: [u32; 6]) -> String {
        format!("sw ${}, 0x${:x}(${})", args[1], args[4], args[0])
    }

    // J type

    fn j(args: [u32; 6]) -> String {
        format!("j 0x{:07x}", args[5] << 2)
    }

    fn jal(args: [u32; 6]) -> String {
        format!("jal 0x{:07x}", args[5] << 2)
    }

    fn eret_mfc0_mtc0(args: [u32; 6]) -> String {
        if args[5] == 0b1_0000_0000_0000_0000_000_011000 {
            String::from("eret")
        } else {
            if args[0] == 0b000_00 {
                format!("mfc0 ${}, ${}, ${}", args[1], args[2], args[4] & 0b111)
            } else if args[0] == 0b001_00 {
                format!("mtc0 ${}, ${}, ${}", args[1], args[2], args[4] & 0b111)
            } else {
                String::from("")
            }
        }
    }

    /// instr code - fn map
    static INST_MAP: phf::Map<&'static str, fn([u32; 6]) -> String> = phf::phf_map! {
        "001000"  => addi,
        "001001"  => addiu,
        "001010"  => slti,
        "001011"  => sltiu,
        "001100"  => andi,
        "001111"  => lui,
        "001101"  => ori,
        "001110"  => xori,
        "000100"  => beq,
        "000101"  => bne,
        "000001"  => bgez_bltz_bgezal_bltzal, // bgez, bltz, BGEZAL, BLTZAL
        "000111"  => bgtz, // bgtz
        "000110"  => blez, // blez
        "100000"  => lb,
        "100100"  => lbu,
        "100001"  => lh,
        "100101"  => lhu,
        "100011"  => lw,
        "101000"  => sb,
        "101001"  => sh,
        "101011"  => sw,
        
        "000010"  => j,
        "000011"  => jal,

        "_100000" => add,
        "_100001" => addu,
        "_100010" => sub,
        "_100011" => subu,
        "_101010" => slt,
        "_000000" => sll,
        "_000100" => sllv,
        "_101011" => sltu,
        "_000111" => srav,
        "_000011" => sra,
        "_000110" => srlv,
        "_000010" => srl,
        "_011010" => div,
        "_011011" => divu,
        "_011000" => mult,
        "_011001" => multu,
        "_100100" => and,
        "_100111" => nor,
        "_100101" => or,
        "_100110" => xor,
        "_001000" => jr,
        "_001001" => jalr,
        "_001101" => _break,
        "_001100" => syscall,
        "_010000" => mfhi,
        "_010010" => mflo,
        "_010001" => mthi,
        "_010011" => mtlo,

        "010000"  => eret_mfc0_mtc0, // eret mfc0 mtc0
    };

    /// convert 32bit hex string to instruction
    pub fn hex2instr(hex: &String) -> String {
        let word = u32::from_str_radix(hex, 16).unwrap();

        let op = (word & 0b111111_00000_00000_00000_00000_000000) >> 26;
        let rs = (word & 0b000000_11111_00000_00000_00000_000000) >> 21;
        let rt = (word & 0b000000_00000_11111_00000_00000_000000) >> 16;
        let rd = (word & 0b000000_00000_00000_11111_00000_000000) >> 11;
        let sa = (word & 0b000000_00000_00000_00000_11111_000000) >> 6;
        let ft = (word & 0b000000_00000_00000_00000_00000_111111) >> 0;
        let imm16 = (word & 0b000000_00000_00000_11111_11111_111111) >> 0;
        let imm26 = (word & 0b000000_11111_11111_11111_11111_111111) >> 0;
        
        if op == 0 {
            if word == 0 {return String::from("nop");}

            let ft_str = format!("_{:06b}", ft);
            if INST_MAP.contains_key(&ft_str[..]) {
                return INST_MAP[&ft_str[..]]([rs, rt, rd, sa, imm16, imm26]);
            } else {
                return String::from("");
            }
        } else {
            let op_str = format!("{:06b}", op);
            if INST_MAP.contains_key(&op_str[..]) {
                return INST_MAP[&format!("{:06b}", op)[..]]([rs, rt, rd, sa, imm16, imm26]);
            } else {
                return String::from("");
            }
        }
    }
}