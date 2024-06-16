import fs from "fs/promises";
import fsSync from "fs";
import readline from "node:readline";
import { hrtime } from "node:process";

const INPUTS_FOLDER = "../inputs";

const main = async () => {

  const fileNames = await fs.readdir(INPUTS_FOLDER);

  const startTime = hrtime.bigint();

  for (const fileName of fileNames) {
    const fileStartTime = hrtime.bigint();

    const fileStream = fsSync.createReadStream(`${INPUTS_FOLDER}/${fileName}`);

    const rl = readline.createInterface({
      input: fileStream,
      crlfDelay: Infinity,
    });
  
    let hash = 0;
    for await (const line of rl) {
      for (let charIndex = 0; charIndex < line.length; charIndex++) {
        hash += line.charCodeAt(charIndex);
      }
    }

    const elapsed = Number(hrtime.bigint() - fileStartTime) / 1000000;

    console.log({ fileName, hash, elapsed });
  }

  console.log({
    overallElapsed: Number(hrtime.bigint() - startTime) / 1000000
  });
};

main();