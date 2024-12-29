export class workTimeData {
  constructor(public prop: workTime) {}
}

export type workTime = {
  id: number;
  targetDay: string;
  start?: Date;
  end?: Date;
  restStart?: Date;
  restEnd?: Date;
  memo?: string;
  createdAt: Date;
  updatedAt: Date;
};
