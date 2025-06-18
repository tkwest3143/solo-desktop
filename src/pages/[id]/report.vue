<template>
  <div class="container mx-auto px-4 py-6">
    <div v-if="isLoading" class="flex justify-center items-center h-64">
      <Loading loading-text="レポートデータを取得中..." />
    </div>
    <div v-else>
      <h1 class="text-2xl font-bold text-basic-800 mb-6">勤務時間レポート</h1>
      
      <!-- Filters -->
      <div class="bg-white rounded-lg shadow-md p-6 mb-6">
        <div class="flex flex-wrap gap-4 items-end">
          <div class="flex-1 min-w-40">
            <CommonSelect
              id="reportType"
              label="表示タイプ"
              v-model="reportType"
              :options="reportTypeOptions"
              @change="onReportTypeChange"
            />
          </div>
          <div class="flex-1 min-w-40">
            <CommonSelect
              id="startYear"
              label="開始年"
              v-model="startYear"
              :options="yearOptions"
              @change="loadReportData"
            />
          </div>
          <div v-if="reportType === 'yearly'" class="flex-1 min-w-40">
            <CommonSelect
              id="endYear"
              label="終了年"
              v-model="endYear"
              :options="yearOptions"
              @change="loadReportData"
            />
          </div>
          <div v-if="reportType === 'monthly'" class="flex-1 min-w-40">
            <CommonSelect
              id="displayYear"
              label="表示年"
              v-model="displayYear"
              :options="yearOptions"
              @change="loadReportData"
            />
          </div>
          <div class="flex-1 min-w-40">
            <button
              @click="loadReportData"
              class="w-full px-4 py-2 text-basic-0 bg-primary-400 hover:bg-primary-500 rounded-lg transition-colors"
            >
              データ更新
            </button>
          </div>
        </div>
      </div>

      <!-- Chart -->
      <div class="bg-white rounded-lg shadow-md p-6 mb-6">
        <WorkTimeChart
          :type="reportType"
          :data="chartData"
          :title="chartTitle"
        />
      </div>

      <!-- Summary Statistics -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h2 class="text-xl font-bold text-basic-800 mb-4">統計サマリー</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="bg-blue-50 p-4 rounded-lg">
            <div class="text-sm text-blue-600 font-medium">総勤務時間</div>
            <div class="text-2xl font-bold text-blue-800">
              {{ totalHours.toFixed(1) }}時間
            </div>
          </div>
          <div class="bg-green-50 p-4 rounded-lg">
            <div class="text-sm text-green-600 font-medium">総勤務日数</div>
            <div class="text-2xl font-bold text-green-800">
              {{ totalWorkDays }}日
            </div>
          </div>
          <div class="bg-purple-50 p-4 rounded-lg">
            <div class="text-sm text-purple-600 font-medium">平均勤務時間/日</div>
            <div class="text-2xl font-bold text-purple-800">
              {{ averageHoursPerDay.toFixed(1) }}時間
            </div>
          </div>
          <div class="bg-orange-50 p-4 rounded-lg">
            <div class="text-sm text-orange-600 font-medium">対象期間</div>
            <div class="text-lg font-bold text-orange-800">
              {{ periodText }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import CommonSelect from "~/components/CommonSelect.vue";
import Loading from "~/components/Loading.vue";
import WorkTimeChart from "~/components/report/WorkTimeChart.vue";
import {
  aggregateWorkTimeByMonth,
  aggregateWorkTimeByYear,
  type MonthlyReportData,
  type YearlyReportData,
} from "~/functions/reportData";
import { UserData } from "~/models/user";
import { workTimeData } from "~/models/workTime";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";

export default defineComponent({
  components: {
    CommonSelect,
    Loading,
    WorkTimeChart,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      isLoading: true,
      reportType: "monthly" as "monthly" | "yearly",
      startYear: new Date().getFullYear(),
      endYear: new Date().getFullYear(),
      displayYear: new Date().getFullYear(),
      workTimes: [] as workTimeData[],
      monthlyData: [] as MonthlyReportData[],
      yearlyData: [] as YearlyReportData[],
    };
  },
  computed: {
    reportTypeOptions() {
      return [
        { value: "monthly", text: "月別表示" },
        { value: "yearly", text: "年別表示" },
      ];
    },
    yearOptions() {
      const currentYear = new Date().getFullYear();
      const options = [];
      for (let year = currentYear - 10; year <= currentYear + 1; year++) {
        options.push({ value: year, text: `${year}年` });
      }
      return options;
    },
    chartData() {
      if (this.reportType === "monthly") {
        return this.monthlyData.filter(item => item.year === this.displayYear);
      } else {
        return this.yearlyData;
      }
    },
    chartTitle() {
      if (this.reportType === "monthly") {
        return `${this.displayYear}年 月別勤務時間`;
      } else {
        return `${this.startYear}年〜${this.endYear}年 年別勤務時間`;
      }
    },
    totalHours() {
      if (this.reportType === "monthly") {
        return this.monthlyData
          .filter(item => item.year === this.displayYear)
          .reduce((sum, item) => sum + item.totalHours, 0);
      } else {
        return this.yearlyData
          .filter(item => item.year >= this.startYear && item.year <= this.endYear)
          .reduce((sum, item) => sum + item.totalHours, 0);
      }
    },
    totalWorkDays() {
      if (this.reportType === "monthly") {
        return this.monthlyData
          .filter(item => item.year === this.displayYear)
          .reduce((sum, item) => sum + item.workDays, 0);
      } else {
        return this.yearlyData
          .filter(item => item.year >= this.startYear && item.year <= this.endYear)
          .reduce((sum, item) => sum + item.workDays, 0);
      }
    },
    averageHoursPerDay() {
      return this.totalWorkDays > 0 ? this.totalHours / this.totalWorkDays : 0;
    },
    periodText() {
      if (this.reportType === "monthly") {
        return `${this.displayYear}年`;
      } else {
        return `${this.startYear}年〜${this.endYear}年`;
      }
    },
  },
  async mounted() {
    await this.init();
    await this.loadReportData();
    this.isLoading = false;
  },
  methods: {
    async init() {
      const userId = this.$route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
    },
    async loadReportData() {
      this.isLoading = true;
      try {
        const userId = this.$route.params.id as string;
        
        let workTimeRes;
        if (this.reportType === "monthly") {
          // Load data for the display year
          workTimeRes = await WorkTimeRepository.getWorkTimeByYear(
            userId,
            this.displayYear
          );
        } else {
          // Load data for the year range
          workTimeRes = await WorkTimeRepository.getWorkTimeByYearRange(
            userId,
            this.startYear,
            this.endYear
          );
        }
        
        this.workTimes = workTimeRes.map((wt) => new workTimeData(wt));
        this.monthlyData = aggregateWorkTimeByMonth(this.workTimes);
        this.yearlyData = aggregateWorkTimeByYear(this.workTimes);
      } catch (error) {
        console.error("Failed to load report data:", error);
      } finally {
        this.isLoading = false;
      }
    },
    onReportTypeChange() {
      this.loadReportData();
    },
  },
});
</script>

<style scoped>
.container {
  max-width: 1200px;
}
</style>