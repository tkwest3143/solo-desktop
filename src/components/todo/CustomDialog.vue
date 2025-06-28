<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40"
  >
    <div class="bg-white rounded-lg shadow-lg max-w-sm w-full p-6">
      <div class="mb-4 text-lg text-slate-800 font-semibold">
        <slot name="title">{{ title }}</slot>
      </div>
      <div class="mb-6 text-slate-700">
        <slot>{{ message }}</slot>
      </div>
      <div class="flex justify-end space-x-3">
        <button
          v-if="type === 'confirm'"
          @click="$emit('cancel')"
          class="px-4 py-2 rounded bg-slate-200 text-slate-700 hover:bg-slate-300 font-medium"
        >
          キャンセル
        </button>
        <button
          @click="$emit('ok')"
          :class="
            type === 'alert'
              ? 'bg-blue-500 hover:bg-blue-600 text-white'
              : 'bg-red-500 hover:bg-red-600 text-white'
          "
          class="px-4 py-2 rounded font-medium"
        >
          {{ okText }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  props: {
    show: { type: Boolean, required: true },
    title: { type: String, default: "" },
    message: { type: String, default: "" },
    type: { type: String, default: "alert" }, // 'alert' or 'confirm'
    okText: { type: String, default: "OK" },
  },
});
</script>

<style scoped></style>
