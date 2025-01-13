import { JapaneseHolidayData } from "@/models/japaneseHoliday";
import { workTimeData, type workTime } from "@/models/workTime";
import { describe, expect, it } from "vitest";

describe("workTimeData", () => {
  const workTime: workTime = {
    id: 1,
    target_day: "2023-10-01",
    start: new Date("2023-10-01T09:00:00"),
    end: new Date("2023-10-01T17:00:00"),
    rest_start: new Date("2023-10-01T12:00:00"),
    rest_end: new Date("2023-10-01T13:00:00"),
    memo: "Test memo",
    created_at: new Date(),
    updated_at: new Date(),
  };

  it("returns formatted start time", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.startByText).toBe("09:00");
  });

  it("returns default start time when start is not defined", () => {
    const workTimeWithoutStart: workTime = { ...workTime, start: undefined };
    const workTimeInstance = new workTimeData(workTimeWithoutStart);
    expect(workTimeInstance.startByText).toBe("00:00");
  });

  it("returns default end time when end is not defined", () => {
    const workTimeWithoutEnd: workTime = { ...workTime, end: undefined };
    const workTimeInstance = new workTimeData(workTimeWithoutEnd);
    expect(workTimeInstance.endByText).toBe("00:00");
  });

  it("returns default rest start time when rest start is not defined", () => {
    const workTimeWithoutRestStart: workTime = {
      ...workTime,
      rest_start: undefined,
    };
    const workTimeInstance = new workTimeData(workTimeWithoutRestStart);
    expect(workTimeInstance.restStartByText).toBe("00:00");
  });

  it("returns default rest end time when rest end is not defined", () => {
    const workTimeWithoutRestEnd: workTime = {
      ...workTime,
      rest_end: undefined,
    };
    const workTimeInstance = new workTimeData(workTimeWithoutRestEnd);
    expect(workTimeInstance.restEndByText).toBe("00:00");
  });

  it("returns default rest duration when rest start or rest end is not defined", () => {
    const workTimeWithoutRest: workTime = {
      ...workTime,
      rest_start: undefined,
      rest_end: undefined,
    };
    const workTimeInstance = new workTimeData(workTimeWithoutRest);
    expect(workTimeInstance.restDurationByText).toBe("00:00");
  });

  it("returns default work duration when start or end is not defined", () => {
    const workTimeWithoutStartOrEnd: workTime = {
      ...workTime,
      start: undefined,
      end: undefined,
    };
    const workTimeInstance = new workTimeData(workTimeWithoutStartOrEnd);
    expect(workTimeInstance.workDurationByText).toBe("00:00");
  });

  it("returns formatted end time", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.endByText).toBe("17:00");
  });

  it("returns formatted rest start time", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.restStartByText).toBe("12:00");
  });

  it("returns formatted rest end time", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.restEndByText).toBe("13:00");
  });

  it("returns formatted rest duration", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.restDurationByText).toBe("01:00");
  });

  it("returns formatted work duration", () => {
    const workTimeInstance = new workTimeData(workTime);
    expect(workTimeInstance.workDurationByText).toBe("07:00");
  });

  it("sets start time", () => {
    const workTimeInstance = new workTimeData(workTime);
    const newStart = new Date("2023-10-01T08:00:00");
    workTimeInstance.start = newStart;
    expect(workTimeInstance.prop.start).toBe(newStart);
  });

  it("sets end time", () => {
    const workTimeInstance = new workTimeData(workTime);
    const newEnd = new Date("2023-10-01T18:00:00");
    workTimeInstance.end = newEnd;
    expect(workTimeInstance.prop.end).toBe(newEnd);
  });

  it("sets rest start time", () => {
    const workTimeInstance = new workTimeData(workTime);
    const newRestStart = new Date("2023-10-01T11:00:00");
    workTimeInstance.restStart = newRestStart;
    expect(workTimeInstance.prop.rest_start).toBe(newRestStart);
  });

  it("sets rest end time", () => {
    const workTimeInstance = new workTimeData(workTime);
    const newRestEnd = new Date("2023-10-01T14:00:00");
    workTimeInstance.restEnd = newRestEnd;
    expect(workTimeInstance.prop.rest_end).toBe(newRestEnd);
  });

  it("sets memo", () => {
    const workTimeInstance = new workTimeData(workTime);
    const newMemo = "Updated memo";
    workTimeInstance.memo = newMemo;
    expect(workTimeInstance.prop.memo).toBe(newMemo);
  });
  describe("getDayTextWithWeek", () => {
    it("returns formatted day text with week and holiday", () => {
      const holidays: JapaneseHolidayData[] = [
        {
          prop: { id: 1, subject: "テスト", day: new Date("2023-10-01") },
          isDay: (date: Date) =>
            date.toDateString() === new Date("2023-10-01").toDateString(),
        },
      ];
      const workTimeInstance = new workTimeData(workTime);
      expect(workTimeInstance.getDayTextWithWeek(holidays)).toBe(
        "2023/10/1(日・祝)"
      );
    });

    it("returns formatted day text with week without holiday", () => {
      const holidays: JapaneseHolidayData[] = [
        {
          prop: { id: 1, subject: "テスト", day: new Date("2023-10-02") },
          isDay: (date: Date) =>
            date.toDateString() === new Date("2023-10-02").toDateString(),
        },
      ];
      const workTimeInstance = new workTimeData(workTime);
      expect(workTimeInstance.getDayTextWithWeek(holidays)).toBe(
        "2023/10/1(日)"
      );
    });
  });
  describe("workTimeData additional tests", () => {
    const workTime: workTime = {
      id: 1,
      target_day: "2023-10-01",
      start: new Date("2023-10-01T09:00:00"),
      end: new Date("2023-10-01T17:00:00"),
      rest_start: new Date("2023-10-01T12:00:00"),
      rest_end: new Date("2023-10-01T13:00:00"),
      memo: "Test memo",
      created_at: new Date(),
      updated_at: new Date(),
    };

    describe("workDurationByMinute", () => {
      it("returns correct work duration in minutes", () => {
        const workTimeInstance = new workTimeData(workTime);
        expect(workTimeInstance.workDurationByMinute).toBe(420);
      });

      it("returns 0 when start is not defined", () => {
        const workTimeWithoutStart: workTime = {
          ...workTime,
          start: undefined,
        };
        const workTimeInstance = new workTimeData(workTimeWithoutStart);
        expect(workTimeInstance.workDurationByMinute).toBe(0);
      });

      it("returns 0 when end is not defined", () => {
        const workTimeWithoutEnd: workTime = { ...workTime, end: undefined };
        const workTimeInstance = new workTimeData(workTimeWithoutEnd);
        expect(workTimeInstance.workDurationByMinute).toBe(0);
      });

      it("returns correct work duration in minutes without rest time", () => {
        const workTimeWithoutRest: workTime = {
          ...workTime,
          rest_start: undefined,
          rest_end: undefined,
        };
        const workTimeInstance = new workTimeData(workTimeWithoutRest);
        expect(workTimeInstance.workDurationByMinute).toBe(480);
      });
    });

    describe("workDurationByText", () => {
      it("returns correct work duration in HH:mm format", () => {
        const workTimeInstance = new workTimeData(workTime);
        expect(workTimeInstance.workDurationByText).toBe("07:00");
      });

      it("returns 00:00 when start is not defined", () => {
        const workTimeWithoutStart: workTime = {
          ...workTime,
          start: undefined,
        };
        const workTimeInstance = new workTimeData(workTimeWithoutStart);
        expect(workTimeInstance.workDurationByText).toBe("00:00");
      });

      it("returns 00:00 when end is not defined", () => {
        const workTimeWithoutEnd: workTime = { ...workTime, end: undefined };
        const workTimeInstance = new workTimeData(workTimeWithoutEnd);
        expect(workTimeInstance.workDurationByText).toBe("00:00");
      });

      it("returns correct work duration in HH:mm format without rest time", () => {
        const workTimeWithoutRest: workTime = {
          ...workTime,
          rest_start: undefined,
          rest_end: undefined,
        };
        const workTimeInstance = new workTimeData(workTimeWithoutRest);
        expect(workTimeInstance.workDurationByText).toBe("08:00");
      });
    });
  });
});
