<template>
  <button
    @click="resizeWindow"
    class="bg-primary-400 text-basic-50 rounded-lg hover:bg-primary-500"
  >
    サイズを変更
  </button>
</template>

<script lang="ts">
import {
  getCurrentWindow,
  LogicalPosition,
  LogicalSize,
} from "@tauri-apps/api/window";
import { defineComponent } from "vue";
import type { RouteRecord } from "vue-router";

const handlePopState = () => {
  history.go(1);
};
export default defineComponent({
  data() {
    return {
      windowMode: "normal" as "normal" | "minimized",
      currentRoute: [] as RouteRecord[],
    };
  },
  methods: {
    async resizeWindow() {
      if (this.windowMode === "normal") {
        history.pushState(null, "", location.href);
        addEventListener("popstate", handlePopState);
        const currentWindow = getCurrentWindow();
        await currentWindow.setPosition(new LogicalPosition(0, 0));
        await getCurrentWindow().setSize(new LogicalSize(200, 400));
        this.windowMode = "minimized";
      } else {
        removeEventListener("popstate", handlePopState);
        await getCurrentWindow().setSize(new LogicalSize(800, 600));
        const currentWindow = getCurrentWindow();
        const screenSize = await currentWindow.innerSize();
        await currentWindow.setPosition(
          new LogicalPosition(
            (screenSize.width - 800) / 2,
            (screenSize.height - 600) / 2
          )
        );

        this.windowMode = "normal";
      }
    },
  },
});
</script>
