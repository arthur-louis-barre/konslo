import {Component, HostListener, inject} from '@angular/core';
import {HabitService} from '../habit.service';
import {GoalPeriod, HabitWithCheck} from '../habit.model';
import {HabitCardComponent} from "./habit-card/habit-card.component";
import {ActivatedRoute, Router, RouterLink} from "@angular/router";
import {takeUntilDestroyed} from "@angular/core/rxjs-interop";

@Component({
    selector: 'k-habit-list',
    imports: [
        HabitCardComponent,
        RouterLink
    ],
    templateUrl: './habit-list.component.html',
    styleUrl: './habit-list.component.css'
})
export class HabitListComponent {
    private router = inject(Router);
    private route = inject(ActivatedRoute);
    private habitsService = inject(HabitService);

    protected selectedDate: string | undefined;
    protected habits: HabitWithCheck[] = [];
    protected openDropdownId: string | null = null;

    constructor() {
        this.route.queryParamMap
            .pipe(takeUntilDestroyed())
            .subscribe((params) => {
                const date = params.get('date') ?? undefined;
                this.selectedDate = date;
                this.loadHabits(date);
            });
    }

    deleteHabit(id: string) {
        this.habitsService.deleteHabit(id).subscribe({
            next: () => { this.loadHabits(this.selectedDate); },
            error: (err) => { console.error(err); }
        });
    }

    loadHabits(date?: string) {
        this.habitsService.getHabits(date).subscribe({
            next: (habits) => { this.habits = habits; },
            error: (err) => { console.error(err); }
        });
    }

    toggleDropdown(id: string) {
        this.openDropdownId = this.openDropdownId === id ? null : id;
    }

    // closes dropdown on outside click
    @HostListener('document:click', ['$event'])
    closeDropdown(event: MouseEvent) {
        if ((event.target as Element).closest('.h-card') == null) {
            this.openDropdownId = null;
        }
    }

    // GETTERS

    get habitsByPeriod() {
        const periods: GoalPeriod[] = ['day', 'week', 'month'];
        const periodLabels: Record<GoalPeriod, string> = {
            day: 'Daily',
            week: 'Weekly',
            month: 'Monthly'
        };

        return periods
            .map(period => ({
                period,
                label: periodLabels[period],
                habits: this.habits.filter(h => h.goal_period === period)
            }))
            .filter(period => period.habits.length > 0)
    }

    get greeting() {
        const hour = new Date().getHours();
        if (hour < 6) return "Good night"
        else if (hour < 12) return "Good morning"
        else if (hour < 18) return "Good afternoon"
        else return "Good evening"
    }

    // NAVIGATION

    goToNewHabit() {
        this.router.navigate(['/new-habit']);
    }
}