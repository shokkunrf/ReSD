import { readPascalString, readText } from '../utils/text';

const IMAGE_RESOURCE_BLOCKS_LENGTH_LENGTH = Uint32Array.BYTES_PER_ELEMENT;

// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1037504
export class ImageResourcesSection {
  imageResourceBlocksLength: number;
  imageResourceBlocks: ImageResourcesBlock[] = [];

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.imageResourceBlocksLength = view.getUint32(0);
    for (let i = 0; i < this.imageResourceBlocksLength; ) {
      const irb = new ImageResourcesBlock(binary, start + 4 + i);
      this.imageResourceBlocks.push(irb);
      i += irb.length;
    }
  }

  get length(): number {
    return IMAGE_RESOURCE_BLOCKS_LENGTH_LENGTH + this.imageResourceBlocksLength;
  }
}

class ImageResourcesBlock {
  signature: '8BIM';
  id: number;
  name: string;
  dataLength: number;
  length: number;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    const sig = readText(
      view.buffer.slice(view.byteOffset, view.byteOffset + 4),
      'utf-8'
    );
    if (sig !== '8BIM') {
      throw new Error('ImageResourceBlock.signature invalid');
    }
    this.signature = sig;
    this.id = view.getUint16(4);
    const [name, nameLength] = readPascalString(
      binary.slice(view.byteOffset + 6),
      'utf-8'
    );
    this.name = name;
    const padding = nameLength % 2 === 0 ? 0 : 1;

    this.dataLength = view.getUint32(6 + nameLength + padding);
    const dataLength =
      this.dataLength % 2 === 0 ? this.dataLength : this.dataLength + 1;
    this.length = 6 + nameLength + padding + 4 + dataLength;
  }
}
