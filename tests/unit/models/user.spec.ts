import { UserData, type user } from "@/models/user";
import { describe, expect, it } from "vitest";

describe("UserData", () => {
  const user: user = {
    id: 1,
    name: "Test User",
    email: "test@example.com",
    default_work_setting_id: 1,
    last_login_time: new Date("2023-10-01T09:00:00"),
    created_at: new Date(),
    updated_at: new Date(),
  };

  it("returns formatted last login time", () => {
    const userInstance = new UserData(user);
    expect(userInstance.lastLoginTimeText).toBe("2023/10/1 09:00:00");
  });

  it("returns '-' if last login time is not set", () => {
    const userWithoutLoginTime = { ...user, last_login_time: undefined };
    const userInstance = new UserData(userWithoutLoginTime);
    expect(userInstance.lastLoginTimeText).toBe("-");
  });
});
