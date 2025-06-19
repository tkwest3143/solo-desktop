import { invoke } from "@tauri-apps/api/core";
import type { TodoItem, TodoItemForInsert, TodoItemForUpdate } from "~/models/todo";

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

  static async getUpcomingTodoItems(userId: number, days?: number): Promise<TodoItem[]> {
    const res = await invoke<string>("get_upcoming_todo_items", { userId, days });
    return JSON.parse(res) as TodoItem[];
  }

  static async getTodoItemsByCategoryId(categoryId: number): Promise<TodoItem[]> {
    const res = await invoke<string>("get_todo_items_by_category_id", { categoryId });
    return JSON.parse(res) as TodoItem[];
  }

  static async createTodoItem(todoItem: TodoItemForInsert): Promise<string> {
    const res = await invoke<string>("create_todo_item", { 
      todoItem: JSON.stringify(todoItem) 
    });
    return res;
  }

  static async updateTodoItem(todoItem: TodoItemForUpdate): Promise<string> {
    const res = await invoke<string>("update_todo_item", { 
      todoItem: JSON.stringify(todoItem) 
    });
    return res;
  }

  static async deleteTodoItem(id: number): Promise<string> {
    const res = await invoke<string>("delete_todo_item", { id });
    return res;
  }
}