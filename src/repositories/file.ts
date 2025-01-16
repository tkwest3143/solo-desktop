import { BaseDirectory, create } from "@tauri-apps/plugin-fs";

export class FileDownloadRepository {
  constructor() {}
  static async createCsv(
    rows: {
      [key: string]: string;
    }[],
    fileName: string
  ) {
    const csvHeaders = Object.keys(rows[0]).join(",") + "\n";
    const csvRows = rows
      .map((row) => {
        return Object.values(row).join(",") + "\n";
      })
      .join("");
    const csvContent = csvHeaders + csvRows;
    const file = await create(fileName + ".csv", {
      baseDir: BaseDirectory.Download,
    });
    await file.write(new TextEncoder().encode(csvContent));
    await file.close();
  }
}
