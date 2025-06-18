<template>
  <div class="form-group">
    <label :for="id" class="block text-gray-700 text-sm font-bold mb-2">
      {{ label }}
    </label>
    <input
      :id="id"
      type="time"
      :value="modelValue"
      @input="
        $emit('update:modelValue', ($event.target as HTMLInputElement).value)
      "
      :required="required"
      :class="[
        'shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline',
        { 'border-error-500': errorMessage },
      ]"
    />
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
    required: {
      type: Boolean,
      default: false,
    },
    errorMessage: {
      type: String,
      default: "",
    },
  },
});
</script>