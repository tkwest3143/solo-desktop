import { invoke } from "@tauri-apps/api/core";
import { BaseDirectory, writeFile } from "@tauri-apps/plugin-fs";
import type {
  TodoItem,
  TodoItemForInsert,
  TodoItemForUpdate,
} from "~/models/todo";

export class TodoItemRepository {
  static async getTodoItem(id: number): Promise<TodoItem> {
    const res = await invoke<string>("get_todo_item_by_id", { id });
    return JSON.parse(res) as TodoItem;
  }

  static async getAllTodoItems(userId: number): Promise<TodoItem[]> {
    const res = await invoke<string>("get_all_todo_items", { userId });
    return JSON.parse(res) as TodoItem[];
  }

  static async getTodayTodoItems(userId: number): Promise<TodoItem[]> {
    const res = await invoke<string>("get_today_todo_items", { userId });
    return JSON.parse(res) as TodoItem[];
  }

  static async getUpcomingTodoItems(
    userId: number,
    days?: number
  ): Promise<TodoItem[]> {
    const res = await invoke<string>("get_upcoming_todo_items", {
      userId,
      days,
    });
    return JSON.parse(res) as TodoItem[];
  }

  static async getTodoItemsByCategoryId(
    categoryId: number
  ): Promise<TodoItem[]> {
    const res = await invoke<string>("get_todo_items_by_category_id", {
      categoryId,
    });
    return JSON.parse(res) as TodoItem[];
  }

  static async createTodoItem(todoItem: TodoItemForInsert): Promise<string> {
    const res = await invoke<string>("create_todo_item", {
      todoItem: JSON.stringify(todoItem),
    });
    return res;
  }

  static async updateTodoItem(todoItem: TodoItemForUpdate): Promise<string> {
    const res = await invoke<string>("update_todo_item", {
      todoItem: JSON.stringify(todoItem),
    });
    return res;
  }

  static async deleteTodoItem(id: number): Promise<string> {
    const res = await invoke<string>("delete_todo_item", { id });
    return res;
  }

  static async createTodoAndIcs(
    userId: number,
    startDate: string,
    endDate: string
  ): Promise<string> {
    const res = await invoke<string>("create_todo_and_ics", {
      query: JSON.stringify({
        user_id: userId,
        start_date: startDate,
        end_date: endDate,
      }),
    });
    return res;
  }

  static async createTodoAndIcsAndSave(
    userId: number,
    startDate: string,
    endDate: string,
    fileName: string = "todo.ics"
  ): Promise<string> {
    // ICSテキスト取得
    const icsText = await TodoItemRepository.createTodoAndIcs(
      userId,
      startDate,
      endDate
    );
    // ダウンロードディレクトリに保存
    const encoder = new TextEncoder();
    const icsBytes = encoder.encode(icsText);
    await writeFile(fileName, icsBytes, { baseDir: BaseDirectory.Download });
    return "ICSファイルをダウンロードディレクトリに保存しました";
  }

  static getGoogleCalendarUrl(todo: TodoItem): string {
    // Googleカレンダーのイベント作成URLを生成
    const title = encodeURIComponent(todo.title);
    const details = encodeURIComponent(todo.content || "");
    // 期日をUTC形式に変換（例: 20240701T010000Z）
    const date = new Date(todo.due_date);
    const start = date
      .toISOString()
      .replace(/[-:]/g, "")
      .replace(/\.\d{3}Z$/, "Z");
    // 終了時刻は+1時間（必要に応じて調整）
    const endDate = new Date(date.getTime() + 60 * 60 * 1000);
    const end = endDate
      .toISOString()
      .replace(/[-:]/g, "")
      .replace(/\.\d{3}Z$/, "Z");
    return `https://calendar.google.com/calendar/render?action=TEMPLATE&text=${title}&details=${details}&dates=${start}/${end}`;
  }
}
