<template>
  <Loading v-if="isLoading" />
  <div v-else class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <!-- Page Header -->
    <div class="bg-white shadow-sm border-b border-slate-200">
      <div class="container mx-auto px-6 py-6">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-4">
            <button
              @click="previousMonth"
              class="p-2 rounded-lg hover:bg-slate-100 transition-colors duration-200"
            >
              <Icon
                name="fluent:arrow-left-20-filled"
                size="1.2em"
                class="text-slate-600"
              />
            </button>
            <button
              @click="nextMonth"
              class="p-2 rounded-lg hover:bg-slate-100 transition-colors duration-200"
            >
              <Icon
                name="fluent:arrow-right-20-filled"
                size="1.2em"
                class="text-slate-600"
              />
            </button>
            <h1 class="text-2xl font-bold text-slate-800">
              {{ selectedMonth.yearText }}年{{
                selectedMonth.monthText
              }}月の勤務時間
            </h1>
            <button
              @click="thisMonth"
              class="px-4 py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition-colors duration-200 text-sm font-medium"
            >
              今月
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="container mx-auto px-6 py-6 space-y-6">
      <!-- Work Settings Card -->
      <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
        <h2 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
          <Icon
            name="fluent:briefcase-settings-20-filled"
            class="mr-2 text-blue-600"
            size="1.3em"
          />
          勤務設定
        </h2>

        <div class="flex items-center space-x-4 mb-4">
          <div class="flex-1">
            <CommonSelect
              id="workSetting"
              label="勤務設定を選択"
              v-model="selectedWorkSettingId"
              :options="
                workSettings.map((setting) => ({
                  value: setting.prop.id,
                  text: setting.prop.title,
                }))
              "
            />
          </div>
          <div class="flex-shrink-0">
            <button
              v-if="selectedWorkSettingId"
              class="px-4 py-2 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg"
            >
              設定を編集する
            </button>
            <NuxtLink
              v-else
              class="px-4 py-2 bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg"
              :to="{
                name: 'id-settings-workSetting-add',
                params: { id: user?.prop.id },
              }"
            >
              設定を登録する
            </NuxtLink>
          </div>
        </div>

        <!-- Work Settings Details -->
        <div
          class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4 p-4 bg-slate-50 rounded-lg"
        >
          <div class="text-center">
            <div class="text-xs text-slate-500 mb-1">開始時間</div>
            <div class="font-semibold text-slate-800">
              {{ getSelectedWorkSetting()?.startByText ?? "-" }}
            </div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-500 mb-1">終了時間</div>
            <div class="font-semibold text-slate-800">
              {{ getSelectedWorkSetting()?.endByText ?? "-" }}
            </div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-500 mb-1">休憩開始</div>
            <div class="font-semibold text-slate-800">
              {{ getSelectedWorkSetting()?.restStartByText ?? "-" }}
            </div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-500 mb-1">休憩終了</div>
            <div class="font-semibold text-slate-800">
              {{ getSelectedWorkSetting()?.restEndByText ?? "-" }}
            </div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-500 mb-1">時間単位</div>
            <div class="font-semibold text-slate-800">
              {{ getSelectedWorkSetting()?.prop.working_unit ?? "-" }}分
            </div>
          </div>
        </div>
      </div>

      <!-- Actions Bar -->
      <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <button
              :disabled="selectedWorkSettingId === 0"
              @click="setAllDefaultWorkTime"
              class="px-4 py-2 bg-gradient-to-r from-purple-500 to-purple-600 hover:from-purple-600 hover:to-purple-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg disabled:cursor-not-allowed text-sm font-medium"
            >
              <Icon
                name="fluent:magic-wand-20-filled"
                class="mr-2"
                size="1em"
              />
              未入力を自動登録
            </button>
            <ExportFileDialog
              :selectableColumns="[
                ...selectedMonth.monthWorkTimes[0].exportFileColumn.keys(),
              ]"
              @download="downloadWorkTime"
            />
          </div>
          <div
            class="flex items-center space-x-2 bg-gradient-to-r from-green-50 to-green-100 px-4 py-2 rounded-lg"
          >
            <Icon
              name="fluent:clock-20-filled"
              class="text-green-600"
              size="1.2em"
            />
            <span class="text-sm font-medium text-green-700"
              >今月の勤務時間:</span
            >
            <span class="text-lg font-bold text-green-800">{{
              totalWorkTime()
            }}</span>
          </div>
        </div>
      </div>
      <!-- Work Time Table -->
      <div
        class="bg-white rounded-xl shadow-lg border border-slate-200 overflow-hidden"
      >
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-slate-200">
            <thead class="bg-slate-50">
              <tr>
                <th
                  scope="col"
                  class="w-12 px-2 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                ></th>
                <th
                  scope="col"
                  class="px-6 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  日付
                </th>
                <th
                  scope="col"
                  class="w-20 px-2 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  稼働/非稼働
                </th>
                <th
                  scope="col"
                  class="px-4 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  勤務時間
                </th>
                <th
                  scope="col"
                  class="px-4 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  休憩時間
                </th>
                <th
                  scope="col"
                  class="px-4 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  業務時間
                </th>
                <th
                  scope="col"
                  class="px-4 py-3 text-center text-xs font-medium text-slate-500 uppercase tracking-wider"
                >
                  備考
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-slate-200">
              <tr
                v-for="(workTime, index) in selectedMonth.monthWorkTimes"
                :key="index"
                class="hover:bg-slate-50 transition-colors duration-200"
              >
                <td class="px-2 py-3 text-center">
                  <button
                    :disabled="selectedWorkSettingId === 0"
                    @click="setDefaultWorkTime(workTime)"
                    class="p-2 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg transition-all duration-200 shadow-sm hover:shadow-md disabled:cursor-not-allowed"
                    title="デフォルト設定を適用"
                  >
                    <Icon name="fluent:magic-wand-20-filled" size="1em" />
                  </button>
                </td>
                <td
                  class="px-6 py-3 text-center text-sm font-medium"
                  :class="{
                    'text-red-600':
                      workTime.isHoliday(japaneseHolidays) ||
                      workTime.isSunday(),
                    'text-blue-600': workTime.isSaturday(),
                    'text-slate-900':
                      !workTime.isHoliday(japaneseHolidays) &&
                      !workTime.isSaturday() &&
                      !workTime.isSunday(),
                  }"
                >
                  {{ workTime.getDayTextWithWeek(japaneseHolidays) }}
                </td>
                <td class="px-2 py-3 text-center">
                  <div class="flex justify-center">
                    <Icon
                      v-if="workTime.workDurationByMinute !== 0"
                      name="fluent:checkmark-circle-20-filled"
                      class="text-green-600"
                      size="1.5em"
                      title="稼働"
                    />
                    <Icon
                      v-else
                      name="fluent:subtract-circle-20-filled"
                      class="text-slate-400"
                      size="1.5em"
                      title="非稼働"
                    />
                  </div>
                </td>
                <td
                  @click="openEditDialog(workTime)"
                  class="px-4 py-3 text-center text-sm text-slate-900 cursor-pointer hover:bg-blue-50 transition-colors"
                >
                  <div class="text-xs text-slate-500">
                    {{ workTime.startByText }} - {{ workTime.endByText }}
                  </div>
                </td>
                <td
                  @click="openEditDialog(workTime)"
                  class="px-4 py-3 text-center text-sm text-slate-900 cursor-pointer hover:bg-blue-50 transition-colors"
                >
                  <div class="space-y-1">
                    <div class="text-xs text-slate-500">
                      {{ workTime.restStartByText }} -
                      {{ workTime.restEndByText }}
                    </div>
                    <div class="text-sm font-medium">
                      {{ workTime.restDurationByText ?? "00:00" }}
                    </div>
                  </div>
                </td>
                <td
                  class="px-4 py-3 text-center text-sm font-medium text-slate-900"
                >
                  {{ workTime.workDurationByText ?? "00:00" }}
                </td>
                <td
                  class="px-4 py-3 text-center text-sm text-slate-900 max-w-32"
                >
                  <div class="flex items-center justify-center space-x-2">
                    <button
                      @click="showMemoDialog(workTime)"
                      class="p-1 bg-gradient-to-r from-slate-500 to-slate-600 hover:from-slate-600 hover:to-slate-700 text-white rounded-lg transition-all duration-200 shadow-sm hover:shadow-md flex-shrink-0"
                      title="備考を表示"
                    >
                      <Icon name="fluent:note-20-filled" size="0.9em" />
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- Save/Cancel Footer -->
    <div
      v-if="changedWorkTimes.length > 0"
      class="fixed bottom-0 left-0 w-full bg-white shadow-2xl border-t border-slate-200 p-4 z-50"
    >
      <div class="container mx-auto flex justify-between items-center">
        <div class="flex items-center space-x-2">
          <Icon
            name="fluent:edit-20-filled"
            class="text-blue-600"
            size="1.2em"
          />
          <span class="text-slate-700 font-medium"
            >{{ changedWorkTimes.length }}件の変更があります</span
          >
        </div>
        <div class="flex space-x-3">
          <button
            @click="resetChanges"
            class="px-4 py-2 bg-gradient-to-r from-slate-500 to-slate-600 hover:from-slate-600 hover:to-slate-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
          >
            <Icon name="fluent:arrow-reset-20-filled" class="mr-2" size="1em" />
            リセット
          </button>
          <button
            @click="saveChanges"
            class="px-4 py-2 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg transition-all duration-200 shadow-md hover:shadow-lg font-medium"
          >
            <Icon name="fluent:save-20-filled" class="mr-2" size="1em" />
            保存
          </button>
        </div>
      </div>
    </div>

    <!-- Memo Dialog -->
    <MemoDialog
      :show="showMemo"
      :memo="selectedMemo"
      :date="selectedMemoDate"
      @close="closeMemoDialog"
    />

    <!-- Edit Work Time Dialog -->
    <EditWorkTimeDialog
      :show="showEditDialog"
      :work-time="editDialogWorkTime"
      :date="editDialogDate"
      @close="closeEditDialog"
      @save="saveWorkTimeFromDialog"
    />
  </div>
</template>

<script lang="ts">
import { format } from "date-fns";
import { defineComponent } from "vue";
import CommonSelect from "~/components/CommonSelect.vue";
import EditWorkTimeDialog from "~/components/EditWorkTimeDialog.vue";
import Loading from "~/components/Loading.vue";
import MemoDialog from "~/components/MemoDialog.vue";
import ExportFileDialog from "~/components/worktime/ExportFileDialog.vue";
import { JapaneseHolidayData } from "~/models/japaneseHoliday";
import { MonthForWork } from "~/models/monthForWork";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { workTimeData } from "~/models/workTime";
import { JapaneseHolidayRepository } from "~/repositories/tauri-commands/japaneseHoliday";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";
const NOT_SET_WORK_TIME_ID = -1;

export default defineComponent({
  components: {
    Loading,
    CommonSelect,
    ExportFileDialog,
    MemoDialog,
    EditWorkTimeDialog,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      isLoading: true,
      selectedMonth: new MonthForWork(new Date(), []),
      workTimes: [] as workTimeData[],
      editingWorkTime: null as workTimeData | null,
      changedWorkTimes: [] as workTimeData[],
      workSettings: [] as workSettingData[],
      selectedWorkSettingId: 0,
      japaneseHolidays: [] as JapaneseHolidayData[],
      showMemo: false,
      selectedMemo: "",
      selectedMemoDate: "",
      showEditDialog: false,
      editDialogWorkTime: {
        start: "",
        end: "",
        restStart: "",
        restEnd: "",
        memo: "",
      },
      editDialogDate: "",
      currentEditingWorkTime: null as workTimeData | null,
    };
  },
  async mounted() {
    this.isLoading = true;
    await this.init();
    this.selectedMonth = new MonthForWork(new Date(), this.workTimes);
    this.isLoading = false;
  },
  methods: {
    async init() {
      const userId = this.$route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
      const workTimeRes = await WorkTimeRepository.getWorkTimeByMonth(
        userId,
        this.selectedMonth.byText
      );
      this.workTimes = workTimeRes.map((wt) => new workTimeData(wt));
      const workSettingRes = await WorkSettingRepository.getByUserId(
        parseInt(userId)
      );
      this.workSettings = workSettingRes.map((ws) => new workSettingData(ws));
      if (this.user.prop.default_work_setting_id) {
        this.selectedWorkSettingId = this.user.prop.default_work_setting_id;
      }
      const holiday = await JapaneseHolidayRepository.get(
        new Date().getFullYear().toString()
      );
      if (holiday.length === 0) {
        await JapaneseHolidayRepository.import();
        this.japaneseHolidays = await JapaneseHolidayRepository.get(
          new Date().getFullYear().toString()
        );
      } else {
        this.japaneseHolidays = holiday;
      }
    },
    getWorkTime(date: string): workTimeData | undefined {
      const workTime = this.workTimes.find(
        (workTime) => workTime.prop.target_day === date
      );
      if (workTime) {
        return workTime;
      }
    },
    totalWorkTime(): string {
      const total = this.workTimes.reduce((acc, workTime) => {
        return acc + workTime.workDurationByMinute;
      }, 0);
      return `${Math.floor(total / 60)}時間${total % 60}分`;
    },
    async previousMonth() {
      this.selectedMonth = this.selectedMonth.previousMonth;
      await this.fetchWorkTimeByMonth();
    },
    async nextMonth() {
      this.selectedMonth = this.selectedMonth.nextMonth;
      await this.fetchWorkTimeByMonth();
    },
    async fetchWorkTimeByMonth() {
      this.isLoading = true;
      if (this.user) {
        await WorkTimeRepository.getWorkTimeByMonth(
          this.user.prop.id.toString(),
          this.selectedMonth.byText
        )
          .then(
            (val) => (this.workTimes = val.map((wt) => new workTimeData(wt)))
          )
          .finally(() => (this.isLoading = false));
      }
    },
    async thisMonth() {
      this.selectedMonth = this.selectedMonth.thisMonth;
      await this.fetchWorkTimeByMonth();
    },
    changeWorkTime(
      workTime: workTimeData,
      updateProp: {
        start?: string;
        end?: string;
        restStart?: string;
        restEnd?: string;
        memo?: string;
      }
    ) {
      if (updateProp.start) {
        const start = new Date(
          `${workTime.prop.target_day}T${updateProp.start}`
        );
        workTime.prop.start = start;
      }
      if (updateProp.end) {
        const end = new Date(`${workTime.prop.target_day}T${updateProp.end}`);
        workTime.prop.end = end;
      }
      if (updateProp.restStart) {
        const restStart = new Date(
          `${workTime.prop.target_day}T${updateProp.restStart}`
        );
        workTime.prop.rest_start = restStart;
      }
      if (updateProp.restEnd) {
        const restEnd = new Date(
          `${workTime.prop.target_day}T${updateProp.restEnd}`
        );
        workTime.prop.rest_end = restEnd;
      }
      if (updateProp.memo) {
        workTime.prop.memo = updateProp.memo;
      }

      if (
        this.workTimes.some(
          (wt) => wt.prop.target_day === workTime.prop.target_day
        )
      ) {
        this.workTimes = this.workTimes.map((wt) => {
          if (wt.prop.target_day === workTime.prop.target_day) {
            return workTime;
          }
          return wt;
        });
      } else {
        this.workTimes.push(workTime);
      }
      this.selectedMonth = new MonthForWork(
        this.selectedMonth.month,
        this.workTimes
      );
      if (
        this.changedWorkTimes.some(
          (wt) => wt.prop.target_day === workTime.prop.target_day
        )
      ) {
        this.changedWorkTimes = this.changedWorkTimes.map((wt) => {
          if (wt.prop.target_day === workTime.prop.target_day) {
            return workTime;
          }
          return wt;
        });
      } else {
        this.changedWorkTimes.push(workTime);
      }
    },
    async saveChanges() {
      this.isLoading = true;
      for (let workTime of this.changedWorkTimes) {
        if (workTime.prop.id === NOT_SET_WORK_TIME_ID) {
          await WorkTimeRepository.create({
            start: workTime.prop.start,
            end: workTime.prop.end,
            restStart: workTime.prop.rest_start,
            restEnd: workTime.prop.rest_end,
            memo: workTime.prop.memo,
            userId: this.user!.prop.id,
            targetDay: workTime.prop.target_day,
          });
        } else {
          await WorkTimeRepository.update({
            start: workTime.prop.start,
            end: workTime.prop.end,
            restStart: workTime.prop.rest_start,
            restEnd: workTime.prop.rest_end,
            memo: workTime.prop.memo,
            userId: this.user!.prop.id,
            targetDay: workTime.prop.target_day,
          });
        }
      }
      this.changedWorkTimes = [];
      this.editingWorkTime = null;
      await this.init();
      this.selectedMonth = new MonthForWork(
        this.selectedMonth.month,
        this.workTimes
      );
      this.isLoading = false;
    },
    async resetChanges() {
      this.changedWorkTimes = [];
      this.editingWorkTime = null;
      await this.init();
    },
    getSelectedWorkSetting() {
      return this.workSettings.find(
        (ws) => ws.prop.id === this.selectedWorkSettingId
      );
    },
    setDefaultWorkTime(workTime: workTimeData) {
      const workSetting = this.getSelectedWorkSetting();
      if (!workSetting) return;
      this.changeWorkTime(workTime, {
        start: format(workSetting.prop.start, "HH:mm"),
        end: format(workSetting.prop.end, "HH:mm"),
        restStart: format(workSetting.prop.rest_start, "HH:mm"),
        restEnd: format(workSetting.prop.rest_end, "HH:mm"),
        memo: workSetting.prop.memo,
      });
    },
    async downloadWorkTime(
      selectedColumns: string[],
      separator: string,
      isHeader: boolean
    ) {
      await WorkTimeRepository.fileCreate(
        this.selectedMonth.monthWorkTimes,
        this.selectedMonth.monthText + "月勤務時間",
        selectedColumns,
        separator,
        isHeader
      );
    },
    setAllDefaultWorkTime() {
      this.selectedMonth.monthWorkTimes.forEach((workTime) => {
        if (workTime.prop.id === NOT_SET_WORK_TIME_ID) {
          if (
            !workTime.isHoliday(this.japaneseHolidays) &&
            !workTime.isSaturday() &&
            !workTime.isSunday()
          ) {
            this.setDefaultWorkTime(workTime);
          }
        }
      });
    },
    showMemoDialog(workTime: workTimeData) {
      this.selectedMemo = workTime.memo || "";
      this.selectedMemoDate = workTime.getDayTextWithWeek(
        this.japaneseHolidays
      );
      this.showMemo = true;
    },
    closeMemoDialog() {
      this.showMemo = false;
      this.selectedMemo = "";
      this.selectedMemoDate = "";
    },
    openEditDialog(workTime: workTimeData) {
      this.currentEditingWorkTime = workTime;
      this.editDialogWorkTime = {
        start: workTime.startByText,
        end: workTime.endByText,
        restStart: workTime.restStartByText,
        restEnd: workTime.restEndByText,
        memo: workTime.memo || "",
      };
      this.editDialogDate = workTime.getDayTextWithWeek(this.japaneseHolidays);
      this.showEditDialog = true;
    },
    closeEditDialog() {
      this.showEditDialog = false;
      this.currentEditingWorkTime = null;
    },
    saveWorkTimeFromDialog(editedData: any) {
      if (this.currentEditingWorkTime) {
        this.changeWorkTime(this.currentEditingWorkTime, {
          start: editedData.start,
          end: editedData.end,
          restStart: editedData.restStart,
          restEnd: editedData.restEnd,
          memo: editedData.memo,
        });
      }
      this.closeEditDialog();
    },
  },
});
</script>
