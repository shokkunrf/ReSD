import * as fs from 'fs';
import { Psd } from './src/psd';

const file = fs.readFileSync(process.argv[2]);
const w = new Psd(file.buffer);
console.log(w.layerAndMaskInformationSection.layerInfo.layerCount);
