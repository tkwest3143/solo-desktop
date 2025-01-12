<template>
  <div class="form-group">
    <label :for="id">{{ label }}</label>
    <input
      :type="type"
      :id="id"
      :value="modelValue"
      @input="
        $emit('update:modelValue', ($event.target as HTMLInputElement).value)
      "
      :required="required"
      :class="[
        'w-full border rounded p-2',
        { 'border-error-500': errorMessage },
      ]"
    />
    <span v-if="errorMessage" class="text-error-500">{{ errorMessage }}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from "vue";

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
    type: {
      type: String as PropType<"text" | "email" | "password">,
      default: "text",
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
