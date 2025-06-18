<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">Todo管理</h1>
          <p class="text-slate-600">効率的なタスク管理で生産性を向上させましょう</p>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">{{ currentDate }}</div>
          <div class="text-lg font-semibold text-slate-700">{{ currentDay }}</div>
        </div>
      </div>
    </div>

    <!-- Main Dashboard Content -->
    <div class="p-8">
      <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6 mb-8">
        <!-- Today's Stats Card -->
        <div class="bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-blue-100 text-sm">今日のタスク</p>
              <p class="text-3xl font-bold">{{ stats.today }}</p>
              <p class="text-blue-200 text-sm">{{ stats.todayCompleted }}個完了済み</p>
            </div>
            <Icon name="fluent:calendar-today-20-filled" size="3em" class="text-blue-200" />
          </div>
        </div>

        <!-- Upcoming Tasks Card -->
        <div class="bg-gradient-to-br from-orange-500 to-orange-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-orange-100 text-sm">期日が近い</p>
              <p class="text-3xl font-bold">{{ stats.upcoming }}</p>
              <p class="text-orange-200 text-sm">要注意タスク</p>
            </div>
            <Icon name="fluent:calendar-clock-20-filled" size="3em" class="text-orange-200" />
          </div>
        </div>

        <!-- Total Tasks Card -->
        <div class="bg-gradient-to-br from-green-500 to-green-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-green-100 text-sm">総タスク数</p>
              <p class="text-3xl font-bold">{{ stats.total }}</p>
              <p class="text-green-200 text-sm">{{ stats.categoryCount }}カテゴリ</p>
            </div>
            <Icon name="fluent:task-list-square-20-filled" size="3em" class="text-green-200" />
          </div>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <NuxtLink
          :to="{ name: 'id-todo-today', params: { id: $route.params.id } }"
          class="bg-white border-2 border-slate-200 hover:border-blue-300 rounded-lg p-6 text-center transition-all duration-200 hover:shadow-lg group"
        >
          <Icon name="fluent:calendar-today-20-filled" size="2.5em" class="text-blue-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 class="font-semibold text-slate-800 mb-1">今日のTodo</h3>
          <p class="text-sm text-slate-600">今日締切のタスクを確認</p>
        </NuxtLink>

        <NuxtLink
          :to="{ name: 'id-todo-upcoming', params: { id: $route.params.id } }"
          class="bg-white border-2 border-slate-200 hover:border-orange-300 rounded-lg p-6 text-center transition-all duration-200 hover:shadow-lg group"
        >
          <Icon name="fluent:calendar-clock-20-filled" size="2.5em" class="text-orange-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 class="font-semibold text-slate-800 mb-1">期日が近い</h3>
          <p class="text-sm text-slate-600">優先的に対応が必要</p>
        </NuxtLink>

        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-white border-2 border-slate-200 hover:border-green-300 rounded-lg p-6 text-center transition-all duration-200 hover:shadow-lg group"
        >
          <Icon name="fluent:add-20-filled" size="2.5em" class="text-green-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 class="font-semibold text-slate-800 mb-1">新規作成</h3>
          <p class="text-sm text-slate-600">新しいタスクを追加</p>
        </NuxtLink>

        <NuxtLink
          :to="{ name: 'id-todo-categories', params: { id: $route.params.id } }"
          class="bg-white border-2 border-slate-200 hover:border-purple-300 rounded-lg p-6 text-center transition-all duration-200 hover:shadow-lg group"
        >
          <Icon name="fluent:folder-20-filled" size="2.5em" class="text-purple-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 class="font-semibold text-slate-800 mb-1">カテゴリ管理</h3>
          <p class="text-sm text-slate-600">カテゴリの設定・編集</p>
        </NuxtLink>
      </div>

      <!-- Recent Tasks Preview -->
      <div class="bg-white rounded-xl shadow-sm border border-slate-200">
        <div class="p-6 border-b border-slate-200">
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold text-slate-800">最近のタスク</h2>
            <NuxtLink
              :to="{ name: 'id-todo-all', params: { id: $route.params.id } }"
              class="text-blue-600 hover:text-blue-700 text-sm font-medium"
            >
              すべて表示 →
            </NuxtLink>
          </div>
        </div>
        <div class="p-6">
          <div v-if="loading" class="text-center py-8">
            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            <p class="mt-2 text-slate-600">読み込み中...</p>
          </div>
          <div v-else-if="recentTasks.length === 0" class="text-center py-8">
            <Icon name="fluent:document-add-20-filled" size="3em" class="text-slate-300 mb-4" />
            <p class="text-slate-500">まだタスクがありません</p>
            <NuxtLink
              :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
              class="mt-2 inline-block text-blue-600 hover:text-blue-700 text-sm font-medium"
            >
              最初のタスクを作成 →
            </NuxtLink>
          </div>
          <div v-else class="space-y-3">
            <div 
              v-for="task in recentTasks" 
              :key="task.id"
              class="flex items-center p-3 bg-slate-50 rounded-lg hover:bg-slate-100 transition-colors"
            >
              <div 
                :class="[
                  'w-4 h-4 border-2 rounded mr-3',
                  task.completed ? 'bg-green-500 border-green-500' : 'border-slate-300'
                ]"
              ></div>
              <div class="flex-1">
                <div :class="['font-medium', task.completed ? 'text-slate-500 line-through' : 'text-slate-800']">
                  {{ task.title }}
                </div>
                <div class="text-sm text-slate-500">
                  期限: {{ formatDate(task.due_date) }}
                </div>
              </div>
              <div 
                v-if="task.category"
                class="w-3 h-3 rounded-full"
                :style="{ backgroundColor: task.category.color || '#6b7280' }"
                :title="task.category.name"
              ></div>
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
      currentDay: "",
      loading: true,
      stats: {
        today: 0,
        todayCompleted: 0,
        upcoming: 0,
        total: 0,
        categoryCount: 0,
      },
      recentTasks: [] as TodoItem[],
    };
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
      
      const days = ["日", "月", "火", "水", "木", "金", "土"];
      this.currentDay = days[now.getDay()] + "曜日";
    },
    async fetchData() {
      try {
        this.loading = true;
        const userId = this.$route.params.id as string;
        
        // Fetch all data in parallel
        const [allTodos, todayTodos, upcomingTodos, categories] = await Promise.all([
          TodoItemRepository.getAllTodoItems(userId),
          TodoItemRepository.getTodayTodoItems(userId),
          TodoItemRepository.getUpcomingTodoItems(userId),
          TodoCategoryRepository.getTodoCategoriesByUserId(userId),
        ]);
        
        // Update stats
        this.stats.today = todayTodos.length;
        this.stats.todayCompleted = todayTodos.filter(todo => todo.completed).length;
        this.stats.upcoming = upcomingTodos.length;
        this.stats.total = allTodos.length;
        this.stats.categoryCount = categories.length;
        
        // Get recent tasks (last 5 tasks sorted by creation date)
        this.recentTasks = allTodos
          .sort((a, b) => new Date(b.created_at || '').getTime() - new Date(a.created_at || '').getTime())
          .slice(0, 5);
        
      } catch (error) {
        console.error("Failed to fetch dashboard data:", error);
      } finally {
        this.loading = false;
      }
    },
    formatDate(dateString: string): string {
      if (!dateString) return "";
      const date = new Date(dateString);
      return date.toLocaleDateString("ja-JP");
    },
  },
});
</script>