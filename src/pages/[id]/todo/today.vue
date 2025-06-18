<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">今日のTodo</h1>
          <p class="text-slate-600">{{ currentDate }}のタスク一覧</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新規作成
        </NuxtLink>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <!-- Filter/Sort Options -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-4">
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>すべて</option>
            <option>未完了</option>
            <option>完了済み</option>
          </select>
          <select v-model="selectedCategoryId" class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option value="">すべてのカテゴリ</option>
            <option v-for="category in categories" :key="category.id" :value="category.id">
              {{ category.name }}
            </option>
          </select>
        </div>
        <div class="text-sm text-slate-500">
          {{ filteredTodos.length }}件のタスク
        </div>
      </div>

      <!-- Task Items -->
      <div v-if="loading" class="flex justify-center py-8">
        <div class="text-slate-500">読み込み中...</div>
      </div>

      <div v-else-if="filteredTodos.length === 0" class="text-center py-8">
        <div class="text-slate-500 text-lg">今日のタスクはありません</div>
        <p class="text-slate-400 mt-2">新しいタスクを追加してみましょう</p>
      </div>

      <div v-else class="space-y-4">
        <div
          v-for="todo in filteredTodos"
          :key="todo.id"
          class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300"
          :class="getTodoPriorityClass(todo)"
        >
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">{{ todo.title }}</h3>
                <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">通常</span>
                <div
                  v-if="todo.color"
                  class="w-3 h-3 rounded-full"
                  :style="{ backgroundColor: todo.color }"
                ></div>
              </div>
              <p v-if="todo.content" class="text-slate-600 mb-3">{{ todo.content }}</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>期限: {{ formatDueDate(todo.due_date) }}</span>
                <span v-if="getCategoryName(todo.category_id)">カテゴリ: {{ getCategoryName(todo.category_id) }}</span>
                <span>作成日: {{ formatDate(todo.created_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import type { TodoItem, TodoCategory } from "~/models/todo";

export default defineComponent({
  layout: "todo",
  data() {
    return {
      currentDate: "",
      todos: [] as TodoItem[],
      categories: [] as TodoCategory[],
      selectedCategoryId: "",
      loading: true,
    };
  },
  computed: {
    filteredTodos(): TodoItem[] {
      if (!this.selectedCategoryId) {
        return this.todos;
      }
      return this.todos.filter(todo => 
        todo.category_id === parseInt(this.selectedCategoryId)
      );
    }
  },
  async mounted() {
    this.updateDateTime();
    await this.fetchData();
  },
  methods: {
    updateDateTime() {
      const now = new Date();
      const options: Intl.DateTimeFormatOptions = {
        year: "numeric",
        month: "long",
        day: "numeric",
      };
      this.currentDate = now.toLocaleDateString("ja-JP", options);
    },
    async fetchData() {
      try {
        this.loading = true;
        const userId = 1; // TODO: Get from user context/auth
        
        // Fetch today's todos and categories in parallel
        const [todosResponse, categoriesResponse] = await Promise.all([
          TodoItemRepository.getTodayTodoItems(userId),
          TodoCategoryRepository.getTodoCategoriesByUserId(userId)
        ]);
        
        this.todos = todosResponse;
        this.categories = categoriesResponse;
      } catch (error) {
        console.error("Failed to fetch data:", error);
      } finally {
        this.loading = false;
      }
    },
    formatDueDate(dateString: string): string {
      const date = new Date(dateString);
      const today = new Date();
      const isToday = date.toDateString() === today.toDateString();
      
      if (isToday) {
        return `今日 ${date.toLocaleTimeString("ja-JP", { hour: "2-digit", minute: "2-digit" })}`;
      }
      
      return date.toLocaleDateString("ja-JP", {
        month: "numeric",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit"
      });
    },
    formatDate(dateString: string): string {
      const date = new Date(dateString);
      return date.toLocaleDateString("ja-JP", {
        year: "numeric",
        month: "numeric",
        day: "numeric"
      });
    },
    getCategoryName(categoryId?: number): string {
      if (!categoryId) return "";
      const category = this.categories.find(c => c.id === categoryId);
      return category?.name || "";
    },
    getTodoPriorityClass(todo: TodoItem): string {
      const dueDate = new Date(todo.due_date);
      const now = new Date();
      const hoursUntilDue = (dueDate.getTime() - now.getTime()) / (1000 * 60 * 60);
      
      if (hoursUntilDue < 2) {
        return "bg-red-50 border-red-300";
      } else if (hoursUntilDue < 6) {
        return "bg-orange-50 border-orange-200";
      }
      return "";
    }
  },
});
</script>