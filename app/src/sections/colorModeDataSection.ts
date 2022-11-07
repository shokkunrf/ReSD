// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1037450

const COLOR_DATA_LENGTH_LENGTH = Uint32Array.BYTES_PER_ELEMENT;

export class ColorModeDataSection {
  colorDataLength: number;

  constructor(view: DataView, start: number) {
    this.colorDataLength = view.getUint32(start);
  }

  get length(): number {
    return COLOR_DATA_LENGTH_LENGTH + this.colorDataLength;
  }
}
