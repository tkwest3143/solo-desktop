<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">すべてのTodo</h1>
          <p class="text-slate-600">作成されたすべてのタスクを管理</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-green-500 hover:bg-green-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新規作成
        </NuxtLink>
      </div>
    </div>

    <!-- Filters -->
    <div class="p-8 border-b border-slate-100">
      <div class="flex flex-col lg:flex-row lg:items-center space-y-4 lg:space-y-0 lg:space-x-4">
        <!-- Search -->
        <div class="flex-1">
          <div class="relative">
            <Icon name="fluent:search-20-filled" class="absolute left-3 top-1/2 transform -translate-y-1/2 text-slate-400" size="1.2em" />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="タスクを検索..."
              class="w-full pl-10 pr-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>
        
        <!-- Filters -->
        <div class="flex space-x-3">
          <select v-model="statusFilter" class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option value="">すべて</option>
            <option value="pending">未完了</option>
            <option value="completed">完了済み</option>
          </select>
          <select v-model="selectedCategoryId" class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option value="">すべてのカテゴリ</option>
            <option v-for="category in categories" :key="category.id" :value="category.id">
              {{ category.name }}
            </option>
          </select>
          <select v-model="sortBy" class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option value="due_date">期日順</option>
            <option value="created_at">作成日順</option>
            <option value="title">タイトル順</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <div class="mb-4 text-sm text-slate-500">
        {{ filteredTodos.length }}件のタスク
      </div>

      <div v-if="loading" class="flex justify-center py-8">
        <div class="text-slate-500">読み込み中...</div>
      </div>

      <div v-else-if="filteredTodos.length === 0" class="text-center py-8">
        <div class="text-slate-500 text-lg">
          {{ searchQuery || selectedCategoryId || statusFilter ? 'マッチするタスクがありません' : 'タスクがありません' }}
        </div>
        <p class="text-slate-400 mt-2">新しいタスクを追加してみましょう</p>
      </div>

      <!-- Tasks Grid -->
      <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div
          v-for="todo in filteredTodos"
          :key="todo.id"
          class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300"
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
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期日: {{ formatDueDate(todo.due_date) }}</span>
                  <span v-if="getCategoryName(todo.category_id)">{{ getCategoryName(todo.category_id) }}</span>
                </div>
                <div class="flex space-x-2">
                  <NuxtLink
                    :to="{
                      name: 'id-todo-edit',
                      params: { id: $route.params.id },
                      query: { id: todo.id }
                    }"
                    class="p-2 text-slate-400 hover:text-blue-500 transition-colors"
                  >
                    <Icon name="fluent:edit-20-filled" size="1.2em" />
                  </NuxtLink>
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
      searchQuery: "",
      selectedCategoryId: "",
      statusFilter: "",
      sortBy: "due_date",
      loading: true,
    };
  },
  computed: {
    filteredTodos(): TodoItem[] {
      let filtered = [...this.todos];

      // Filter by search query
      if (this.searchQuery) {
        const query = this.searchQuery.toLowerCase();
        filtered = filtered.filter(todo => 
          todo.title.toLowerCase().includes(query) ||
          (todo.content && todo.content.toLowerCase().includes(query))
        );
      }

      // Filter by category
      if (this.selectedCategoryId) {
        filtered = filtered.filter(todo => 
          todo.category_id === parseInt(this.selectedCategoryId)
        );
      }

      // Filter by status (not implemented in backend yet, so just return all for now)
      // TODO: Add status/completed field to todo items

      // Sort
      if (this.sortBy === "due_date") {
        filtered.sort((a, b) => new Date(a.due_date).getTime() - new Date(b.due_date).getTime());
      } else if (this.sortBy === "created_at") {
        filtered.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
      } else if (this.sortBy === "title") {
        filtered.sort((a, b) => a.title.localeCompare(b.title));
      }

      return filtered;
    }
  },
  async mounted() {
    await this.fetchData();
    // Set category filter from query parameter
    if (this.$route.query.category) {
      this.selectedCategoryId = this.$route.query.category as string;
    }
  },
  watch: {
    '$route.query.category': {
      handler(newCategory) {
        this.selectedCategoryId = newCategory as string || "";
      },
      immediate: false,
    },
  },
  methods: {
    async fetchData() {
      try {
        this.loading = true;
        const userId = parseInt(this.$route.params.id as string);
        
        // Fetch all todos and categories in parallel
        const [todosResponse, categoriesResponse] = await Promise.all([
          TodoItemRepository.getAllTodoItems(userId),
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
        hour: "2-digit",
        minute: "2-digit"
      });
    },
    getCategoryName(categoryId?: number): string {
      if (!categoryId) return "";
      const category = this.categories.find(c => c.id === categoryId);
      return category?.name || "";
    },
    async deleteTodo(id: number) {
      if (!confirm("このタスクを削除しますか？")) {
        return;
      }
      
      try {
        await TodoItemRepository.deleteTodoItem(id);
        // Remove from local array
        this.todos = this.todos.filter(todo => todo.id !== id);
        alert("タスクが削除されました");
      } catch (error) {
        console.error("Failed to delete todo:", error);
        alert("タスクの削除に失敗しました");
      }
    }
  },
});
</script>