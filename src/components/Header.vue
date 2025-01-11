<template>
  <header class="h-16">
    <nav>
      <ul>
        <li>
          <NuxtLink to="/">ユーザ一覧</NuxtLink>
        </li>
        <li v-if="userData">
          <NuxtLink :to="{ name: 'id', params: { id: userData.prop.id } }"
            >ホーム</NuxtLink
          >
        </li>
        <li v-if="userData">
          <NuxtLink
            :to="{ name: 'id-workTimeList', params: { id: userData.prop.id } }"
            >勤怠一覧</NuxtLink
          >
        </li>
        <li v-if="userData">
          <button @click="goBack">戻る</button>
        </li>
        <li v-if="userData">
          <NuxtLink
            :to="{ name: 'id-settings', params: { id: userData.prop.id } }"
            ><Icon
              name="fluent:settings-20-filled"
              style="color: slategray"
              size="2em"
          /></NuxtLink>
        </li>
      </ul>
    </nav>
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

<style scoped>
header {
  background-color: #343a40;
  padding: 10px 0;
}

nav {
  display: flex;
  justify-content: center;
}

nav ul {
  display: flex;
  list-style: none;
  padding: 0;
  margin: 0;
}

nav ul li {
  margin-right: 20px;
}

nav ul li a,
nav ul li button {
  color: white;
  text-decoration: none;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
}

nav ul li button:hover,
nav ul li a:hover {
  text-decoration: underline;
}
</style>
