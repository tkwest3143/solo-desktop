<template>
  <div class="custom-select-container">
    <div 
      class="custom-select"
      :class="[
        'relative w-full',
        disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
        focused ? 'ring-2 ring-blue-500' : '',
        error ? 'border-red-300 focus:border-red-500 focus:ring-red-500' : 'border-slate-300 focus:border-blue-500 focus:ring-blue-500',
        size === 'sm' ? 'px-3 py-2 text-sm' : 'px-4 py-3',
        'bg-white border rounded-lg transition-all duration-200 ease-in-out',
        'hover:border-slate-400 hover:shadow-sm',
        'focus:outline-none focus:ring-2 focus:ring-opacity-50',
        'group'
      ]"
      @click="toggleDropdown"
      @keydown.space.prevent="toggleDropdown"
      @keydown.enter.prevent="toggleDropdown"
      @keydown.escape="closeDropdown"
      @keydown.up.prevent="navigateOptions(-1)"
      @keydown.down.prevent="navigateOptions(1)"
      tabindex="0"
      :aria-expanded="isOpen"
      :aria-haspopup="true"
      role="combobox"
    >
      <div class="flex items-center justify-between">
        <span 
          class="block truncate"
          :class="[
            selectedOption ? 'text-slate-900' : 'text-slate-500',
            size === 'sm' ? 'text-sm' : 'text-base'
          ]"
        >
          {{ selectedOption ? selectedOption.label : placeholder }}
        </span>
        <div class="flex items-center space-x-2">
          <div 
            v-if="selectedOption && selectedOption.color"
            class="w-3 h-3 rounded-full border border-slate-300"
            :style="{ backgroundColor: selectedOption.color }"
          ></div>
          <Icon 
            :name="isOpen ? 'fluent:chevron-up-20-filled' : 'fluent:chevron-down-20-filled'"
            class="transition-transform duration-200"
            :class="[
              'text-slate-400 group-hover:text-slate-600',
              size === 'sm' ? 'text-sm' : 'text-base'
            ]"
          />
        </div>
      </div>
    </div>

    <!-- Dropdown Options -->
    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div 
        v-if="isOpen"
        class="absolute z-50 w-full mt-1 bg-white border border-slate-200 rounded-lg shadow-lg overflow-hidden"
        :class="[
          'max-h-60 overflow-y-auto',
          'ring-1 ring-black ring-opacity-5',
          'focus:outline-none'
        ]"
        role="listbox"
      >
        <div
          v-for="(option, index) in options"
          :key="option.value"
          @click="selectOption(option)"
          @mouseenter="highlightedIndex = index"
          class="relative px-4 py-3 cursor-pointer select-none transition-colors duration-150"
          :class="[
            index === highlightedIndex ? 'bg-blue-50 text-blue-900' : 'text-slate-900 hover:bg-slate-50',
            selectedOption?.value === option.value ? 'bg-blue-100 text-blue-900 font-medium' : '',
            size === 'sm' ? 'py-2 text-sm' : 'py-3'
          ]"
          role="option"
          :aria-selected="selectedOption?.value === option.value"
        >
          <div class="flex items-center justify-between">
            <span class="block truncate">{{ option.label }}</span>
            <div class="flex items-center space-x-2">
              <div 
                v-if="option.color"
                class="w-3 h-3 rounded-full border border-slate-300"
                :style="{ backgroundColor: option.color }"
              ></div>
              <Icon 
                v-if="selectedOption?.value === option.value"
                name="fluent:checkmark-20-filled"
                class="text-blue-600"
                :class="size === 'sm' ? 'text-sm' : 'text-base'"
              />
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'

export interface SelectOption {
  value: string | number
  label: string
  color?: string
  disabled?: boolean
}

export default defineComponent({
  name: 'CustomSelect',
  props: {
    modelValue: {
      type: [String, Number],
      default: ''
    },
    options: {
      type: Array as PropType<SelectOption[]>,
      required: true
    },
    placeholder: {
      type: String,
      default: '選択してください'
    },
    disabled: {
      type: Boolean,
      default: false
    },
    error: {
      type: Boolean,
      default: false
    },
    size: {
      type: String as PropType<'sm' | 'md'>,
      default: 'md'
    }
  },
  emits: ['update:modelValue', 'change'],
  data() {
    return {
      isOpen: false,
      focused: false,
      highlightedIndex: 0
    }
  },
  computed: {
    selectedOption(): SelectOption | null {
      return this.options.find(option => option.value === this.modelValue) || null
    }
  },
  watch: {
    isOpen(newVal) {
      if (newVal) {
        this.highlightedIndex = this.options.findIndex(option => option.value === this.modelValue)
        if (this.highlightedIndex === -1) this.highlightedIndex = 0
      }
    }
  },
  mounted() {
    document.addEventListener('click', this.handleClickOutside)
  },
  beforeUnmount() {
    document.removeEventListener('click', this.handleClickOutside)
  },
  methods: {
    toggleDropdown() {
      if (this.disabled) return
      this.isOpen = !this.isOpen
      this.focused = this.isOpen
    },
    closeDropdown() {
      this.isOpen = false
      this.focused = false
    },
    selectOption(option: SelectOption) {
      if (option.disabled) return
      this.$emit('update:modelValue', option.value)
      this.$emit('change', option)
      this.closeDropdown()
    },
    navigateOptions(direction: number) {
      if (!this.isOpen) {
        this.toggleDropdown()
        return
      }
      
      const newIndex = this.highlightedIndex + direction
      if (newIndex >= 0 && newIndex < this.options.length) {
        this.highlightedIndex = newIndex
      }
    },
    handleClickOutside(event: Event) {
      const target = event.target as HTMLElement
      if (!this.$el.contains(target)) {
        this.closeDropdown()
      }
    }
  }
})
</script>

<style scoped>
.custom-select-container {
  position: relative;
}

.custom-select:focus {
  outline: none;
}

/* Custom scrollbar for dropdown */
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