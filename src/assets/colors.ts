export interface ColorOption {
  name: string;
  value: string;
  bg: string;
  light?: string;
  dark?: string;
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

// Default color for new items
export const defaultColor = "#3b82f6"; // blue-500