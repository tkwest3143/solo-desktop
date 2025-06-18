import { describe, it, expect } from "vitest";
import { aggregateWorkTimeByMonth, aggregateWorkTimeByYear } from "~/functions/reportData";
import { workTimeData } from "~/models/workTime";

describe("reportData", () => {
  const createMockWorkTime = (targetDay: string, workMinutes: number) => {
    const startTime = new Date(`${targetDay}T09:00:00`);
    const endTime = new Date(startTime.getTime() + workMinutes * 60 * 1000);
    
    return new workTimeData({
      id: 1,
      target_day: targetDay,
      start: startTime,
      end: endTime,
      rest_start: new Date(`${targetDay}T12:00:00`),
      rest_end: new Date(`${targetDay}T13:00:00`),
      created_at: new Date(),
      updated_at: new Date(),
    });
  };

  describe("aggregateWorkTimeByMonth", () => {
    it("should aggregate work time data by month", () => {
      const workTimes = [
        createMockWorkTime("2024-01-15", 480), // 8 hours
        createMockWorkTime("2024-01-16", 480), // 8 hours
        createMockWorkTime("2024-02-15", 420), // 7 hours
      ];

      const result = aggregateWorkTimeByMonth(workTimes);

      expect(result).toHaveLength(2);
      
      // January 2024
      expect(result[0].year).toBe(2024);
      expect(result[0].month).toBe(1);
      expect(result[0].totalMinutes).toBe(900); // 480 + 420 (8 + 7 hours minus 1 hour break each)
      expect(result[0].totalHours).toBe(15);
      expect(result[0].workDays).toBe(2);
      expect(result[0].averageHoursPerDay).toBe(7.5);

      // February 2024
      expect(result[1].year).toBe(2024);
      expect(result[1].month).toBe(2);
      expect(result[1].totalMinutes).toBe(360); // 420 - 60 (7 hours minus 1 hour break)
      expect(result[1].totalHours).toBe(6);
      expect(result[1].workDays).toBe(1);
      expect(result[1].averageHoursPerDay).toBe(6);
    });

    it("should handle empty work times", () => {
      const result = aggregateWorkTimeByMonth([]);
      expect(result).toHaveLength(0);
    });

    it("should ignore work times with zero minutes", () => {
      const workTimes = [createMockWorkTime("2024-01-15", 0)];
      const result = aggregateWorkTimeByMonth(workTimes);
      expect(result).toHaveLength(0);
    });
  });

  describe("aggregateWorkTimeByYear", () => {
    it("should aggregate work time data by year", () => {
      const workTimes = [
        createMockWorkTime("2024-01-15", 480), // 8 hours
        createMockWorkTime("2024-02-15", 420), // 7 hours
        createMockWorkTime("2025-01-15", 480), // 8 hours
      ];

      const result = aggregateWorkTimeByYear(workTimes);

      expect(result).toHaveLength(2);
      
      // 2024
      expect(result[0].year).toBe(2024);
      expect(result[0].totalMinutes).toBe(780); // 420 + 360 (7 + 6 hours after breaks)
      expect(result[0].totalHours).toBe(13);
      expect(result[0].workDays).toBe(2);
      expect(result[0].monthlyData).toHaveLength(2);

      // 2025
      expect(result[1].year).toBe(2025);
      expect(result[1].totalMinutes).toBe(420); // 7 hours after break
      expect(result[1].totalHours).toBe(7);
      expect(result[1].workDays).toBe(1);
      expect(result[1].monthlyData).toHaveLength(1);
    });
  });
});