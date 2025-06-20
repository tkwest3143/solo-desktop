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
                  <div class="p-2 bg-blue-100 rounded-lg">
                    <Icon name="fluent:edit-20-filled" class="text-blue-600" size="1.25em" />
                  </div>
                  <h2 class="text-xl font-semibold text-slate-800">
                    勤務時間編集
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
              <div class="mb-6">
                <div class="flex items-center space-x-2 text-sm text-slate-600">
                  <Icon name="fluent:calendar-20-filled" size="1em" />
                  <span>{{ date }}</span>
                </div>
              </div>

              <!-- Form -->
              <div class="space-y-6">
                <!-- Work Time Section -->
                <div class="bg-slate-50 rounded-lg p-4">
                  <h3 class="text-sm font-medium text-slate-700 mb-3 flex items-center">
                    <Icon name="fluent:clock-20-filled" class="mr-2 text-blue-600" size="1em" />
                    勤務時間
                  </h3>
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label class="block text-xs font-medium text-slate-600 mb-1">開始時間</label>
                      <input
                        type="time"
                        v-model="localWorkTime.start"
                        class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                      />
                    </div>
                    <div>
                      <label class="block text-xs font-medium text-slate-600 mb-1">終了時間</label>
                      <input
                        type="time"
                        v-model="localWorkTime.end"
                        class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                      />
                    </div>
                  </div>
                </div>

                <!-- Rest Time Section -->
                <div class="bg-slate-50 rounded-lg p-4">
                  <h3 class="text-sm font-medium text-slate-700 mb-3 flex items-center">
                    <Icon name="fluent:pause-20-filled" class="mr-2 text-green-600" size="1em" />
                    休憩時間
                  </h3>
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label class="block text-xs font-medium text-slate-600 mb-1">開始時間</label>
                      <input
                        type="time"
                        v-model="localWorkTime.restStart"
                        class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                      />
                    </div>
                    <div>
                      <label class="block text-xs font-medium text-slate-600 mb-1">終了時間</label>
                      <input
                        type="time"
                        v-model="localWorkTime.restEnd"
                        class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                      />
                    </div>
                  </div>
                </div>

                <!-- Memo Section -->
                <div class="bg-slate-50 rounded-lg p-4">
                  <h3 class="text-sm font-medium text-slate-700 mb-3 flex items-center">
                    <Icon name="fluent:note-20-filled" class="mr-2 text-purple-600" size="1em" />
                    備考
                  </h3>
                  <textarea
                    v-model="localWorkTime.memo"
                    rows="3"
                    class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                    placeholder="備考を入力してください..."
                  />
                </div>
              </div>
            </div>

            <!-- Actions -->
            <div class="px-6 py-4 bg-slate-50 border-t border-slate-200 flex justify-end space-x-3">
              <button
                @click="close"
                class="px-4 py-2 text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition-colors font-medium"
              >
                <Icon name="fluent:dismiss-20-filled" class="mr-2" size="1em" />
                キャンセル
              </button>
              <button
                @click="save"
                class="px-4 py-2 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg transition-all duration-200 shadow-sm hover:shadow-md font-medium"
              >
                <Icon name="fluent:save-20-filled" class="mr-2" size="1em" />
                保存
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

interface WorkTimeEdit {
  start: string
  end: string
  restStart: string
  restEnd: string
  memo: string
}

export default defineComponent({
  name: 'EditWorkTimeDialog',
  props: {
    show: {
      type: Boolean,
      default: false
    },
    workTime: {
      type: Object,
      default: () => ({
        start: '',
        end: '',
        restStart: '',
        restEnd: '',
        memo: ''
      })
    },
    date: {
      type: String,
      default: ''
    }
  },
  emits: ['close', 'save'],
  data() {
    return {
      localWorkTime: {
        start: '',
        end: '',
        restStart: '',
        restEnd: '',
        memo: ''
      } as WorkTimeEdit
    }
  },
  watch: {
    show(newVal) {
      if (newVal) {
        this.localWorkTime = { ...this.workTime }
      }
    },
    workTime: {
      handler(newVal) {
        this.localWorkTime = { ...newVal }
      },
      deep: true
    }
  },
  methods: {
    close() {
      this.$emit('close')
    },
    save() {
      this.$emit('save', this.localWorkTime)
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