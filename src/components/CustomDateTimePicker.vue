<template>
  <div class="custom-datetime-picker">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Date Picker -->
      <div>
        <label :for="dateId" class="block text-sm font-semibold mb-2" :class="getTextClass('primary')">
          {{ dateLabel }}
          <span v-if="required" :class="getStateClasses('error', 'text')">*</span>
        </label>
        <div class="relative">
          <div 
            class="flex items-center w-full px-4 py-3 border rounded-lg focus-within:outline-none focus-within:ring-2 focus-within:border-transparent transition-colors cursor-pointer hover:border-slate-400"
            :class="[
              getBgClass('primary'),
              error ? `${getStateClasses('error', 'border')} focus-within:${getStateClasses('error', 'border')} focus-within:ring-red-500` : `${getBorderClass('default')} focus-within:${getBorderClass('focus')} focus-within:ring-blue-500`
            ]"
            @click="focusDateInput"
          >
            <Icon name="fluent:calendar-20-filled" class="mr-3" :class="getTextClass('tertiary')" />
            <input
              :id="dateId"
              ref="dateInput"
              v-model="internalDate"
              type="date"
              :min="minDate"
              :max="maxDate"
              :required="required"
              class="flex-1 bg-transparent border-none outline-none placeholder-slate-500"
              :class="getTextClass('primary')"
              @change="updateDateTime"
            />
          </div>
          <div class="absolute right-3 top-1/2 transform -translate-y-1/2 pointer-events-none">
            <div class="flex space-x-1">
              <button
                v-if="!isToday && !isTomorrow"
                @click.stop="setToday"
                type="button"
                class="px-2 py-1 text-xs rounded-md transition-colors pointer-events-auto"
                :class="`${getStateClasses('info', 'bg')} ${getStateClasses('info', 'text')} ${getStateClasses('info', 'hover')}`"
              >
                今日
              </button>
              <button
                v-if="!isTomorrow"
                @click.stop="setTomorrow"
                type="button"
                class="px-2 py-1 text-xs rounded-md transition-colors pointer-events-auto"
                :class="`${getStateClasses('success', 'bg')} ${getStateClasses('success', 'text')} ${getStateClasses('success', 'hover')}`"
              >
                明日
              </button>
            </div>
          </div>
        </div>
        <div class="mt-1 text-xs" :class="getTextClass('tertiary')">
          {{ dateDisplayText }}
        </div>
      </div>

      <!-- Time Picker -->
      <div>
        <label :for="timeId" class="block text-sm font-semibold mb-2" :class="getTextClass('primary')">
          {{ timeLabel }}
        </label>
        <div class="relative">
          <div 
            class="flex items-center w-full px-4 py-3 border rounded-lg focus-within:outline-none focus-within:ring-2 focus-within:border-transparent transition-colors cursor-pointer hover:border-slate-400"
            :class="[
              getBgClass('primary'),
              error ? `${getStateClasses('error', 'border')} focus-within:${getStateClasses('error', 'border')} focus-within:ring-red-500` : `${getBorderClass('default')} focus-within:${getBorderClass('focus')} focus-within:ring-blue-500`
            ]"
            @click="focusTimeInput"
          >
            <Icon name="fluent:clock-20-filled" class="mr-3" :class="getTextClass('tertiary')" />
            <input
              :id="timeId"
              ref="timeInput"
              v-model="internalTime"
              type="time"
              class="flex-1 bg-transparent border-none outline-none placeholder-slate-500"
              :class="getTextClass('primary')"
              @change="updateDateTime"
            />
          </div>
          <div class="absolute right-3 top-1/2 transform -translate-y-1/2 pointer-events-none">
            <div class="flex space-x-1">
              <button
                @click.stop="setTime('09:00')"
                type="button"
                class="px-2 py-1 text-xs rounded-md transition-colors pointer-events-auto"
                :class="`${getStateClasses('warning', 'bg')} ${getStateClasses('warning', 'text')} ${getStateClasses('warning', 'hover')}`"
              >
                9:00
              </button>
              <button
                @click.stop="setTime('18:00')"
                type="button"
                class="px-2 py-1 text-xs rounded-md transition-colors pointer-events-auto"
                :class="`${getStateClasses('warning', 'bg')} ${getStateClasses('warning', 'text')} ${getStateClasses('warning', 'hover')}`"
              >
                18:00
              </button>
              <button
                @click.stop="setTime('23:59')"
                type="button"
                class="px-2 py-1 text-xs rounded-md transition-colors pointer-events-auto"
                :class="getThemeAwareColorClasses('purple')"
              >
                終日
              </button>
            </div>
          </div>
        </div>
        <div class="mt-1 text-xs" :class="getTextClass('tertiary')">
          {{ timeDisplayText }}
        </div>
      </div>
    </div>

    <!-- Quick Presets -->
    <div class="mt-4 flex flex-wrap gap-2">
      <button
        v-for="preset in quickPresets"
        :key="preset.label"
        @click="applyPreset(preset)"
        type="button"
        class="px-3 py-1 text-sm border rounded-full transition-all duration-200 shadow-sm hover:shadow"
        :class="[
          getBgClass('secondary'),
          getTextClass('primary'),
          getBorderClass('default'),
          isCurrentPreset(preset) ? 'ring-2 ring-blue-500 ring-opacity-50' : '',
          'hover:' + getBgClass('hover'),
          'dark:bg-slate-700 dark:border-slate-600 dark:hover:bg-slate-600'
        ]"
      >
        <Icon :name="preset.icon" class="mr-1" size="0.875em" />
        {{ preset.label }}
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

interface QuickPreset {
  label: string
  icon: string
  date: string
  time: string
}

export default defineComponent({
  name: 'CustomDateTimePicker',
  props: {
    date: {
      type: String,
      default: ''
    },
    time: {
      type: String,
      default: ''
    },
    required: {
      type: Boolean,
      default: false
    },
    error: {
      type: Boolean,
      default: false
    },
    dateLabel: {
      type: String,
      default: '期日'
    },
    timeLabel: {
      type: String,
      default: '時刻'
    },
    minDate: {
      type: String,
      default: ''
    },
    maxDate: {
      type: String,
      default: ''
    }
  },
  emits: ['update:date', 'update:time', 'change'],
  setup() {
    const { getBgClass, getTextClass, getBorderClass, getStateClasses } = useTheme();
    
    const getThemeAwareColorClasses = (colorName: string) => {
      // For purple state similar to the colorOptions
      if (colorName === 'purple') {
        return 'bg-purple-50 text-purple-700 hover:bg-purple-100 dark:bg-purple-900 dark:text-purple-300 dark:hover:bg-purple-800';
      }
      return '';
    };
    
    return {
      getBgClass,
      getTextClass,
      getBorderClass,
      getStateClasses,
      getThemeAwareColorClasses
    };
  },
  data() {
    return {
      internalDate: this.date,
      internalTime: this.time,
      dateId: `date-${Math.random().toString(36).substr(2, 9)}`,
      timeId: `time-${Math.random().toString(36).substr(2, 9)}`,
    }
  },
  computed: {
    quickPresets(): QuickPreset[] {
      const today = new Date()
      const tomorrow = new Date(today)
      tomorrow.setDate(tomorrow.getDate() + 1)
      const nextWeek = new Date(today)
      nextWeek.setDate(nextWeek.getDate() + 7)
      
      return [
        {
          label: '今日 9:00',
          icon: 'fluent:calendar-today-20-filled',
          date: today.toISOString().split('T')[0],
          time: '09:00'
        },
        {
          label: '明日 9:00',
          icon: 'fluent:calendar-arrow-right-20-filled',
          date: tomorrow.toISOString().split('T')[0],
          time: '09:00'
        },
        {
          label: '今日終了時',
          icon: 'fluent:clock-dismiss-20-filled',
          date: today.toISOString().split('T')[0],
          time: '18:00'
        },
        {
          label: '明日終了時',
          icon: 'fluent:calendar-clock-20-filled',
          date: tomorrow.toISOString().split('T')[0],
          time: '18:00'
        },
        {
          label: '来週',
          icon: 'fluent:calendar-week-numbers-20-filled',
          date: nextWeek.toISOString().split('T')[0],
          time: '09:00'
        }
      ]
    },
    isToday(): boolean {
      const today = new Date().toISOString().split('T')[0]
      return this.internalDate === today
    },
    isTomorrow(): boolean {
      const tomorrow = new Date()
      tomorrow.setDate(tomorrow.getDate() + 1)
      return this.internalDate === tomorrow.toISOString().split('T')[0]
    },
    dateDisplayText(): string {
      if (!this.internalDate) return ''
      
      const date = new Date(this.internalDate)
      const today = new Date()
      const diffTime = date.getTime() - today.getTime()
      const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
      
      if (diffDays === 0) return '今日'
      if (diffDays === 1) return '明日'
      if (diffDays === -1) return '昨日'
      if (diffDays > 0) return `${diffDays}日後`
      return `${Math.abs(diffDays)}日前`
    },
    timeDisplayText(): string {
      if (!this.internalTime) return ''
      
      const [hours, minutes] = this.internalTime.split(':').map(Number)
      
      if (hours === 23 && minutes === 59) return '終日'
      if (hours === 9 && minutes === 0) return '朝'
      if (hours === 12 && minutes === 0) return '昼'
      if (hours === 18 && minutes === 0) return '夕方'
      
      const hour12 = hours > 12 ? hours - 12 : hours
      const ampm = hours >= 12 ? '午後' : '午前'
      return `${ampm}${hour12}:${minutes.toString().padStart(2, '0')}`
    }
  },
  watch: {
    date(newVal) {
      this.internalDate = newVal
    },
    time(newVal) {
      this.internalTime = newVal
    }
  },
  methods: {
    updateDateTime() {
      this.$emit('update:date', this.internalDate)
      this.$emit('update:time', this.internalTime)
      this.$emit('change', {
        date: this.internalDate,
        time: this.internalTime
      })
    },
    focusDateInput() {
      (this.$refs.dateInput as HTMLInputElement)?.focus()
    },
    focusTimeInput() {
      (this.$refs.timeInput as HTMLInputElement)?.focus()
    },
    setToday() {
      this.internalDate = new Date().toISOString().split('T')[0]
      this.updateDateTime()
    },
    setTomorrow() {
      const tomorrow = new Date()
      tomorrow.setDate(tomorrow.getDate() + 1)
      this.internalDate = tomorrow.toISOString().split('T')[0]
      this.updateDateTime()
    },
    setTime(time: string) {
      this.internalTime = time
      this.updateDateTime()
    },
    applyPreset(preset: QuickPreset) {
      this.internalDate = preset.date
      this.internalTime = preset.time
      this.updateDateTime()
    },
    isCurrentPreset(preset: QuickPreset): boolean {
      return this.internalDate === preset.date && this.internalTime === preset.time
    }
  }
})
</script>

<style scoped>
/* Hide default date/time input styling while keeping functionality */
input[type="date"]::-webkit-calendar-picker-indicator,
input[type="time"]::-webkit-calendar-picker-indicator {
  opacity: 0;
  position: absolute;
  right: 0;
  width: 100%;
  height: 100%;
  cursor: pointer;
}

input[type="date"],
input[type="time"] {
  -webkit-appearance: none;
  -moz-appearance: textfield;
}

input[type="date"]::-webkit-inner-spin-button,
input[type="date"]::-webkit-outer-spin-button,
input[type="time"]::-webkit-inner-spin-button,
input[type="time"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>