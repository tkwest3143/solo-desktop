<template>
  <header
    class="bg-white shadow-lg border-b border-slate-200 flex items-center"
  >
    <div class="h-14 flex items-center justify-between w-full px-4">
      <!-- Left side - Logo/Brand -->
      <div class="flex items-center">
        <Icon 
          name="fluent:organization-20-filled" 
          class="text-blue-600 mr-2" 
          size="1.8em" 
        />
        <span class="text-xl font-bold text-slate-800">Solo Desktop</span>
      </div>

      <!-- Right side - Navigation -->
      <div class="flex items-center space-x-1">
        <NuxtLink
          to="/"
          class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
        >
          <Icon name="fluent:people-20-filled" class="mr-2" size="1.2em" />
          ユーザ一覧
        </NuxtLink>
        
        <div v-if="userData" class="flex items-center space-x-1">
          <NuxtLink
            :to="{ name: 'id', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon name="fluent:home-20-filled" class="mr-2" size="1.2em" />
            ホーム
          </NuxtLink>
          
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
          
          <!-- User Profile Dropdown -->
          <div class="flex items-center ml-4 pl-4 border-l border-slate-200">
            <NuxtLink
              :to="{ name: 'id-settings', params: { id: userData.prop.id } }"
              class="flex items-center px-3 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
            >
              <Icon name="fluent:person-20-filled" class="mr-2" size="1.2em" />
              {{ userData.prop.name }}さん
              <Icon name="fluent:settings-20-filled" class="ml-2" size="1.1em" />
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { UserData } from "~/models/user";

export default defineComponent({
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
