import { createReadStream } from 'fs';
import { createRequire } from 'module';

// Welcome to the future!
// This is the only way to import N-API files in ESM.
const text = createRequire(import.meta.url)('./text.node');

console.log(text.countWords([...process.argv].pop()))