export type SocCapability = "gpio" | "i2c" | "spi" | "uart" | "pwm" | "adc" | "wifi" | "ble" | "gpu" | "cuda";

export type SocProfile = {
  name: string;
  vendor: string;
  architecture: string;
  clockMhz: number;
  ramMb: number;
  gpioPins: number;
  i2cBuses: number;
  spiBuses: number;
  uartPorts: number;
  adcChannels: number;
  pwmChannels: number;
  capabilities: SocCapability[];
};

export const SOC_PROFILES: Record<string, SocProfile> = {
  RaspberryPi4: {
    name: "RaspberryPi4",
    vendor: "Broadcom",
    architecture: "aarch64",
    clockMhz: 1500,
    ramMb: 4096,
    gpioPins: 40,
    i2cBuses: 2,
    spiBuses: 2,
    uartPorts: 2,
    adcChannels: 0,
    pwmChannels: 2,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "wifi", "ble"],
  },
  RaspberryPi5: {
    name: "RaspberryPi5",
    vendor: "Broadcom",
    architecture: "aarch64",
    clockMhz: 2400,
    ramMb: 8192,
    gpioPins: 40,
    i2cBuses: 3,
    spiBuses: 2,
    uartPorts: 2,
    adcChannels: 0,
    pwmChannels: 4,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "wifi", "ble"],
  },
  ESP32: {
    name: "ESP32",
    vendor: "Espressif",
    architecture: "xtensa",
    clockMhz: 240,
    ramMb: 4,
    gpioPins: 34,
    i2cBuses: 2,
    spiBuses: 3,
    uartPorts: 3,
    adcChannels: 18,
    pwmChannels: 16,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "adc", "wifi", "ble"],
  },
  ESP32S3: {
    name: "ESP32S3",
    vendor: "Espressif",
    architecture: "xtensa",
    clockMhz: 240,
    ramMb: 8,
    gpioPins: 45,
    i2cBuses: 2,
    spiBuses: 4,
    uartPorts: 3,
    adcChannels: 20,
    pwmChannels: 16,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "adc", "wifi", "ble"],
  },
  STM32F4: {
    name: "STM32F4",
    vendor: "STMicroelectronics",
    architecture: "arm_cortex_m4",
    clockMhz: 168,
    ramMb: 0.192,
    gpioPins: 82,
    i2cBuses: 3,
    spiBuses: 3,
    uartPorts: 6,
    adcChannels: 16,
    pwmChannels: 12,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "adc"],
  },
  JetsonNano: {
    name: "JetsonNano",
    vendor: "NVIDIA",
    architecture: "aarch64",
    clockMhz: 1479,
    ramMb: 4096,
    gpioPins: 40,
    i2cBuses: 2,
    spiBuses: 2,
    uartPorts: 2,
    adcChannels: 0,
    pwmChannels: 2,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "gpu", "cuda"],
  },
  JetsonOrin: {
    name: "JetsonOrin",
    vendor: "NVIDIA",
    architecture: "aarch64",
    clockMhz: 2200,
    ramMb: 32768,
    gpioPins: 40,
    i2cBuses: 3,
    spiBuses: 2,
    uartPorts: 3,
    adcChannels: 0,
    pwmChannels: 4,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "gpu", "cuda", "wifi"],
  },
  ArduinoUno: {
    name: "ArduinoUno",
    vendor: "Arduino",
    architecture: "avr",
    clockMhz: 16,
    ramMb: 0.002,
    gpioPins: 20,
    i2cBuses: 1,
    spiBuses: 1,
    uartPorts: 1,
    adcChannels: 6,
    pwmChannels: 6,
    capabilities: ["gpio", "i2c", "spi", "uart", "pwm", "adc"],
  },
};

export type SocValidationError = {
  message: string;
  line?: number;
  column?: number;
};

export function getSocProfile(name: string): SocProfile | undefined {
  return SOC_PROFILES[name];
}

export function validateHalAgainstSoc(
  profile: SocProfile,
  halMembers: import("../hal/index.js").HalMemberConfig[],
): SocValidationError[] {
  const errors: SocValidationError[] = [];
  let i2cCount = 0;
  let spiCount = 0;
  let uartCount = 0;
  let adcCount = 0;
  let pwmCount = 0;

  for (const m of halMembers) {
    switch (m.kind) {
      case "i2c":
        i2cCount++;
        if (i2cCount > profile.i2cBuses) {
          errors.push({ message: `SoC ${profile.name} supports max ${profile.i2cBuses} I2C bus(es)` });
        }
        if (!profile.capabilities.includes("i2c")) {
          errors.push({ message: `SoC ${profile.name} does not support I2C` });
        }
        break;
      case "spi":
        spiCount++;
        if (spiCount > profile.spiBuses) {
          errors.push({ message: `SoC ${profile.name} supports max ${profile.spiBuses} SPI bus(es)` });
        }
        break;
      case "uart":
        uartCount++;
        if (uartCount > profile.uartPorts) {
          errors.push({ message: `SoC ${profile.name} supports max ${profile.uartPorts} UART port(s)` });
        }
        break;
      case "adc":
        adcCount++;
        if (adcCount > profile.adcChannels) {
          errors.push({ message: `SoC ${profile.name} supports max ${profile.adcChannels} ADC channel(s)` });
        }
        if (!profile.capabilities.includes("adc")) {
          errors.push({ message: `SoC ${profile.name} does not support ADC` });
        }
        break;
      case "pwm":
        pwmCount++;
        if (pwmCount > profile.pwmChannels) {
          errors.push({ message: `SoC ${profile.name} supports max ${profile.pwmChannels} PWM channel(s)` });
        }
        break;
      case "gpio":
        if (m.pin >= profile.gpioPins) {
          errors.push({ message: `GPIO pin ${m.pin} exceeds ${profile.name} limit (${profile.gpioPins} pins)` });
        }
        break;
    }
  }

  return errors;
}

export function listSocProfiles(): SocProfile[] {
  return Object.values(SOC_PROFILES);
}
