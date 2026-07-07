# risc-v

A RISC-V (RV32IM) emulator written from scratch in Rust. Full CPU pipeline
(fetch, decode, execute), a bus-mapped MMIO device model, an interrupt
controller, and a handful of bare-metal demo programs that exercise it.

Currently focused entirely on the hardware side: CPU correctness, the
device/bus architecture, and interrupts. The software side (bootloader,
filesystem, a tiny OS, a terminal) is the next phase, once the hardware is
solid.

## Features

- RV32IM instruction set: full base integer ISA plus the M-extension
  (MUL/MULH/MULHSU/MULHU, DIV/DIVU/REM/REMU), including the spec-mandated
  non-trapping edge cases (divide by zero, `INT_MIN / -1` overflow).
- A `Bus` with a minimal `Device` trait (`read8`/`write8`, width-aware
  `read16/32`/`write16/32` built on top), so devices are mapped by address
  range and dispatched dynamically.
- MMIO devices: RAM, UART (byte in/out), a Timer (counter/compare, fires an
  interrupt), and a Screen (320x240 double-buffered framebuffer with a
  swap-request/status handshake).
- An `InterruptController` and CPU-side interrupt handling (`mepc`,
  `interrupt_base`, `MRET`), currently wired for timer interrupts.
- A host-side runner (`main.rs`) using `minifb` to display the Screen
  device's framebuffer in a real window.

## Project structure

```
src
├── assembler/   raw instruction encoders (assemble_add, assemble_mul, ...)
├── bus/         Bus + MappedDevice: address-range dispatch to devices
├── cpu/         CPU state, fetch/decode/execute, instruction types
├── devices/     Device trait + Memory, UART, Timer, Screen
├── interrupt/   InterruptController
├── machine/     Machine (CPU + Bus + InterruptController) and its step loop
├── platform/    memory map constants, MMIO register offsets
├── utils/       register dump and other debug helpers
├── lib.rs
└── main.rs      host window + program loader
```

## Memory map

| Region | Base | Size | Notes |
|---|---|---|---|
| RAM | `0x0000_0000` | 64 KiB | code, data, stack |
| UART | `0x1000_0000` | 256 B | `DATA` (0x00), `STATUS` (0x04), `CONTROL` (0x08) |
| Timer | `0x1000_0100` | 256 B | `COUNTER` (0x00), `COMPARE` (0x04), `STATUS` (0x08) |
| Screen | `0x8000_0000` | ~307 KiB | 320x240 RGBA framebuffer, control reg at offset `FRAMEBUFFER_SIZE`, status reg 4 bytes after that |
| Interrupt vector table | `0x0000_8000` | — | reserved |

## Building and running

The emulator itself is a normal cargo project:

```
cargo run
```

`main.rs` loads a flat binary (`program.bin` by default) into RAM at address
0, then steps the CPU in batches, rendering the Screen device's front buffer
into a `minifb` window each frame.

Guest programs are cross-compiled separately with a RISC-V toolchain and
turned into a flat binary:

```
riscv32-elf-as -march=rv32im -mabi=ilp32 start.s -o start.o
riscv32-elf-gcc -march=rv32i -mabi=ilp32 -nostdlib -ffreestanding -c program.c -o program.o
riscv32-elf-ld -T linker.ld -o program.elf start.o program.o
riscv32-elf-objcopy -O binary program.elf program.bin
```

(Use `-march=rv32im` only where you actually need `mul`/`div`; the demos
below intentionally avoid them where a shift-based trick is enough, to keep
the emulator's instruction coverage requirements minimal.)

## Demos

- **fib** — bare-metal RV32I assembly computing `fib(10)`, the first program
  to run correctly end to end on the CPU.
- **dvd** — a bouncing square on the Screen device, DVD-logo style: clears
  and redraws the back buffer, requests a swap, waits on the status register
  for the swap to actually complete (avoids tearing/flicker), then bounces
  off the edges and flips color on each hit. Written both as hand-assembled
  RV32I (`dvd.s`) and as bare-metal C (`dvd.c` + `start.s` + linker script),
  compiled with `-march=rv32i` only — pixel-offset math uses shifts
  (`320 = 256 + 64`) instead of multiply, so it runs on RV32I alone.
- **test_m** — a headless RV32IM test program (`test_m.s` + `src/bin/test_m.rs`)
  exercising all 8 M-extension instructions plus divide-by-zero and signed-
  overflow edge cases, checked by dumping registers after execution.

## Roadmap

Hardware, in progress or done:
- [x] RV32I base ISA
- [x] Bus/Device MMIO architecture
- [x] Screen, UART, Timer devices
- [x] Interrupt controller + MRET
- [x] RV32IM (multiply/divide)
- [ ] Full interrupt design (timer-driven preemption, nested interrupts) —
      being worked out independently, not just bolted on

Software, once the hardware is stable:
- [ ] Bootloader
- [ ] Tiny kernel
- [ ] A minimal filesystem
- [ ] A terminal / shell running on top of it

The goal is to eventually run something that looks and feels like a real
(if tiny) operating system on top of this CPU, built the same way the CPU
itself was: from scratch, one working piece at a time.
