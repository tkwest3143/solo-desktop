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
              <p class="text-3xl font-bold">5</p>
              <p class="text-blue-200 text-sm">3個完了済み</p>
            </div>
            <Icon name="fluent:calendar-today-20-filled" size="3em" class="text-blue-200" />
          </div>
        </div>

        <!-- Upcoming Tasks Card -->
        <div class="bg-gradient-to-br from-orange-500 to-orange-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-orange-100 text-sm">期日が近い</p>
              <p class="text-3xl font-bold">3</p>
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
              <p class="text-3xl font-bold">12</p>
              <p class="text-green-200 text-sm">4カテゴリ</p>
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
          <!-- Sample task items -->
          <div class="space-y-3">
            <div class="flex items-center p-3 bg-slate-50 rounded-lg">
              <div class="w-4 h-4 border-2 border-slate-300 rounded mr-3"></div>
              <div class="flex-1">
                <div class="text-slate-800 font-medium">サンプルタスク1</div>
                <div class="text-sm text-slate-500">期限: 2024-06-20</div>
              </div>
              <div class="w-3 h-3 rounded-full bg-red-400"></div>
            </div>
            <div class="flex items-center p-3 bg-slate-50 rounded-lg">
              <div class="w-4 h-4 border-2 border-slate-300 rounded mr-3"></div>
              <div class="flex-1">
                <div class="text-slate-800 font-medium">サンプルタスク2</div>
                <div class="text-sm text-slate-500">期限: 2024-06-22</div>
              </div>
              <div class="w-3 h-3 rounded-full bg-green-400"></div>
            </div>
            <div class="flex items-center p-3 bg-slate-50 rounded-lg">
              <div class="w-4 h-4 border-2 border-slate-300 rounded mr-3"></div>
              <div class="flex-1">
                <div class="text-slate-800 font-medium">サンプルタスク3</div>
                <div class="text-sm text-slate-500">期限: 2024-06-25</div>
              </div>
              <div class="w-3 h-3 rounded-full bg-blue-400"></div>
            </div>
          </div>
          <div class="mt-4 text-center text-slate-500 text-sm">
            実際のデータはこれから実装予定です
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  layout: "todo",
  data() {
    return {
      currentDate: "",
      currentDay: "",
    };
  },
  mounted() {
    this.updateDateTime();
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
  },
});
</script>