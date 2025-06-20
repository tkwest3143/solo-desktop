<template>
  <div
    v-if="isLoading"
    class="fixed inset-0 bg-slate-800 bg-opacity-75 flex items-center justify-center z-50"
  >
    <Loading />
  </div>
  
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 p-6">
    <div class="max-w-4xl mx-auto">
      <!-- Page Header -->
      <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6 mb-6">
        <div class="flex items-center space-x-4">
          <div class="w-12 h-12 bg-gradient-to-br from-green-500 to-green-600 rounded-full flex items-center justify-center">
            <Icon name="fluent:briefcase-settings-20-filled" size="1.5em" class="text-white" />
          </div>
          <div>
            <h1 class="text-2xl font-bold text-slate-800 mb-1">勤務設定を追加</h1>
            <p class="text-slate-600">新しい勤務時間パターンを登録します</p>
          </div>
        </div>
      </div>

      <!-- Main Form Card -->
      <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-8">
        <form @submit.prevent="submitForm" class="space-y-8">
          <!-- Title Section -->
          <div>
            <label class="block text-slate-700 text-sm font-semibold mb-3" for="title">
              <Icon name="fluent:text-20-filled" class="mr-2 text-blue-600" size="1em" />
              タイトル
            </label>
            <input
              v-model="form.title"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 text-slate-900"
              id="title"
              type="text"
              placeholder="例: 標準勤務時間、フレックスタイムなど"
            />
          </div>

          <!-- Time Settings Section -->
          <div>
            <div class="flex items-center mb-4">
              <Icon name="fluent:clock-20-filled" class="mr-2 text-blue-600" size="1.2em" />
              <h3 class="text-lg font-semibold text-slate-800">時間設定</h3>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
              <div>
                <label class="block text-slate-700 text-sm font-medium mb-2" for="startTime">
                  開始時間
                </label>
                <input
                  v-model="form.startTime"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                  id="startTime"
                  type="time"
                />
              </div>
              <div>
                <label class="block text-slate-700 text-sm font-medium mb-2" for="endTime">
                  終了時間
                </label>
                <input
                  v-model="form.endTime"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                  id="endTime"
                  type="time"
                />
              </div>
              <div>
                <label class="block text-slate-700 text-sm font-medium mb-2" for="breakStartTime">
                  休憩開始時間
                </label>
                <input
                  v-model="form.breakStartTime"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                  id="breakStartTime"
                  type="time"
                />
              </div>
              <div>
                <label class="block text-slate-700 text-sm font-medium mb-2" for="breakEndTime">
                  休憩終了時間
                </label>
                <input
                  v-model="form.breakEndTime"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                  id="breakEndTime"
                  type="time"
                />
              </div>
              <div>
                <label class="block text-slate-700 text-sm font-medium mb-2" for="workUnit">
                  勤務時間単位 (分)
                </label>
                <input
                  v-model="form.workUnit"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                  id="workUnit"
                  type="number"
                  min="1"
                  max="99"
                  placeholder="15"
                />
              </div>
            </div>
          </div>

          <!-- Memo Section -->
          <div>
            <label class="block text-slate-700 text-sm font-semibold mb-3" for="memo">
              <Icon name="fluent:note-20-filled" class="mr-2 text-blue-600" size="1em" />
              メモ
            </label>
            <textarea
              v-model="form.memo"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 text-slate-900"
              id="memo"
              placeholder="この勤務設定に関する補足情報があれば入力してください"
              rows="3"
            ></textarea>
          </div>

          <!-- Default Setting Toggle -->
          <div class="bg-slate-50 rounded-lg p-6">
            <button
              @click="form.isDefaultWorkSetting = !form.isDefaultWorkSetting"
              :class="{
                'bg-gradient-to-r from-blue-500 to-blue-600 text-white border-blue-500': form.isDefaultWorkSetting,
                'bg-white text-slate-700 border-slate-300 hover:border-slate-400': !form.isDefaultWorkSetting,
              }"
              class="flex items-center justify-start w-full px-4 py-3 border-2 rounded-lg transition-all duration-200 shadow-sm hover:shadow-md"
              type="button"
            >
              <div
                class="flex items-center justify-center w-6 h-6 rounded mr-3"
                :class="{
                  'bg-white': form.isDefaultWorkSetting,
                  'bg-slate-100 border-2 border-slate-300': !form.isDefaultWorkSetting,
                }"
              >
                <Icon
                  name="fluent:checkmark-20-filled"
                  v-if="form.isDefaultWorkSetting"
                  size="1em"
                  class="text-blue-600"
                />
              </div>
              <div class="text-left">
                <div class="font-semibold">デフォルトの勤務時間として登録する</div>
                <div class="text-sm opacity-75">このユーザーの標準的な勤務パターンとして設定されます</div>
              </div>
            </button>
          </div>

          <!-- Action Buttons -->
          <div class="flex items-center justify-end space-x-4 pt-6 border-t border-slate-200">
            <button
              type="button"
              @click="$router.back()"
              class="px-6 py-3 bg-gradient-to-r from-slate-500 to-slate-600 hover:from-slate-600 hover:to-slate-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
            >
              <Icon name="fluent:arrow-left-20-filled" class="mr-2" size="1em" />
              キャンセル
            </button>
            <button
              class="px-6 py-3 bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
              type="submit"
            >
              <Icon name="fluent:save-20-filled" class="mr-2" size="1em" />
              保存
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default defineComponent({
  components: {
    Loading,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      workSettings: [] as workSettingData[],
      form: {
        title: "",
        startTime: "",
        endTime: "",
        breakStartTime: "",
        breakEndTime: "",
        workUnit: 0,
        memo: "",
        isDefaultWorkSetting: false,
      },
      isLoading: false,
    };
  },
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const route = useRoute();
      const userId = route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
    },
    async submitForm() {
      // フォーム送信処理を実装
      if (!this.user) return;
      this.isLoading = true;
      await WorkSettingRepository.create({
        userId: this.user.prop.id,
        title: this.form.title,
        start: new Date(`2024-01-01T${this.form.startTime}`),
        end: new Date(`2024-01-01T${this.form.endTime}`),
        restStart: new Date(`2024-01-01T${this.form.breakStartTime}`),
        restEnd: new Date(`2024-01-01T${this.form.breakEndTime}`),
        workingUnit: this.form.workUnit,
        memo: this.form.memo,
        isDefault: this.form.isDefaultWorkSetting,
      });
      // フォームをリセット
      this.form = {
        title: "",
        startTime: "",
        endTime: "",
        breakStartTime: "",
        breakEndTime: "",
        workUnit: 0,
        memo: "",
        isDefaultWorkSetting: false,
      };
      this.isLoading = false;
    },
  },
});
</script>
