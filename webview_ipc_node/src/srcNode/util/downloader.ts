import * as http from 'http';
import * as https from 'https';
import * as fs from 'fs';
import { URL } from 'url';

export async function downloadFile(src: string, filePath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const url = new URL(src);
    const protocol = url.protocol === 'https:' ? https : http;

    // Gunakan flags 'w' untuk memastikan file dibuka untuk ditulisi
    const file = fs.createWriteStream(filePath, { flags: 'w' });

    protocol.get(src, (response: http.IncomingMessage) => {
      const { statusCode } = response;

      if (statusCode && statusCode >= 300 && statusCode < 400 && response.headers.location) {
        file.destroy(); // Hancurkan stream sebelum redirect
        return resolve(downloadFile(response.headers.location, filePath));
      }

      if (statusCode !== 200) {
        file.destroy();
        fs.unlink(filePath, () => {});
        return reject(new Error(`Download gagal! Status: ${statusCode}`));
      }

      response.pipe(file);

      // GANTI DISINI: Gunakan event 'close' bukan 'finish'
      file.on('close', () => {
        resolve(filePath);
      });

      file.on('error', (err: Error) => {
        fs.unlink(filePath, () => {});
        reject(err);
      });

    }).on('error', (err: Error) => {
      file.destroy();
      fs.unlink(filePath, () => {});
      reject(err);
    });
  });
}