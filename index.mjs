import { createReadStream, createWriteStream } from 'fs';
import { Stream } from 'stream';

const stream = createReadStream([...process.argv].pop());
const words = new Map();

stream.on('data', buffer => {
    buffer.toString().split(' ').forEach(word => {
        const cleanWord = word.toLowerCase().match(/[a-z\-]+/);

        if (!cleanWord || !cleanWord[0].length) return;

        const lowerCaseWord = cleanWord[0];

        if (words.has(lowerCaseWord)) {
            words.set(lowerCaseWord, words.get(lowerCaseWord) + 1);

            return;
        }

        words.set(lowerCaseWord, 1);
    });
});

stream.on('end', () => {
    const file = createWriteStream('./output.txt');
    const writeStream = new Stream();

    writeStream.pipe(file);

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

    writeStream.emit('data', `${header}\n`);
    writeStream.emit('data', `${header.replace(/./g, '-')}\n`);

    wordEntries.forEach(([word, count]) => {
        writeStream.emit('data', `${word.padEnd(col1Length)} | ${count.toString().padEnd(col2Length)}\n`);
    });
});
