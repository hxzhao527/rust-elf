//! Optional module for getting string representations of ELF constants
use crate::abi;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    format,
    string::{String, ToString},
};

pub fn e_osabi_to_str(e_osabi: u8) -> Option<&'static str> {
    match e_osabi {
        abi::ELFOSABI_SYSV => Some("ELFOSABI_SYSV"),
        abi::ELFOSABI_HPUX => Some("ELFOSABI_HPUX"),
        abi::ELFOSABI_NETBSD => Some("ELFOSABI_NETBSD"),
        abi::ELFOSABI_LINUX => Some("ELFOSABI_LINUX"),
        abi::ELFOSABI_SOLARIS => Some("ELFOSABI_SOLARIS"),
        abi::ELFOSABI_AIX => Some("ELFOSABI_AIX"),
        abi::ELFOSABI_IRIX => Some("ELFOSABI_IRIX"),
        abi::ELFOSABI_FREEBSD => Some("ELFOSABI_FREEBSD"),
        abi::ELFOSABI_TRU64 => Some("ELFOSABI_TRU64"),
        abi::ELFOSABI_MODESTO => Some("ELFOSABI_MODESTO"),
        abi::ELFOSABI_OPENBSD => Some("ELFOSABI_OPENBSD"),
        abi::ELFOSABI_OPENVMS => Some("ELFOSABI_OPENVMS"),
        abi::ELFOSABI_NSK => Some("ELFOSABI_NSK"),
        abi::ELFOSABI_AROS => Some("ELFOSABI_AROS"),
        abi::ELFOSABI_FENIXOS => Some("ELFOSABI_FENIXOS"),
        abi::ELFOSABI_CLOUDABI => Some("ELFOSABI_CLOUDABI"),
        abi::ELFOSABI_OPENVOS => Some("ELFOSABI_OPENVOS"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn e_osabi_to_string(e_osabi: u8) -> String {
    match e_osabi_to_str(e_osabi) {
        Some(s) => s.to_string(),
        None => format!("e_osabi({e_osabi:#x})"),
    }
}

pub fn e_type_to_human_str(e_type: u16) -> Option<&'static str> {
    match e_type {
        abi::ET_NONE => Some("No file type"),
        abi::ET_REL => Some("Relocatable file"),
        abi::ET_EXEC => Some("Executable file"),
        abi::ET_DYN => Some("Shared object file"),
        abi::ET_CORE => Some("Core file"),
        _ => None,
    }
}

pub fn e_type_to_str(e_type: u16) -> Option<&'static str> {
    match e_type {
        abi::ET_NONE => Some("ET_NONE"),
        abi::ET_REL => Some("ET_REL"),
        abi::ET_EXEC => Some("ET_EXEC"),
        abi::ET_DYN => Some("ET_DYN"),
        abi::ET_CORE => Some("ET_CORE"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn e_type_to_string(e_type: u16) -> String {
    match e_type_to_str(e_type) {
        Some(s) => s.to_string(),
        None => format!("e_type({e_type:#x})"),
    }
}

pub fn e_machine_to_human_str(e_machine: u16) -> Option<&'static str> {
    match e_machine {
        abi::EM_NONE => Some("No machine"),
        abi::EM_M32 => Some("AT&T WE 32100"),
        abi::EM_SPARC => Some("SPARC"),
        abi::EM_386 => Some("Intel 80386"),
        abi::EM_68K => Some("Motorola 68000"),
        abi::EM_88K => Some("Motorola 88000"),
        abi::EM_IAMCU => Some("Intel MCU"),
        abi::EM_860 => Some("Intel 80860"),
        abi::EM_MIPS => Some("MIPS I Architecture"),
        abi::EM_S370 => Some("IBM System/370 Processor"),
        abi::EM_MIPS_RS3_LE => Some("MIPS RS3000 Little-endian"),
        abi::EM_PARISC => Some("Hewlett-Packard PA-RISC"),
        abi::EM_VPP500 => Some("Fujitsu VPP500"),
        abi::EM_SPARC32PLUS => Some("Enhanced instruction set SPARC"),
        abi::EM_960 => Some("Intel 80960"),
        abi::EM_PPC => Some("PowerPC"),
        abi::EM_PPC64 => Some("64-bit PowerPC"),
        abi::EM_S390 => Some("IBM System/390 Processor"),
        abi::EM_SPU => Some("IBM SPU/SPC"),
        abi::EM_V800 => Some("NEC V800"),
        abi::EM_FR20 => Some("Fujitsu FR20"),
        abi::EM_RH32 => Some("TRW RH-32"),
        abi::EM_RCE => Some("Motorola RCE"),
        abi::EM_ARM => Some("ARM 32-bit architecture (AARCH32)"),
        abi::EM_ALPHA => Some("Digital Alpha"),
        abi::EM_SH => Some("Hitachi SH"),
        abi::EM_SPARCV9 => Some("SPARC Version 9"),
        abi::EM_TRICORE => Some("Siemens TriCore embedded processor"),
        abi::EM_ARC => Some("Argonaut RISC Core, Argonaut Technologies Inc."),
        abi::EM_H8_300 => Some("Hitachi H8/300"),
        abi::EM_H8_300H => Some("Hitachi H8/300H"),
        abi::EM_H8S => Some("Hitachi H8S"),
        abi::EM_H8_500 => Some("Hitachi H8/500"),
        abi::EM_IA_64 => Some("Intel IA-64 processor architecture"),
        abi::EM_MIPS_X => Some("Stanford MIPS-X"),
        abi::EM_COLDFIRE => Some("Motorola ColdFire"),
        abi::EM_68HC12 => Some("Motorola M68HC12"),
        abi::EM_MMA => Some("Fujitsu MMA Multimedia Accelerator"),
        abi::EM_PCP => Some("Siemens PCP"),
        abi::EM_NCPU => Some("Sony nCPU embedded RISC processor"),
        abi::EM_NDR1 => Some("Denso NDR1 microprocessor"),
        abi::EM_STARCORE => Some("Motorola Star*Core processor"),
        abi::EM_ME16 => Some("Toyota ME16 processor"),
        abi::EM_ST100 => Some("STMicroelectronics ST100 processor"),
        abi::EM_TINYJ => Some("Advanced Logic Corp. TinyJ embedded processor family"),
        abi::EM_X86_64 => Some("AMD x86-64 architecture"),
        abi::EM_PDSP => Some("Sony DSP Processor"),
        abi::EM_PDP10 => Some("Digital Equipment Corp. PDP-10"),
        abi::EM_PDP11 => Some("Digital Equipment Corp. PDP-11"),
        abi::EM_FX66 => Some("Siemens FX66 microcontroller"),
        abi::EM_ST9PLUS => Some("STMicroelectronics ST9+ 8/16 bit microcontroller"),
        abi::EM_ST7 => Some("STMicroelectronics ST7 8-bit microcontroller"),
        abi::EM_68HC16 => Some("Motorola MC68HC16 Microcontroller"),
        abi::EM_68HC11 => Some("Motorola MC68HC11 Microcontroller"),
        abi::EM_68HC08 => Some("Motorola MC68HC08 Microcontroller"),
        abi::EM_68HC05 => Some("Motorola MC68HC05 Microcontroller"),
        abi::EM_SVX => Some("Silicon Graphics SVx"),
        abi::EM_ST19 => Some("STMicroelectronics ST19 8-bit microcontroller"),
        abi::EM_VAX => Some("Digital VAX"),
        abi::EM_CRIS => Some("Axis Communications 32-bit embedded processor"),
        abi::EM_JAVELIN => Some("Infineon Technologies 32-bit embedded processor"),
        abi::EM_FIREPATH => Some("Element 14 64-bit DSP Processor"),
        abi::EM_ZSP => Some("LSI Logic 16-bit DSP Processor"),
        abi::EM_MMIX => Some("Donald Knuth's educational 64-bit processor"),
        abi::EM_HUANY => Some("Harvard University machine-independent object files"),
        abi::EM_PRISM => Some("SiTera Prism"),
        abi::EM_AVR => Some("Atmel AVR 8-bit microcontroller"),
        abi::EM_FR30 => Some("Fujitsu FR30"),
        abi::EM_D10V => Some("Mitsubishi D10V"),
        abi::EM_D30V => Some("Mitsubishi D30V"),
        abi::EM_V850 => Some("NEC v850"),
        abi::EM_M32R => Some("Mitsubishi M32R"),
        abi::EM_MN10300 => Some("Matsushita MN10300"),
        abi::EM_MN10200 => Some("Matsushita MN10200"),
        abi::EM_PJ => Some("picoJava"),
        abi::EM_OPENRISC => Some("OpenRISC 32-bit embedded processor"),
        abi::EM_ARC_COMPACT => Some("ARC International ARCompact processor"),
        abi::EM_XTENSA => Some("Tensilica Xtensa Architecture"),
        abi::EM_VIDEOCORE => Some("Alphamosaic VideoCore processor"),
        abi::EM_TMM_GPP => Some("Thompson Multimedia General Purpose Processor"),
        abi::EM_NS32K => Some("National Semiconductor 32000 series"),
        abi::EM_TPC => Some("Tenor Network TPC processor"),
        abi::EM_SNP1K => Some("Trebia SNP 1000 processor"),
        abi::EM_ST200 => Some("STMicroelectronics (www.st.com) ST200 microcontroller"),
        abi::EM_IP2K => Some("Ubicom IP2xxx microcontroller family"),
        abi::EM_MAX => Some("MAX Processor"),
        abi::EM_CR => Some("National Semiconductor CompactRISC microprocessor"),
        abi::EM_F2MC16 => Some("Fujitsu F2MC16"),
        abi::EM_MSP430 => Some("Texas Instruments embedded microcontroller msp430"),
        abi::EM_BLACKFIN => Some("Analog Devices Blackfin (DSP) processor"),
        abi::EM_SE_C33 => Some("S1C33 Family of Seiko Epson processors"),
        abi::EM_SEP => Some("Sharp embedded microprocessor"),
        abi::EM_ARCA => Some("Arca RISC Microprocessor"),
        abi::EM_UNICORE => {
            Some("Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University")
        }
        abi::EM_EXCESS => Some("eXcess: 16/32/64-bit configurable embedded CPU"),
        abi::EM_DXP => Some("Icera Semiconductor Inc. Deep Execution Processor"),
        abi::EM_ALTERA_NIOS2 => Some("Altera Nios II soft-core processor"),
        abi::EM_CRX => Some("National Semiconductor CompactRISC CRX microprocessor"),
        abi::EM_XGATE => Some("Motorola XGATE embedded processor"),
        abi::EM_C166 => Some("Infineon C16x/XC16x processor"),
        abi::EM_M16C => Some("Renesas M16C series microprocessors"),
        abi::EM_DSPIC30F => Some("Microchip Technology dsPIC30F Digital Signal Controller"),
        abi::EM_CE => Some("Freescale Communication Engine RISC core"),
        abi::EM_M32C => Some("Renesas M32C series microprocessors"),
        abi::EM_TSK3000 => Some("Altium TSK3000 core"),
        abi::EM_RS08 => Some("Freescale RS08 embedded processor"),
        abi::EM_SHARC => Some("Analog Devices SHARC family of 32-bit DSP processors"),
        abi::EM_ECOG2 => Some("Cyan Technology eCOG2 microprocessor"),
        abi::EM_SCORE7 => Some("Sunplus S+core7 RISC processor"),
        abi::EM_DSP24 => Some("New Japan Radio (NJR) 24-bit DSP Processor"),
        abi::EM_VIDEOCORE3 => Some("Broadcom VideoCore III processor"),
        abi::EM_LATTICEMICO32 => Some("RISC processor for Lattice FPGA architecture"),
        abi::EM_SE_C17 => Some("Seiko Epson C17 family"),
        abi::EM_TI_C6000 => Some("The Texas Instruments TMS320C6000 DSP family"),
        abi::EM_TI_C2000 => Some("The Texas Instruments TMS320C2000 DSP family"),
        abi::EM_TI_C5500 => Some("The Texas Instruments TMS320C55x DSP family"),
        abi::EM_TI_ARP32 => {
            Some("Texas Instruments Application Specific RISC Processor, 32bit fetch")
        }
        abi::EM_TI_PRU => Some("Texas Instruments Programmable Realtime Unit"),
        abi::EM_MMDSP_PLUS => Some("STMicroelectronics 64bit VLIW Data Signal Processor"),
        abi::EM_CYPRESS_M8C => Some("Cypress M8C microprocessor"),
        abi::EM_R32C => Some("Renesas R32C series microprocessors"),
        abi::EM_TRIMEDIA => Some("NXP Semiconductors TriMedia architecture family"),
        abi::EM_QDSP6 => Some("QUALCOMM DSP6 Processor"),
        abi::EM_8051 => Some("Intel 8051 and variants"),
        abi::EM_STXP7X => {
            Some("STMicroelectronics STxP7x family of configurable and extensible RISC processors")
        }
        abi::EM_NDS32 => Some("Andes Technology compact code size embedded RISC processor family"),
        abi::EM_ECOG1X => Some("Cyan Technology eCOG1X family"),
        abi::EM_MAXQ30 => Some("Dallas Semiconductor MAXQ30 Core Micro-controllers"),
        abi::EM_XIMO16 => Some("New Japan Radio (NJR) 16-bit DSP Processor"),
        abi::EM_MANIK => Some("M2000 Reconfigurable RISC Microprocessor"),
        abi::EM_CRAYNV2 => Some("Cray Inc. NV2 vector architecture"),
        abi::EM_RX => Some("Renesas RX family"),
        abi::EM_METAG => Some("Imagination Technologies META processor architecture"),
        abi::EM_MCST_ELBRUS => Some("MCST Elbrus general purpose hardware architecture"),
        abi::EM_ECOG16 => Some("Cyan Technology eCOG16 family"),
        abi::EM_CR16 => Some("National Semiconductor CompactRISC CR16 16-bit microprocessor"),
        abi::EM_ETPU => Some("Freescale Extended Time Processing Unit"),
        abi::EM_SLE9X => Some("Infineon Technologies SLE9X core"),
        abi::EM_L10M => Some("Intel L10M"),
        abi::EM_K10M => Some("Intel K10M"),
        abi::EM_AARCH64 => Some("ARM 64-bit architecture (AARCH64)"),
        abi::EM_AVR32 => Some("Atmel Corporation 32-bit microprocessor family"),
        abi::EM_STM8 => Some("STMicroeletronics STM8 8-bit microcontroller"),
        abi::EM_TILE64 => Some("Tilera TILE64 multicore architecture family"),
        abi::EM_TILEPRO => Some("Tilera TILEPro multicore architecture family"),
        abi::EM_MICROBLAZE => Some("Xilinx MicroBlaze 32-bit RISC soft processor core"),
        abi::EM_CUDA => Some("NVIDIA CUDA architecture"),
        abi::EM_TILEGX => Some("Tilera TILE-Gx multicore architecture family"),
        abi::EM_CLOUDSHIELD => Some("CloudShield architecture family"),
        abi::EM_COREA_1ST => Some("KIPO-KAIST Core-A 1st generation processor family"),
        abi::EM_COREA_2ND => Some("KIPO-KAIST Core-A 2nd generation processor family"),
        abi::EM_ARC_COMPACT2 => Some("Synopsys ARCompact V2"),
        abi::EM_OPEN8 => Some("Open8 8-bit RISC soft processor core"),
        abi::EM_RL78 => Some("Renesas RL78 family"),
        abi::EM_VIDEOCORE5 => Some("Broadcom VideoCore V processor"),
        abi::EM_78KOR => Some("Renesas 78KOR family"),
        abi::EM_56800EX => Some("Freescale 56800EX Digital Signal Controller (DSC)"),
        abi::EM_BA1 => Some("Beyond BA1 CPU architecture"),
        abi::EM_BA2 => Some("Beyond BA2 CPU architecture"),
        abi::EM_XCORE => Some("XMOS xCORE processor family"),
        abi::EM_MCHP_PIC => Some("Microchip 8-bit PIC(r) family"),
        abi::EM_INTEL205 => Some("Reserved by Intel"),
        abi::EM_INTEL206 => Some("Reserved by Intel"),
        abi::EM_INTEL207 => Some("Reserved by Intel"),
        abi::EM_INTEL208 => Some("Reserved by Intel"),
        abi::EM_INTEL209 => Some("Reserved by Intel"),
        abi::EM_KM32 => Some("KM211 KM32 32-bit processor"),
        abi::EM_KMX32 => Some("KM211 KMX32 32-bit processor"),
        abi::EM_KMX16 => Some("KM211 KMX16 16-bit processor"),
        abi::EM_KMX8 => Some("KM211 KMX8 8-bit processor"),
        abi::EM_KVARC => Some("KM211 KVARC processor"),
        abi::EM_CDP => Some("Paneve CDP architecture family"),
        abi::EM_COGE => Some("Cognitive Smart Memory Processor"),
        abi::EM_COOL => Some("Bluechip Systems CoolEngine"),
        abi::EM_NORC => Some("Nanoradio Optimized RISC"),
        abi::EM_CSR_KALIMBA => Some("CSR Kalimba architecture family"),
        abi::EM_Z80 => Some("Zilog Z80"),
        abi::EM_VISIUM => Some("Controls and Data Services VISIUMcore processor"),
        abi::EM_FT32 => Some("FTDI Chip FT32 high performance 32-bit RISC architecture"),
        abi::EM_MOXIE => Some("Moxie processor family"),
        abi::EM_AMDGPU => Some("AMD GPU architecture"),
        abi::EM_RISCV => Some("RISC-V"),
        abi::EM_BPF => Some("Linux BPF"),
        _ => None,
    }
}

pub fn e_machine_to_str(e_machine: u16) -> Option<&'static str> {
    match e_machine {
        abi::EM_NONE => Some("EM_NONE"),
        abi::EM_M32 => Some("EM_M32"),
        abi::EM_SPARC => Some("EM_SPARC"),
        abi::EM_386 => Some("EM_386"),
        abi::EM_68K => Some("EM_68K"),
        abi::EM_88K => Some("EM_88K"),
        abi::EM_IAMCU => Some("EM_IAMCU"),
        abi::EM_860 => Some("EM_860"),
        abi::EM_MIPS => Some("EM_MIPS"),
        abi::EM_S370 => Some("EM_S370"),
        abi::EM_MIPS_RS3_LE => Some("EM_MIPS_RS3_LE"),
        abi::EM_PARISC => Some("EM_PARISC"),
        abi::EM_VPP500 => Some("EM_VPP500"),
        abi::EM_SPARC32PLUS => Some("EM_SPARC32PLUS"),
        abi::EM_960 => Some("EM_960"),
        abi::EM_PPC => Some("EM_PPC"),
        abi::EM_PPC64 => Some("EM_PPC64"),
        abi::EM_S390 => Some("EM_S390"),
        abi::EM_SPU => Some("EM_SPU"),
        abi::EM_V800 => Some("EM_V800"),
        abi::EM_FR20 => Some("EM_FR20"),
        abi::EM_RH32 => Some("EM_RH32"),
        abi::EM_RCE => Some("EM_RCE"),
        abi::EM_ARM => Some("EM_ARM"),
        abi::EM_ALPHA => Some("EM_ALPHA"),
        abi::EM_SH => Some("EM_SH"),
        abi::EM_SPARCV9 => Some("EM_SPARCV9"),
        abi::EM_TRICORE => Some("EM_TRICORE"),
        abi::EM_ARC => Some("EM_ARC"),
        abi::EM_H8_300 => Some("EM_H8_300"),
        abi::EM_H8_300H => Some("EM_H8_300H"),
        abi::EM_H8S => Some("EM_H8S"),
        abi::EM_H8_500 => Some("EM_H8_500"),
        abi::EM_IA_64 => Some("EM_IA_64"),
        abi::EM_MIPS_X => Some("EM_MIPS_X"),
        abi::EM_COLDFIRE => Some("EM_COLDFIRE"),
        abi::EM_68HC12 => Some("EM_68HC12"),
        abi::EM_MMA => Some("EM_MMA"),
        abi::EM_PCP => Some("EM_PCP"),
        abi::EM_NCPU => Some("EM_NCPU"),
        abi::EM_NDR1 => Some("EM_NDR1"),
        abi::EM_STARCORE => Some("EM_STARCORE"),
        abi::EM_ME16 => Some("EM_ME16"),
        abi::EM_ST100 => Some("EM_ST100"),
        abi::EM_TINYJ => Some("EM_TINYJ"),
        abi::EM_X86_64 => Some("EM_X86_64"),
        abi::EM_PDSP => Some("EM_PDSP"),
        abi::EM_PDP10 => Some("EM_PDP10"),
        abi::EM_PDP11 => Some("EM_PDP11"),
        abi::EM_FX66 => Some("EM_FX66"),
        abi::EM_ST9PLUS => Some("EM_ST9PLUS"),
        abi::EM_ST7 => Some("EM_ST7"),
        abi::EM_68HC16 => Some("EM_68HC16"),
        abi::EM_68HC11 => Some("EM_68HC11"),
        abi::EM_68HC08 => Some("EM_68HC08"),
        abi::EM_68HC05 => Some("EM_68HC05"),
        abi::EM_SVX => Some("EM_SVX"),
        abi::EM_ST19 => Some("EM_ST19"),
        abi::EM_VAX => Some("EM_VAX"),
        abi::EM_CRIS => Some("EM_CRIS"),
        abi::EM_JAVELIN => Some("EM_JAVELIN"),
        abi::EM_FIREPATH => Some("EM_FIREPATH"),
        abi::EM_ZSP => Some("EM_ZSP"),
        abi::EM_MMIX => Some("EM_MMIX"),
        abi::EM_HUANY => Some("EM_HUANY"),
        abi::EM_PRISM => Some("EM_PRISM"),
        abi::EM_AVR => Some("EM_AVR"),
        abi::EM_FR30 => Some("EM_FR30"),
        abi::EM_D10V => Some("EM_D10V"),
        abi::EM_D30V => Some("EM_D30V"),
        abi::EM_V850 => Some("EM_V850"),
        abi::EM_M32R => Some("EM_M32R"),
        abi::EM_MN10300 => Some("EM_MN10300"),
        abi::EM_MN10200 => Some("EM_MN10200"),
        abi::EM_PJ => Some("EM_PJ"),
        abi::EM_OPENRISC => Some("EM_OPENRISC"),
        abi::EM_ARC_COMPACT => Some("EM_ARC_COMPACT"),
        abi::EM_XTENSA => Some("EM_XTENSA"),
        abi::EM_VIDEOCORE => Some("EM_VIDEOCORE"),
        abi::EM_TMM_GPP => Some("EM_TMM_GPP"),
        abi::EM_NS32K => Some("EM_NS32K"),
        abi::EM_TPC => Some("EM_TPC"),
        abi::EM_SNP1K => Some("EM_SNP1K"),
        abi::EM_ST200 => Some("EM_ST200"),
        abi::EM_IP2K => Some("EM_IP2K"),
        abi::EM_MAX => Some("EM_MAX"),
        abi::EM_CR => Some("EM_CR"),
        abi::EM_F2MC16 => Some("EM_F2MC16"),
        abi::EM_MSP430 => Some("EM_MSP430"),
        abi::EM_BLACKFIN => Some("EM_BLACKFIN"),
        abi::EM_SE_C33 => Some("EM_SE_C33"),
        abi::EM_SEP => Some("EM_SEP"),
        abi::EM_ARCA => Some("EM_ARCA"),
        abi::EM_UNICORE => Some("EM_UNICORE"),
        abi::EM_EXCESS => Some("EM_EXCESS"),
        abi::EM_DXP => Some("EM_DXP"),
        abi::EM_ALTERA_NIOS2 => Some("EM_ALTERA_NIOS2"),
        abi::EM_CRX => Some("EM_CRX"),
        abi::EM_XGATE => Some("EM_XGATE"),
        abi::EM_C166 => Some("EM_C166"),
        abi::EM_M16C => Some("EM_M16C"),
        abi::EM_DSPIC30F => Some("EM_DSPIC30F"),
        abi::EM_CE => Some("EM_CE"),
        abi::EM_M32C => Some("EM_M32C"),
        abi::EM_TSK3000 => Some("EM_TSK3000"),
        abi::EM_RS08 => Some("EM_RS08"),
        abi::EM_SHARC => Some("EM_SHARC"),
        abi::EM_ECOG2 => Some("EM_ECOG2"),
        abi::EM_SCORE7 => Some("EM_SCORE7"),
        abi::EM_DSP24 => Some("EM_DSP24"),
        abi::EM_VIDEOCORE3 => Some("EM_VIDEOCORE3"),
        abi::EM_LATTICEMICO32 => Some("EM_LATTICEMICO32"),
        abi::EM_SE_C17 => Some("EM_SE_C17"),
        abi::EM_TI_C6000 => Some("EM_TI_C6000"),
        abi::EM_TI_C2000 => Some("EM_TI_C2000"),
        abi::EM_TI_C5500 => Some("EM_TI_C5500"),
        abi::EM_TI_ARP32 => Some("EM_TI_ARP32"),
        abi::EM_TI_PRU => Some("EM_TI_PRU"),
        abi::EM_MMDSP_PLUS => Some("EM_MMDSP_PLUS"),
        abi::EM_CYPRESS_M8C => Some("EM_CYPRESS_M8C"),
        abi::EM_R32C => Some("EM_R32C"),
        abi::EM_TRIMEDIA => Some("EM_TRIMEDIA"),
        abi::EM_QDSP6 => Some("EM_QDSP6"),
        abi::EM_8051 => Some("EM_8051"),
        abi::EM_STXP7X => Some("EM_STXP7X"),
        abi::EM_NDS32 => Some("EM_NDS32"),
        abi::EM_ECOG1X => Some("EM_ECOG1X"),
        abi::EM_MAXQ30 => Some("EM_MAXQ30"),
        abi::EM_XIMO16 => Some("EM_XIMO16"),
        abi::EM_MANIK => Some("EM_MANIK"),
        abi::EM_CRAYNV2 => Some("EM_CRAYNV2"),
        abi::EM_RX => Some("EM_RX"),
        abi::EM_METAG => Some("EM_METAG"),
        abi::EM_MCST_ELBRUS => Some("EM_MCST_ELBRUS"),
        abi::EM_ECOG16 => Some("EM_ECOG16"),
        abi::EM_CR16 => Some("EM_CR16"),
        abi::EM_ETPU => Some("EM_ETPU"),
        abi::EM_SLE9X => Some("EM_SLE9X"),
        abi::EM_L10M => Some("EM_L10M"),
        abi::EM_K10M => Some("EM_K10M"),
        abi::EM_AARCH64 => Some("EM_AARCH64"),
        abi::EM_AVR32 => Some("EM_AVR32"),
        abi::EM_STM8 => Some("EM_STM8"),
        abi::EM_TILE64 => Some("EM_TILE64"),
        abi::EM_TILEPRO => Some("EM_TILEPRO"),
        abi::EM_MICROBLAZE => Some("EM_MICROBLAZE"),
        abi::EM_CUDA => Some("EM_CUDA"),
        abi::EM_TILEGX => Some("EM_TILEGX"),
        abi::EM_CLOUDSHIELD => Some("EM_CLOUDSHIELD"),
        abi::EM_COREA_1ST => Some("EM_COREA_1ST"),
        abi::EM_COREA_2ND => Some("EM_COREA_2ND"),
        abi::EM_ARC_COMPACT2 => Some("EM_ARC_COMPACT2"),
        abi::EM_OPEN8 => Some("EM_OPEN8"),
        abi::EM_RL78 => Some("EM_RL78"),
        abi::EM_VIDEOCORE5 => Some("EM_VIDEOCORE5"),
        abi::EM_78KOR => Some("EM_78KOR"),
        abi::EM_56800EX => Some("EM_56800EX"),
        abi::EM_BA1 => Some("EM_BA1"),
        abi::EM_BA2 => Some("EM_BA2"),
        abi::EM_XCORE => Some("EM_XCORE"),
        abi::EM_MCHP_PIC => Some("EM_MCHP_PIC"),
        abi::EM_INTEL205 => Some("EM_INTEL205"),
        abi::EM_INTEL206 => Some("EM_INTEL206"),
        abi::EM_INTEL207 => Some("EM_INTEL207"),
        abi::EM_INTEL208 => Some("EM_INTEL208"),
        abi::EM_INTEL209 => Some("EM_INTEL209"),
        abi::EM_KM32 => Some("EM_KM32"),
        abi::EM_KMX32 => Some("EM_KMX32"),
        abi::EM_KMX16 => Some("EM_KMX16"),
        abi::EM_KMX8 => Some("EM_KMX8"),
        abi::EM_KVARC => Some("EM_KVARC"),
        abi::EM_CDP => Some("EM_CDP"),
        abi::EM_COGE => Some("EM_COGE"),
        abi::EM_COOL => Some("EM_COOL"),
        abi::EM_NORC => Some("EM_NORC"),
        abi::EM_CSR_KALIMBA => Some("EM_CSR_KALIMBA"),
        abi::EM_Z80 => Some("EM_Z80"),
        abi::EM_VISIUM => Some("EM_VISIUM"),
        abi::EM_FT32 => Some("EM_FT32"),
        abi::EM_MOXIE => Some("EM_MOXIE"),
        abi::EM_AMDGPU => Some("EM_AMDGPU"),
        abi::EM_RISCV => Some("EM_RISCV"),
        abi::EM_BPF => Some("EM_BPF"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn e_machine_to_string(e_machine: u16) -> String {
    match e_machine_to_str(e_machine) {
        Some(s) => s.to_string(),
        None => format!("e_machine({e_machine:#x})"),
    }
}

pub fn sh_type_to_str(sh_type: u32) -> Option<&'static str> {
    match sh_type {
        abi::SHT_NULL => Some("SHT_NULL"),
        abi::SHT_PROGBITS => Some("SHT_PROGBITS"),
        abi::SHT_SYMTAB => Some("SHT_SYMTAB"),
        abi::SHT_STRTAB => Some("SHT_STRTAB"),
        abi::SHT_RELA => Some("SHT_RELA"),
        abi::SHT_HASH => Some("SHT_HASH"),
        abi::SHT_DYNAMIC => Some("SHT_DYNAMIC"),
        abi::SHT_NOTE => Some("SHT_NOTE"),
        abi::SHT_NOBITS => Some("SHT_NOBITS"),
        abi::SHT_REL => Some("SHT_REL"),
        abi::SHT_SHLIB => Some("SHT_SHLIB"),
        abi::SHT_DYNSYM => Some("SHT_DYNSYM"),
        abi::SHT_INIT_ARRAY => Some("SHT_INIT_ARRAY"),
        abi::SHT_FINI_ARRAY => Some("SHT_FINI_ARRAY"),
        abi::SHT_PREINIT_ARRAY => Some("SHT_PREINIT_ARRAY"),
        abi::SHT_GROUP => Some("SHT_GROUP"),
        abi::SHT_SYMTAB_SHNDX => Some("SHT_SYMTAB_SHNDX"),
        abi::SHT_GNU_ATTRIBUTES => Some("SHT_GNU_ATTRIBUTES"),
        abi::SHT_GNU_HASH => Some("SHT_GNU_HASH"),
        abi::SHT_GNU_LIBLIST => Some("SHT_GNU_LIBLIST"),
        abi::SHT_GNU_VERDEF => Some("SHT_GNU_VERDEF"),
        abi::SHT_GNU_VERNEED => Some("SHT_GNU_VERNEED"),
        abi::SHT_GNU_VERSYM => Some("SHT_GNU_VERSYM"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn sh_type_to_string(sh_type: u32) -> String {
    match sh_type_to_str(sh_type) {
        Some(s) => s.to_string(),
        None => format!("sh_type({sh_type:#x})"),
    }
}

#[cfg(feature = "alloc")]
pub fn p_flags_to_string(p_flags: u32) -> String {
    match p_flags < 8 {
        true => {
            let r = if p_flags & abi::PF_R != 0 { "R" } else { " " };
            let w = if p_flags & abi::PF_W != 0 { "W" } else { " " };
            let x = if p_flags & abi::PF_X != 0 { "E" } else { " " };
            format!("{r}{w}{x}")
        }
        false => format!("p_flags({p_flags:#x})"),
    }
}

pub fn p_type_to_str(p_type: u32) -> Option<&'static str> {
    match p_type {
        abi::PT_NULL => Some("PT_NULL"),
        abi::PT_LOAD => Some("PT_LOAD"),
        abi::PT_DYNAMIC => Some("PT_DYNAMIC"),
        abi::PT_INTERP => Some("PT_INTERP"),
        abi::PT_NOTE => Some("PT_NOTE"),
        abi::PT_SHLIB => Some("PT_SHLIB"),
        abi::PT_PHDR => Some("PT_PHDR"),
        abi::PT_TLS => Some("PT_TLS"),
        abi::PT_GNU_EH_FRAME => Some("PT_GNU_EH_FRAME"),
        abi::PT_GNU_STACK => Some("PT_GNU_STACK"),
        abi::PT_GNU_RELRO => Some("PT_GNU_RELRO"),
        abi::PT_GNU_PROPERTY => Some("PT_GNU_PROPERTY"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn p_type_to_string(p_type: u32) -> String {
    match p_type_to_str(p_type) {
        Some(s) => s.to_string(),
        None => format!("p_type({p_type:#x})"),
    }
}

pub fn st_symtype_to_str(st_symtype: u8) -> Option<&'static str> {
    match st_symtype {
        abi::STT_NOTYPE => Some("STT_NOTYPE"),
        abi::STT_OBJECT => Some("STT_OBJECT"),
        abi::STT_FUNC => Some("STT_FUNC"),
        abi::STT_SECTION => Some("STT_SECTION"),
        abi::STT_FILE => Some("STT_FILE"),
        abi::STT_COMMON => Some("STT_COMMON"),
        abi::STT_TLS => Some("STT_TLS"),
        abi::STT_GNU_IFUNC => Some("STT_GNU_IFUNC"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn st_symtype_to_string(st_symtype: u8) -> String {
    match st_symtype_to_str(st_symtype) {
        Some(s) => s.to_string(),
        None => format!("st_symtype({st_symtype:#x})"),
    }
}

pub fn st_bind_to_str(st_bind: u8) -> Option<&'static str> {
    match st_bind {
        abi::STB_LOCAL => Some("STB_LOCAL"),
        abi::STB_GLOBAL => Some("STB_GLOBAL"),
        abi::STB_WEAK => Some("STB_WEAK"),
        abi::STB_GNU_UNIQUE => Some("STB_GNU_UNIQUE"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn st_bind_to_string(st_bind: u8) -> String {
    match st_bind_to_str(st_bind) {
        Some(s) => s.to_string(),
        None => format!("st_bind({st_bind:#x})"),
    }
}

pub fn st_vis_to_str(st_vis: u8) -> Option<&'static str> {
    match st_vis {
        abi::STV_DEFAULT => Some("STV_DEFAULT"),
        abi::STV_INTERNAL => Some("STV_INTERNAL"),
        abi::STV_HIDDEN => Some("STV_HIDDEN"),
        abi::STV_PROTECTED => Some("STV_PROTECTED"),
        _ => None,
    }
}

#[cfg(feature = "alloc")]
pub fn st_vis_to_string(st_vis: u8) -> String {
    match st_vis_to_str(st_vis) {
        Some(s) => s.to_string(),
        None => format!("st_vis({st_vis:#x})"),
    }
}

pub fn ch_type_to_str(ch_type: u32) -> Option<&'static str> {
    match ch_type {
        abi::ELFCOMPRESS_ZLIB => Some("ELFCOMPRESS_ZLIB"),
        abi::ELFCOMPRESS_ZSTD => Some("ELFCOMPRESS_ZSTD "),
        _ => None,
    }
}

pub fn note_abi_tag_os_to_str(os: u32) -> Option<&'static str> {
    match os {
        abi::ELF_NOTE_GNU_ABI_TAG_OS_LINUX => Some("Linux"),
        abi::ELF_NOTE_GNU_ABI_TAG_OS_GNU => Some("GNU"),
        abi::ELF_NOTE_GNU_ABI_TAG_OS_SOLARIS2 => Some("Solaris"),
        abi::ELF_NOTE_GNU_ABI_TAG_OS_FREEBSD => Some("FreeBSD"),
        _ => None,
    }
}

pub fn d_tag_to_str(d_tag: i64) -> Option<&'static str> {
    match d_tag {
        abi::DT_NULL => Some("DT_NULL"),
        abi::DT_NEEDED => Some("DT_NEEDED"),
        abi::DT_PLTRELSZ => Some("DT_PLTRELSZ"),
        abi::DT_PLTGOT => Some("DT_PLTGOT"),
        abi::DT_HASH => Some("DT_HASH"),
        abi::DT_STRTAB => Some("DT_STRTAB"),
        abi::DT_SYMTAB => Some("DT_SYMTAB"),
        abi::DT_RELA => Some("DT_RELA"),
        abi::DT_RELASZ => Some("DT_RELASZ"),
        abi::DT_RELAENT => Some("DT_RELAENT"),
        abi::DT_STRSZ => Some("DT_STRSZ"),
        abi::DT_SYMENT => Some("DT_SYMENT"),
        abi::DT_INIT => Some("DT_INIT"),
        abi::DT_FINI => Some("DT_FINI"),
        abi::DT_SONAME => Some("DT_SONAME"),
        abi::DT_RPATH => Some("DT_RPATH"),
        abi::DT_SYMBOLIC => Some("DT_SYMBOLIC"),
        abi::DT_REL => Some("DT_REL"),
        abi::DT_RELSZ => Some("DT_RELSZ"),
        abi::DT_RELENT => Some("DT_RELENT"),
        abi::DT_PLTREL => Some("DT_PLTREL"),
        abi::DT_DEBUG => Some("DT_DEBUG"),
        abi::DT_TEXTREL => Some("DT_TEXTREL"),
        abi::DT_JMPREL => Some("DT_JMPREL"),
        abi::DT_BIND_NOW => Some("DT_BIND_NOW"),
        abi::DT_INIT_ARRAY => Some("DT_INIT_ARRAY"),
        abi::DT_FINI_ARRAY => Some("DT_FINI_ARRAY"),
        abi::DT_INIT_ARRAYSZ => Some("DT_INIT_ARRAYSZ"),
        abi::DT_FINI_ARRAYSZ => Some("DT_FINI_ARRAYSZ"),
        abi::DT_RUNPATH => Some("DT_RUNPATH"),
        abi::DT_FLAGS => Some("DT_FLAGS"),
        abi::DT_PREINIT_ARRAY => Some("DT_PREINIT_ARRAY"),
        abi::DT_PREINIT_ARRAYSZ => Some("DT_PREINIT_ARRAYSZ"),
        abi::DT_SYMTAB_SHNDX => Some("DT_SYMTAB_SHNDX"),
        abi::DT_GUILE_GC_ROOT => Some("DT_GUILE_GC_ROOT"),
        abi::DT_GUILE_GC_ROOT_SZ => Some("DT_GUILE_GC_ROOT_SZ"),
        abi::DT_GUILE_ENTRY => Some("DT_GUILE_ENTRY"),
        abi::DT_GUILE_VM_VERSION => Some("DT_GUILE_VM_VERSION"),
        abi::DT_GUILE_FRAME_MAPS => Some("DT_GUILE_FRAME_MAPS"),
        abi::DT_LOOS => Some("DT_LOOS"),
        abi::DT_GNU_PRELINKED => Some("DT_GNU_PRELINKED"),
        abi::DT_GNU_CONFLICTSZ => Some("DT_GNU_CONFLICTSZ"),
        abi::DT_GNU_LIBLISTSZ => Some("DT_GNU_LIBLISTSZ"),
        abi::DT_CHECKSUM => Some("DT_CHECKSUM"),
        abi::DT_PLTPADSZ => Some("DT_PLTPADSZ"),
        abi::DT_MOVEENT => Some("DT_MOVEENT"),
        abi::DT_MOVESZ => Some("DT_MOVESZ"),
        abi::DT_FEATURE_1 => Some("DT_FEATURE_1"),
        abi::DT_POSFLAG_1 => Some("DT_POSFLAG_1"),
        abi::DT_SYMINSZ => Some("DT_SYMINSZ"),
        abi::DT_SYMINENT => Some("DT_SYMINENT"),
        abi::DT_GNU_HASH => Some("DT_GNU_HASH"),
        abi::DT_TLSDESC_PLT => Some("DT_TLSDESC_PLT"),
        abi::DT_TLSDESC_GOT => Some("DT_TLSDESC_GOT"),
        abi::DT_GNU_CONFLICT => Some("DT_GNU_CONFLICT"),
        abi::DT_GNU_LIBLIST => Some("DT_GNU_LIBLIST"),
        abi::DT_CONFIG => Some("DT_CONFIG"),
        abi::DT_DEPAUDIT => Some("DT_DEPAUDIT"),
        abi::DT_AUDIT => Some("DT_AUDIT"),
        abi::DT_PLTPAD => Some("DT_PLTPAD"),
        abi::DT_MOVETAB => Some("DT_MOVETAB"),
        abi::DT_SYMINFO => Some("DT_SYMINFO"),
        abi::DT_VERSYM => Some("DT_VERSYM"),
        abi::DT_RELACOUNT => Some("DT_RELACOUNT"),
        abi::DT_RELCOUNT => Some("DT_RELCOUNT"),
        abi::DT_FLAGS_1 => Some("DT_FLAGS_1"),
        abi::DT_VERDEF => Some("DT_VERDEF"),
        abi::DT_VERDEFNUM => Some("DT_VERDEFNUM"),
        abi::DT_VERNEED => Some("DT_VERNEED"),
        abi::DT_VERNEEDNUM => Some("DT_VERNEEDNUM"),
        abi::DT_HIOS => Some("DT_HIOS"),
        abi::DT_LOPROC => Some("DT_LOPROC"),
        abi::DT_HIPROC => Some("DT_HIPROC"),
        _ => None,
    }
}
