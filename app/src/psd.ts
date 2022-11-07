import { ColorModeDataSection } from './sections/colorModeDataSection';
import { ImageDataSection } from './sections/imageDataSection';
import { ImageResourcesSection } from './sections/imageResourcesSection';
import { LayerAndMaskInformationSection } from './sections/layerAndMaskInformation';

export class Psd {
  // fileHeaderSection: FileHeaderSection;
  colorModeDataSection: ColorModeDataSection;
  imageResourcesSection: ImageResourcesSection;
  layerAndMaskInformationSection: LayerAndMaskInformationSection;
  ImageDataSection: ImageDataSection;

  constructor(view: DataView) {
    // File Header Section
    // https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_pgfId-1055726
    // this.fileHeaderSection = new FileHeaderSection(binary);

    // Color Mode Data Section
    const colorModeDataSectionStart = 26;
    this.colorModeDataSection = new ColorModeDataSection(
      view,
      colorModeDataSectionStart
    );

    // Image Resources Section
    const imageResourcesSectionStart =
      colorModeDataSectionStart + this.colorModeDataSection.length;
    this.imageResourcesSection = new ImageResourcesSection(
      view,
      imageResourcesSectionStart
    );

    // Layer and Mask Information Section
    const layerAndMaskInformationSectionStart =
      imageResourcesSectionStart + this.imageResourcesSection.length;
    this.layerAndMaskInformationSection = new LayerAndMaskInformationSection(
      view,
      layerAndMaskInformationSectionStart
    );

    // Image Data Section
    const imageDataSectionStart =
      layerAndMaskInformationSectionStart +
      this.layerAndMaskInformationSection.length;
    this.ImageDataSection = new ImageDataSection(view, imageDataSectionStart);
  }
}
