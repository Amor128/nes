class CPU:
    def __init__(self, memory):
        '''
        see https://www.nesdev.org/obelisk-6502-guide/registers.html
        '''
        self.pc = 0x0000  # Program Counter
        self.sp = 0x01FF    # Stack Pointer
        self.a = 0x00       # Accumulator
        self.x = 0x00       # X Register
        self.y = 0x00       # Y Register

        self.P = 0x24      # 初始状态
        self.FLAG_C = 0b00000001  # Carry
        self.FLAG_Z = 0b00000010  # Zero
        self.FLAG_I = 0b00000100  # Interrupt Disable
        self.FLAG_D = 0b00001000  # Decimal (NES未使用)
        self.FLAG_B = 0b00010000  # Break
        self.FLAG_V = 0b01000000  # Overflow
        self.FLAG_N = 0b10000000  # Negative

        self.cycles = 0

        self.memory = memory

    def set_flag(self, flag, value):
        if value:
            self.P |= flag
        else:
            self.P &= ~flag


    # ========== Addressing ==========
    # https://www.nesdev.org/obelisk-6502-guide/addressing.html
    def addr_implicit(self):
        return None
    
    def addr_accumulator(self):
        return None
    
    def addr_immediate(self):
        address = self.pc
        self.pc = (self.pc + 1) & 0xFFFF
        return address
    
    def addr_zero_page(self):
        address = self.memory.read(self.pc)
        self.pc = (self.pc + 1) & 0xFFFF
        return address
    
    def addr_zero_page_x(self):
          address = (self.memory.read(self.pc) + self.x) & 0xFF
          self.pc = (self.pc + 1) & 0xFFFF
          return address

    def addr_zero_page_y(self):
        address = (self.memory.read(self.pc) + self.y) & 0xFF
        self.pc = (self.pc + 1) & 0xFFFF
        return address
    
    def addr_relative(self):
        offset = self.memory.read(self.pc)
        self.pc = (self.pc + 1) & 0xFFFF
        if offset < 0x80:
            return (self.pc + offset) & 0xFFFF
        else:
            return (self.pc + offset - 0x100) & 0xFFFF
        
    def addr_absolute(self):
          """绝对寻址 - 16位地址"""
          low = self.memory.read(self.PC)
          self.PC = (self.PC + 1) & 0xFFFF
          high = self.memory.read(self.PC)
          self.PC = (self.PC + 1) & 0xFFFF
          return (high << 8) | low

    def addr_absolute_x(self):
        base = self.addr_absolute()
        address = (base + self.X) & 0xFFFF

        # 跨页检测（影响周期数）
        if (base & 0xFF00) != (address & 0xFF00):
            self.cycles += 1

        return address

    def addr_absolute_y(self):
        base = self.addr_absolute()
        address = (base + self.Y) & 0xFFFF

        # 跨页检测
        if (base & 0xFF00) != (address & 0xFF00):
            self.cycles += 1

        return address
    
    def addr_indirect(self):
        """间接寻址 - 仅用于JMP指令"""
        ptr_low = self.memory.read(self.pc)
        self.pc = (self.pc + 1) & 0xFFFF
        ptr_high = self.memory.read(self.pc)
        self.pc = (self.pc + 1) & 0xFFFF
        ptr = (ptr_high << 8) | ptr_low

        # 6502硬件Bug：如果指针在页边界，高字节从同页的0x00读取
        if ptr_low == 0xFF:
            low = self.memory.read(ptr)
            high = self.memory.read(ptr & 0xFF00)  # Bug！
        else:
            low = self.memory.read(ptr)
            high = self.memory.read(ptr + 1)

        return (high << 8) | low

    def addr_indexed_indirect(self):
          """索引间接 (X) - ($44,X)"""
          base = self.memory.read(self.PC)
          self.PC = (self.PC + 1) & 0xFFFF

          ptr = (base + self.X) & 0xFF
          low = self.memory.read(ptr)
          high = self.memory.read((ptr + 1) & 0xFF)

          return (high << 8) | low

    def addr_indirect_indexed(self):
        """间接索引 (Y) - ($44),Y"""
        ptr = self.memory.read(self.pc)
        self.pc = (self.pc + 1) & 0xFFFF

        low = self.memory.read(ptr)
        high = self.memory.read((ptr + 1) & 0xFF)

        base = (high << 8) | low
        address = (base + self.Y) & 0xFFFF

        # 跨页检测
        if (base & 0xFF00) != (address & 0xFF00):
            self.cycles += 1

        return address