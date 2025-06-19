<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <div class="flex h-screen">
      <!-- Left Sidebar -->
      <div class="w-80 bg-white shadow-xl border-r border-slate-200 flex flex-col">
        <!-- Sidebar Header -->
        <div class="p-6 border-b border-slate-200">
          <h1 class="text-2xl font-bold text-slate-800 flex items-center">
            <Icon name="fluent:task-list-square-20-filled" class="mr-3 text-blue-600" size="2em" />
            Todo管理
          </h1>
        </div>

        <!-- Navigation Menu -->
        <nav class="flex-1 p-4 space-y-2">
          <!-- Quick Actions -->
          <div class="mb-6">
            <NuxtLink
              :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
              class="w-full bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg px-4 py-3 flex items-center justify-center transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
            >
              <Icon name="fluent:add-20-filled" class="mr-2" size="1.2em" />
              新しいTodoを作成
            </NuxtLink>
          </div>

          <!-- Menu Items -->
          <div class="space-y-1">
            <NuxtLink
              :to="{ name: 'id-todo-today', params: { id: $route.params.id } }"
              class="flex items-center p-3 rounded-lg hover:bg-slate-100 transition-colors group"
              :class="{ 'bg-blue-50 border-r-4 border-blue-500': $route.name === 'id-todo-today' }"
            >
              <Icon name="fluent:calendar-today-20-filled" class="mr-3 text-blue-500 group-hover:text-blue-600" size="1.5em" />
              <span class="font-medium text-slate-700 group-hover:text-slate-900">今日のTodo</span>
            </NuxtLink>

            <NuxtLink
              :to="{ name: 'id-todo-upcoming', params: { id: $route.params.id } }"
              class="flex items-center p-3 rounded-lg hover:bg-slate-100 transition-colors group"
              :class="{ 'bg-blue-50 border-r-4 border-blue-500': $route.name === 'id-todo-upcoming' }"
            >
              <Icon name="fluent:calendar-clock-20-filled" class="mr-3 text-orange-500 group-hover:text-orange-600" size="1.5em" />
              <span class="font-medium text-slate-700 group-hover:text-slate-900">期日が近いTodo</span>
            </NuxtLink>

            <NuxtLink
              :to="{ name: 'id-todo-all', params: { id: $route.params.id } }"
              class="flex items-center p-3 rounded-lg hover:bg-slate-100 transition-colors group"
              :class="{ 'bg-blue-50 border-r-4 border-blue-500': $route.name === 'id-todo-all' }"
            >
              <Icon name="fluent:list-20-filled" class="mr-3 text-slate-500 group-hover:text-slate-600" size="1.5em" />
              <span class="font-medium text-slate-700 group-hover:text-slate-900">すべてのTodo</span>
            </NuxtLink>
          </div>

          <!-- Categories Section -->
          <div class="mt-8">
            <div class="flex items-center justify-between mb-3">
              <h3 class="text-sm font-semibold text-slate-500 uppercase tracking-wider">カテゴリ</h3>
              <NuxtLink
                :to="{ name: 'id-todo-categories', params: { id: $route.params.id } }"
                class="text-slate-400 hover:text-slate-600 transition-colors"
              >
                <Icon name="fluent:settings-20-filled" size="1.2em" />
              </NuxtLink>
            </div>
            <div class="space-y-1">
              <!-- Dynamic Categories -->
              <div v-if="loading" class="text-center py-2">
                <div class="inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-slate-400"></div>
              </div>
              <div v-else-if="categories.length === 0" class="text-center py-2">
                <p class="text-xs text-slate-400">カテゴリなし</p>
              </div>
              <div v-else>
                <NuxtLink
                  v-for="category in categories" 
                  :key="category.id"
                  :to="{ name: 'id-todo-all', params: { id: $route.params.id }, query: { category: category.id } }"
                  class="flex items-center p-2 rounded-lg hover:bg-slate-100 transition-colors cursor-pointer group"
                  :class="{ 'bg-blue-50 border-r-4 border-blue-500': parseInt($route.query.category as string || '0') === category.id }"
                >
                  <div 
                    class="w-3 h-3 rounded-full mr-3"
                    :style="{ backgroundColor: category.color || '#6b7280' }"
                  ></div>
                  <span class="text-sm text-slate-600 group-hover:text-slate-800">{{ category.name }}</span>
                  <span class="ml-auto text-xs text-slate-400 bg-slate-100 px-2 py-1 rounded-full">
                    {{ categoryTaskCounts[category.id] || 0 }}
                  </span>
                </NuxtLink>
              </div>
            </div>
          </div>
        </nav>

        <!-- Footer -->
        <div class="p-4 border-t border-slate-200">
          <div class="text-xs text-slate-500 text-center">
            Todo管理システム
          </div>
        </div>
      </div>

      <!-- Main Content Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import type { TodoCategory } from "~/models/todo";

export default defineComponent({
  name: "TodoLayout",
  data() {
    return {
      categories: [] as TodoCategory[],
      categoryTaskCounts: {} as Record<number, number>,
      loading: true,
    };
  },
  async mounted() {
    await this.fetchCategories();
  },
  watch: {
    '$route.params.id': {
      handler() {
        this.fetchCategories();
      },
      immediate: false,
    },
  },
  methods: {
    async fetchCategories() {
      if (!this.$route.params.id) return;
      
      try {
        this.loading = true;
        const userId = parseInt(this.$route.params.id as string);
        
        // Fetch categories
        this.categories = await TodoCategoryRepository.getTodoCategoriesByUserId(userId);
        
        // Fetch task counts for each category
        for (const category of this.categories) {
          try {
            const todos = await TodoItemRepository.getTodoItemsByCategoryId(category.id);
            this.categoryTaskCounts[category.id] = todos.length;
          } catch (error) {
            console.error(`Failed to fetch todos for category ${category.id}:`, error);
            this.categoryTaskCounts[category.id] = 0;
          }
        }
      } catch (error) {
        console.error("Failed to fetch categories:", error);
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>

<style scoped>
/* Custom scrollbar for sidebar */
nav {
  scrollbar-width: thin;
  scrollbar-color: #cbd5e1 #f1f5f9;
}

nav::-webkit-scrollbar {
  width: 6px;
}

nav::-webkit-scrollbar-track {
  background: #f1f5f9;
  border-radius: 3px;
}

nav::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

nav::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>