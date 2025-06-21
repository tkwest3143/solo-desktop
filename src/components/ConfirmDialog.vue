<template>
  <div v-if="show" class="fixed inset-0 z-50 overflow-y-auto">
    <!-- Backdrop -->
    <div class="fixed inset-0 transition-opacity" :class="`bg-black bg-opacity-50 dark:bg-opacity-70`" @click="onCancel"></div>
    
    <!-- Dialog -->
    <div class="flex min-h-screen items-center justify-center p-4">
      <div class="relative rounded-lg shadow-xl max-w-md w-full mx-auto transform transition-all" :class="`${getBgClass('primary')} dark:bg-slate-800`">
        <!-- Header -->
        <div class="p-6 pb-4">
          <div class="flex items-center">
            <div class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full" :class="getStateClasses('error', 'bg')">
              <Icon name="fluent:warning-20-filled" class="h-6 w-6" :class="getStateClasses('error', 'text')" />
            </div>
          </div>
          <div class="mt-3 text-center">
            <h3 class="text-lg font-medium" :class="getTextClass('primary')">{{ title }}</h3>
            <div class="mt-2">
              <p class="text-sm" :class="getTextClass('secondary')">{{ message }}</p>
            </div>
          </div>
        </div>
        
        <!-- Actions -->
        <div class="px-6 py-4 rounded-b-lg flex space-x-3 justify-end" :class="`${getBgClass('secondary')} dark:bg-slate-700`">
          <button
            @click="onCancel"
            type="button"
            class="px-4 py-2 text-sm font-medium rounded-md focus:outline-none focus:ring-2 transition-colors"
            :class="`${getTextClass('secondary')} ${getBgClass('primary')} ${getBorderClass('default')} border ${getStateClasses('info', 'hover')} focus:ring-blue-500 dark:bg-slate-600 dark:border-slate-500 dark:hover:bg-slate-500`"
          >
            {{ cancelText }}
          </button>
          <button
            @click="onConfirm"
            type="button"
            class="px-4 py-2 text-sm font-medium border border-transparent rounded-md focus:outline-none focus:ring-2 transition-colors"
            :class="`${getTextClass('inverse')} ${getStateClasses('error', 'bg').replace('bg-red-50', 'bg-red-600').replace('dark:bg-red-900', 'dark:bg-red-600')} ${getStateClasses('error', 'hover').replace('hover:bg-red-100', 'hover:bg-red-700').replace('dark:hover:bg-red-800', 'dark:hover:bg-red-700')} focus:ring-red-500`"
          >
            {{ confirmText }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "ConfirmDialog",
  props: {
    show: {
      type: Boolean,
      default: false,
    },
    title: {
      type: String,
      default: "確認",
    },
    message: {
      type: String,
      default: "この操作を実行しますか？",
    },
    confirmText: {
      type: String,
      default: "削除",
    },
    cancelText: {
      type: String,
      default: "キャンセル",
    },
  },
  emits: ["confirm", "cancel"],
  setup() {
    const { getBgClass, getTextClass, getBorderClass, getStateClasses } = useTheme();
    
    return {
      getBgClass,
      getTextClass,
      getBorderClass,
      getStateClasses,
    };
  },
  methods: {
    onConfirm() {
      this.$emit("confirm");
    },
    onCancel() {
      this.$emit("cancel");
    },
  },
});
</script>

<style scoped>
/* Ensure the dialog appears above everything */
.fixed {
  position: fixed;
}
</style>