export class ImageDataSection {
  compression: 0 | 1 | 2 | 3;
  imageData: ArrayBuffer;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    const compression = view.getUint16(0);
    if (
      !(
        compression === 0 ||
        compression === 1 ||
        compression === 2 ||
        compression === 3
      )
    ) {
      throw new Error(`ImageDataSection.compression invalid ${compression}`);
    }
    this.compression = compression;
    this.imageData = binary.slice(start + 2);
  }
}
