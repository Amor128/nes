class Memory:
    def __init__(self):
        '''
        Address range	Size	Device
        $0000-$07FF     $0800	2 KB internal RAM
        $0800-$0FFF     $0800	Mirrors of $0000-$07FF
        $1000-$17FF     $0800   Mirrors of $0000-$07FF
        $1800-$1FFF     $0800   Mirrors of $0000-$07FF

        $2000-$2007     $0008	NES PPU registers
        $2008-$3FFF     $1FF8	Mirrors of $2000-$2007 (repeats every 8 bytes)

        $4000-$4017     $0018	NES APU and I/O registers
        $4018-$401F     $0008	APU and I/O functionality that is normally disabled. See CPU Test Mode.
        $4020-$FFFF     $BFE0   Unmapped. Available for cartridge use.
        $6000-$7FFF     $2000   Usually cartridge RAM, when present.
        $8000-$FFFF 	$8000   Usually cartridge ROM and mapper registers.
        '''
        self.ram = bytearray(0x0800)
        
        self.ppu = None
        self.apu = None
        self.cartridge = None
        self.controller = None
    def read(self, address):
        if 0x0000 <= address <= 0x1FFF:
            return self.ram[address % 0x0800]
        elif 0x2000 <= address <= 0x3FFF:
            # TODO: Handle PPU register reads
            raise NotImplementedError("PPU register read not implemented yet")
        elif 0x4000 <= address <= 0x4017:
            # TODO: Handle APU and I/O register reads
            raise NotImplementedError("APU and I/O register read not implemented yet")
        elif 0x4018 <= address <= 0x401F:
            return 0  # APU and I/O functionality that is normally disabled
        elif 0x4020 <= address <= 0xFFFF:
            # TODO: Handle cartridge read (ROM/RAM)
            raise NotImplementedError("Cartridge read not implemented yet")
        else:
            raise ValueError(f"Invalid memory read at address {hex(address)}")
    
    def write(self, address, value):
        if 0x0000 <= address <= 0x1FFF:
            self.ram[address % 0x0800] = value
        elif 0x2000 <= address <= 0x3FFF:
            # TODO: Handle PPU register writes
            raise NotImplementedError("PPU register write not implemented yet")
        elif 0x4000 <= address <= 0x4017:
            # TODO: Handle APU and I/O register writes
            raise NotImplementedError("APU and I/O register write not implemented yet")
        elif 0x4018 <= address <= 0x401F:
            pass  # APU and I/O functionality that is normally disabled
        elif 0x4020 <= address <= 0xFFFF:
            # TODO: Handle cartridge write (RAM)
            raise NotImplementedError("Cartridge write not implemented yet")
        else:
            raise ValueError(f"Invalid memory write at address {hex(address)}")