// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1037450

const COLOR_DATA_LENGTH_LENGTH = Uint32Array.BYTES_PER_ELEMENT;

export class ColorModeDataSection {
  colorDataLength: number;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.colorDataLength = view.getUint32(0);
  }

  get length(): number {
    return COLOR_DATA_LENGTH_LENGTH + this.colorDataLength;
  }
}
