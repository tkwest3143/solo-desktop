import { MonthForWork } from "@/models/monthForWork";
import { workTimeData } from "@/models/workTime";
import { describe, expect, it } from "vitest";

describe("MonthForWork", () => {
  const workTimes = [
    new workTimeData({
      id: 1,
      target_day: "2023-10-01",
      start: new Date("2023-10-01T09:00:00"),
      end: new Date("2023-10-01T17:00:00"),
      rest_start: new Date("2023-10-01T12:00:00"),
      rest_end: new Date("2023-10-01T13:00:00"),
      created_at: new Date(),
      updated_at: new Date(),
    }),
  ];

  const month = new MonthForWork(new Date("2023-10-01"), workTimes);
  it("returns month text", () => {
    expect(month.byText).toBe("2023-10");
  });

  it("returns year text", () => {
    expect(month.yearText).toBe("2023");
  });

  it("returns month text", () => {
    expect(month.monthText).toBe("10");
  });

  it("returns month work times", () => {
    expect(month.monthWorkTimes.length).toBeGreaterThan(0);
  });

  it("returns days in month", () => {
    expect(month.days.length).toBe(31);
  });

  it("returns previous month", () => {
    expect(month.previousMonth.byText).toBe("2023-09");
  });

  it("returns next month", () => {
    expect(month.nextMonth.byText).toBe("2023-11");
  });

  it("returns this month", () => {
    expect(month.thisMonth.byText).toBe(
      new MonthForWork(new Date(), workTimes).byText
    );
  });
  it("sets workTimeDatas", () => {
    const newWorkTimes = [
      new workTimeData({
        id: 2,
        target_day: "2023-10-02",
        start: new Date("2023-10-02T09:00:00"),
        end: new Date("2023-10-02T17:00:00"),
        rest_start: new Date("2023-10-02T12:00:00"),
        rest_end: new Date("2023-10-02T13:00:00"),
        created_at: new Date(),
        updated_at: new Date(),
      }),
    ];
    month.workTimeDatas = newWorkTimes;
    expect(month.monthWorkTimes.length).toBe(31);
    expect(month.monthWorkTimes[1].prop.id).toBe(2);
  });

  it("handles empty workTimes", () => {
    const emptyMonth = new MonthForWork(new Date("2023-10-01"), []);
    expect(emptyMonth.monthWorkTimes.length).toBe(31);
    expect(emptyMonth.monthWorkTimes[0].prop.id).toBe(-1);
  });

  it("handles month with no work times", () => {
    const noWorkTimesMonth = new MonthForWork(new Date("2023-11-01"), []);
    expect(noWorkTimesMonth.monthWorkTimes.length).toBe(30);
    expect(noWorkTimesMonth.monthWorkTimes[0].prop.id).toBe(-1);
  });

  it("handles leap year February", () => {
    const leapYearMonth = new MonthForWork(new Date("2024-02-01"), []);
    expect(leapYearMonth.days.length).toBe(29);
  });

  it("handles non-leap year February", () => {
    const nonLeapYearMonth = new MonthForWork(new Date("2023-02-01"), []);
    expect(nonLeapYearMonth.days.length).toBe(28);
  });

  it("handles month with 30 days", () => {
    const monthWith30Days = new MonthForWork(new Date("2023-04-01"), []);
    expect(monthWith30Days.days.length).toBe(30);
  });

  it("handles month with 31 days", () => {
    const monthWith31Days = new MonthForWork(new Date("2023-01-01"), []);
    expect(monthWith31Days.days.length).toBe(31);
  });
});
