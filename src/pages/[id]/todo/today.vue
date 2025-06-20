<template>
  <div class="h-full bg-white overflow-y-auto">
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
          <CustomSelect
            v-model="selectedStatusFilter"
            :options="statusFilterOptions"
            placeholder="すべて"
            size="sm"
            class="w-32"
          />
          <CustomSelect
            v-model="selectedCategoryId"
            :options="categoryOptions"
            placeholder="すべてのカテゴリ"
            size="sm"
            class="w-48"
          />
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
          class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300 cursor-pointer"
          :class="getTodoPriorityClass(todo)"
          @click="showTaskDetail(todo)"
        >
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input
                type="checkbox"
                :checked="todo.status === 'completed'"
                @click.stop="toggleTodoStatus(todo)"
                class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500"
              />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">
                  {{ todo.title }}
                </h3>
                <span
                  class="text-xs px-2 py-1 rounded-full font-medium"
                  :class="getPriorityBadgeClass(todo.priority)"
                >
                  {{ getPriorityLabel(todo.priority) }}
                </span>
                <div
                  v-if="todo.color"
                  class="w-3 h-3 rounded-full"
                  :style="{ backgroundColor: todo.color }"
                ></div>
              </div>
              <p v-if="todo.content" class="text-slate-600 mb-3">
                {{ todo.content }}
              </p>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4 text-sm text-slate-500">
                  <span>期限: {{ formatDueDate(todo.due_date) }}</span>
                  <span v-if="getCategoryName(todo.category_id)"
                    >カテゴリ: {{ getCategoryName(todo.category_id) }}</span
                  >
                  <span>作成日: {{ formatDate(todo.created_at) }}</span>
                </div>
                <div class="flex space-x-2">
                  <NuxtLink
                    :to="{
                      name: 'id-todo-edit',
                      params: { id: $route.params.id },
                      query: { id: todo.id },
                    }"
                    class="p-2 text-slate-400 hover:text-blue-500 transition-colors"
                  >
                    <Icon name="fluent:edit-20-filled" size="1.2em" />
                  </NuxtLink>
                  <button
                    @click.stop="showDeleteDialog(todo)"
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

    <!-- Task Detail Dialog -->
    <TaskDetailDialog
      v-if="taskDetailDialog.todo"
      :show="taskDetailDialog.show"
      :todo="taskDetailDialog.todo"
      :category="taskDetailDialog.category"
      @close="closeTaskDetail"
      @edit="editTask"
      @delete="showDeleteDialog"
    />

    <!-- Delete Confirmation Dialog -->
    <ConfirmDialog
      :show="deleteDialog.show"
      title="タスクの削除"
      :message="`「${deleteDialog.todo?.title}」を削除しますか？この操作は取り消せません。`"
      confirm-text="削除"
      cancel-text="キャンセル"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ConfirmDialog from "~/components/ConfirmDialog.vue";
import CustomSelect from "~/components/CustomSelect.vue";
import TaskDetailDialog from "~/components/TaskDetailDialog.vue";
import type { TodoCategory, TodoItem } from "~/models/todo";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";

definePageMeta({
  layout: "todo",
});

export default defineComponent({
  components: {
    ConfirmDialog,
    TaskDetailDialog,
    CustomSelect,
  },
  data() {
    return {
      currentDate: "",
      todos: [] as TodoItem[],
      categories: [] as TodoCategory[],
      selectedCategoryId: "",
      selectedStatusFilter: "",
      loading: true,
      deleteDialog: {
        show: false,
        todo: null as TodoItem | null,
      },
      taskDetailDialog: {
        show: false,
        todo: null as TodoItem | null,
        category: null as TodoCategory | null,
      },
    };
  },
  computed: {
    statusFilterOptions() {
      return [
        { value: "", label: "すべて" },
        { value: "incomplete", label: "未完了" },
        { value: "completed", label: "完了済み" },
      ];
    },
    categoryOptions() {
      const options: { value: string; label: string; color?: string }[] = [
        { value: "", label: "すべてのカテゴリ", color: "" },
      ];

      this.categories.forEach((category) => {
        options.push({
          value: category.id?.toString() || "",
          label: category.name,
          color: category.color,
        });
      });

      return options;
    },
    filteredTodos(): TodoItem[] {
      let filtered = this.todos;

      // Filter by category
      if (this.selectedCategoryId) {
        filtered = filtered.filter(
          (todo) => todo.category_id === parseInt(this.selectedCategoryId)
        );
      }

      // Filter by status
      if (this.selectedStatusFilter === "completed") {
        filtered = filtered.filter((todo) => todo.status === "completed");
      } else if (this.selectedStatusFilter === "incomplete") {
        filtered = filtered.filter((todo) => todo.status !== "completed");
      }

      return filtered;
    },
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
        const userId = parseInt(this.$route.params.id as string);

        // Fetch today's todos and categories in parallel
        const [todosResponse, categoriesResponse] = await Promise.all([
          TodoItemRepository.getTodayTodoItems(userId),
          TodoCategoryRepository.getTodoCategoriesByUserId(userId),
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
        return `今日 ${date.toLocaleTimeString("ja-JP", {
          hour: "2-digit",
          minute: "2-digit",
        })}`;
      }

      return date.toLocaleDateString("ja-JP", {
        month: "numeric",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    },
    formatDate(dateString: string): string {
      const date = new Date(dateString);
      return date.toLocaleDateString("ja-JP", {
        year: "numeric",
        month: "numeric",
        day: "numeric",
      });
    },
    getCategoryName(categoryId?: number): string {
      if (!categoryId) return "";
      const category = this.categories.find((c) => c.id === categoryId);
      return category?.name || "";
    },
    getTodoPriorityClass(todo: TodoItem): string {
      const dueDate = new Date(todo.due_date);
      const now = new Date();
      const hoursUntilDue =
        (dueDate.getTime() - now.getTime()) / (1000 * 60 * 60);

      if (hoursUntilDue < 2) {
        return "bg-red-50 border-red-300";
      } else if (hoursUntilDue < 6) {
        return "bg-orange-50 border-orange-200";
      }
      return "";
    },
    showDeleteDialog(todo: TodoItem) {
      this.deleteDialog.todo = todo;
      this.deleteDialog.show = true;
    },
    cancelDelete() {
      this.deleteDialog.show = false;
      this.deleteDialog.todo = null;
    },
    async confirmDelete() {
      if (!this.deleteDialog.todo) return;

      try {
        await TodoItemRepository.deleteTodoItem(this.deleteDialog.todo.id);
        // Remove from local array
        this.todos = this.todos.filter(
          (todo) => todo.id !== this.deleteDialog.todo?.id
        );
        this.cancelDelete();
      } catch (error) {
        console.error("Failed to delete todo:", error);
        alert("タスクの削除に失敗しました");
      }
    },
    showTaskDetail(todo: TodoItem) {
      const category =
        this.categories.find((cat) => cat.id === todo.category_id) || null;
      this.taskDetailDialog.todo = todo;
      this.taskDetailDialog.category = category;
      this.taskDetailDialog.show = true;
    },
    closeTaskDetail() {
      this.taskDetailDialog.show = false;
      this.taskDetailDialog.todo = null;
      this.taskDetailDialog.category = null;
    },
    editTask(todo: TodoItem) {
      this.$router.push({
        name: "id-todo-edit",
        params: { id: this.$route.params.id },
        query: { id: todo.id },
      });
    },
    async toggleTodoStatus(todo: TodoItem) {
      try {
        const newStatus =
          todo.status === "completed" ? "incomplete" : "completed";
        const updateData: any = {
          id: todo.id,
          title: todo.title,
          content: todo.content,
          link: todo.link,
          color: todo.color,
          priority: todo.priority,
          due_date: todo.due_date,
          category_id: todo.category_id,
          user_id: todo.user_id,
          status: newStatus,
        };

        await TodoItemRepository.updateTodoItem(updateData);

        // Update the local state
        const index = this.todos.findIndex((t) => t.id === todo.id);
        if (index !== -1) {
          this.todos[index].status = newStatus;
        }
      } catch (error) {
        console.error("Failed to toggle todo status:", error);
        alert("ステータスの更新に失敗しました");
      }
    },
    getPriorityLabel(priority?: string): string {
      switch (priority) {
        case "high":
          return "高優先度";
        case "low":
          return "低優先度";
        default:
          return "通常";
      }
    },
    getPriorityBadgeClass(priority?: string): string {
      switch (priority) {
        case "high":
          return "bg-red-100 text-red-800";
        case "low":
          return "bg-gray-100 text-gray-800";
        default:
          return "bg-blue-100 text-blue-800";
      }
    },
  },
});
</script>
