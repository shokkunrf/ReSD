import * as fs from 'fs';
import { Psd } from './src/psd';

const file = fs.readFileSync(process.argv[2]);
const view = new DataView(file.buffer);
const w = new Psd(view);
console.log(w.layerAndMaskInformationSection.layerInfo.layerCount);
