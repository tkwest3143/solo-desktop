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
        <span class="text-xl font-bold text-slate-800">Solo</span>
      </div>

      <!-- Right side - Navigation -->
      <div class="flex items-center space-x-1">
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
            :to="{ name: 'id-todo', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon
              name="fluent:task-list-square-20-filled"
              class="mr-2"
              size="1.2em"
            />
            Todo管理
          </NuxtLink>

          <NuxtLink
            :to="{ name: 'id-productivity', params: { id: userData.prop.id } }"
            class="flex items-center px-4 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200"
          >
            <Icon
              name="fluent:timer-20-filled"
              class="mr-2"
              size="1.2em"
            />
            タイマー
          </NuxtLink>

          <!-- User Profile Dropdown -->
          <div
            class="flex items-center ml-4 pl-4 border-l border-slate-200 relative"
          >
            <button
              ref="dropdownBtn"
              @click="toggleDropdown"
              class="flex items-center px-3 py-2 rounded-lg text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200 focus:outline-none"
            >
              <Icon name="fluent:person-20-filled" class="mr-2" size="1.2em" />
              {{ userData.prop.name }}さん
              <Icon
                name="fluent:chevron-down-20-filled"
                class="ml-1"
                size="1em"
              />
            </button>
            <transition name="fade">
              <div
                v-if="showDropdown"
                class="absolute left-0 mt-2 w-44 bg-white border border-slate-200 rounded-lg shadow-lg z-50 py-2"
                :style="dropdownStyle"
              >
                <NuxtLink
                  :to="{
                    name: 'id-settings',
                    params: { id: userData.prop.id },
                  }"
                  class="block px-4 py-2 text-slate-700 hover:bg-blue-50 hover:text-blue-700 text-sm transition-colors"
                  @click="closeDropdown"
                >
                  <Icon
                    name="fluent:settings-20-filled"
                    class="mr-2"
                    size="1em"
                  />
                  設定
                </NuxtLink>
                <button
                  @click="changeUser"
                  class="w-full text-left px-4 py-2 text-slate-700 hover:bg-blue-50 hover:text-blue-700 text-sm transition-colors"
                >
                  <Icon
                    name="fluent:people-swap-20-filled"
                    class="mr-2"
                    size="1em"
                  />
                  ユーザ変更
                </button>
              </div>
            </transition>
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
  data() {
    return {
      showDropdown: false,
    };
  },
  computed: {
    dropdownStyle() {
      // ドロップダウンを氏名ボタンの下に表示
      return {
        top: "100%",
        marginTop: "0.5rem",
      };
    },
  },
  mounted() {
    document.addEventListener("click", this.handleClickOutside);
  },
  beforeUnmount() {
    document.removeEventListener("click", this.handleClickOutside);
  },
  methods: {
    goBack() {
      this.$router.back();
    },
    toggleDropdown() {
      this.showDropdown = !this.showDropdown;
    },
    closeDropdown() {
      this.showDropdown = false;
    },
    changeUser() {
      this.closeDropdown();
      this.$router.push({ name: "index" });
    },
    handleClickOutside(event: MouseEvent) {
      const dropdownBtn = this.$refs.dropdownBtn as HTMLElement | undefined;
      const dropdownMenu = this.$el.querySelector(
        ".absolute"
      ) as HTMLElement | null;
      if (!this.showDropdown) return;
      if (dropdownBtn && dropdownBtn.contains(event.target as Node)) {
        return;
      }
      if (dropdownMenu && dropdownMenu.contains(event.target as Node)) {
        return;
      }
      this.closeDropdown();
    },
  },
});
</script>

<style scoped></style>
