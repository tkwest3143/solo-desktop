import { workSettingData, type workSetting } from "@/models/workSetting";
import { format } from "date-fns";
import { describe, expect, it } from "vitest";

describe("workSettingData", () => {
  const workSetting: workSetting = {
    id: 1,
    user_id: 1,
    title: "Test Setting",
    start: new Date("2023-10-01T09:00:00"),
    end: new Date("2023-10-01T17:00:00"),
    rest_start: new Date("2023-10-01T12:00:00"),
    rest_end: new Date("2023-10-01T13:00:00"),
    working_unit: 60,
    memo: "Test memo",
    is_default: false,
    created_at: new Date(),
    updated_at: new Date(),
  };

  it("returns formatted start time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.startByText).toBe("09:00");
  });

  it("returns formatted end time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.endByText).toBe("17:00");
  });

  it("returns formatted rest start time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.restStartByText).toBe("12:00");
  });

  it("returns formatted rest end time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.restEndByText).toBe("13:00");
  });

  it("returns formatted rest duration", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.restDurationByText).toBe("01:00");
  });

  it("returns formatted work duration", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.workDurationByText).toBe("07:00");
  });

  it("sets start time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    const newStart = new Date("2023-10-01T08:00:00");
    workSettingInstance.start = newStart;
    expect(workSettingInstance.prop.start).toBe(newStart);
  });

  it("sets end time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    const newEnd = new Date("2023-10-01T18:00:00");
    workSettingInstance.end = newEnd;
    expect(workSettingInstance.prop.end).toBe(newEnd);
  });

  it("sets rest start time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    const newRestStart = new Date("2023-10-01T11:00:00");
    workSettingInstance.restStart = newRestStart;
    expect(workSettingInstance.prop.rest_start).toBe(newRestStart);
  });

  it("sets rest end time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    const newRestEnd = new Date("2023-10-01T14:00:00");
    workSettingInstance.restEnd = newRestEnd;
    expect(workSettingInstance.prop.rest_end).toBe(newRestEnd);
  });

  it("sets memo", () => {
    const workSettingInstance = new workSettingData(workSetting);
    const newMemo = "Updated memo";
    workSettingInstance.memo = newMemo;
    expect(workSettingInstance.prop.memo).toBe(newMemo);
  });
  it("returns formatted created at time", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.createdAtText).toBe(
      format(workSetting.created_at, "yyyy-MM-dd HH:mm:ss")
    );
  });

  it("returns work duration in minutes", () => {
    const workSettingInstance = new workSettingData(workSetting);
    expect(workSettingInstance.workDurationByMinute).toBe(420);
  });

  it("returns work duration in minutes without rest", () => {
    const workSettingNoRest: workSetting = {
      ...workSetting,
      start: new Date("2023-10-01T09:00:00"),
      end: new Date("2023-10-01T17:00:00"),
      rest_start: new Date("2023-10-01T00:00:00"),
      rest_end: new Date("2023-10-01T00:00:00"),
      is_default: false,
    };
    const workSettingInstance = new workSettingData(workSettingNoRest);
    expect(workSettingInstance.workDurationByMinute).toBe(480);
  });

  it("returns formatted work duration without rest", () => {
    const workSettingNoRest: workSetting = {
      ...workSetting,
      start: new Date("2023-10-01T09:00:00"),
      end: new Date("2023-10-01T17:00:00"),
      rest_start: new Date("2023-10-01T00:00:00"),
      rest_end: new Date("2023-10-01T00:00:00"),
      is_default: false,
    };
    const workSettingInstance = new workSettingData(workSettingNoRest);
    expect(workSettingInstance.workDurationByText).toBe("08:00");
  });

  it("returns default formatted start time when start is undefined", () => {
    const workSettingUndefinedStart: workSetting = {
      ...workSetting,
      start: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(workSettingUndefinedStart);
    expect(workSettingInstance.startByText).toBe("00:00");
  });

  it("returns default formatted end time when end is undefined", () => {
    const workSettingUndefinedEnd: workSetting = {
      ...workSetting,
      end: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(workSettingUndefinedEnd);
    expect(workSettingInstance.endByText).toBe("00:00");
  });

  it("returns default formatted rest start time when rest start is undefined", () => {
    const workSettingUndefinedRestStart: workSetting = {
      ...workSetting,
      rest_start: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(
      workSettingUndefinedRestStart
    );
    expect(workSettingInstance.restStartByText).toBe("00:00");
  });

  it("returns default formatted rest end time when rest end is undefined", () => {
    const workSettingUndefinedRestEnd: workSetting = {
      ...workSetting,
      rest_end: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(
      workSettingUndefinedRestEnd
    );
    expect(workSettingInstance.restEndByText).toBe("00:00");
  });

  it("returns default formatted rest duration when rest start or rest end is undefined", () => {
    const workSettingUndefinedRest: workSetting = {
      ...workSetting,
      rest_start: undefined as unknown as Date,
      rest_end: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(workSettingUndefinedRest);
    expect(workSettingInstance.restDurationByText).toBe("00:00");
  });

  it("returns default formatted work duration when start or end is undefined", () => {
    const workSettingUndefinedStartEnd: workSetting = {
      ...workSetting,
      start: undefined as unknown as Date,
      end: undefined as unknown as Date,
      is_default: false,
    };
    const workSettingInstance = new workSettingData(
      workSettingUndefinedStartEnd
    );
    expect(workSettingInstance.workDurationByText).toBe("00:00");
  });
});
