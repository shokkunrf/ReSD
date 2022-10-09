// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1037504

const IMAGE_RESOURCE_BLOCKS_LENGTH_LENGTH = Uint32Array.BYTES_PER_ELEMENT;

export class ImageResourcesSection {
  imageResourceBlocksLength: number;

  constructor(binary: ArrayBuffer, start: number) {
    this.imageResourceBlocksLength = new DataView(binary, start).getUint32(0);
    // // Image Resource Blocks
    // const imageResourceBlocksSignatureLength = 4; // 8BIM
    // const imageResourceBlocksIdLength = 2;
    // const imageResourceBlocksNameStart =
    //   start +
    //   imageResourceBlocksLengthLength +
    //   imageResourceBlocksSignatureLength +
    //   imageResourceBlocksIdLength;

    // const [_, imageResourceBlocksNameLength] = getPascalString(
    //   binary,
    //   imageResourceBlocksNameStart
    // );
    // const ResourceDataLengthLength = 4;
    // const ResourceDataStart =
    //   imageResourceBlocksNameStart + imageResourceBlocksNameLength;
    // const ResourceDataLength = new DataView(binary).getUint32(
    //   ResourceDataStart
    // );

    // // 一致していてほしかった
    // console.log(
    //   this.length ===
    //   imageResourceBlocksLengthLength +
    //     imageResourceBlocksSignatureLength +
    //     imageResourceBlocksIdLength +
    //     imageResourceBlocksNameLength +
    //     ResourceDataLengthLength +
    //     ResourceDataLength
    // );
  }

  get length(): number {
    return IMAGE_RESOURCE_BLOCKS_LENGTH_LENGTH + this.imageResourceBlocksLength;
  }
}
