<template>
  <div class="form-group mb-4">
    <label :for="id" class="block text-gray-700 text-sm font-bold mb-2">
      {{ label }}
    </label>
    <textarea
      :id="id"
      :value="modelValue"
      @input="
        $emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)
      "
      :required="required"
      :placeholder="placeholder"
      :rows="rows"
      :class="[
        'shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline',
        { 'border-error-500': errorMessage },
      ]"
    ></textarea>
    <span v-if="errorMessage" class="text-error-500">{{ errorMessage }}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    id: {
      type: String,
      required: true,
    },
    label: {
      type: String,
      required: true,
    },
    modelValue: {
      type: String,
      required: true,
    },
    placeholder: {
      type: String,
      default: "",
    },
    required: {
      type: Boolean,
      default: false,
    },
    rows: {
      type: Number,
      default: 3,
    },
    errorMessage: {
      type: String,
      default: "",
    },
  },
});
</script>