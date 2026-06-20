/**
 * index module (hal/index.ts).
 * @module
 */

export type HalBusKind = "i2c" | "spi" | "uart" | "usb" | "ethernet";

export type HalMemberConfig =
  | { kind: "i2c"; name: string; address: number }
  | { kind: "spi"; name: string; bus: number; csPin?: number }
  | { kind: "gpio"; name: string; pin: number; direction: "in" | "out" }
  | { kind: "pwm"; name: string; pin: number; frequencyHz: number }
  | { kind: "uart"; name: string; device: string; baud: number }
  | { kind: "adc"; name: string; channel: number };

export type HalBlockConfig = {
  members: HalMemberConfig[];
};

export interface HalBackend {
  configure(members: HalMemberConfig[]): void;
  readGpio(name: string): boolean;
  writeGpio(name: string, value: boolean): void;
  readI2c(name: string, register: number, length: number): number[];
  writeI2c(name: string, register: number, data: number[]): void;
  transferSpi(name: string, data: number[]): number[];
  readUart(name: string): string;
  readAdc(name: string): number;
  setPwm(name: string, dutyCycle: number): void;
  getMember(name: string): HalMemberConfig | undefined;
  listMembers(): HalMemberConfig[];
}

export class SimHalBackend implements HalBackend {
  private members = new Map<string, HalMemberConfig>();
  private gpioState = new Map<string, boolean>();
  private i2cRegisters = new Map<string, Map<number, number>>();
  private adcValues = new Map<string, number>();
  private pwmDuty = new Map<string, number>();
  private uartBuffers = new Map<string, string>();

  configure(members: HalMemberConfig[]): void {
    // Configure.
    //
    // Parameters:
    // - `members` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = configure(members);

    this.members.clear();
    for (const m of members) {
      this.members.set(m.name, m);
      if (m.kind === "gpio") this.gpioState.set(m.name, false);
      if (m.kind === "adc") this.adcValues.set(m.name, 0);
      if (m.kind === "pwm") this.pwmDuty.set(m.name, 0);
      if (m.kind === "uart") this.uartBuffers.set(m.name, "");
      if (m.kind === "i2c") this.i2cRegisters.set(m.name, new Map());
    }
  }

  readGpio(name: string): boolean {
    // ReadGpio.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = readGpio(name);

    return this.gpioState.get(name) ?? false;
  }

  writeGpio(name: string, value: boolean): void {
    // WriteGpio.
    //
    // Parameters:
    // - `name` — input value
    // - `value` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = writeGpio(name, value);

    this.gpioState.set(name, value);
  }

  readI2c(name: string, register: number, length: number): number[] {
    // ReadI2c.
    //
    // Parameters:
    // - `name` — input value
    // - `register` — input value
    // - `length` — input value
    //
    // Returns:
    // number[].
    //
    // Options:
    // None.
    //
    // Example:
    // const result = readI2c(name, register, length);

    const regs = this.i2cRegisters.get(name) ?? new Map();
    const result: number[] = [];
    for (let i = 0; i < length; i++) {
      result.push(regs.get(register + i) ?? 0);
    }
    return result;
  }

  writeI2c(name: string, register: number, data: number[]): void {
    // WriteI2c.
    //
    // Parameters:
    // - `name` — input value
    // - `register` — input value
    // - `data` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = writeI2c(name, register, data);

    let regs = this.i2cRegisters.get(name);
    if (!regs) {
      regs = new Map();
      this.i2cRegisters.set(name, regs);
    }
    for (let i = 0; i < data.length; i++) {
      regs.set(register + i, data[i]);
    }
  }

  transferSpi(_name: string, data: number[]): number[] {
    // TransferSpi.
    //
    // Parameters:
    // - `_name` — input value
    // - `data` — input value
    //
    // Returns:
    // number[].
    //
    // Options:
    // None.
    //
    // Example:
    // const result = transferSpi(_name, data);

    return data.map((b) => (b ^ 0xff) & 0xff);
  }

  readUart(name: string): string {
    // ReadUart.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = readUart(name);

    return this.uartBuffers.get(name) ?? "";
  }

  simulateUartData(name: string, data: string): void {
    this.uartBuffers.set(name, data);
  }

  readAdc(name: string): number {
    return this.adcValues.get(name) ?? 0;
  }

  setAdcValue(name: string, value: number): void {
    this.adcValues.set(name, value);
  }

  setPwm(name: string, dutyCycle: number): void {
    this.pwmDuty.set(name, Math.max(0, Math.min(1, dutyCycle)));
  }

  getMember(name: string): HalMemberConfig | undefined {
    return this.members.get(name);
  }

  listMembers(): HalMemberConfig[] {
    return [...this.members.values()];
  }

  seedImuRegisters(busName: string, yaw: number): void {
    const yawInt = Math.floor(yaw * 100);
    this.writeI2c(busName, 0x1a, [yawInt & 0xff, (yawInt >> 8) & 0xff]);
  }
}

export function createSimHal(): SimHalBackend {
  // CreateSimHal.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // `SimHalBackend`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = createSimHal();

  return new SimHalBackend();
}

export function halMemberFromDecl(decl: import("../ast/nodes.js").HalMemberDecl): HalMemberConfig {
  // HalMemberFromDecl.
  //
  // Parameters:
  // - `decl` — input value
  //
  // Returns:
  // HalMemberConfig.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = halMemberFromDecl(decl);

  switch (decl.kind) {
    case "HalI2cDecl":
      return { kind: "i2c", name: decl.name, address: decl.address };
    case "HalSpiDecl":
      return { kind: "spi", name: decl.name, bus: decl.bus, csPin: decl.csPin ?? undefined };
    case "HalGpioDecl":
      return { kind: "gpio", name: decl.name, pin: decl.pin, direction: decl.direction };
    case "HalPwmDecl":
      return { kind: "pwm", name: decl.name, pin: decl.pin, frequencyHz: decl.frequencyHz };
    case "HalUartDecl":
      return { kind: "uart", name: decl.name, device: decl.device, baud: decl.baud };
    case "HalAdcDecl":
      return { kind: "adc", name: decl.name, channel: decl.channel };
  }
}
