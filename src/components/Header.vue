<template>
  <header>
    <nav>
      <ul>
        <li>
          <NuxtLink :to="{ name: 'id', params: { id: userId } }"
            >ホーム</NuxtLink
          >
        </li>
        <li>
          <NuxtLink :to="{ name: 'id-workTimeList', params: { id: userId } }"
            >勤怠一覧</NuxtLink
          >
        </li>
        <li>
          <button @click="goBack">戻る</button>
        </li>
        <li>
          <NuxtLink :to="{ name: 'id-settings', params: { id: userId } }"
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

export default defineComponent({
  setup() {
    const route = useRoute();
    const userId = ref(route.params.id as string);

    return {
      userId,
    };
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

nav ul {
  display: flex;
  list-style: none;
  padding: 0;
  margin: 0;
  justify-content: center;
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
