export const SEPARATORS: { [name: string]: { text: string; value: string } } = {
  conmma: { text: ",", value: "," },
  tab: { text: "タブ", value: "\t" },
  halfspace: { text: "半角スペース", value: " " },
  fullspace: { text: "全角スペース", value: "　" },
  semicolon: { text: ";", value: ";" },
  colon: { text: ":", value: ":" },
  pipe: { text: "|", value: "|" },
  slash: { text: "/", value: "/" },
  backslash: { text: "\\", value: "\\" },
} as const;

export const SeparatorArray = Object.values(SEPARATORS);
