<template>
  <div class="h-screen bg-white flex flex-col">
    <AppHeader @compact-view-changed="handleCompactViewChange" />
    
    <!-- Compact Content Area -->
    <div class="flex-1 overflow-hidden">
      <!-- No User State -->
      <div v-if="!$route.params.id" class="h-full flex items-center justify-center p-4">
        <div class="text-center">
          <Icon name="fluent:person-20-filled" class="text-slate-300 mb-4" size="3em" />
          <p class="text-sm text-slate-600">ユーザーを選択してください</p>
          <NuxtLink
            to="/"
            class="mt-4 inline-block bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium py-2 px-4 rounded-lg transition-colors duration-200"
          >
            ユーザー一覧へ
          </NuxtLink>
        </div>
      </div>
      
      <!-- User-specific Views -->
      <div v-else>
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