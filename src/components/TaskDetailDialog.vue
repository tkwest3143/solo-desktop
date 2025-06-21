<template>
  <Transition
    enter-active-class="transition ease-out duration-300"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition ease-in duration-200"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="show"
      class="fixed inset-0 z-[1000] overflow-y-auto"
      @click="handleBackdropClick"
    >
      <!-- Backdrop -->
      <div 
        class="fixed inset-0 bg-black bg-opacity-50 transition-opacity" 
        aria-hidden="true"
      ></div>

      <!-- Dialog -->
      <div class="flex min-h-full items-center justify-center p-4">
        <Transition
          enter-active-class="transition ease-out duration-300"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
          leave-active-class="transition ease-in duration-200"
          leave-from-class="opacity-100 scale-100"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="show"
            class="relative w-full max-w-2xl transform overflow-hidden rounded-xl bg-white shadow-xl transition-all"
            @click.stop
          >
            <!-- Header -->
            <div class="px-6 py-4 border-b border-slate-200 bg-gradient-to-r from-slate-50 to-white">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                  <div 
                    v-if="todo.color"
                    class="w-4 h-4 rounded-full border-2 border-white shadow-sm"
                    :style="{ backgroundColor: todo.color }"
                  ></div>
                  <h2 class="text-xl font-semibold text-slate-800">
                    タスク詳細
                  </h2>
                </div>
                <button
                  @click="close"
                  class="p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 rounded-full transition-all duration-200"
                >
                  <Icon name="fluent:dismiss-20-filled" size="1.25em" />
                </button>
              </div>
            </div>

            <!-- Content -->
            <div class="px-6 py-6 max-h-96 overflow-y-auto">
              <!-- Title -->
              <div class="mb-6">
                <div class="flex items-center justify-between mb-2">
                  <h3 class="text-2xl font-bold text-slate-800">{{ todo.title }}</h3>
                  <div class="flex items-center space-x-2">
                    <span 
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                      :class="getPriorityBadgeClass(todo.priority)"
                    >
                      {{ getPriorityLabel(todo.priority) }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Description -->
              <div v-if="todo.content" class="mb-6">
                <h4 class="text-sm font-semibold text-slate-700 mb-2 flex items-center">
                  <Icon name="fluent:text-description-20-filled" class="mr-2 text-slate-500" />
                  詳細説明
                </h4>
                <div class="bg-slate-50 rounded-lg p-4 text-slate-700 whitespace-pre-wrap">
                  {{ todo.content }}
                </div>
              </div>

              <!-- Metadata Grid -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <!-- Due Date -->
                <div class="bg-gradient-to-br from-blue-50 to-blue-100 rounded-lg p-4">
                  <h4 class="text-sm font-semibold text-blue-800 mb-2 flex items-center">
                    <Icon name="fluent:calendar-20-filled" class="mr-2" />
                    期限
                  </h4>
                  <div class="text-blue-900">
                    <div class="text-lg font-medium">{{ formatDueDate(todo.due_date) }}</div>
                    <div class="text-sm opacity-75">{{ getRelativeDate(todo.due_date) }}</div>
                  </div>
                </div>

                <!-- Category -->
                <div v-if="category" class="bg-gradient-to-br from-green-50 to-green-100 rounded-lg p-4">
                  <h4 class="text-sm font-semibold text-green-800 mb-2 flex items-center">
                    <Icon name="fluent:folder-20-filled" class="mr-2" />
                    カテゴリ
                  </h4>
                  <div class="flex items-center space-x-2">
                    <div 
                      v-if="category.color"
                      class="w-3 h-3 rounded-full border border-green-300"
                      :style="{ backgroundColor: category.color }"
                    ></div>
                    <span class="text-green-900 font-medium">{{ category.name }}</span>
                  </div>
                </div>
              </div>

              <!-- Timestamps -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div class="bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg p-4">
                  <h4 class="text-sm font-semibold text-purple-800 mb-2 flex items-center">
                    <Icon name="fluent:calendar-add-20-filled" class="mr-2" />
                    作成日時
                  </h4>
                  <div class="text-purple-900 text-sm">{{ formatDateTime(todo.created_at) }}</div>
                </div>

                <div class="bg-gradient-to-br from-orange-50 to-orange-100 rounded-lg p-4">
                  <h4 class="text-sm font-semibold text-orange-800 mb-2 flex items-center">
                    <Icon name="fluent:calendar-edit-20-filled" class="mr-2" />
                    更新日時
                  </h4>
                  <div class="text-orange-900 text-sm">{{ formatDateTime(todo.updated_at) }}</div>
                </div>
              </div>

              <!-- Link -->
              <div v-if="todo.link" class="mb-6">
                <h4 class="text-sm font-semibold text-slate-700 mb-2 flex items-center">
                  <Icon name="fluent:link-20-filled" class="mr-2 text-slate-500" />
                  関連リンク
                </h4>
                <a
                  :href="todo.link"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="inline-flex items-center px-3 py-2 bg-blue-50 text-blue-700 rounded-lg hover:bg-blue-100 transition-colors text-sm"
                >
                  <Icon name="fluent:open-20-filled" class="mr-2" />
                  リンクを開く
                </a>
              </div>
            </div>

            <!-- Actions -->
            <div class="px-6 py-4 bg-slate-50 border-t border-slate-200 flex justify-between items-center">
              <div class="flex space-x-3">
                <button
                  @click="editTodo"
                  class="inline-flex items-center px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors shadow-sm"
                >
                  <Icon name="fluent:edit-20-filled" class="mr-2" />
                  編集
                </button>
                <button
                  @click="deleteTodo"
                  class="inline-flex items-center px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors shadow-sm"
                >
                  <Icon name="fluent:delete-20-filled" class="mr-2" />
                  削除
                </button>
              </div>
              <button
                @click="close"
                class="px-4 py-2 text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition-colors"
              >
                閉じる
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'
import type { TodoItem, TodoCategory } from '~/models/todo'

export default defineComponent({
  name: 'TaskDetailDialog',
  props: {
    show: {
      type: Boolean,
      default: false
    },
    todo: {
      type: Object as PropType<TodoItem>,
      required: true
    },
    category: {
      type: Object as PropType<TodoCategory | null>,
      default: null
    }
  },
  emits: ['close', 'edit', 'delete'],
  setup() {
    const { getPriorityBadgeClass, getPriorityLabel } = useTheme();
    
    return {
      getPriorityBadgeClass,
      getPriorityLabel
    };
  },
  methods: {
    close() {
      this.$emit('close')
    },
    editTodo() {
      this.$emit('edit', this.todo)
    },
    deleteTodo() {
      this.$emit('delete', this.todo)
    },
    handleBackdropClick(event: Event) {
      if (event.target === event.currentTarget) {
        this.close()
      }
    },
    formatDueDate(dateString: string): string {
      try {
        const date = new Date(dateString)
        return date.toLocaleDateString('ja-JP', {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'short'
        })
      } catch {
        return dateString
      }
    },
    formatDateTime(dateString: string): string {
      try {
        const date = new Date(dateString)
        return date.toLocaleString('ja-JP', {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit'
        })
      } catch {
        return dateString
      }
    },
    getRelativeDate(dateString: string): string {
      try {
        const date = new Date(dateString)
        const today = new Date()
        const diffTime = date.getTime() - today.getTime()
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
        
        if (diffDays === 0) return '今日'
        if (diffDays === 1) return '明日'
        if (diffDays === -1) return '昨日'
        if (diffDays > 0) return `${diffDays}日後`
        return `${Math.abs(diffDays)}日前`
      } catch {
        return ''
      }
    },
  }
})
</script>

<style scoped>
/* Custom scrollbar for content area */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #f1f5f9;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>