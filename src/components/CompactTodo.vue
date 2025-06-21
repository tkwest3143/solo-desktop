<template>
  <div class="h-full flex flex-col p-4 space-y-4">
    <!-- Header -->
    <div class="text-center">
      <h2 class="text-sm font-bold text-slate-800">今日のTodo</h2>
    </div>

    <!-- Todo List -->
    <div class="flex-1 overflow-y-auto space-y-2">
      <div v-if="isLoading" class="text-center py-4">
        <div class="text-xs text-slate-500">読み込み中...</div>
      </div>
      
      <div v-else-if="recentTodos.length === 0" class="text-center py-8">
        <Icon name="fluent:task-list-square-20-filled" class="text-slate-300 mb-2" size="2em" />
        <div class="text-xs text-slate-500">未完了のTodoがありません</div>
      </div>
      
      <div v-else class="space-y-2">
        <div
          v-for="todo in recentTodos"
          :key="todo.id"
          class="bg-slate-50 rounded-lg p-3 border border-slate-200 hover:bg-slate-100 transition-colors duration-200"
        >
          <div class="flex items-start space-x-2">
            <input
              type="checkbox"
              :checked="todo.status === 'completed'"
              @change="toggleTodoStatus(todo)"
              class="mt-1 w-4 h-4 text-blue-500 rounded border-2 border-blue-300 focus:ring-blue-500"
            />
            <div class="flex-1 min-w-0">
              <h3 
                class="text-xs font-medium text-slate-800 truncate"
                :class="todo.status === 'completed' ? 'line-through text-green-700' : ''"
              >
                {{ todo.title }}
              </h3>
              <div class="flex items-center justify-between mt-1">
                <span
                  v-if="todo.status !== 'completed'"
                  class="text-xs px-1 py-0.5 rounded font-medium"
                  :class="getPriorityBadgeClass(todo.priority)"
                >
                  {{ getPriorityLabel(todo.priority) }}
                </span>
                <span
                  v-if="todo.status === 'completed'"
                  class="bg-green-500 text-white text-xs px-1 py-0.5 rounded font-medium"
                >
                  完了
                </span>
                <div
                  v-if="todo.color"
                  class="w-2 h-2 rounded-full ml-auto"
                  :style="{ backgroundColor: todo.color }"
                ></div>
              </div>
              <div v-if="todo.due_date" class="text-xs text-slate-500 mt-1">
                期限: {{ formatDate(todo.due_date) }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="border-t border-slate-200 pt-3">
      <button
        @click="openFullTodo"
        class="w-full bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium py-2 px-3 rounded-lg transition-colors duration-200"
      >
        Todo管理を開く
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useRoute, useRouter } from "vue-router";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";

interface TodoItem {
  id: number;
  title: string;
  description: string;
  status: "todo" | "in_progress" | "completed";
  priority: "low" | "medium" | "high";
  due_date: string | null;
  created_at: string;
  updated_at: string;
  user_id: number;
  category_id: number | null;
  color?: string;
}

export default defineComponent({
  data() {
    return {
      recentTodos: [] as TodoItem[],
      isLoading: true,
    };
  },
  async mounted() {
    await this.loadRecentTodos();
  },
  methods: {
    async loadRecentTodos() {
      try {
        const route = useRoute();
        const userId = route.params.id as string;
        
        if (userId) {
          // Get today's todos and filter incomplete ones
          const todos = await TodoItemRepository.getTodayTodoItems(parseInt(userId));
          this.recentTodos = todos
            .filter((todo: TodoItem) => todo.status !== "completed")
            .slice(0, 5); // Show max 5 todos in compact view
            
          // Get categories to add colors
          const categories = await TodoCategoryRepository.getTodoCategoriesByUserId(parseInt(userId));
          this.recentTodos = this.recentTodos.map((todo: TodoItem) => {
            const category = categories.find((cat: any) => cat.id === todo.category_id);
            return {
              ...todo,
              color: category?.color || "#6b7280"
            };
          });
        }
      } catch (error) {
        console.error("Failed to load todos:", error);
      } finally {
        this.isLoading = false;
      }
    },
    async toggleTodoStatus(todo: TodoItem) {
      try {
        const newStatus = todo.status === "completed" ? "todo" : "completed";
        await TodoItemRepository.updateTodoItem({
          id: todo.id,
          status: newStatus,
        });
        
        // Update local state
        const index = this.recentTodos.findIndex(t => t.id === todo.id);
        if (index !== -1) {
          this.recentTodos[index].status = newStatus;
        }
      } catch (error) {
        console.error("Failed to update todo status:", error);
      }
    },
    getPriorityLabel(priority: string) {
      const labels = {
        low: "低",
        medium: "中",
        high: "高",
      };
      return labels[priority as keyof typeof labels] || "中";
    },
    getPriorityBadgeClass(priority: string) {
      const classes = {
        low: "bg-slate-200 text-slate-700",
        medium: "bg-yellow-200 text-yellow-800",
        high: "bg-red-200 text-red-800",
      };
      return classes[priority as keyof typeof classes] || "bg-slate-200 text-slate-700";
    },
    formatDate(dateString: string) {
      const date = new Date(dateString);
      return date.toLocaleDateString("ja-JP", {
        month: "short",
        day: "numeric",
      });
    },
    openFullTodo() {
      const route = useRoute();
      const router = useRouter();
      const userId = route.params.id as string;
      
      if (userId) {
        // Remove compact mode and navigate to todo page
        router.push({
          name: "id-todo",
          params: { id: userId },
        });
      }
    },
  },
});
</script>