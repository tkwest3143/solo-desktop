<template>
  <button
    @click="resizeWindow"
    class="bg-primary-400 text-basic-50 rounded-lg hover:bg-primary-500 px-4 py-2 text-sm font-medium transition-all duration-200"
  >
    {{ windowMode === "normal" ? "小さく表示" : "大きく表示" }}
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
      windowMode: "normal" as "normal" | "compact",
      currentRoute: [] as RouteRecord[],
      resizeObserver: null as ResizeObserver | null,
    };
  },
  mounted() {
    // Add window state to global for other components to access
    (window as any).windowState = {
      mode: this.windowMode,
      setMode: (mode: "normal" | "compact") => {
        this.windowMode = mode;
        (window as any).windowState.mode = mode;
      }
    };

    // Listen for window resize events to detect manual resizing
    this.setupWindowListener();
  },
  beforeUnmount() {
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
    }
  },
  methods: {
    setupWindowListener() {
      // Use ResizeObserver to detect window size changes
      this.resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          const { width, height } = entry.contentRect;
          
          // If window is manually resized to large dimensions and we're in compact mode
          if (this.windowMode === "compact" && (width > 500 || height > 600)) {
            this.windowMode = "normal";
            (window as any).windowState.mode = "normal";
            
            // Update route to remove compact query
            const route = this.$route;
            if (route.query.compact === "true") {
              const { compact, ...otherQuery } = route.query;
              this.$router.push({ ...route, query: otherQuery });
            }
          }
        }
      });
      
      this.resizeObserver.observe(document.body);
    },
    async resizeWindow() {
      const currentWindow = getCurrentWindow();
      
      if (this.windowMode === "normal") {
        // Enable compact mode - position at bottom-right corner
        history.pushState(null, "", location.href);
        addEventListener("popstate", handlePopState);
        
        // Get screen dimensions
        const screenSize = await currentWindow.outerSize();
        const compactWidth = 320;
        const compactHeight = 500;
        
        // Position at bottom-right corner with some padding
        const x = screenSize.width - compactWidth - 20;
        const y = screenSize.height - compactHeight - 60; // 60px for taskbar
        
        await currentWindow.setSize(new LogicalSize(compactWidth, compactHeight));
        await currentWindow.setPosition(new LogicalPosition(x, y));
        
        this.windowMode = "compact";
        (window as any).windowState.mode = "compact";
        
        // Trigger route change to compact layout if needed
        const route = this.$route;
        if (route.query.compact !== "true") {
          this.$router.push({ ...route, query: { ...route.query, compact: "true" } });
        }
      } else {
        // Return to normal mode
        removeEventListener("popstate", handlePopState);
        
        const normalWidth = 1200;
        const normalHeight = 800;
        
        await currentWindow.setSize(new LogicalSize(normalWidth, normalHeight));
        
        // Center the window
        const screenSize = await currentWindow.outerSize();
        const x = (screenSize.width - normalWidth) / 2;
        const y = (screenSize.height - normalHeight) / 2;
        await currentWindow.setPosition(new LogicalPosition(x, y));

        this.windowMode = "normal";
        (window as any).windowState.mode = "normal";
        
        // Remove compact query parameter
        const route = this.$route;
        const { compact, ...otherQuery } = route.query;
        this.$router.push({ ...route, query: otherQuery });
      }
    },
  },
});
</script>
