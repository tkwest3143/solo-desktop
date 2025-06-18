<template>
  <Switch
    :class="[
      modelValue ? activeClass : inactiveClass,
      'relative inline-flex h-6 w-11 items-center rounded-full'
    ]"
  >
    <span class="sr-only">{{ label || 'Toggle' }}</span>
    <span
      :class="[
        modelValue ? 'translate-x-6' : 'translate-x-1',
        'inline-block h-4 w-4 transform rounded-full bg-white transition'
      ]"
    />
  </Switch>
</template>

<script lang="ts">
import { Switch } from "@headlessui/vue";
import { defineComponent } from "vue";

export default defineComponent({
  components: {
    Switch,
  },
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    },
    label: {
      type: String,
      default: "",
    },
    activeClass: {
      type: String,
      default: "bg-primary-600",
    },
    inactiveClass: {
      type: String,
      default: "bg-basic-200",
    },
  },
});
</script>