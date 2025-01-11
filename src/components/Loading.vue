<template>
  <div
    class="loading-container flex justify-center items-center h-screen w-full"
  >
    <div class="loading-line"></div>
    <div class="loading-text">{{ loadingText }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    loadingText: {
      type: String,
      default: "Loading",
    },
  },
});
</script>

<style scoped>
.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  position: relative;
}

.loading-line {
  position: absolute;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, transparent, #000, transparent);
  animation: loading 1.5s infinite;
}

.loading-text {
  position: relative;
  font-size: 20px;
  font-weight: bold;
  margin-left: 10px;
}

@keyframes loading {
  0% {
    left: -100%;
  }
  100% {
    left: 100%;
  }
}
</style>
