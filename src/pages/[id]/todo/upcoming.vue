<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">期日が近いTodo</h1>
          <p class="text-slate-600">優先的に対応が必要なタスク</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-orange-500 hover:bg-orange-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新規作成
        </NuxtLink>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <!-- Filter Options -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-4">
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-orange-500">
            <option>すべて</option>
            <option>未完了</option>
            <option>完了済み</option>
          </select>
          <select v-model="selectedCategoryId" class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-orange-500">
            <option value="">すべてのカテゴリ</option>
            <option v-for="category in categories" :key="category.id" :value="category.id">
              {{ category.name }}
            </option>
          </select>
        </div>
        <div class="text-sm text-slate-500">
          {{ filteredTodos.length }}件の期日が近いタスク
        </div>
      </div>

      <div v-if="loading" class="flex justify-center py-8">
        <div class="text-slate-500">読み込み中...</div>
      </div>

      <div v-else-if="filteredTodos.length === 0" class="text-center py-8">
        <div class="text-slate-500 text-lg">期日が近いタスクはありません</div>
        <p class="text-slate-400 mt-2">素晴らしいです！スケジュール通りに進んでいますね</p>
      </div>

      <div v-else>
        <!-- Urgent Tasks (Today) -->
        <div v-if="todayTasks.length > 0" class="mb-8">
          <h2 class="text-xl font-semibold text-red-600 mb-4 flex items-center">
            <Icon name="fluent:warning-20-filled" class="mr-2" />
            今日期限（緊急）
          </h2>
          <div class="space-y-4">
            <div
              v-for="todo in todayTasks"
              :key="todo.id"
              class="bg-red-50 border-2 border-red-300 rounded-xl p-6 transition-all hover:shadow-lg"
            >
              <div class="flex items-start space-x-4">
                <div class="mt-1">
                  <input type="checkbox" class="w-5 h-5 text-red-500 rounded border-2 border-red-300 focus:ring-red-500" />
                </div>
                <div class="flex-1">
                  <div class="flex items-center space-x-3 mb-2">
                    <h3 class="text-lg font-semibold text-slate-800">{{ todo.title }}</h3>
                    <span class="bg-red-200 text-red-800 text-xs px-2 py-1 rounded-full font-medium">今日期限</span>
                    <div
                      v-if="todo.color"
                      class="w-3 h-3 rounded-full"
                      :style="{ backgroundColor: todo.color }"
                    ></div>
                  </div>
                  <p v-if="todo.content" class="text-slate-600 mb-3">{{ todo.content }}</p>
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4 text-sm text-slate-500">
                      <span class="text-red-600 font-medium">期限: {{ formatDueDate(todo.due_date) }}（{{ getTimeUntilDue(todo.due_date) }}）</span>
                      <span v-if="getCategoryName(todo.category_id)">カテゴリ: {{ getCategoryName(todo.category_id) }}</span>
                    </div>
                    <div class="flex space-x-2">
                      <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                        <Icon name="fluent:edit-20-filled" size="1.2em" />
                      </button>
                      <button 
                        @click="deleteTodo(todo.id)"
                        class="p-2 text-slate-400 hover:text-red-500 transition-colors"
                      >
                        <Icon name="fluent:delete-20-filled" size="1.2em" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Tomorrow's Tasks -->
        <div v-if="tomorrowTasks.length > 0" class="mb-8">
          <h2 class="text-xl font-semibold text-orange-600 mb-4 flex items-center">
            <Icon name="fluent:calendar-20-filled" class="mr-2" />
            明日期限
          </h2>
          <div class="space-y-4">
            <div
              v-for="todo in tomorrowTasks"
              :key="todo.id"
              class="bg-orange-50 border-2 border-orange-200 rounded-xl p-6 transition-all hover:shadow-lg"
            >
              <div class="flex items-start space-x-4">
                <div class="mt-1">
                  <input type="checkbox" class="w-5 h-5 text-orange-500 rounded border-2 border-orange-300 focus:ring-orange-500" />
                </div>
                <div class="flex-1">
                  <div class="flex items-center space-x-3 mb-2">
                    <h3 class="text-lg font-semibold text-slate-800">{{ todo.title }}</h3>
                    <span class="bg-orange-200 text-orange-800 text-xs px-2 py-1 rounded-full font-medium">明日期限</span>
                    <div
                      v-if="todo.color"
                      class="w-3 h-3 rounded-full"
                      :style="{ backgroundColor: todo.color }"
                    ></div>
                  </div>
                  <p v-if="todo.content" class="text-slate-600 mb-3">{{ todo.content }}</p>
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4 text-sm text-slate-500">
                      <span class="text-orange-600 font-medium">期限: {{ formatDueDate(todo.due_date) }}</span>
                      <span v-if="getCategoryName(todo.category_id)">カテゴリ: {{ getCategoryName(todo.category_id) }}</span>
                    </div>
                    <div class="flex space-x-2">
                      <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                        <Icon name="fluent:edit-20-filled" size="1.2em" />
                      </button>
                      <button 
                        @click="deleteTodo(todo.id)"
                        class="p-2 text-slate-400 hover:text-red-500 transition-colors"
                      >
                        <Icon name="fluent:delete-20-filled" size="1.2em" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- This Week's Tasks -->
        <div v-if="thisWeekTasks.length > 0" class="mb-8">
          <h2 class="text-xl font-semibold text-yellow-600 mb-4 flex items-center">
            <Icon name="fluent:calendar-week-start-20-filled" class="mr-2" />
            今週期限
          </h2>
          <div class="space-y-4">
            <div
              v-for="todo in thisWeekTasks"
              :key="todo.id"
              class="bg-yellow-50 border-2 border-yellow-200 rounded-xl p-6 transition-all hover:shadow-lg"
            >
              <div class="flex items-start space-x-4">
                <div class="mt-1">
                  <input type="checkbox" class="w-5 h-5 text-yellow-500 rounded border-2 border-yellow-300 focus:ring-yellow-500" />
                </div>
                <div class="flex-1">
                  <div class="flex items-center space-x-3 mb-2">
                    <h3 class="text-lg font-semibold text-slate-800">{{ todo.title }}</h3>
                    <span class="bg-yellow-200 text-yellow-800 text-xs px-2 py-1 rounded-full font-medium">今週期限</span>
                    <div
                      v-if="todo.color"
                      class="w-3 h-3 rounded-full"
                      :style="{ backgroundColor: todo.color }"
                    ></div>
                  </div>
                  <p v-if="todo.content" class="text-slate-600 mb-3">{{ todo.content }}</p>
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4 text-sm text-slate-500">
                      <span class="text-yellow-600 font-medium">期限: {{ formatDueDate(todo.due_date) }}</span>
                      <span v-if="getCategoryName(todo.category_id)">カテゴリ: {{ getCategoryName(todo.category_id) }}</span>
                    </div>
                    <div class="flex space-x-2">
                      <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                        <Icon name="fluent:edit-20-filled" size="1.2em" />
                      </button>
                      <button 
                        @click="deleteTodo(todo.id)"
                        class="p-2 text-slate-400 hover:text-red-500 transition-colors"
                      >
                        <Icon name="fluent:delete-20-filled" size="1.2em" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Tips Section -->
      <div class="bg-blue-50 border border-blue-200 rounded-xl p-6">
        <div class="flex items-start space-x-3">
          <Icon name="fluent:lightbulb-20-filled" class="text-blue-500 mt-1" size="1.5em" />
          <div>
            <h3 class="font-semibold text-slate-800 mb-2">効果的なタスク管理のヒント</h3>
            <ul class="text-sm text-slate-600 space-y-1">
              <li>• 緊急度と重要度を区別して優先順位を付ける</li>
              <li>• 大きなタスクは小さなタスクに分割する</li>
              <li>• 期限の前日にリマインダーを設定する</li>
              <li>• 定期的にタスクの進捗状況を確認する</li>
            </ul>
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

definePageMeta({
  layout: 'todo'
});

export default defineComponent({
  data() {
    return {
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
    },
    todayTasks(): TodoItem[] {
      const today = new Date();
      const todayStr = today.toDateString();
      return this.filteredTodos.filter(todo => {
        const dueDate = new Date(todo.due_date);
        return dueDate.toDateString() === todayStr;
      });
    },
    tomorrowTasks(): TodoItem[] {
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);
      const tomorrowStr = tomorrow.toDateString();
      return this.filteredTodos.filter(todo => {
        const dueDate = new Date(todo.due_date);
        return dueDate.toDateString() === tomorrowStr;
      });
    },
    thisWeekTasks(): TodoItem[] {
      const today = new Date();
      const dayOfWeek = today.getDay();
      const daysUntilSunday = (7 - dayOfWeek) % 7;
      const endOfWeek = new Date(today);
      endOfWeek.setDate(today.getDate() + daysUntilSunday);
      
      return this.filteredTodos.filter(todo => {
        const dueDate = new Date(todo.due_date);
        const todayStr = today.toDateString();
        const tomorrowStr = new Date(today.getTime() + 24 * 60 * 60 * 1000).toDateString();
        
        // Exclude today and tomorrow tasks
        return dueDate > today && 
               dueDate <= endOfWeek && 
               dueDate.toDateString() !== todayStr && 
               dueDate.toDateString() !== tomorrowStr;
      });
    }
  },
  async mounted() {
    await this.fetchData();
  },
  methods: {
    async fetchData() {
      try {
        this.loading = true;
        const userId = parseInt(this.$route.params.id as string);
        
        // Fetch upcoming todos (7 days) and categories in parallel
        const [todosResponse, categoriesResponse] = await Promise.all([
          TodoItemRepository.getUpcomingTodoItems(userId, 7),
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
      const tomorrow = new Date(today);
      tomorrow.setDate(today.getDate() + 1);
      
      if (date.toDateString() === today.toDateString()) {
        return `今日 ${date.toLocaleTimeString("ja-JP", { hour: "2-digit", minute: "2-digit" })}`;
      } else if (date.toDateString() === tomorrow.toDateString()) {
        return `明日 ${date.toLocaleTimeString("ja-JP", { hour: "2-digit", minute: "2-digit" })}`;
      }
      
      return date.toLocaleDateString("ja-JP", {
        month: "numeric",
        day: "numeric",
        weekday: "short",
        hour: "2-digit",
        minute: "2-digit"
      });
    },
    getCategoryName(categoryId?: number): string {
      if (!categoryId) return "";
      const category = this.categories.find(c => c.id === categoryId);
      return category?.name || "";
    },
    getTimeUntilDue(dateString: string): string {
      const dueDate = new Date(dateString);
      const now = new Date();
      const hoursUntilDue = (dueDate.getTime() - now.getTime()) / (1000 * 60 * 60);
      
      if (hoursUntilDue < 0) {
        return "期限超過";
      } else if (hoursUntilDue < 1) {
        const minutes = Math.floor(hoursUntilDue * 60);
        return `残り${minutes}分`;
      } else if (hoursUntilDue < 24) {
        return `残り${Math.floor(hoursUntilDue)}時間`;
      } else {
        const days = Math.floor(hoursUntilDue / 24);
        return `残り${days}日`;
      }
    },
    async deleteTodo(id: number) {
      console.log("deleteTodo called with id:", id);
      
      if (!confirm("このタスクを削除しますか？")) {
        console.log("User cancelled deletion");
        return;
      }
      
      try {
        console.log("Attempting to delete todo with id:", id);
        await TodoItemRepository.deleteTodoItem(id);
        // Remove from local array
        this.todos = this.todos.filter(todo => todo.id !== id);
        console.log("Successfully deleted todo");
        alert("タスクが削除されました");
      } catch (error) {
        console.error("Failed to delete todo:", error);
        alert("タスクの削除に失敗しました");
      }
    }
  },
});
</script>