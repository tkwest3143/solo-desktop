<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <NuxtLink
            :to="{ name: 'id-productivity', params: { id: $route.params.id } }"
            class="p-2 hover:bg-slate-100 rounded-lg transition-colors"
          >
            <Icon name="fluent:arrow-left-20-filled" size="1.5em" class="text-slate-600" />
          </NuxtLink>
          <div>
            <h1 class="text-3xl font-bold text-slate-800 mb-2">タイムブロッキング</h1>
            <p class="text-slate-600">時間をブロックに分けて効率的にタスクを管理しましょう</p>
          </div>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">{{ selectedDate }}</div>
          <div class="text-lg font-semibold text-slate-700">{{ blocksCount }}個のブロック</div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-8">
      <div class="max-w-6xl mx-auto">
        <!-- Date Selection and Controls -->
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center space-x-4">
            <div class="flex items-center space-x-2">
              <button
                @click="previousDay"
                class="p-2 hover:bg-slate-100 rounded-lg transition-colors"
              >
                <Icon name="fluent:chevron-left-20-filled" class="text-slate-600" />
              </button>
              <input
                v-model="selectedDate"
                type="date"
                class="px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              <button
                @click="nextDay"
                class="p-2 hover:bg-slate-100 rounded-lg transition-colors"
              >
                <Icon name="fluent:chevron-right-20-filled" class="text-slate-600" />
              </button>
            </div>
            <button
              @click="goToToday"
              class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
            >
              今日
            </button>
          </div>
          <button
            @click="showAddBlockDialog = true"
            class="px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded-lg transition-colors flex items-center space-x-2"
          >
            <Icon name="fluent:add-20-filled" />
            <span>ブロック追加</span>
          </button>
        </div>

        <!-- Time Grid -->
        <div class="bg-white rounded-xl shadow-lg border border-slate-200 overflow-hidden">
          <!-- Hour Headers -->
          <div class="flex border-b border-slate-200">
            <div class="w-16 flex-shrink-0"></div>
            <div class="flex-1 grid grid-cols-24 border-l border-slate-200">
              <div
                v-for="hour in 24"
                :key="hour"
                class="h-12 border-r border-slate-200 flex items-center justify-center text-xs text-slate-600 font-medium"
                :class="{
                  'bg-blue-50': isCurrentHour(hour - 1),
                  'text-blue-600': isCurrentHour(hour - 1)
                }"
              >
                {{ formatHour(hour - 1) }}
              </div>
            </div>
          </div>

          <!-- Time Blocks -->
          <div class="relative">
            <!-- Time slots grid (15-minute intervals) -->
            <div class="flex">
              <div class="w-16 flex-shrink-0">
                <div
                  v-for="slot in timeSlots"
                  :key="slot.id"
                  class="h-6 border-b border-slate-100 flex items-center justify-end pr-2 text-xs text-slate-400"
                >
                  <span v-if="slot.minute === 0">{{ formatHour(slot.hour) }}</span>
                </div>
              </div>
              <div class="flex-1 relative border-l border-slate-200">
                <!-- Grid lines -->
                <div class="grid grid-cols-24">
                  <div
                    v-for="hour in 24"
                    :key="hour"
                    class="border-r border-slate-200 relative"
                  >
                    <div
                      v-for="quarter in 4"
                      :key="quarter"
                      class="h-6 border-b border-slate-100"
                      :class="{
                        'border-slate-200': quarter === 4,
                        'bg-blue-50': isCurrentHour(hour - 1) && isCurrentQuarter(quarter - 1)
                      }"
                    ></div>
                  </div>
                </div>

                <!-- Time Blocks -->
                <div
                  v-for="block in timeBlocks"
                  :key="block.id"
                  :style="getBlockStyle(block)"
                  class="absolute rounded-lg shadow-sm cursor-pointer transition-all hover:shadow-md group"
                  :class="getBlockColorClass(block.category)"
                  @click="editBlock(block)"
                >
                  <div class="p-2 h-full flex flex-col justify-between">
                    <div>
                      <h4 class="font-medium text-sm truncate group-hover:text-clip">{{ block.title }}</h4>
                      <p class="text-xs opacity-75 truncate">{{ block.description }}</p>
                    </div>
                    <div class="text-xs opacity-75">
                      {{ formatTime(block.startTime) }} - {{ formatTime(block.endTime) }}
                    </div>
                  </div>
                </div>

                <!-- Current time indicator -->
                <div
                  v-if="isToday"
                  :style="getCurrentTimeStyle()"
                  class="absolute left-0 right-0 h-0.5 bg-red-500 z-10"
                >
                  <div class="absolute -left-2 -top-2 w-4 h-4 bg-red-500 rounded-full"></div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Block Categories -->
        <div class="mt-6 bg-white rounded-xl shadow-lg border border-slate-200 p-6">
          <h3 class="text-lg font-semibold text-slate-800 mb-4">カテゴリ管理</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
            <div
              v-for="category in categories"
              :key="category.id"
              class="flex items-center space-x-2 p-3 border rounded-lg hover:bg-slate-50 cursor-pointer"
              @click="toggleCategory(category.id)"
            >
              <div
                class="w-4 h-4 rounded-full"
                :class="category.color"
              ></div>
              <span class="text-sm font-medium text-slate-700">{{ category.name }}</span>
              <span class="text-xs text-slate-500">({{ getCategoryBlockCount(category.id) }})</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Block Dialog -->
    <div
      v-if="showAddBlockDialog || editingBlock"
      class="fixed inset-0 z-50 overflow-y-auto"
      @click="closeDialog"
    >
      <div class="flex min-h-full items-center justify-center p-4">
        <div
          class="bg-white rounded-xl shadow-2xl border border-slate-200 p-6 w-full max-w-md"
          @click.stop
        >
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold text-slate-800">
              {{ editingBlock ? 'ブロック編集' : 'ブロック追加' }}
            </h3>
            <button
              @click="closeDialog"
              class="p-1 hover:bg-slate-100 rounded-lg transition-colors"
            >
              <Icon name="fluent:dismiss-20-filled" class="text-slate-600" />
            </button>
          </div>

          <form @submit.prevent="saveBlock" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">タイトル</label>
              <input
                v-model="blockForm.title"
                type="text"
                required
                class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="タスクやイベントの名前"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">説明</label>
              <textarea
                v-model="blockForm.description"
                rows="2"
                class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="詳細な説明（任意）"
              ></textarea>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">開始時間</label>
                <input
                  v-model="blockForm.startTime"
                  type="time"
                  required
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">終了時間</label>
                <input
                  v-model="blockForm.endTime"
                  type="time"
                  required
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">カテゴリ</label>
              <select
                v-model="blockForm.category"
                class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option
                  v-for="category in categories"
                  :key="category.id"
                  :value="category.id"
                >
                  {{ category.name }}
                </option>
              </select>
            </div>

            <div class="flex items-center justify-between pt-4">
              <button
                v-if="editingBlock"
                @click="deleteBlock"
                type="button"
                class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors"
              >
                削除
              </button>
              <div class="flex space-x-2 ml-auto">
                <button
                  @click="closeDialog"
                  type="button"
                  class="px-4 py-2 bg-slate-300 hover:bg-slate-400 text-slate-700 rounded-lg transition-colors"
                >
                  キャンセル
                </button>
                <button
                  type="submit"
                  class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                >
                  {{ editingBlock ? '更新' : '追加' }}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

interface TimeBlock {
  id: string
  title: string
  description: string
  startTime: string
  endTime: string
  category: string
  date: string
}

interface Category {
  id: string
  name: string
  color: string
}

interface TimeSlot {
  id: string
  hour: number
  minute: number
}

export default defineComponent({
  name: 'TimeBlocking',
  data() {
    return {
      selectedDate: new Date().toISOString().split('T')[0],
      showAddBlockDialog: false,
      editingBlock: null as TimeBlock | null,
      timeBlocks: [] as TimeBlock[],
      blockForm: {
        title: '',
        description: '',
        startTime: '09:00',
        endTime: '10:00',
        category: 'work'
      },
      categories: [
        { id: 'work', name: '仕事', color: 'bg-blue-500' },
        { id: 'personal', name: '個人', color: 'bg-green-500' },
        { id: 'meeting', name: '会議', color: 'bg-red-500' },
        { id: 'break', name: '休憩', color: 'bg-yellow-500' },
        { id: 'study', name: '学習', color: 'bg-purple-500' },
        { id: 'exercise', name: '運動', color: 'bg-orange-500' }
      ] as Category[]
    }
  },
  computed: {
    blocksCount(): number {
      return this.timeBlocks.filter(block => block.date === this.selectedDate).length
    },
    isToday(): boolean {
      return this.selectedDate === new Date().toISOString().split('T')[0]
    },
    timeSlots(): TimeSlot[] {
      const slots: TimeSlot[] = []
      for (let hour = 0; hour < 24; hour++) {
        for (let minute = 0; minute < 60; minute += 15) {
          slots.push({
            id: `${hour}-${minute}`,
            hour,
            minute
          })
        }
      }
      return slots
    }
  },
  mounted() {
    this.loadTimeBlocks()
  },
  methods: {
    previousDay() {
      const date = new Date(this.selectedDate)
      date.setDate(date.getDate() - 1)
      this.selectedDate = date.toISOString().split('T')[0]
    },
    nextDay() {
      const date = new Date(this.selectedDate)
      date.setDate(date.getDate() + 1)
      this.selectedDate = date.toISOString().split('T')[0]
    },
    goToToday() {
      this.selectedDate = new Date().toISOString().split('T')[0]
    },
    formatHour(hour: number): string {
      return `${hour.toString().padStart(2, '0')}:00`
    },
    formatTime(time: string): string {
      return time
    },
    isCurrentHour(hour: number): boolean {
      if (!this.isToday) return false
      return new Date().getHours() === hour
    },
    isCurrentQuarter(quarter: number): boolean {
      if (!this.isToday) return false
      const currentMinutes = new Date().getMinutes()
      const quarterStart = quarter * 15
      const quarterEnd = (quarter + 1) * 15
      return currentMinutes >= quarterStart && currentMinutes < quarterEnd
    },
    getBlockStyle(block: TimeBlock): object {
      const [startHour, startMinute] = block.startTime.split(':').map(Number)
      const [endHour, endMinute] = block.endTime.split(':').map(Number)
      
      const startMinutes = startHour * 60 + startMinute
      const endMinutes = endHour * 60 + endMinute
      const duration = endMinutes - startMinutes
      
      const left = (startMinutes / (24 * 60)) * 100
      const width = (duration / (24 * 60)) * 100
      
      return {
        left: `${left}%`,
        width: `${width}%`,
        top: '0',
        height: '100%'
      }
    },
    getBlockColorClass(categoryId: string): string {
      const category = this.categories.find(c => c.id === categoryId)
      return category ? `${category.color} text-white` : 'bg-gray-500 text-white'
    },
    getCurrentTimeStyle(): object {
      const now = new Date()
      const currentMinutes = now.getHours() * 60 + now.getMinutes()
      const top = (currentMinutes / (24 * 60)) * 100
      return { top: `${top}%` }
    },
    getCategoryBlockCount(categoryId: string): number {
      return this.timeBlocks.filter(
        block => block.category === categoryId && block.date === this.selectedDate
      ).length
    },
    editBlock(block: TimeBlock) {
      this.editingBlock = { ...block }
      this.blockForm = {
        title: block.title,
        description: block.description,
        startTime: block.startTime,
        endTime: block.endTime,
        category: block.category
      }
    },
    closeDialog() {
      this.showAddBlockDialog = false
      this.editingBlock = null
      this.resetForm()
    },
    resetForm() {
      this.blockForm = {
        title: '',
        description: '',
        startTime: '09:00',
        endTime: '10:00',
        category: 'work'
      }
    },
    saveBlock() {
      if (this.editingBlock) {
        // Update existing block
        const index = this.timeBlocks.findIndex(b => b.id === this.editingBlock!.id)
        if (index !== -1) {
          this.timeBlocks[index] = {
            ...this.editingBlock,
            ...this.blockForm
          }
        }
      } else {
        // Create new block
        const newBlock: TimeBlock = {
          id: Date.now().toString(),
          ...this.blockForm,
          date: this.selectedDate
        }
        this.timeBlocks.push(newBlock)
      }
      
      this.saveTimeBlocks()
      this.closeDialog()
    },
    deleteBlock() {
      if (this.editingBlock) {
        this.timeBlocks = this.timeBlocks.filter(b => b.id !== this.editingBlock!.id)
        this.saveTimeBlocks()
        this.closeDialog()
      }
    },
    toggleCategory(categoryId: string) {
      // This could be used to filter blocks by category
      console.log('Toggle category:', categoryId)
    },
    loadTimeBlocks() {
      const saved = localStorage.getItem('timeBlocks')
      if (saved) {
        this.timeBlocks = JSON.parse(saved)
      }
    },
    saveTimeBlocks() {
      localStorage.setItem('timeBlocks', JSON.stringify(this.timeBlocks))
    }
  }
})
</script>