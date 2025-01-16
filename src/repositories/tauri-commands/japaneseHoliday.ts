import { invoke } from "@tauri-apps/api/core";
import { JapaneseHolidayData } from "~/models/japaneseHoliday";
export class JapaneseHolidayRepository {
  static async import() {
    await invoke<String>("import_japanese_holiday");
  }
  static async get(startYear: string) {
    const res = await invoke<String>("get_all_japanese_holidays", {
      startYear,
    });
    return JapaneseHolidayData.fromJson(res as string);
  }
}
