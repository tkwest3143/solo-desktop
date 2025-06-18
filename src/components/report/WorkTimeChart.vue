<template>
  <div class="chart-container">
    <canvas ref="chartCanvas"></canvas>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref, watch } from "vue";
import type { PropType } from "vue";
import {
  Chart,
  registerables,
} from "chart.js";
import type { ChartConfiguration, ChartType } from "chart.js";
import type { MonthlyReportData, YearlyReportData } from "~/functions/reportData";

Chart.register(...registerables);

export default defineComponent({
  props: {
    type: {
      type: String as PropType<"monthly" | "yearly">,
      required: true,
    },
    data: {
      type: Array as PropType<MonthlyReportData[] | YearlyReportData[]>,
      required: true,
    },
    title: {
      type: String,
      default: "勤務時間レポート",
    },
  },
  setup(props) {
    const chartCanvas = ref<HTMLCanvasElement>();
    let chartInstance: Chart | null = null;

    const createChart = () => {
      if (!chartCanvas.value || !props.data.length) return;

      // Destroy existing chart
      if (chartInstance) {
        chartInstance.destroy();
      }

      let labels: string[];
      let dataValues: number[];

      if (props.type === "monthly") {
        const monthlyData = props.data as MonthlyReportData[];
        labels = monthlyData.map((item) => `${item.year}/${item.month}`);
        dataValues = monthlyData.map((item) => Math.round(item.totalHours * 10) / 10);
      } else {
        const yearlyData = props.data as YearlyReportData[];
        labels = yearlyData.map((item) => `${item.year}年`);
        dataValues = yearlyData.map((item) => Math.round(item.totalHours * 10) / 10);
      }

      const config: ChartConfiguration = {
        type: "bar" as ChartType,
        data: {
          labels,
          datasets: [
            {
              label: "勤務時間（時間）",
              data: dataValues,
              backgroundColor: "rgba(59, 130, 246, 0.6)",
              borderColor: "rgba(59, 130, 246, 1)",
              borderWidth: 1,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: {
              display: true,
              text: props.title,
              font: {
                size: 16,
              },
            },
            legend: {
              display: true,
            },
          },
          scales: {
            y: {
              beginAtZero: true,
              title: {
                display: true,
                text: "時間",
              },
            },
            x: {
              title: {
                display: true,
                text: props.type === "monthly" ? "年月" : "年",
              },
            },
          },
        },
      };

      chartInstance = new Chart(chartCanvas.value, config);
    };

    onMounted(() => {
      createChart();
    });

    watch(
      () => [props.data, props.type],
      () => {
        createChart();
      },
      { deep: true }
    );

    return {
      chartCanvas,
    };
  },
});
</script>

<style scoped>
.chart-container {
  position: relative;
  height: 400px;
  width: 100%;
}
</style>