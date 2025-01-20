import { BaseDirectory, create } from "@tauri-apps/plugin-fs";

export class FileDownloadRepository {
  constructor() {}
  static async createText(
    rows: Array<Array<string | number>>,
    fileName: string,
    separator: string = ","
  ) {
    const csvRows = rows
      .map((row) => {
        return Object.values(row).join(separator).trim() + "\n";
      })
      .join("");
    const file = await create(fileName + ".txt", {
      baseDir: BaseDirectory.Download,
    });
    await file.write(new TextEncoder().encode(csvRows));
    await file.close();
  }
}
