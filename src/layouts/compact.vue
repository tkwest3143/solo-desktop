<template>
  <div class="h-screen bg-white flex flex-col">
    <AppHeader @compact-view-changed="handleCompactViewChange" />
    
    <!-- Compact Content Area -->
    <div class="flex-1 overflow-hidden">
      <!-- Attendance View -->
      <div v-if="currentView === 'attendance'" class="h-full">
        <CompactAttendance />
      </div>
      
      <!-- Todo View -->
      <div v-else-if="currentView === 'todo'" class="h-full">
        <CompactTodo />
      </div>
      
      <!-- Fallback -->
      <div v-else class="h-full">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return {
      currentView: "attendance" as "attendance" | "todo",
    };
  },
  methods: {
    handleCompactViewChange(view: "attendance" | "todo") {
      this.currentView = view;
    },
  },
});
</script>