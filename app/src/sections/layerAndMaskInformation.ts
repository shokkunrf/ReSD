import { TodoError } from '../utils/error';
import {
  readText,
  readPascalString,
  readUnicodeString,
  readSignature,
} from '../utils/text';

// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1031423

export class LayerAndMaskInformationSection {
  private readonly FIELDS_LENGTH_LENGTH = Uint32Array.BYTES_PER_ELEMENT;

  fieldsLength: number;
  layerInfo: LayerInfo;
  globalLayerMaskInfo: GlobalLayerMaskInfo;
  // addtionalLayerInformation: AddtionalLayerInformation; // 存在しない？

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.fieldsLength = view.getUint32(0);
    this.layerInfo = new LayerInfo(binary, start + this.FIELDS_LENGTH_LENGTH);
    this.globalLayerMaskInfo = new GlobalLayerMaskInfo(
      binary,
      start + this.FIELDS_LENGTH_LENGTH + this.layerInfo.length
    );
    // this.addtionalLayerInformation
  }

  get length(): number {
    return this.FIELDS_LENGTH_LENGTH + this.fieldsLength;
  }
}

class LayerInfo {
  private readonly LAYER_INFO_LENGTH_LENGTH = 4;
  private readonly LAYER_COUNT_LENGTH = 2;

  layerInfoLength: number;
  layerCount: number;
  layerRecords: LayerRecord[];
  channelImageData: ChannelImageData[][];
  length: number;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.layerInfoLength = view.getUint32(0);
    this.layerCount = view.getUint16(this.LAYER_INFO_LENGTH_LENGTH);

    this.layerRecords = [];
    let layerRecordPosition =
      view.byteOffset + this.LAYER_INFO_LENGTH_LENGTH + this.LAYER_COUNT_LENGTH;
    for (let i = 0; i < this.layerCount; i++) {
      const layerRecord = new LayerRecord(binary, layerRecordPosition);
      layerRecordPosition += layerRecord.length;
      this.layerRecords.push(layerRecord);
    }

    this.channelImageData = [];
    for (let i = 0; i < this.layerCount; i++) {
      const layerChannelImageData = this.layerRecords[
        i
      ].channelInformations.map((c) => {
        const channelImageData = new ChannelImageData(
          binary,
          layerRecordPosition,
          this.layerRecords[i]
        );
        if (channelImageData.length !== c.channelDataLength) {
          throw new Error();
        }

        layerRecordPosition += channelImageData.length;
        return channelImageData;
      });
      this.channelImageData.push(layerChannelImageData);
    }

    this.length = layerRecordPosition - start;
  }
}

class LayerRecord {
  coordinates: { top: number; left: number; bottom: number; right: number };
  channelNum: number;
  channelInformations: { channelId: number; channelDataLength: number }[] = [];
  signature: string; // 8BIM
  blendModeKey: string;
  opacity: number;
  clipping: number;
  flags: number;
  filler: number;
  extraDataFieldLength: number;
  layerMaskData: LayerMaskData;
  layerBlendingRanges: LayerBlendingRanges;
  layerName: string;
  additionalLayerInformations: AddtionalLayerInformation[] = [];

  lll: number;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.coordinates = {
      top: view.getUint32(0),
      left: view.getUint32(4),
      bottom: view.getUint32(8),
      right: view.getUint32(12),
    };
    this.channelNum = view.getUint16(16);

    let channelInformationPosition = 18;
    for (let i = 0; i < this.channelNum; i++) {
      const info = {
        channelId: view.getInt16(channelInformationPosition),
        channelDataLength: view.getUint32(channelInformationPosition + 2),
      };
      this.channelInformations.push(info);
      channelInformationPosition += 6;
    }

    const signatureLength = 4;
    this.signature = readText(
      view.buffer.slice(
        view.byteOffset + channelInformationPosition,
        view.byteOffset + channelInformationPosition + signatureLength
      ),
      'utf-8'
    ); // 8BIM

    const blendModeKeyLength = 4;
    this.blendModeKey = readText(
      view.buffer.slice(
        view.byteOffset + channelInformationPosition + signatureLength,
        view.byteOffset +
          channelInformationPosition +
          signatureLength +
          blendModeKeyLength
      ),
      'utf-8'
    );

    this.opacity = view.getUint8(
      channelInformationPosition + signatureLength + blendModeKeyLength
    );
    this.clipping = view.getUint8(
      channelInformationPosition + signatureLength + blendModeKeyLength + 1
    );
    this.flags = view.getUint8(
      channelInformationPosition + signatureLength + blendModeKeyLength + 1 + 1
    );
    this.filler = view.getUint8(
      channelInformationPosition +
        signatureLength +
        blendModeKeyLength +
        1 +
        1 +
        1
    );
    this.extraDataFieldLength = view.getUint32(
      channelInformationPosition +
        signatureLength +
        blendModeKeyLength +
        1 +
        1 +
        1 +
        1
    );

    const a =
      channelInformationPosition +
      signatureLength +
      blendModeKeyLength +
      1 +
      1 +
      1 +
      1 +
      4;
    this.layerMaskData = new LayerMaskData(view.buffer, view.byteOffset + a);
    this.layerBlendingRanges = new LayerBlendingRanges(
      view.buffer,
      view.byteOffset + a + this.layerMaskData.length
    );
    const b =
      view.byteOffset +
      a +
      this.layerMaskData.length +
      this.layerBlendingRanges.length;

    const [layerName, layerNameLength] = readPascalString(
      binary.slice(b),
      'shift-jis'
    );
    const padding =
      layerNameLength +
      (layerNameLength % 4 === 0 ? 0 : 4 - (layerNameLength % 4));

    this.layerName = layerName;
    let len = 0;
    for (;;) {
      const sig = readSignature(
        binary.slice(b + padding + len, b + padding + 4 + len)
      );
      if (sig === null) {
        break;
      }

      const ali = new AddtionalLayerInformation(view.buffer, b + padding + len);
      this.additionalLayerInformations.push(ali);
      len += ali.length;
    }

    this.lll = b + padding - view.byteOffset + len;
  }

  get length(): number {
    return this.lll;
  }
}

class LayerMaskData {
  layerMaskDataLength: number;
  rectangle: object | null = null;
  defaultColor: number | null = null; // 0 or 255
  flags: number | null = null;
  maskParameters: number | null = null;
  maskParametersFlags: number | null = null;
  padding: number | null = null;
  realFlags: number | null = null;
  realUserMaskBackground: number | null = null; // 0 or 255
  rectangleEnclosingLayerMask: object | null = null;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.layerMaskDataLength = view.getUint32(0);
    if (this.layerMaskDataLength === 0) {
      return;
    }
    this.rectangle = {
      top: view.getUint32(4),
      left: view.getUint32(8),
      bottom: view.getUint32(12),
      right: view.getUint32(16),
    };
    this.defaultColor = view.getUint8(20);
    this.flags = view.getUint8(21);

    let position = 22;
    if (this.flags === 8) {
      this.maskParameters = view.getUint8(position);
      position += 1;
      if (this.maskParameters === 0) {
        this.maskParametersFlags = view.getUint8(position);
        position += 1;
      } else if (this.maskParameters === 1) {
        this.maskParametersFlags = view.getFloat64(position);
        position += 8;
      } else if (this.maskParameters === 2) {
        this.maskParametersFlags = view.getUint8(position);
        position += 1;
      } else if (this.maskParameters === 4) {
        this.maskParametersFlags = view.getFloat64(position);
        position += 8;
      }
    }

    if (this.layerMaskDataLength === 20) {
      this.padding = view.getUint16(position);
      return;
    }

    this.realFlags = view.getUint8(position);
    this.realUserMaskBackground = view.getUint8(position + 1);
    this.rectangleEnclosingLayerMask = {
      top: view.getUint32(position + 1 + 1),
      left: view.getUint32(position + 1 + 1 + 4),
      bottom: view.getUint32(position + 1 + 1 + 8),
      right: view.getUint32(position + 1 + 1 + 12),
    };
  }

  get length(): number {
    return 4 + this.layerMaskDataLength;
  }
}

class LayerBlendingRanges {
  layerBlendhingRangesLength: number;
  compositeGrayBlendSource: object | null = null;
  compositeGrayBlendDestinationRange: number | null = null;
  channelSources: object[] = [];

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.layerBlendhingRangesLength = view.getUint32(0);
    if (this.layerBlendhingRangesLength === 0) {
      return;
    }
    this.compositeGrayBlendSource = {
      b1: view.getUint8(4),
      b2: view.getUint8(5),
      w1: view.getUint8(6),
      w2: view.getUint8(7),
    };
    this.compositeGrayBlendDestinationRange = view.getUint32(8);

    for (
      let position = 12;
      position < this.layerBlendhingRangesLength;
      position += 8
    ) {
      this.channelSources.push({
        sourceRange: view.getUint32(position),
        destinationRange: view.getUint32(position + 4),
      });
    }
  }

  get length(): number {
    return 4 + this.layerBlendhingRangesLength;
  }
}

class AddtionalLayerInformation {
  signature: string; // 8BIM
  key: string;
  dataLength: number;
  data: Luni | Lsct;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);

    const sig = readSignature(
      view.buffer.slice(view.byteOffset, view.byteOffset + 4)
    );
    this.signature = sig ?? '';

    this.key = readText(
      view.buffer.slice(view.byteOffset + 4, view.byteOffset + 8),
      'utf-8'
    );

    this.dataLength = view.getUint32(8);

    if (this.key === 'luni') {
      this.data = new Luni(binary, view.byteOffset + 12);
      return;
    }
    if (this.key === 'lsct') {
      this.data = new Lsct(binary, view.byteOffset + 12, this.dataLength);
      return;
    }
    throw new Error(`another ali ${this.key}`);
  }

  get length(): number {
    return (
      Uint32Array.BYTES_PER_ELEMENT +
      Uint32Array.BYTES_PER_ELEMENT +
      Uint32Array.BYTES_PER_ELEMENT +
      this.dataLength
    );
  }
}

class Luni {
  text: string;
  length: number;

  constructor(binary: ArrayBuffer, start: number) {
    const [utf16Text, utf16Length] = readUnicodeString(binary.slice(start));
    this.text = utf16Text;
    this.length = utf16Length;
  }
}

class Lsct {
  type: 0 | 1 | 2 | 3;
  signature: '8BIM' | null = null;
  blendModeKey: string | null = null;
  subType: 0 | 1 | null = null;

  constructor(binary: ArrayBuffer, start: number, length: number) {
    const view = new DataView(binary, start);
    const type = view.getUint32(0);
    if (!(type === 0 || type === 1 || type === 2 || type === 3)) {
      throw new Error('Lsct.type invalid');
    }
    this.type = type;

    if (length >= 12) {
      this.signature = (() => {
        const a = readText(
          view.buffer.slice(view.byteOffset + 4, view.byteOffset + 8),
          'utf-8'
        ); // 8BIM
        if (a !== '8BIM') {
          return null;
        }
        return a;
      })();

      // this.blendModeKey = view.getUint32(8);
      this.blendModeKey = readText(
        view.buffer.slice(view.byteOffset + 4, view.byteOffset + 8),
        'utf-8'
      );
    }

    if (length >= 18) {
      const subType = view.getUint32(12);
      if (!(subType === 0 || subType === 1)) {
        throw new Error('');
      }
      this.subType = subType;
    }
  }
}

class ChannelImageData {
  compression: 0 | 1 | 2 | 3;
  imageData: ArrayBuffer;
  length: number;

  constructor(binary: ArrayBuffer, start: number, layerRecord: LayerRecord) {
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
      throw new Error(`channelImageData invalid compression ${compression}`);
    }
    this.compression = compression;

    if (this.compression === 0) {
      const len =
        (layerRecord.coordinates.bottom - layerRecord.coordinates.top) *
        (layerRecord.coordinates.right - layerRecord.coordinates.left);
      this.imageData = binary.slice(start + 2, start + 2 + len);
      this.length = 2 + len;
      // this.length = 2 + len + (len % 2 === 0 ? 0 : 1);
      return;
    }

    if (this.compression === 1) {
      let pos = 2; // imageData start

      const rowCount =
        layerRecord.coordinates.bottom - layerRecord.coordinates.top;
      let len = 0;
      for (let i = 0; i < rowCount; i++) {
        len += view.getUint16(pos + 2 * i);
      }

      const rleHeadLength = 2 * rowCount;
      // console.log(len % 2 === 0 ? 0 : 1);
      pos += rleHeadLength + len;
      // pos += rleHeadLength + len + (len % 2 === 0 ? 0 : 1);

      // console.log(c.channelDataLength, lll);
      this.length = pos;
      this.imageData = binary.slice(start + 2, start + 2 + this.length);
      return;
    }

    if (this.compression === 2) {
      throw new TodoError('compression 2');
    }
    throw new TodoError('compression 3');
  }
}

class GlobalLayerMaskInfo {
  sectionLength: number;
  overlayColorSpace: number | null = null;
  colorComponents: number[] | null = null;
  opacity: 0 | 100 | null = null;
  kind: 0 | 1 | 128 | null = null;
  filler: string | null = null;

  constructor(binary: ArrayBuffer, start: number) {
    const view = new DataView(binary, start);
    this.sectionLength = view.getUint32(0);
    if (this.sectionLength === 0) {
      return;
    }

    this.overlayColorSpace = view.getUint16(4);
    this.colorComponents = [];
    for (let i = 0; i < 4; i++) {
      this.colorComponents.push(view.getUint16(6 + i * 2));
    }
    const opacity = view.getUint16(14);
    if (!(opacity === 0 || opacity === 100)) {
      throw new Error('GlobalLayerMaskInfo.opacity invalid');
    }
    this.opacity = opacity;

    const kind = view.getUint8(16);
    if (!(kind === 0 || kind === 1 || kind === 128)) {
      throw new Error('GlobalLayerMaskInfo.kind invalid');
    }
    this.kind = kind;

    this.filler = '';
    for (let i = 0; i < this.sectionLength - 17; i++) {
      this.filler += '0';
    }
  }

  get length(): number {
    return Uint32Array.BYTES_PER_ELEMENT + this.sectionLength;
  }
}
