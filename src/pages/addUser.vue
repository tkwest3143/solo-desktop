<template>
  <div
    class="bg-gradient-to-br from-slate-50 to-blue-50 flex items-center justify-center p-6"
  >
    <div
      class="bg-white rounded-xl shadow-xl border border-slate-200 p-8 w-full max-w-md"
    >
      <div class="text-center mb-8">
        <div
          class="mx-auto w-16 h-16 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center mb-4"
        >
          <Icon
            name="fluent:person-add-20-filled"
            size="2em"
            class="text-white"
          />
        </div>
        <h1 class="text-2xl font-bold text-slate-800 mb-2">ユーザ追加</h1>
        <p class="text-slate-600 text-sm">
          新しいユーザー情報を入力してください
        </p>
      </div>

      <form @submit.prevent="addUser" class="space-y-6">
        <CommonInput
          id="name"
          label="名前"
          v-model.trim="name"
          required
          class="w-full"
        />
        <CommonInput
          id="email"
          label="メールアドレス"
          type="email"
          v-model.trim="email"
          required
          class="w-full"
        />

        <div class="flex space-x-3 pt-4">
          <button
            type="button"
            class="flex-1 px-4 py-3 bg-gradient-to-r from-slate-500 to-slate-600 hover:from-slate-600 hover:to-slate-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
            @click="cancel"
          >
            <Icon name="fluent:arrow-left-20-filled" class="mr-2" size="1em" />
            キャンセル
          </button>
          <button
            type="submit"
            class="flex-1 px-4 py-3 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
          >
            <Icon name="fluent:person-add-20-filled" class="mr-2" size="1em" />
            追加
          </button>
        </div>
      </form>
      <CustomDialog
        :show="dialog.show"
        :title="dialog.title"
        :message="dialog.message"
        :type="dialog.type"
        :okText="dialog.okText"
        @ok="onDialogOk"
        @cancel="onDialogCancel"
      />
    </div>
  </div>
</template>

<script lang="ts">
import CommonInput from "~/components/CommonInput.vue";
import CustomDialog from "~/components/todo/CustomDialog.vue";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  components: {
    CommonInput,
    CustomDialog,
  },
  data() {
    return {
      name: "",
      email: "",
      dialog: {
        show: false,
        title: "",
        message: "",
        type: "confirm",
        okText: "追加する",
        resolve: null as null | ((result: boolean) => void),
        userId: null as null | number,
      },
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
      // ユーザー追加後に勤務設定追加を促す
      this.dialog = {
        show: true,
        title: "ユーザー追加完了",
        message: "続けて勤務設定を追加しますか？",
        type: "confirm",
        okText: "勤務設定を追加",
        resolve: async (result: boolean) => {
          if (result) {
            // 追加直後のユーザーを取得（最も新しいユーザーを利用）
            const users = await UserRepository.getAll();
            const lastUser = users[users.length - 1];
            if (lastUser && lastUser.id) {
              this.$router.push({
                name: "id-settings-workSetting-add",
                params: { id: lastUser.id },
              });
              return;
            }
          }
          this.$router.push("/");
        },
        userId: null,
      };
    },
    onDialogOk() {
      if (this.dialog.resolve) this.dialog.resolve(true);
      this.dialog.show = false;
    },
    onDialogCancel() {
      if (this.dialog.resolve) this.dialog.resolve(false);
      this.dialog.show = false;
    },
    cancel() {
      this.$router.push("/");
    },
  },
};
</script>
