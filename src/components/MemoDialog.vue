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
            class="relative w-full max-w-lg transform overflow-hidden rounded-xl bg-white shadow-xl transition-all"
            @click.stop
          >
            <!-- Header -->
            <div class="px-6 py-4 border-b border-slate-200 bg-gradient-to-r from-slate-50 to-white">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                  <div class="p-2 bg-blue-100 rounded-lg">
                    <Icon name="fluent:note-20-filled" class="text-blue-600" size="1.25em" />
                  </div>
                  <h2 class="text-xl font-semibold text-slate-800">
                    備考詳細
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
            <div class="px-6 py-6">
              <!-- Date -->
              <div class="mb-4">
                <div class="flex items-center space-x-2 text-sm text-slate-600">
                  <Icon name="fluent:calendar-20-filled" size="1em" />
                  <span>{{ date }}</span>
                </div>
              </div>

              <!-- Memo Content -->
              <div class="bg-slate-50 rounded-lg p-4">
                <div v-if="memo && memo.trim()" class="text-slate-700 whitespace-pre-wrap leading-relaxed">
                  {{ memo }}
                </div>
                <div v-else class="text-slate-400 text-center py-8">
                  <Icon name="fluent:document-text-20-regular" size="2em" class="mx-auto mb-2" />
                  <p>備考が入力されていません</p>
                </div>
              </div>
            </div>

            <!-- Actions -->
            <div class="px-6 py-4 bg-slate-50 border-t border-slate-200 flex justify-end">
              <button
                @click="close"
                class="px-4 py-2 text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition-colors font-medium"
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
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'MemoDialog',
  props: {
    show: {
      type: Boolean,
      default: false
    },
    memo: {
      type: String,
      default: ''
    },
    date: {
      type: String,
      default: ''
    }
  },
  emits: ['close'],
  methods: {
    close() {
      this.$emit('close')
    },
    handleBackdropClick(event: Event) {
      if (event.target === event.currentTarget) {
        this.close()
      }
    }
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