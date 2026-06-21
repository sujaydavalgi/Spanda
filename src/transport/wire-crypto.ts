/**
 * AEAD wire encryption for Spanda transport frames (AES-256-GCM).
 * @module
 */

import { createCipheriv, createDecipheriv, randomBytes } from "node:crypto";
import { sha256 } from "@noble/hashes/sha256";

/** Production session cipher for encrypted transport wire payloads. */
export class WireCryptoSession {
  private key: Uint8Array;
  readonly cipherSuite = "AES-256-GCM";

  /** Derive a 256-bit session key from configured cert/key material. */
  constructor(material: string) {
    // Hash configured material into a fixed-width AES key.
    this.key = sha256(new TextEncoder().encode(material));
  }

  static fromMaterial(material: string): WireCryptoSession {
    // Build a session from cert/key material strings.
    //
    // Parameters:
    // - `material` — cert path and key secret combined
    //
    // Returns:
    // WireCryptoSession instance.
    //
    // Options:
    // None.
    //
    // Example:
    // const session = WireCryptoSession.fromMaterial("certs/rover.pem:motion_key");

    return new WireCryptoSession(material);
  }

  encrypt(plaintext: Uint8Array): Uint8Array {
    // Encrypt plaintext with a random 12-byte nonce prepended to ciphertext.
    //
    // Parameters:
    // - `plaintext` — bytes to encrypt
    //
    // Returns:
    // Nonce + ciphertext + auth tag bytes.
    //
    // Options:
    // None.
    //
    // Example:
    // const encrypted = session.encrypt(new TextEncoder().encode("{}"));

    const nonce = randomBytes(12);
    const cipher = createCipheriv("aes-256-gcm", this.key, nonce);
    const encrypted = Buffer.concat([cipher.update(plaintext), cipher.final()]);
    const tag = cipher.getAuthTag();
    return Buffer.concat([nonce, encrypted, tag]);
  }

  decrypt(data: Uint8Array): Uint8Array {
    // Decrypt nonce-prefixed AES-GCM wire bytes.
    //
    // Parameters:
    // - `data` — nonce + ciphertext + auth tag
    //
    // Returns:
    // Plaintext bytes.
    //
    // Options:
    // None.
    //
    // Example:
    // const plain = session.decrypt(wireBytes);

    if (data.length < 13) {
      throw new Error("ciphertext too short");
    }
    const nonce = data.subarray(0, 12);
    const tag = data.subarray(data.length - 16);
    const ciphertext = data.subarray(12, data.length - 16);
    const decipher = createDecipheriv("aes-256-gcm", this.key, nonce);
    decipher.setAuthTag(tag);
    return Buffer.concat([decipher.update(ciphertext), decipher.final()]);
  }
}
