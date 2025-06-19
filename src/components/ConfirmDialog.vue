<template>
  <div v-if="show" class="fixed inset-0 z-50 overflow-y-auto">
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-black bg-opacity-50 transition-opacity" @click="onCancel"></div>
    
    <!-- Dialog -->
    <div class="flex min-h-screen items-center justify-center p-4">
      <div class="relative bg-white rounded-lg shadow-xl max-w-md w-full mx-auto transform transition-all">
        <!-- Header -->
        <div class="p-6 pb-4">
          <div class="flex items-center">
            <div class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-red-100">
              <Icon name="fluent:warning-20-filled" class="h-6 w-6 text-red-600" />
            </div>
          </div>
          <div class="mt-3 text-center">
            <h3 class="text-lg font-medium text-gray-900">{{ title }}</h3>
            <div class="mt-2">
              <p class="text-sm text-gray-500">{{ message }}</p>
            </div>
          </div>
        </div>
        
        <!-- Actions -->
        <div class="px-6 py-4 bg-gray-50 rounded-b-lg flex space-x-3 justify-end">
          <button
            @click="onCancel"
            type="button"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
          >
            {{ cancelText }}
          </button>
          <button
            @click="onConfirm"
            type="button"
            class="px-4 py-2 text-sm font-medium text-white bg-red-600 border border-transparent rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 transition-colors"
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