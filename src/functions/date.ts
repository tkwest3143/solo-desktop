import { formatDate } from "date-fns";

export function getDayTextForDB(date: Date): string {
  return formatDate(new Date(), "yyyy-MM-dd");
}

export function getDateTextForDB(date: Date): string {
  return formatDate(new Date(), "yyyy-MM-dd HH:mm:00");
}
