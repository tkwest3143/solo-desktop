import {
  JapaneseHolidayData,
  type japaneseHoliday,
} from "@/models/japaneseHoliday";
import { describe, expect, it } from "vitest";

describe("JapaneseHolidayData", () => {
  const holiday: japaneseHoliday = {
    id: 1,
    day: new Date("2023-10-01"),
    subject: "Test Holiday",
  };

  it("checks if a date is a holiday", () => {
    const holidayInstance = new JapaneseHolidayData(holiday);
    expect(holidayInstance.isDay(new Date("2023-10-01"))).toBe(true);
    expect(holidayInstance.isDay(new Date("2023-10-02"))).toBe(false);
  });

  it("checks boundary conditions", () => {
    const holidayInstance = new JapaneseHolidayData(holiday);
    expect(holidayInstance.isDay(new Date("2023-09-30T23:59:59"))).toBe(false);
    expect(holidayInstance.isDay(new Date("2023-10-01T00:00:00"))).toBe(true);
    expect(holidayInstance.isDay(new Date("2023-10-01T23:59:59"))).toBe(true);
    expect(holidayInstance.isDay(new Date("2023-10-02T00:00:00"))).toBe(false);
  });

  it("checks different year", () => {
    const holidayInstance = new JapaneseHolidayData(holiday);
    expect(holidayInstance.isDay(new Date("2022-10-01"))).toBe(false);
    expect(holidayInstance.isDay(new Date("2024-10-01"))).toBe(false);
  });

  it("checks different month", () => {
    const holidayInstance = new JapaneseHolidayData(holiday);
    expect(holidayInstance.isDay(new Date("2023-09-01"))).toBe(false);
    expect(holidayInstance.isDay(new Date("2023-11-01"))).toBe(false);
  });
});
