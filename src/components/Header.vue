<template>
  <header
    class="bg-basic-50 text-basic-700 border-b-2 border-basic-200 flex items-center"
  >
    <div class="h-12 flex items-center justify-end w-full">
      <div class="hover:bg-basic-200 h-full">
        <NuxtLink
          to="/"
          class="flex items-center justify-center px-6 w-full h-full"
        >
          ユーザ一覧
        </NuxtLink>
      </div>
      <div v-if="userData" class="hover:bg-basic-200 h-full">
        <NuxtLink
          :to="{ name: 'id', params: { id: userData.prop.id } }"
          class="flex items-center justify-center px-6 w-full h-full"
        >
          ホーム
        </NuxtLink>
      </div>
      <div v-if="userData" class="hover:bg-basic-200 h-full">
        <NuxtLink
          :to="{ name: 'id-workTimeList', params: { id: userData.prop.id } }"
          class="flex items-center justify-center px-6 w-full h-full"
        >
          勤怠一覧
        </NuxtLink>
      </div>
      <div v-if="userData" class="hover:bg-basic-200 h-full">
        <NuxtLink
          :to="{ name: 'id-report', params: { id: userData.prop.id } }"
          class="flex items-center justify-center px-6 w-full h-full"
        >
          レポート
        </NuxtLink>
      </div>
      <div v-if="userData" class="hover:bg-basic-200 h-full">
        <NuxtLink
          :to="{ name: 'id-todo', params: { id: userData.prop.id } }"
          class="flex items-center justify-center px-6 w-full h-full"
        >
          Todo管理
        </NuxtLink>
      </div>
      <div v-if="userData" class="hover:bg-basic-200 h-full">
        <NuxtLink
          :to="{ name: 'id-settings', params: { id: userData.prop.id } }"
          class="flex items-center justify-center px-6 hover:bg-basic-200 w-full h-full"
        >
          <Icon name="fluent:settings-20-filled" class="mr-2" size="1.5em" />{{
            userData.prop.name + "さん"
          }}
        </NuxtLink>
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
