import moment from 'moment';


export function toDate(date: number | Date): Date {
  if (date instanceof Date) return date;
  return new Date(date * 1000);
}

export function toFromNowFormattedDate(date: number | Date): string {
  return moment(toDate(date)).fromNow();
}