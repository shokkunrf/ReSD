export class ImageDataSection {
  compression: 0 | 1 | 2 | 3;
  imageData: ArrayBuffer;

  constructor(view: DataView, start: number) {
    const compression = view.getUint16(start);
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
    this.imageData = view.buffer.slice(start + 2);
  }
}
