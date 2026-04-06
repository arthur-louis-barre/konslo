import {Component, inject} from '@angular/core';
import {HabitService} from "../../../habit/habit.service";
import {ActivatedRoute, Router} from "@angular/router";
import {
    addMonths,
    endOfMonth,
    format,
    getDaysInMonth, isAfter,
    isSameDay,
    isSameMonth,
    startOfMonth,
    subMonths
} from "date-fns";

interface CalendarDay {
    date: Date;
    isToday: boolean;
    isSelectedDay: boolean;
    isSelectedMonth: boolean;
    hasActivity: boolean;
}

@Component({
    selector: 'k-calendar',
    imports: [],
    templateUrl: './calendar.component.html',
    styleUrl: './calendar.component.css',
})
export class CalendarComponent {
    private router = inject(Router);
    private route = inject(ActivatedRoute);
    private habitService = inject(HabitService);

    private isInit: boolean = false;
    private today: Date = new Date();
    private selectedDate: Date = new Date();
    protected calendarDays: CalendarDay[] = [];

    // LIFE-CYCLE

    ngOnInit(): void {
        this.route.queryParamMap.subscribe((params) => {
            const dateParam = params.get('date');

            const prevSelectedDate = this.selectedDate;
            this.selectedDate = dateParam ? new Date(dateParam) : new Date();

            // the calendar need to be built if the component is being initialized, or if the month has changed
            const needBuild =
                !this.isInit ||
                !isSameMonth(prevSelectedDate, this.selectedDate);

            if (needBuild) {
                this.isInit = true;
                this.buildCalendar();
            } else {
                this.updateSelectedDay();
            }
        })
    }

    buildCalendar(): void {
        const nbDaysSelectedMonth = getDaysInMonth(this.selectedDate);
        const firstDaySelectedMonth = (startOfMonth(this.selectedDate).getDay() + 6) % 7;
        const lastDaySelectedMonth = (endOfMonth(this.selectedDate).getDay() + 6) % 7;

        const nbDaysPreviousMonth = getDaysInMonth(subMonths(this.selectedDate, 1));

        const previousMonthDays: CalendarDay[] = Array.from({length: firstDaySelectedMonth}, (_, i) => {
            return {
                date: new Date(
                    this.selectedDate.getFullYear(),
                    this.selectedDate.getMonth() - 1,
                    nbDaysPreviousMonth - firstDaySelectedMonth + i + 1,
                ),
                isToday: false,
                isSelectedMonth: false,
                isSelectedDay: false,
                hasActivity: false
            }
        });

        const monthDays: CalendarDay[] = Array.from({length: nbDaysSelectedMonth}, (_, i) => {
            const day = new Date(this.selectedDate.getFullYear(), this.selectedDate.getMonth(), i + 1);
            return {
                date: day,
                isToday: isSameDay(day, this.today),
                isSelectedMonth: true,
                isSelectedDay: isSameDay(day, this.selectedDate),
                hasActivity: false
            };
        });

        const nextMonthDays: CalendarDay[] = Array.from({length: (7 - (lastDaySelectedMonth + 1)) % 7}, (_, i) => {
            return {
                date: new Date(
                    this.selectedDate.getFullYear(),
                    this.selectedDate.getMonth() + 1,
                    i + 1,
                ),
                isToday: false,
                isSelectedMonth: false,
                isSelectedDay: false,
                hasActivity: false
            }
        });

        this.calendarDays = [...previousMonthDays, ...monthDays, ...nextMonthDays]
        this.loadActivity(
            format(startOfMonth(this.selectedDate), 'yyyy-MM-dd'),
            format(endOfMonth(this.selectedDate), 'yyyy-MM-dd')
        )
    }

    loadActivity(from: string, to: string): void {
        this.habitService
            .getActivity(from, to)
            .subscribe({
                next: (activity) => {
                    const activitySet = new Set(activity.dates);

                    for (let day of this.calendarDays) {
                        if (!day.isSelectedMonth) continue;
                        day.hasActivity = activitySet.has(format(day.date, 'yyyy-MM-dd'));
                    }
                },
                error: (err) => { console.error(err); }
            })
    }

    updateSelectedDay(): void {
        const prev = this.calendarDays.find(d => d.isSelectedDay);
        if (prev) prev.isSelectedDay = false;

        const next = this.calendarDays.find(d => isSameDay(d.date, this.selectedDate));
        if (next) next.isSelectedDay = true;
    }

    // GETTERS

    get monthLabel(): string {
        return format(this.selectedDate, 'MMMM');
    }

    get year(): string {
        return format(this.selectedDate, 'yyyy');
    }

    dayNumber(date: Date): string {
        return format(date, 'd');
    }

    // CLICK-HANDLERS

    goToMonth(offset: -1 | 1): void {
        let targetDate = addMonths(this.selectedDate, offset);

        if (isAfter(startOfMonth(targetDate), startOfMonth(this.today))) return;

        if (offset === -1) {
            targetDate = endOfMonth(targetDate);
        } else {
            targetDate = startOfMonth(targetDate);
        }

        this.router.navigate(["/habits"], {queryParams: {date: format(targetDate, 'yyyy-MM-dd')}});
    }

    onDayClick(day: CalendarDay) {
        if (day.isSelectedDay || !day.isSelectedMonth) return;
        if (day.date > this.today) return;

        this.router.navigate(['/habits'], {queryParams: {date: format(day.date, "yyyy-MM-dd")}});
    }
}