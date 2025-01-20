type JsonToTypeScriptOptions = {
  enums?: Record<string, string[]>; // Enum定義（例: { Status: ["ACTIVE", "INACTIVE"] }）
};

export function jsonToTypeScript(
  res: string,
  options?: JsonToTypeScriptOptions
): string {
  const json = JSON.parse(res);
  const enums = options?.enums || {};

  function getType(value: any): string {
    if (value === null || value === undefined) {
      return "string | number | null"; // デフォルト型を指定
    }

    if (typeof value === "string") {
      // Date型かどうかを判定
      if (isValidDate(value)) {
        return "Date";
      }
      return "string";
    }

    if (typeof value === "number") {
      return "number";
    }

    if (Array.isArray(value)) {
      // 配列の場合、配列内の要素型を推測
      if (value.length === 0) {
        return "any[]"; // 空配列の場合
      }
      const elementType = getType(value[0]);
      return `${elementType}[]`;
    }

    if (typeof value === "object") {
      // オブジェクトの場合、再帰的に型を生成
      return `{ ${Object.entries(value)
        .map(([key, val]) => `${key}: ${getType(val)};`)
        .join(" ")} }`;
    }

    return "any"; // その他の場合
  }

  function isValidDate(value: string): boolean {
    const date = new Date(value);
    return !isNaN(date.getTime());
  }

  // 型を生成
  return `type GeneratedType = {
    ${Object.entries(json)
      .map(([key, value]) => {
        // Enum判定
        if (enums[key]) {
          return `${key}: ${key}Enum;`;
        }
        return `${key}: ${getType(value)};`;
      })
      .join("\n")}
  };
  
  // Enum定義
  ${Object.entries(enums)
    .map(
      ([enumName, values]) =>
        `enum ${enumName}Enum { ${values
          .map((v) => `${v} = "${v}"`)
          .join(", ")} }`
    )
    .join("\n")}
  `;
}
