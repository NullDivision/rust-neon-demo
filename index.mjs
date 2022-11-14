import { createReadStream } from 'fs';
import { createRequire } from 'module';

// Welcome to the future!
// This is the only way to import N-API files in ESM.
const text = createRequire(import.meta.url)('./text.node');

const stream = createReadStream([...process.argv].pop());
const words = new Map();

stream.on('open', () => {
    console.log('Reading lines...');
});

stream.on('data', buffer => {
    buffer.toString().split(' ').forEach(word => {
        // !!! This is extrememe slow (4m10s) !!!
        const cleanWord = text.getText(word.toLowerCase());

        if (!cleanWord.length) return;

        const lowerCaseWord = cleanWord;

        if (words.has(lowerCaseWord)) {
            words.set(lowerCaseWord, words.get(lowerCaseWord) + 1);

            return;
        }

        words.set(lowerCaseWord, 1);
    });
});

stream.on('end', () => {
    console.log('File read!');

    let col1Length = 0;
    let col2Length = 0;

    words.forEach((count, word) => {
        if (col1Length < word.length) {
            col1Length = word.length;
        }

        if (col2Length < count.toString().length) {
            col2Length = count.toString().length;
        }
    });

    const wordEntries = [...words.entries()];

    wordEntries.sort(([, countA], [, countB]) => countA - countB);

    const header = `${'Words'.padEnd(col1Length)} | ${'Count'.padEnd(col2Length)}`;

    console.log(header);
    console.log(header.replace(/./g, '-'));
    wordEntries.forEach(([word, count]) => {
        console.log(word.padEnd(col1Length), ' | ', count.toString().padEnd(col2Length));
    });
});
