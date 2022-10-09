type Encoding = 'utf-8' | 'shift-jis';
const utf8Decoder = new TextDecoder(); // utf-8
const utf16Decoder = new TextDecoder('utf-16be');
const sjisDecoder = new TextDecoder('shift-jis');

export function readText(binary: ArrayBuffer, encoding: Encoding): string {
  const buffer = new Uint8Array(binary);
  if (encoding === 'shift-jis') {
    return sjisDecoder.decode(buffer);
  }
  return utf8Decoder.decode(buffer);
}

export function readPascalString(
  binary: ArrayBuffer,
  encoding: Encoding
): [string, number] {
  const length = new DataView(binary).getUint8(0);
  const text = readText(binary.slice(1, 1 + length), encoding);
  return [text, length + 1];
}

export function readUnicodeString(binary: ArrayBuffer): [string, number] {
  const charCount = new DataView(binary).getUint32(0);
  const len = charCount * 2;
  const text = utf16Decoder.decode(new Uint8Array(binary.slice(4, 4 + len)));
  return [text, 4 + len];
}

export function readSignature(binary: ArrayBuffer): '8BIM' | null {
  if (binary.byteLength !== 4) {
    return null;
  }
  const text = readText(binary, 'utf-8');
  if (text !== '8BIM') {
    return null;
  }
  return text;
}
