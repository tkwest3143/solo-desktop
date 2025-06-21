<template>
  <header
    class="bg-white shadow-lg border-b border-slate-200 flex items-center"
    :class="isCompactMode ? 'h-10' : 'h-14'"
  >
    <!-- Normal Mode Header -->
    <div v-if="!isCompactMode" class="h-14 flex items-center justify-between w-full px-4">
      <!-- Left side - Logo/Brand -->
      <div class="flex items-center">
        <Icon 
          name="fluent:organization-20-filled" 
          class="text-blue-600 mr-2" 
          size="1.8em" 
        />
        <NuxtLink
          :to="userData ? { name: 'id', params: { id: userData.prop.id } } : '/'"
          class="text-xl font-bold text-slate-800 hover:text-blue-600 transition-colors duration-200"
        >
          Solo
        </NuxtLink>
      </div>

      <!-- Right side - Navigation -->
      <div class="flex items-center space-x-1">
        <div v-if="userData" class="flex items-center space-x-1">
          <NuxtLink
            :to="{ name: 'id-workTimeList', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon name="fluent:clock-20-filled" class="mr-2" size="1.2em" />
            勤怠一覧
          </NuxtLink>
          
          <NuxtLink
            :to="{ name: 'id-report', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon name="fluent:document-data-20-filled" class="mr-2" size="1.2em" />
            レポート
          </NuxtLink>
          
          <NuxtLink
            :to="{ name: 'id-todo', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon name="fluent:task-list-square-20-filled" class="mr-2" size="1.2em" />
            Todo管理
          </NuxtLink>
          
          <!-- User Profile Icons -->
          <div class="flex items-center ml-4 pl-4 border-l border-slate-200 space-x-2">
            <NuxtLink
              :to="{ name: 'id-settings', params: { id: userData.prop.id } }"
              class="p-2 rounded-lg text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
              title="設定"
            >
              <Icon name="fluent:settings-20-filled" size="1.2em" />
            </NuxtLink>
            <div class="p-2 rounded-lg text-slate-700" :title="`${userData.prop.name}さん`">
              <Icon name="fluent:person-20-filled" size="1.2em" />
            </div>
          </div>
          
          <!-- Resize Button -->
          <div class="ml-4 pl-4 border-l border-slate-200">
            <ResizeWindowButton />
          </div>
        </div>
      </div>
    </div>

    <!-- Compact Mode Header -->
    <div v-else class="h-10 flex items-center justify-between w-full px-2">
      <!-- Left side - Compact Logo -->
      <NuxtLink
        :to="userData ? { name: 'id', params: { id: userData.prop.id } } : '/'"
        class="flex items-center hover:bg-slate-100 rounded px-2 py-1 transition-all duration-200"
      >
        <Icon 
          name="fluent:organization-20-filled" 
          class="text-blue-600 mr-1" 
          size="1.2em" 
        />
        <span class="text-sm font-bold text-slate-800">Solo</span>
      </NuxtLink>

      <!-- Center - Compact Navigation -->
      <div v-if="userData" class="flex items-center space-x-1">
        <button
          @click="setCompactView('attendance')"
          :class="[
            'px-2 py-1 rounded text-xs font-medium transition-all duration-200',
            compactView === 'attendance' 
              ? 'bg-blue-500 text-white' 
              : 'text-slate-600 hover:text-slate-800 hover:bg-slate-100'
          ]"
        >
          勤怠
        </button>
        <button
          @click="setCompactView('todo')"
          :class="[
            'px-2 py-1 rounded text-xs font-medium transition-all duration-200',
            compactView === 'todo' 
              ? 'bg-blue-500 text-white' 
              : 'text-slate-600 hover:text-slate-800 hover:bg-slate-100'
          ]"
        >
          Todo
        </button>
      </div>

      <!-- Right side - Expand Button -->
      <div class="flex items-center space-x-1">
        <ResizeWindowButton />
      </div>
    </div>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { UserData } from "~/models/user";
import ResizeWindowButton from "./ResizeWindowButton.vue";

export default defineComponent({
  components: {
    ResizeWindowButton,
  },
  props: {
    userData: {
      type: Object as () => UserData,
      required: false,
    },
  },
  data() {
    return {
      compactView: "attendance" as "attendance" | "todo",
    };
  },
  computed: {
    isCompactMode() {
      return this.$route.query.compact === "true";
    },
  },
  watch: {
    isCompactMode(newVal) {
      if (newVal) {
        // Emit event to parent components about entering compact mode
        this.$emit("compact-mode-changed", { isCompact: true, view: this.compactView });
      } else {
        this.$emit("compact-mode-changed", { isCompact: false, view: null });
      }
    },
    compactView(newVal) {
      if (this.isCompactMode) {
        this.$emit("compact-view-changed", newVal);
      }
    }
  },
  methods: {
    goBack() {
      this.$router.back();
    },
    setCompactView(view: "attendance" | "todo") {
      this.compactView = view;
    },
  },
});
</script>

<style scoped></style>
