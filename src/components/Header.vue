<template>
  <header
    class="bg-white dark:bg-slate-800 shadow-lg border-b border-slate-200 dark:border-slate-700 flex items-center transition-colors"
  >
    <div class="h-14 flex items-center justify-between w-full px-4">
      <!-- Left side - Logo/Brand -->
      <div class="flex items-center">
        <Icon 
          name="fluent:organization-20-filled" 
          class="text-blue-600 dark:text-blue-400 mr-2" 
          size="1.8em" 
        />
        <span class="text-xl font-bold text-slate-800 dark:text-slate-100">Solo Desktop</span>
      </div>

      <!-- Right side - Navigation -->
      <div class="flex items-center space-x-1">
        <NuxtLink
          to="/"
          class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
        >
          <Icon name="fluent:people-20-filled" class="mr-2" size="1.2em" />
          ユーザ一覧
        </NuxtLink>
        
        <div v-if="userData" class="flex items-center space-x-1">
          <NuxtLink
            :to="{ name: 'id', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
          >
            <Icon name="fluent:home-20-filled" class="mr-2" size="1.2em" />
            ホーム
          </NuxtLink>
          
          <NuxtLink
            :to="{ name: 'id-workTimeList', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
          >
            <Icon name="fluent:clock-20-filled" class="mr-2" size="1.2em" />
            勤怠一覧
          </NuxtLink>
          
          <NuxtLink
            :to="{ name: 'id-report', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
          >
            <Icon name="fluent:document-data-20-filled" class="mr-2" size="1.2em" />
            レポート
          </NuxtLink>
          
          <NuxtLink
            :to="{ name: 'id-todo', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
          >
            <Icon name="fluent:task-list-square-20-filled" class="mr-2" size="1.2em" />
            Todo管理
          </NuxtLink>
          
          <!-- User Profile and Theme Toggle -->
          <div class="flex items-center ml-4 pl-4 border-l border-slate-200 dark:border-slate-600 space-x-2">
            <!-- Theme Toggle -->
            <ThemeToggle />
            
            <!-- User Profile -->
            <NuxtLink
              :to="{ name: 'id-settings', params: { id: userData.prop.id } }"
              class="flex items-center px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200"
            >
              <Icon name="fluent:person-20-filled" class="mr-2" size="1.2em" />
              {{ userData.prop.name }}さん
              <Icon name="fluent:settings-20-filled" class="ml-2" size="1.1em" />
            </NuxtLink>
          </div>
        </div>
        
        <!-- Theme toggle for when no user is logged in -->
        <div v-else class="ml-4">
          <ThemeToggle />
        </div>
      </div>
    </div>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { UserData } from "~/models/user";
import ThemeToggle from "~/components/ThemeToggle.vue";

export default defineComponent({
  components: {
    ThemeToggle,
  },
  props: {
    userData: {
      type: Object as () => UserData,
      required: false,
    },
  },
  methods: {
    goBack() {
      this.$router.back();
    },
  },
});
</script>

<style scoped></style>
