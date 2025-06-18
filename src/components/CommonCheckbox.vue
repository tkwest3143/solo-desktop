<template>
  <div
    class="flex items-center cursor-pointer"
    :class="containerClass"
    @click="toggle"
  >
    <div
      class="flex items-center justify-center w-6 h-6 border rounded"
      :class="[
        'border-basic-300',
        { 'bg-primary-200': modelValue, 'bg-white': !modelValue }
      ]"
    >
      <Icon
        :name="iconName"
        class="h-5 w-5"
        v-if="modelValue"
      />
    </div>
    <label class="ml-2 cursor-pointer" v-if="label">{{ label }}</label>
    <slot v-else></slot>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    },
    label: {
      type: String,
      default: "",
    },
    iconName: {
      type: String,
      default: "material-symbols:check-rounded",
    },
    containerClass: {
      type: String,
      default: "",
    },
  },
  methods: {
    toggle() {
      this.$emit("update:modelValue", !this.modelValue);
    },
  },
});
</script>