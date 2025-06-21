<template>
  <button
    @click="toggleColorMode"
    class="p-2 rounded-lg transition-all duration-200"
    :class="[
      $colorMode.value === 'dark' 
        ? 'bg-yellow-500 hover:bg-yellow-600 text-white' 
        : 'bg-slate-200 hover:bg-slate-300 text-slate-700 dark:bg-slate-700 dark:hover:bg-slate-600 dark:text-slate-200'
    ]"
    :title="$colorMode.value === 'dark' ? 'ライトモードに切り替え' : 'ダークモードに切り替え'"
  >
    <Icon
      :name="$colorMode.value === 'dark' ? 'fluent:weather-sunny-20-filled' : 'fluent:weather-moon-20-filled'"
      size="1.2em"
    />
  </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'ThemeToggle',
  methods: {
    toggleColorMode() {
      this.$colorMode.preference = this.$colorMode.value === 'dark' ? 'light' : 'dark';
    }
  }
});
</script>