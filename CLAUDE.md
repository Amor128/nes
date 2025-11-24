这是一个用 python 来模拟实现实现 NES64 的项目

项目开发阶段如下：

1. 第一阶段：实现 CPU 核心 + 基础内存
2. 第二阶段：实现 ROM 加载和简单的 Mapper (NROM)
3. 第三阶段：实现 PPU 基础功能（背景渲染）
4. 第四阶段：完善 PPU（精灵渲染）
5. 第五阶段：实现输入和显示输出
6. 第六阶段：实现 APU 音频
7. 第七阶段：实现更多 Mapper 支持

项目结构如下：

```
  nes/
  ├── src/
  │   ├── cpu/           # CPU 6502 模拟
  │   ├── ppu/           # PPU 图形处理
  │   ├── apu/           # APU 音频处理
  │   ├── memory/        # 内存管理
  │   ├── mappers/       # 不同的 Mapper 实现
  │   ├── cartridge.py   # ROM 加载
  │   ├── controller.py  # 输入处理
  │   ├── display.py     # 图形输出
  │   └── nes.py         # 主模拟器类
  ├── tests/             # 测试
  └── roms/              # 测试 ROM 文件
```