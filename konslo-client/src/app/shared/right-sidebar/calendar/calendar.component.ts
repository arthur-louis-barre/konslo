import {Component, inject} from '@angular/core';
import {HabitService} from "../../../habit/habit.service";
import {ActivityResponse} from "../../../habit/habit.model";
import {ActivatedRoute, Router} from "@angular/router";

interface CalendarDay {
    date: number;
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
    private initialized = false

    private habitService = inject(HabitService);

    private currentDate: Date = new Date();
    protected selectedYear: number = this.currentDate.getFullYear();
    private selectedMonth: number = this.currentDate.getMonth();
    private selectedDate: number = this.currentDate.getDate();

    protected days: CalendarDay[] = [];

    ngOnInit() {
        this.route.queryParamMap.subscribe((params) => {
            const date = params.get('date') ?? undefined;

            const date_parsed = date === undefined ? new Date() : new Date(date);

            this.selectedDate = date_parsed.getDate();

            const date_parsed_month = date_parsed.getMonth();
            const date_parsed_year = date_parsed.getFullYear();
            const isSameMonth = date_parsed_month === this.selectedMonth;
            const isSameYear = date_parsed_year === this.selectedYear;
            this.selectedMonth = date_parsed_month;
            this.selectedYear = date_parsed_year;

            if (!this.initialized || !isSameMonth || !isSameYear) {
                this.initialized = true;
                const { from, to } = this.getMonthRange();
                this.loadActivity(from, to);
            } else {
                this.buildCalendar();
            }
        });
    }

    buildCalendar() {
        let nbOfDaysLastMonth = this.countNbOfDays(new Date(this.selectedYear, this.selectedMonth - 1));
        let nbOfDaysSelectedMonth = this.countNbOfDays(new Date(this.selectedYear, this.selectedMonth));

        let firstDayOfSelectedMonth = this.firstDay(new Date(this.selectedYear, this.selectedMonth));
        let lastDayOfSelectedMonth = this.lastDay(new Date(this.selectedYear, this.selectedMonth));

        const isCurrentMonth =
            this.selectedYear === this.currentDate.getFullYear() &&
            this.selectedMonth === this.currentDate.getMonth();

        const lastMonthDays: CalendarDay[] = Array.from({length: firstDayOfSelectedMonth}, (_, i) => ({
            date: nbOfDaysLastMonth - firstDayOfSelectedMonth + i + 1,
            isToday: false,
            isSelectedMonth: false,
            isSelectedDay: false,
            hasActivity: false
        }));

        const selectedMonthDays: CalendarDay[] = Array.from({length: nbOfDaysSelectedMonth}, (_, i) => ({
            date: i + 1,
            isToday: isCurrentMonth && (i + 1 === this.currentDate.getDate()),
            isSelectedMonth: true,
            isSelectedDay: i + 1 === this.selectedDate,
            hasActivity: false
        }));

        const nextMonthDays: CalendarDay[] = Array.from({length: (7 - (lastDayOfSelectedMonth + 1)) % 7}, (_, i) => ({
            date: i + 1,
            isToday: false,
            isSelectedMonth: false,
            isSelectedDay: false,
            hasActivity: false
        }));

        this.days = [...lastMonthDays, ...selectedMonthDays, ...nextMonthDays]
    }

    countNbOfDays(date: Date): number {
        return new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate();
    }

    firstDay(date: Date): number {
        let day = new Date(date.getFullYear(), date.getMonth(), 1).getDay();

        // start of the week should be Monday, not Sunday
        day = (day + 6) % 7;

        return day
    }

    lastDay(date: Date): number {
        let day = new Date(date.getFullYear(), date.getMonth() + 1, 0).getDay();

        // start of the week should be Monday, not Sunday
        day = (day + 6) % 7;

        return day
    }

    goToMonth(offset: -1 | 1): void {
        this.selectedMonth += offset;

        if (this.selectedMonth < 0) {
            this.selectedMonth = 11;
            this.selectedYear -= 1;
        }

        if (this.selectedMonth > 11) {
            this.selectedMonth = 0;
            this.selectedYear += 1;
        }

        const {from, to} = this.getMonthRange();
        this.loadActivity(from, to);
    }

    loadActivity(from: string, to: string): void {
        this.buildCalendar();

        this.habitService
            .getActivity(from, to)
            .subscribe((activity) => {
                this.patchCalendar(activity)
            })
    }

    patchCalendar(activity: ActivityResponse) {
        const activitySet = new Set(activity.dates);
        const month = String(this.selectedMonth + 1).padStart(2, '0');

        for (let day of this.days) {
            if (!day.isSelectedMonth) continue;

            const date = `${this.selectedYear}-${month}-${String(day.date).padStart(2, '0')}`;

            day.hasActivity = activitySet.has(date);
        }
    }

    private getMonthRange(): { from: string; to: string } {
        const month = String(this.selectedMonth + 1).padStart(2, '0');
        const lastDay = this.countNbOfDays(new Date(this.selectedYear, this.selectedMonth));
        return {
            from: `${this.selectedYear}-${month}-01`,
            to: `${this.selectedYear}-${month}-${String(lastDay).padStart(2, '0')}`
        };
    }

    get monthLabel(): string {
        return new Date(this.selectedYear, this.selectedMonth)
            .toLocaleString('en-US', {month: 'long'});
    }

    onDayClick(day: CalendarDay) {
        if (!day.isSelectedMonth || day.isSelectedDay) return;

        const padded_month = String(this.selectedMonth + 1).padStart(2, '0');
        const padded_date = String(day.date).padStart(2, '0');
        const date = new Date(`${this.selectedYear}-${padded_month}-${padded_date}`).toISOString()

        this.router.navigate(['/habits'], {queryParams: {date}});
    }


}
