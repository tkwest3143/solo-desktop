import { format } from "date-fns";

export class UserData {
  constructor(public prop: user) {}
  get lastLoginTimeText() {
    if (!this.prop.last_login_time) return "-";
    return format(this.prop.last_login_time, "yyyy/M/d HH:mm:ss");
  }
}

export type user = {
  id: number;
  name: string;
  email: string;
  default_work_setting_id?: number;
  last_login_time?: Date;
  created_at: Date;
  updated_at: Date;
};
