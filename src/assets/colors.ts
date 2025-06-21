export interface ColorOption {
  name: string;
  value: string;
  bg: string;
  light?: string;
  dark?: string;
}

export interface SemanticColors {
  // Background colors
  background: {
    primary: { light: string; dark: string };
    secondary: { light: string; dark: string };
    tertiary: { light: string; dark: string };
    modal: { light: string; dark: string };
    hover: { light: string; dark: string };
    active: { light: string; dark: string };
  };
  // Text colors
  text: {
    primary: { light: string; dark: string };
    secondary: { light: string; dark: string };
    tertiary: { light: string; dark: string };
    inverse: { light: string; dark: string };
  };
  // Border colors
  border: {
    default: { light: string; dark: string };
    focus: { light: string; dark: string };
    active: { light: string; dark: string };
  };
  // State colors
  state: {
    error: {
      bg: { light: string; dark: string };
      text: { light: string; dark: string };
      border: { light: string; dark: string };
      hover: { light: string; dark: string };
    };
    success: {
      bg: { light: string; dark: string };
      text: { light: string; dark: string };
      border: { light: string; dark: string };
      hover: { light: string; dark: string };
    };
    warning: {
      bg: { light: string; dark: string };
      text: { light: string; dark: string };
      border: { light: string; dark: string };
      hover: { light: string; dark: string };
    };
    info: {
      bg: { light: string; dark: string };
      text: { light: string; dark: string };
      border: { light: string; dark: string };
      hover: { light: string; dark: string };
    };
  };
}

// Centralized color definitions for the application
export const colorOptions: ColorOption[] = [
  {
    name: "ブルー",
    value: "#3b82f6",
    bg: "#3b82f6",
    light: "#3b82f6", // blue-500
    dark: "#60a5fa"   // blue-400 (lighter for dark mode)
  },
  {
    name: "レッド", 
    value: "#ef4444",
    bg: "#ef4444",
    light: "#ef4444", // red-500
    dark: "#f87171"   // red-400
  },
  {
    name: "グリーン",
    value: "#10b981", 
    bg: "#10b981",
    light: "#10b981", // emerald-500
    dark: "#34d399"   // emerald-400
  },
  {
    name: "オレンジ",
    value: "#f59e0b",
    bg: "#f59e0b", 
    light: "#f59e0b", // amber-500
    dark: "#fbbf24"   // amber-400
  },
  {
    name: "パープル",
    value: "#a855f7",
    bg: "#a855f7",
    light: "#a855f7", // purple-500
    dark: "#c084fc"   // purple-400
  },
  {
    name: "ピンク",
    value: "#ec4899",
    bg: "#ec4899",
    light: "#ec4899", // pink-500
    dark: "#f472b6"   // pink-400
  },
  {
    name: "インディゴ",
    value: "#6366f1", 
    bg: "#6366f1",
    light: "#6366f1", // indigo-500
    dark: "#818cf8"   // indigo-400
  },
  {
    name: "グレー",
    value: "#6b7280",
    bg: "#6b7280",
    light: "#6b7280", // gray-500  
    dark: "#9ca3af"   // gray-400
  },
  {
    name: "イエロー",
    value: "#eab308",
    bg: "#eab308",
    light: "#eab308", // yellow-500
    dark: "#facc15"   // yellow-400
  }
];

// Semantic color system for consistent UI states
export const semanticColors: SemanticColors = {
  background: {
    primary: { light: "#ffffff", dark: "#1e293b" },      // white / slate-800
    secondary: { light: "#f8fafc", dark: "#0f172a" },    // slate-50 / slate-900
    tertiary: { light: "#f1f5f9", dark: "#334155" },     // slate-100 / slate-700
    modal: { light: "rgba(0, 0, 0, 0.5)", dark: "rgba(0, 0, 0, 0.7)" },
    hover: { light: "#f1f5f9", dark: "#475569" },        // slate-100 / slate-600
    active: { light: "#eff6ff", dark: "#1e3a8a" },       // blue-50 / blue-800
  },
  text: {
    primary: { light: "#0f172a", dark: "#f8fafc" },      // slate-900 / slate-50
    secondary: { light: "#475569", dark: "#cbd5e1" },    // slate-600 / slate-300
    tertiary: { light: "#94a3b8", dark: "#64748b" },     // slate-400 / slate-500
    inverse: { light: "#ffffff", dark: "#0f172a" },      // white / slate-900
  },
  border: {
    default: { light: "#e2e8f0", dark: "#475569" },      // slate-200 / slate-600
    focus: { light: "#3b82f6", dark: "#60a5fa" },        // blue-500 / blue-400
    active: { light: "#3b82f6", dark: "#60a5fa" },       // blue-500 / blue-400
  },
  state: {
    error: {
      bg: { light: "#fef2f2", dark: "#7f1d1d" },         // red-50 / red-900
      text: { light: "#dc2626", dark: "#fca5a5" },       // red-600 / red-300
      border: { light: "#fca5a5", dark: "#dc2626" },     // red-300 / red-600
      hover: { light: "#fee2e2", dark: "#991b1b" },      // red-100 / red-800
    },
    success: {
      bg: { light: "#f0fdf4", dark: "#14532d" },         // green-50 / green-900
      text: { light: "#16a34a", dark: "#86efac" },       // green-600 / green-300
      border: { light: "#86efac", dark: "#16a34a" },     // green-300 / green-600
      hover: { light: "#dcfce7", dark: "#166534" },      // green-100 / green-800
    },
    warning: {
      bg: { light: "#fffbeb", dark: "#78350f" },         // amber-50 / amber-900
      text: { light: "#d97706", dark: "#fcd34d" },       // amber-600 / amber-300
      border: { light: "#fcd34d", dark: "#d97706" },     // amber-300 / amber-600
      hover: { light: "#fef3c7", dark: "#92400e" },      // amber-100 / amber-800
    },
    info: {
      bg: { light: "#eff6ff", dark: "#1e3a8a" },         // blue-50 / blue-900
      text: { light: "#2563eb", dark: "#93c5fd" },       // blue-600 / blue-300
      border: { light: "#93c5fd", dark: "#2563eb" },     // blue-300 / blue-600
      hover: { light: "#dbeafe", dark: "#1e40af" },      // blue-100 / blue-800
    },
  },
};

// Default color for new items
export const defaultColor = "#3b82f6"; // blue-500