<template>
  <Header />
  <div class="add-user-container">
    <h1>ユーザ追加</h1>
    <form @submit.prevent="addUser" class="add-user-form">
      <div class="form-group">
        <label for="name">名前:</label>
        <input type="text" v-model="name" id="name" required />
      </div>
      <div class="form-group">
        <label for="email">メール:</label>
        <input type="email" v-model="email" id="email" required />
      </div>
      <button type="submit" class="submit-button">追加</button>
    </form>
  </div>
</template>

<script lang="ts">
import Header from "~/components/Header.vue";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  components: {
    Header,
  },
  data() {
    return {
      name: "",
      email: "",
    };
  },
  methods: {
    async addUser() {
      await UserRepository.create({
        name: this.name,
        email: this.email,
      });
      this.name = "";
      this.email = "";
      this.$router.push("/");
    },
  },
};
</script>

<style scoped>
.add-user-container {
  max-width: 400px;
  margin: 50px auto;
  padding: 20px;
  background-color: #fff;
  border-radius: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  text-align: center;
}

h1 {
  margin-bottom: 20px;
  color: #343a40;
}

.add-user-form {
  display: flex;
  flex-direction: column;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  color: #343a40;
}

input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
}

.submit-button {
  padding: 10px 20px;
  border: none;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.3s;
}

.submit-button:hover {
  background-color: #0056b3;
}
</style>
