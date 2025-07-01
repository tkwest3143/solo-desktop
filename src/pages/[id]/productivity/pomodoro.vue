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
            <h1 class="text-3xl font-bold text-slate-800 mb-2">タイマー</h1>
            <p class="text-slate-600">ポモドーロタイマーとカウントアップタイマーで集中管理</p>
          </div>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">
            {{ mode === 'pomodoro' ? `セッション ${state.currentSessionCount + 1}` : 'カウントアップ中' }}
          </div>
          <div class="text-lg font-semibold text-slate-700">
            {{ mode === 'pomodoro' ? `${state.completedCycles}サイクル完了` : '自由時間計測' }}
          </div>
        </div>
      </div>
    </div>

    <!-- Main Timer Content -->
    <div class="p-8">
      <div class="max-w-4xl mx-auto">
        <!-- Mode Selector -->
        <div class="flex justify-center mb-8">
          <div class="bg-slate-100 rounded-lg p-1 flex space-x-1">
            <button
              @click="switchMode('pomodoro')"
              :class="{
                'bg-white shadow-sm': mode === 'pomodoro',
                'text-slate-600 hover:text-slate-800': mode !== 'pomodoro'
              }"
              class="px-6 py-2 rounded-md font-medium transition-all duration-200"
            >
              ポモドーロ
            </button>
            <button
              @click="switchMode('countup')"
              :class="{
                'bg-white shadow-sm': mode === 'countup',
                'text-slate-600 hover:text-slate-800': mode !== 'countup'
              }"
              class="px-6 py-2 rounded-md font-medium transition-all duration-200"
            >
              カウントアップ
            </button>
          </div>
        </div>

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
              <!-- Progress circle (only for pomodoro) -->
              <circle
                v-if="mode === 'pomodoro'"
                cx="160"
                cy="160"
                r="140"
                stroke="currentColor"
                stroke-width="20"
                fill="transparent"
                :stroke-dasharray="circumference"
                :stroke-dashoffset="dashOffset"
                :class="{
                  'text-red-500': pomodoroPhase === 'focus',
                  'text-green-500': pomodoroPhase === 'shortBreak',
                  'text-blue-500': pomodoroPhase === 'longBreak'
                }"
                class="transition-all duration-1000 ease-linear"
              />
            </svg>
            <div class="absolute inset-0 flex items-center justify-center">
              <div class="text-center">
                <div class="text-6xl font-bold text-slate-800 mb-2">
                  {{ displayTime }}
                </div>
                <div class="text-xl text-slate-600 mb-4">
                  {{ displayMode }}
                </div>
                <div class="flex items-center justify-center space-x-2">
                  <button
                    @click="toggleTimer"
                    :class="timerButtonClass"
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

        <!-- Settings and Stats -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
          <!-- Pomodoro Settings (only visible in pomodoro mode) -->
          <div v-if="mode === 'pomodoro'" class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:settings-20-filled" class="mr-2 text-blue-500" />
              ポモドーロ設定
            </h3>
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">
                  集中時間 (分)
                </label>
                <input
                  v-model.number="localSettings.focusTime"
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
                  v-model.number="localSettings.shortBreakTime"
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
                  v-model.number="localSettings.longBreakTime"
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
                  v-model.number="localSettings.longBreakInterval"
                  type="number"
                  min="2"
                  max="10"
                  :disabled="isRunning"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-slate-100"
                />
              </div>
            </div>
          </div>

          <!-- Count-up Info (only visible in count-up mode) -->
          <div v-if="mode === 'countup'" class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:clock-20-filled" class="mr-2 text-green-500" />
              カウントアップ情報
            </h3>
            <div class="space-y-4">
              <div class="p-4 bg-green-50 rounded-lg">
                <div class="flex items-center justify-between">
                  <span class="text-sm font-medium text-slate-700">経過時間</span>
                  <span class="text-2xl font-bold text-green-600">{{ formatTime(timeElapsed) }}</span>
                </div>
              </div>
              <div class="text-sm text-slate-600">
                カウントアップタイマーは、作業にかかった実際の時間を計測します。
                自由な時間で作業を行い、後で実際の作業時間を確認できます。
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
                <span class="text-lg font-bold text-red-600">{{ state.completedFocusSessions }}</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-green-50 rounded-lg">
                <div class="flex items-center space-x-2">
                  <Icon name="fluent:drink-coffee-20-filled" class="text-green-500" />
                  <span class="text-sm font-medium text-slate-700">完了した休憩</span>
                </div>
                <span class="text-lg font-bold text-green-600">{{ state.completedBreaks }}</span>
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
                <span class="text-lg font-bold text-purple-600">{{ state.completedCycles }}</span>
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
import { defineComponent, ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useTimer } from '~/composables/useTimer'

export default defineComponent({
  name: 'TimerPage',
  setup() {
    const timer = useTimer()
    
    const localSettings = ref({
      focusTime: timer.state.value.pomodoroSettings.focusTime,
      shortBreakTime: timer.state.value.pomodoroSettings.shortBreakTime,
      longBreakTime: timer.state.value.pomodoroSettings.longBreakTime,
      longBreakInterval: timer.state.value.pomodoroSettings.longBreakInterval
    })

    // Computed values
    const circumference = computed(() => 2 * Math.PI * 140)
    
    const dashOffset = computed(() => {
      if (timer.mode.value !== 'pomodoro') return circumference.value
      
      const totalTime = getCurrentPomodoroTime() * 60
      const progress = (totalTime - timer.timeLeft.value) / totalTime
      return circumference.value * (1 - progress)
    })

    const displayTime = computed(() => {
      if (timer.mode.value === 'countup') {
        return timer.formatTime(timer.timeElapsed.value)
      } else {
        return timer.formatTime(timer.timeLeft.value)
      }
    })

    const displayMode = computed(() => {
      if (timer.mode.value === 'countup') {
        return 'カウントアップ'
      } else {
        switch (timer.pomodoroPhase.value) {
          case 'focus': return '集中時間'
          case 'shortBreak': return '短い休憩'
          case 'longBreak': return '長い休憩'
          default: return '集中時間'
        }
      }
    })

    const timerButtonClass = computed(() => {
      if (timer.mode.value === 'countup') {
        return 'bg-green-500 hover:bg-green-600'
      } else {
        switch (timer.pomodoroPhase.value) {
          case 'focus': return 'bg-red-500 hover:bg-red-600'
          case 'shortBreak': return 'bg-green-500 hover:bg-green-600'
          case 'longBreak': return 'bg-blue-500 hover:bg-blue-600'
          default: return 'bg-red-500 hover:bg-red-600'
        }
      }
    })

    const totalFocusTime = computed(() => {
      return timer.state.value.completedFocusSessions * timer.state.value.pomodoroSettings.focusTime
    })

    const getCurrentPomodoroTime = (): number => {
      const settings = timer.state.value.pomodoroSettings
      switch (timer.pomodoroPhase.value) {
        case 'focus': return settings.focusTime
        case 'shortBreak': return settings.shortBreakTime
        case 'longBreak': return settings.longBreakTime
        default: return settings.focusTime
      }
    }

    const saveTask = () => {
      console.log('Task saved:', timer.currentTask.value)
    }

    // Watch for settings changes
    watch(localSettings, (newSettings) => {
      timer.updatePomodoroSettings(newSettings)
    }, { deep: true })

    // Initialize settings from timer state
    onMounted(() => {
      localSettings.value = { ...timer.state.value.pomodoroSettings }
    })

    return {
      // Timer composable
      ...timer,
      
      // Local state
      localSettings,
      
      // Computed
      circumference,
      dashOffset,
      displayTime,
      displayMode,
      timerButtonClass,
      totalFocusTime,
      
      // Methods
      saveTask,
      
      // Aliases for cleaner template
      // Aliases for cleaner template
      mode: timer.mode,
      pomodoroPhase: timer.pomodoroPhase,
      isRunning: timer.isRunning,
      timeLeft: timer.timeLeft,
      timeElapsed: timer.timeElapsed,
      state: timer.state,
      currentTask: timer.currentTask,
      formatTime: timer.formatTime,
      resetTimer: timer.resetTimer,
      switchMode: timer.switchMode,
    }
  },
  methods: {
    toggleTimer() {
      if (this.isRunning) {
        this.pauseTimer()
      } else {
        this.startTimer()
      }
    }
  }
})
</script>