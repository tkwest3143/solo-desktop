import { format } from "date-fns";

export class UserData {
  constructor(public prop: user) {}
  get lastLoginTimeText() {
    if (!this.prop.lastLoginTime) return "";
    return format(this.prop.lastLoginTime, "yyyy/MM/dd HH:mm:ss");
  }
}

export type user = {
  id: number;
  name: string;
  email: string;
  defaultWorkSettingId?: number;
  lastLoginTime?: Date;
  createdAt: Date;
  updatedAt: Date;
};
