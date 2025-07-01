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
            <h1 class="text-3xl font-bold text-slate-800 mb-2">ポモドーロタイマー</h1>
            <p class="text-slate-600">25分の集中と5分の休憩で効率的に作業しましょう</p>
          </div>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">セッション {{ currentSessionCount + 1 }}</div>
          <div class="text-lg font-semibold text-slate-700">{{ completedCycles }}サイクル完了</div>
        </div>
      </div>
    </div>

    <!-- Main Timer Content -->
    <div class="p-8">
      <div class="max-w-4xl mx-auto">
        <!-- Timer Circle -->
        <div class="text-center mb-8">
          <div class="inline-block relative">
            <svg class="w-80 h-80 transform -rotate-90" viewBox="0 0 320 320">
              <!-- Background circle -->
              <circle
                cx="160"
                cy="160"
                r="140"
                stroke="currentColor"
                stroke-width="20"
                fill="transparent"
                class="text-slate-200"
              />
              <!-- Progress circle -->
              <circle
                cx="160"
                cy="160"
                r="140"
                stroke="currentColor"
                stroke-width="20"
                fill="transparent"
                :stroke-dasharray="circumference"
                :stroke-dashoffset="dashOffset"
                :class="{
                  'text-red-500': currentMode === 'focus',
                  'text-green-500': currentMode === 'shortBreak',
                  'text-blue-500': currentMode === 'longBreak'
                }"
                class="transition-all duration-1000 ease-linear"
              />
            </svg>
            <div class="absolute inset-0 flex items-center justify-center">
              <div class="text-center">
                <div class="text-6xl font-bold text-slate-800 mb-2">
                  {{ formatTime(timeLeft) }}
                </div>
                <div class="text-xl text-slate-600 mb-4">
                  {{ getModeLabel(currentMode) }}
                </div>
                <div class="flex items-center justify-center space-x-2">
                  <button
                    @click="toggleTimer"
                    :class="{
                      'bg-red-500 hover:bg-red-600': currentMode === 'focus',
                      'bg-green-500 hover:bg-green-600': currentMode === 'shortBreak',
                      'bg-blue-500 hover:bg-blue-600': currentMode === 'longBreak'
                    }"
                    class="px-8 py-3 text-white rounded-lg font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-1 transition-all duration-200"
                  >
                    <Icon 
                      :name="isRunning ? 'fluent:pause-20-filled' : 'fluent:play-20-filled'" 
                      class="mr-2"
                      size="1.2em"
                    />
                    {{ isRunning ? '一時停止' : '開始' }}
                  </button>
                  <button
                    @click="resetTimer"
                    class="px-6 py-3 bg-slate-500 hover:bg-slate-600 text-white rounded-lg font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-1 transition-all duration-200"
                  >
                    <Icon name="fluent:arrow-reset-20-filled" class="mr-2" size="1.2em" />
                    リセット
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Settings Panel -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
          <!-- Timer Settings -->
          <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:settings-20-filled" class="mr-2 text-blue-500" />
              タイマー設定
            </h3>
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">
                  集中時間 (分)
                </label>
                <input
                  v-model.number="settings.focusTime"
                  type="number"
                  min="1"
                  max="60"
                  :disabled="isRunning"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent disabled:bg-slate-100"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">
                  短い休憩 (分)
                </label>
                <input
                  v-model.number="settings.shortBreakTime"
                  type="number"
                  min="1"
                  max="30"
                  :disabled="isRunning"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent disabled:bg-slate-100"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">
                  長い休憩 (分)
                </label>
                <input
                  v-model.number="settings.longBreakTime"
                  type="number"
                  min="1"
                  max="60"
                  :disabled="isRunning"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-slate-100"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">
                  長い休憩までのサイクル数
                </label>
                <input
                  v-model.number="settings.longBreakInterval"
                  type="number"
                  min="2"
                  max="10"
                  :disabled="isRunning"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-slate-100"
                />
              </div>
            </div>
          </div>

          <!-- Session Statistics -->
          <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:data-bar-horizontal-20-filled" class="mr-2 text-green-500" />
              セッション統計
            </h3>
            <div class="space-y-4">
              <div class="flex justify-between items-center p-3 bg-red-50 rounded-lg">
                <div class="flex items-center space-x-2">
                  <Icon name="fluent:timer-20-filled" class="text-red-500" />
                  <span class="text-sm font-medium text-slate-700">完了した集中セッション</span>
                </div>
                <span class="text-lg font-bold text-red-600">{{ completedFocusSessions }}</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-green-50 rounded-lg">
                <div class="flex items-center space-x-2">
                  <Icon name="fluent:drink-coffee-20-filled" class="text-green-500" />
                  <span class="text-sm font-medium text-slate-700">完了した休憩</span>
                </div>
                <span class="text-lg font-bold text-green-600">{{ completedBreaks }}</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-blue-50 rounded-lg">
                <div class="flex items-center space-x-2">
                  <Icon name="fluent:clock-20-filled" class="text-blue-500" />
                  <span class="text-sm font-medium text-slate-700">総集中時間</span>
                </div>
                <span class="text-lg font-bold text-blue-600">{{ totalFocusTime }}分</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-purple-50 rounded-lg">
                <div class="flex items-center space-x-2">
                  <Icon name="fluent:trophy-20-filled" class="text-purple-500" />
                  <span class="text-sm font-medium text-slate-700">完了サイクル数</span>
                </div>
                <span class="text-lg font-bold text-purple-600">{{ completedCycles }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Task Integration -->
        <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
          <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
            <Icon name="fluent:task-list-square-20-filled" class="mr-2 text-purple-500" />
            現在のタスク
          </h3>
          <div class="flex items-center space-x-4">
            <input
              v-model="currentTask"
              type="text"
              placeholder="このセッションで取り組むタスクを入力..."
              class="flex-1 px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
            <button
              @click="saveTask"
              class="px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
            >
              保存
            </button>
          </div>
          <div v-if="currentTask" class="mt-3 p-3 bg-blue-50 rounded-lg">
            <p class="text-sm text-blue-800">現在取り組み中: {{ currentTask }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

type TimerMode = 'focus' | 'shortBreak' | 'longBreak'

interface PomodoroSettings {
  focusTime: number
  shortBreakTime: number
  longBreakTime: number
  longBreakInterval: number
}

export default defineComponent({
  name: 'PomodoroTimer',
  data() {
    return {
      isRunning: false,
      currentMode: 'focus' as TimerMode,
      timeLeft: 25 * 60, // Default 25 minutes in seconds
      currentSessionCount: 0,
      completedCycles: 0,
      completedFocusSessions: 0,
      completedBreaks: 0,
      currentTask: '',
      timer: null as ReturnType<typeof setInterval> | null,
      settings: {
        focusTime: 25,
        shortBreakTime: 5,
        longBreakTime: 15,
        longBreakInterval: 4
      } as PomodoroSettings
    }
  },
  computed: {
    circumference(): number {
      return 2 * Math.PI * 140 // radius of 140
    },
    dashOffset(): number {
      const totalTime = this.getCurrentModeTime() * 60
      const progress = (totalTime - this.timeLeft) / totalTime
      return this.circumference * (1 - progress)
    },
    totalFocusTime(): number {
      return this.completedFocusSessions * this.settings.focusTime
    }
  },
  mounted() {
    this.resetTimer()
    this.loadSettings()
  },
  beforeUnmount() {
    this.clearTimer()
  },
  methods: {
    toggleTimer() {
      if (this.isRunning) {
        this.pauseTimer()
      } else {
        this.startTimer()
      }
    },
    startTimer() {
      this.isRunning = true
      this.timer = setInterval(() => {
        if (this.timeLeft > 0) {
          this.timeLeft--
        } else {
          this.completeSession()
        }
      }, 1000)
    },
    pauseTimer() {
      this.isRunning = false
      this.clearTimer()
    },
    resetTimer() {
      this.isRunning = false
      this.clearTimer()
      this.timeLeft = this.getCurrentModeTime() * 60
    },
    clearTimer() {
      if (this.timer) {
        clearInterval(this.timer)
        this.timer = null
      }
    },
    completeSession() {
      this.clearTimer()
      this.isRunning = false

      if (this.currentMode === 'focus') {
        this.completedFocusSessions++
        this.currentSessionCount++
        
        // Determine next break type
        if (this.currentSessionCount % this.settings.longBreakInterval === 0) {
          this.currentMode = 'longBreak'
          this.completedCycles++
        } else {
          this.currentMode = 'shortBreak'
        }
      } else {
        this.completedBreaks++
        this.currentMode = 'focus'
      }

      this.timeLeft = this.getCurrentModeTime() * 60
      this.showNotification()
    },
    getCurrentModeTime(): number {
      switch (this.currentMode) {
        case 'focus':
          return this.settings.focusTime
        case 'shortBreak':
          return this.settings.shortBreakTime
        case 'longBreak':
          return this.settings.longBreakTime
        default:
          return this.settings.focusTime
      }
    },
    getModeLabel(mode: TimerMode): string {
      switch (mode) {
        case 'focus':
          return '集中時間'
        case 'shortBreak':
          return '短い休憩'
        case 'longBreak':
          return '長い休憩'
        default:
          return '集中時間'
      }
    },
    formatTime(seconds: number): string {
      const minutes = Math.floor(seconds / 60)
      const remainingSeconds = seconds % 60
      return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
    },
    showNotification() {
      const message = this.currentMode === 'focus' 
        ? '休憩時間です！少し休憩しましょう。'
        : '集中時間です！作業を再開しましょう。'
      
      // Simple browser notification (could be enhanced with system notifications)
      if ('Notification' in window && Notification.permission === 'granted') {
        new Notification('ポモドーロタイマー', {
          body: message,
          icon: '/favicon.ico'
        })
      }
    },
    saveTask() {
      // In a real app, this would save to a database or local storage
      console.log('Task saved:', this.currentTask)
    },
    loadSettings() {
      // Load settings from localStorage or user preferences
      const saved = localStorage.getItem('pomodoroSettings')
      if (saved) {
        this.settings = { ...this.settings, ...JSON.parse(saved) }
      }
    },
    saveSettings() {
      localStorage.setItem('pomodoroSettings', JSON.stringify(this.settings))
    }
  },
  watch: {
    settings: {
      handler() {
        this.saveSettings()
        if (!this.isRunning) {
          this.resetTimer()
        }
      },
      deep: true
    }
  }
})
</script>